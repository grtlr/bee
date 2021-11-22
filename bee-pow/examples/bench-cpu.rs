// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    thread::{self},
    time::{self, Duration},
};

use bee_pow::providers::miner::MinerCancel;
use bee_ternary::{
    b1t6::{self},
    Btrit, T1B1Buf, TritBuf,
};
use crypto::hashes::ternary::{
    curl_p::{CurlPBatchHasher, BATCH_SIZE},
    HASH_LENGTH,
};
use structopt::StructOpt;
use thiserror::Error;

const PRINT_STATUS_INTERVAL: Duration = Duration::from_secs(2);
const DURATION: Duration = Duration::from_secs(60);

#[derive(Debug, Error)]
pub enum BenchmarkCPUError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, StructOpt)]
#[structopt(name = env!("CARGO_PKG_NAME"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Opt {
    #[structopt(short = "t", long = "threads")]
    threads: Option<usize>,
}

pub fn main() {
    let opt = Opt::from_args();

    let threads = match opt.threads {
        Some(threads) => threads,
        None => num_cpus::get(),
    };

    println!("Benchmarking CPU with {} threads", threads);

    let cancel = MinerCancel::new();
    let cancel_2 = cancel.clone();
    let cancel_3 = cancel.clone();
    let counter = Arc::new(AtomicU64::new(0));
    let counter_2 = counter.clone();

    let time_start = std::time::Instant::now();

    // TODO: Use `bee-test`
    let pow_digest: [u8; 32] = rand::random();

    let mut workers = Vec::with_capacity(threads + 2);

    // Stop if the timeout has exceeded
    let time_thread = thread::spawn(move || {
        std::thread::sleep(DURATION);
        cancel.trigger();
    });

    let process_thread = thread::spawn(move || {
        while !cancel_2.is_cancelled() {
            std::thread::sleep(PRINT_STATUS_INTERVAL);

            let elapsed = time_start.elapsed();
            let (percentage, remaining) =
                estimate_remaining_time(time_start, elapsed.as_millis() as i64, DURATION.as_millis() as i64);
            let megahashes_per_second =
                counter.load(Ordering::Relaxed) as f64 / (elapsed.as_secs_f64() * 1_000_000 as f64);
            println!(
                "Average CPU speed: {:.2}MH/s ({} thread(s), {:.2}%. {:.2?} left...)",
                megahashes_per_second, threads, percentage, remaining
            );
        }
    });

    let worker_width = u64::MAX / threads as u64;
    for i in 0..threads {
        let start_nonce = i as u64 * worker_width;
        let benchmark_cancel = cancel_3.clone();
        let benchmark_counter = counter_2.clone();
        let _pow_digest = pow_digest.clone();

        workers.push(thread::spawn(move || {
            cpu_benchmark_worker(&pow_digest, start_nonce, benchmark_cancel, benchmark_counter)
        }));
    }

    workers.push(process_thread);
    workers.push(time_thread);

    for worker in workers {
        worker.join().expect("Couldn't stop thread");
    }

    let megahashes_per_second = counter_2.load(Ordering::Relaxed) as f64 / (DURATION.as_secs_f64() * 1_000_000 as f64);
    println!(
        "Average CPU speed: {:.2}MH/s ({} thread(s), took {:.2?})",
        megahashes_per_second, threads, DURATION
    );
}

fn cpu_benchmark_worker(_pow_digest: &[u8], start_nonce: u64, cancel: MinerCancel, counter: Arc<AtomicU64>) {
    let mut pow_digest = TritBuf::<T1B1Buf>::with_capacity(HASH_LENGTH);
    b1t6::encode::<T1B1Buf>(&_pow_digest)
        .iter()
        .for_each(|t| pow_digest.push(t));

    let mut nonce = start_nonce;
    let mut hasher = CurlPBatchHasher::<T1B1Buf>::new(HASH_LENGTH);
    let mut buffers = Vec::<TritBuf<T1B1Buf>>::with_capacity(BATCH_SIZE);

    for _ in 0..BATCH_SIZE {
        let mut buffer = TritBuf::<T1B1Buf>::zeros(HASH_LENGTH);
        buffer[..pow_digest.len()].copy_from(&pow_digest);
        buffers.push(buffer);
    }

    while !cancel.is_cancelled() {
        for (i, buffer) in buffers.iter_mut().enumerate() {
            let nonce_trits = b1t6::encode::<T1B1Buf>(&(nonce + i as u64).to_le_bytes());
            buffer[pow_digest.len()..pow_digest.len() + nonce_trits.len()].copy_from(&nonce_trits);
            hasher.add(buffer.clone());
        }

        for (_i, hash) in hasher.hash().enumerate() {
            let _trailing_zeros = hash.iter().rev().take_while(|t| *t == Btrit::Zero).count();
            counter.fetch_add(BATCH_SIZE as u64, Ordering::Release);
        }

        nonce += BATCH_SIZE as u64;
    }
}

// Calculates the remaining time for a running operation and returns the finished percentage.
fn estimate_remaining_time(time_start: std::time::Instant, current: i64, total: i64) -> (f64, std::time::Duration) {
    let ratio = current as f64 / total as f64;
    let total_time = time::Duration::from_secs_f64(time_start.elapsed().as_secs_f64() / ratio);
    let time_now = std::time::Instant::now();
    if time_now > (time_start + total_time) {
        return (100.0, Duration::from_secs(0));
    }
    let remaining = (time_start + total_time).duration_since(time_now);
    return (ratio * 100.0, remaining);
}

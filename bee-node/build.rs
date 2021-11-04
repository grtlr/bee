// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::process::Command;
use std::fs;

#[derive(Debug)]
enum BuildError {
    GitCommit,
    GitBranch,
}

// TODO: cleanup!
fn fail_on_empty_directory(dir: &str) {
    if fs::read_dir(dir).unwrap().count() == 0 {
        println!(
            "The `{}` directory is empty, did you forget to pull the submodules?",
            dir
        );
        println!("Try `git submodule update --init --recursive`");
        panic!();
    }
}

fn main() -> Result<(), BuildError> {
    match Command::new("git").args(&["rev-parse", "HEAD"]).output() {
        Ok(output) => {
            println!(
                "cargo:rustc-env=GIT_COMMIT={}",
                String::from_utf8(output.stdout).unwrap()
            );
        }
        Err(_) => return Err(BuildError::GitCommit),
    }

    match Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
    {
        Ok(output) => {
            println!("cargo:rerun-if-changed=../.git/HEAD");
            println!(
                "cargo:rerun-if-changed=../.git/refs/heads/{}",
                String::from_utf8(output.stdout).unwrap(),
            );
        }
        Err(_) => return Err(BuildError::GitBranch),
    }

    if cfg!(feature = "dashboard") {
        fail_on_empty_directory("bee-node/src/plugins/dashboard/frontend");
    }

    Ok(())
}

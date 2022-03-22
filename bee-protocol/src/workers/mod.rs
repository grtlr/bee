// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod config;
pub mod event;
pub mod storage;

mod broadcaster;
mod heartbeater;
mod index_updater;
mod message;
mod metrics;
mod mps;
mod packets;
mod peer;
mod propagator;
mod requester;
mod responder;
mod sender;
mod solidifier;
mod status;

use bee_autopeering::event::EventRx as AutopeeringEventRx;
use bee_gossip::NetworkEventReceiver as NetworkEventRx;
use bee_runtime::node::{Node, NodeBuilder};
pub(crate) use self::broadcaster::{BroadcasterWorker, BroadcasterWorkerEvent};
pub(crate) use self::heartbeater::HeartbeaterWorker;
pub(crate) use self::index_updater::{IndexUpdaterWorker, IndexUpdaterWorkerEvent};
pub(crate) use self::message::{
    HasherWorker, HasherWorkerEvent, IndexationPayloadWorker, IndexationPayloadWorkerEvent, MilestonePayloadWorker,
    PayloadWorker, PayloadWorkerEvent, ProcessorWorker, TransactionPayloadWorker, UnreferencedMessageInserterWorker,
    UnreferencedMessageInserterWorkerEvent,
};
pub use self::message::{MessageSubmitterError, MessageSubmitterWorker, MessageSubmitterWorkerEvent};
pub use self::metrics::MetricsWorker;
pub(crate) use self::mps::MpsWorker;
use self::peer::PeerManagerConfig;
pub use self::peer::{PeerManager, PeerManagerResWorker};
pub(crate) use self::peer::{PeerManagerWorker, PeerWorker};
pub(crate) use self::propagator::{PropagatorWorker, PropagatorWorkerEvent};
pub use self::requester::{request_message, MessageRequesterWorker, RequestedMessages, RequestedMilestones};
pub(crate) use self::requester::{MilestoneRequesterWorker, MilestoneRequesterWorkerEvent};
pub(crate) use self::responder::{
    MessageResponderWorker, MessageResponderWorkerEvent, MilestoneResponderWorker, MilestoneResponderWorkerEvent,
};
pub(crate) use self::solidifier::{MilestoneSolidifierWorker, MilestoneSolidifierWorkerEvent};
pub(crate) use self::status::StatusWorker;

pub fn init<N: Node>(
    config: config::ProtocolConfig,
    network_id: (String, u64),
    network_events: NetworkEventRx,
    autopeering_events: Option<AutopeeringEventRx>,
    node_builder: N::Builder,
) -> N::Builder
where
    N::Backend: storage::StorageBackend,
{
    node_builder
        .with_worker::<MetricsWorker>()
        .with_worker::<PeerManagerResWorker>()
        .with_worker_cfg::<PeerManagerWorker>(PeerManagerConfig {
            network_rx: network_events,
            peering_rx: autopeering_events,
            network_name: network_id.0,
        })
        .with_worker_cfg::<HasherWorker>(config.clone())
        .with_worker_cfg::<ProcessorWorker>(network_id.1)
        .with_worker::<MessageResponderWorker>()
        .with_worker::<MilestoneResponderWorker>()
        .with_worker::<MessageRequesterWorker>()
        .with_worker::<MilestoneRequesterWorker>()
        .with_worker::<PayloadWorker>()
        .with_worker::<TransactionPayloadWorker>()
        .with_worker_cfg::<MilestonePayloadWorker>(config.clone())
        .with_worker::<IndexationPayloadWorker>()
        .with_worker::<PayloadWorker>()
        .with_worker::<BroadcasterWorker>()
        .with_worker::<PropagatorWorker>()
        .with_worker::<MpsWorker>()
        .with_worker_cfg::<MilestoneSolidifierWorker>(config.workers.milestone_sync_count)
        .with_worker::<IndexUpdaterWorker>()
        .with_worker_cfg::<StatusWorker>(config.workers.status_interval)
        .with_worker::<HeartbeaterWorker>()
        .with_worker::<MessageSubmitterWorker>()
        .with_worker::<UnreferencedMessageInserterWorker>()
}

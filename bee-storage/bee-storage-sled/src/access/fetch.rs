// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Fetch access operations.

use crate::{storage::Storage, trees::*};

use bee_ledger::types::{
    snapshot::info::SnapshotInfo, ConsumedOutput, CreatedOutput, LedgerIndex, OutputDiff, Receipt, TreasuryOutput,
};
use bee_message::{
    address::Ed25519Address,
    milestone::{Milestone, MilestoneIndex},
    output::OutputId,
    Message, MessageId,
};
use bee_storage::{access::Fetch, backend::StorageBackend, system::System};
use bee_tangle::{
    metadata::MessageMetadata, solid_entry_point::SolidEntryPoint, unreferenced_message::UnreferencedMessage,
};

use packable::PackableExt;

impl Fetch<u8, System> for Storage {
    fn fetch(&self, &key: &u8) -> Result<Option<System>, <Self as StorageBackend>::Error> {
        Ok(self
            .inner
            .get(&[key])?
            // Unpacking from storage is fine.
            .map(|v| System::unpack_unverified(&mut v.as_ref()).unwrap()))
    }
}

impl Fetch<MessageId, Message> for Storage {
    fn fetch(&self, message_id: &MessageId) -> Result<Option<Message>, <Self as StorageBackend>::Error> {
        Ok(self
            .inner
            .open_tree(TREE_MESSAGE_ID_TO_MESSAGE)?
            .get(message_id)?
            // Unpacking from storage is fine.
            .map(|v| Message::unpack_unverified(&mut v.as_ref()).unwrap()))
    }
}

impl Fetch<MessageId, MessageMetadata> for Storage {
    fn fetch(&self, message_id: &MessageId) -> Result<Option<MessageMetadata>, <Self as StorageBackend>::Error> {
        Ok(self
            .inner
            .open_tree(TREE_MESSAGE_ID_TO_METADATA)?
            .get(message_id)?
            // Unpacking from storage is fine.
            .map(|v| MessageMetadata::unpack_unverified(&mut v.as_ref()).unwrap()))
    }
}

impl Fetch<MessageId, Vec<MessageId>> for Storage {
    fn fetch(&self, parent: &MessageId) -> Result<Option<Vec<MessageId>>, <Self as StorageBackend>::Error> {
        Ok(Some(
            self.inner
                .open_tree(TREE_MESSAGE_ID_TO_MESSAGE_ID)?
                .scan_prefix(parent)
                .map(|result| {
                    let (key, _) = result?;
                    let (_, child) = key.split_at(MessageId::LENGTH);
                    // Unpacking from storage is fine.
                    let child: [u8; MessageId::LENGTH] = child.try_into().unwrap();
                    Ok(MessageId::from(child))
                })
                .take(self.config.storage.fetch_edge_limit)
                .collect::<Result<Vec<MessageId>, Self::Error>>()?,
        ))
    }
}

impl Fetch<OutputId, CreatedOutput> for Storage {
    fn fetch(&self, output_id: &OutputId) -> Result<Option<CreatedOutput>, <Self as StorageBackend>::Error> {
        Ok(self
            .inner
            .open_tree(TREE_OUTPUT_ID_TO_CREATED_OUTPUT)?
            .get(output_id.pack_to_vec())?
            // Unpacking from storage is fine.
            .map(|v| CreatedOutput::unpack_unverified(&mut v.as_ref()).unwrap()))
    }
}

impl Fetch<OutputId, ConsumedOutput> for Storage {
    fn fetch(&self, output_id: &OutputId) -> Result<Option<ConsumedOutput>, <Self as StorageBackend>::Error> {
        Ok(self
            .inner
            .open_tree(TREE_OUTPUT_ID_TO_CONSUMED_OUTPUT)?
            .get(output_id.pack_to_vec())?
            // Unpacking from storage is fine.
            .map(|v| ConsumedOutput::unpack_unverified(&mut v.as_ref()).unwrap()))
    }
}

impl Fetch<Ed25519Address, Vec<OutputId>> for Storage {
    fn fetch(&self, address: &Ed25519Address) -> Result<Option<Vec<OutputId>>, <Self as StorageBackend>::Error> {
        Ok(Some(
            self.inner
                .open_tree(TREE_ED25519_ADDRESS_TO_OUTPUT_ID)?
                .scan_prefix(address)
                .map(|result| {
                    let (key, _) = result?;
                    let (_, output_id) = key.split_at(Ed25519Address::LENGTH);
                    // Unpacking from storage is fine.
                    Ok((<[u8; OutputId::LENGTH]>::try_from(output_id).unwrap())
                        .try_into()
                        .unwrap())
                })
                .take(self.config.storage.fetch_output_id_limit)
                .collect::<Result<Vec<OutputId>, Self::Error>>()?,
        ))
    }
}

impl Fetch<(), LedgerIndex> for Storage {
    fn fetch(&self, (): &()) -> Result<Option<LedgerIndex>, <Self as StorageBackend>::Error> {
        Ok(self
            .inner
            .open_tree(TREE_LEDGER_INDEX)?
            .get([0x00u8])?
            // Unpacking from storage is fine.
            .map(|v| LedgerIndex::unpack_unverified(&mut v.as_ref()).unwrap()))
    }
}

impl Fetch<MilestoneIndex, Milestone> for Storage {
    fn fetch(&self, index: &MilestoneIndex) -> Result<Option<Milestone>, <Self as StorageBackend>::Error> {
        Ok(self
            .inner
            .open_tree(TREE_MILESTONE_INDEX_TO_MILESTONE)?
            .get(index.pack_to_vec())?
            // Unpacking from storage is fine.
            .map(|v| Milestone::unpack_unverified(&mut v.as_ref()).unwrap()))
    }
}

impl Fetch<(), SnapshotInfo> for Storage {
    fn fetch(&self, (): &()) -> Result<Option<SnapshotInfo>, <Self as StorageBackend>::Error> {
        Ok(self
            .inner
            .open_tree(TREE_SNAPSHOT_INFO)?
            .get([0x00u8])?
            // Unpacking from storage is fine.
            .map(|v| SnapshotInfo::unpack_unverified(&mut v.as_ref()).unwrap()))
    }
}

impl Fetch<SolidEntryPoint, MilestoneIndex> for Storage {
    fn fetch(&self, sep: &SolidEntryPoint) -> Result<Option<MilestoneIndex>, <Self as StorageBackend>::Error> {
        Ok(self
            .inner
            .open_tree(TREE_SOLID_ENTRY_POINT_TO_MILESTONE_INDEX)?
            .get(sep.as_ref())?
            // Unpacking from storage is fine.
            .map(|v| MilestoneIndex::unpack_unverified(&mut v.as_ref()).unwrap()))
    }
}

impl Fetch<MilestoneIndex, OutputDiff> for Storage {
    fn fetch(&self, index: &MilestoneIndex) -> Result<Option<OutputDiff>, <Self as StorageBackend>::Error> {
        Ok(self
            .inner
            .open_tree(TREE_MILESTONE_INDEX_TO_OUTPUT_DIFF)?
            .get(index.pack_to_vec())?
            // Unpacking from storage is fine.
            .map(|v| OutputDiff::unpack_unverified(&mut v.as_ref()).unwrap()))
    }
}

impl Fetch<MilestoneIndex, Vec<UnreferencedMessage>> for Storage {
    fn fetch(
        &self,
        index: &MilestoneIndex,
    ) -> Result<Option<Vec<UnreferencedMessage>>, <Self as StorageBackend>::Error> {
        Ok(Some(
            self.inner
                .open_tree(TREE_MILESTONE_INDEX_TO_UNREFERENCED_MESSAGE)?
                .scan_prefix(index.pack_to_vec())
                .map(|result| {
                    let (key, _) = result?;
                    let (_, unreferenced_message) = key.split_at(std::mem::size_of::<MilestoneIndex>());
                    // Unpacking from storage is fine.
                    let unreferenced_message: [u8; MessageId::LENGTH] = unreferenced_message.try_into().unwrap();
                    Ok(UnreferencedMessage::from(MessageId::from(unreferenced_message)))
                })
                .collect::<Result<Vec<UnreferencedMessage>, Self::Error>>()?,
        ))
    }
}

impl Fetch<MilestoneIndex, Vec<Receipt>> for Storage {
    fn fetch(&self, index: &MilestoneIndex) -> Result<Option<Vec<Receipt>>, <Self as StorageBackend>::Error> {
        Ok(Some(
            self.inner
                .open_tree(TREE_MILESTONE_INDEX_TO_RECEIPT)?
                .scan_prefix(index.pack_to_vec())
                .map(|result| {
                    let (mut key, _) = result?;
                    let (_, receipt) = key.split_at_mut(std::mem::size_of::<MilestoneIndex>());
                    // Unpacking from storage is fine.
                    #[allow(clippy::useless_asref)]
                    Ok(Receipt::unpack_unverified(&mut receipt.as_ref()).unwrap())
                })
                .collect::<Result<Vec<Receipt>, Self::Error>>()?,
        ))
    }
}

impl Fetch<bool, Vec<TreasuryOutput>> for Storage {
    fn fetch(&self, spent: &bool) -> Result<Option<Vec<TreasuryOutput>>, <Self as StorageBackend>::Error> {
        Ok(Some(
            self.inner
                .open_tree(TREE_SPENT_TO_TREASURY_OUTPUT)?
                .scan_prefix(spent.pack_to_vec())
                .map(|result| {
                    let (mut key, _) = result?;
                    let (_, output) = key.split_at_mut(std::mem::size_of::<bool>());
                    // Unpacking from storage is fine.
                    #[allow(clippy::useless_asref)]
                    Ok(TreasuryOutput::unpack_unverified(&mut output.as_ref()).unwrap())
                })
                .collect::<Result<Vec<TreasuryOutput>, Self::Error>>()?,
        ))
    }
}

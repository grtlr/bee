// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::rand::{
    address::rand_address,
    bytes::rand_bytes,
    milestone::rand_milestone_index,
    number::{rand_number, rand_number_range},
};

use bee_message::output::feature_block::{
    DustDepositReturnFeatureBlock, ExpirationMilestoneIndexFeatureBlock, ExpirationUnixFeatureBlock, FeatureBlock,
    FeatureBlockFlags, IndexationFeatureBlock, IssuerFeatureBlock, MetadataFeatureBlock, SenderFeatureBlock,
    TimelockMilestoneIndexFeatureBlock, TimelockUnixFeatureBlock,
};

/// Generates a random [`SenderFeatureBlock`].
pub fn rand_sender_feature_block() -> SenderFeatureBlock {
    SenderFeatureBlock::new(rand_address())
}

/// Generates a random [`IssuerFeatureBlock`].
pub fn rand_issuer_feature_block() -> IssuerFeatureBlock {
    IssuerFeatureBlock::new(rand_address())
}

/// Generates a random [`DustDepositReturnFeatureBlock`].
pub fn rand_dust_deposit_return_feature_block() -> DustDepositReturnFeatureBlock {
    DustDepositReturnFeatureBlock::new(rand_number_range(DustDepositReturnFeatureBlock::AMOUNT_RANGE)).unwrap()
}

/// Generates a random [`TimelockMilestoneIndexFeatureBlock`].
pub fn rand_timelock_milestone_index_feature_block() -> TimelockMilestoneIndexFeatureBlock {
    TimelockMilestoneIndexFeatureBlock::new(rand_milestone_index())
}

/// Generates a random [`TimelockUnixFeatureBlock`].
pub fn rand_timelock_unix_feature_block() -> TimelockUnixFeatureBlock {
    TimelockUnixFeatureBlock::new(rand_number())
}

/// Generates a random [`ExpirationMilestoneIndexFeatureBlock`].
pub fn rand_expiration_milestone_index_feature_block() -> ExpirationMilestoneIndexFeatureBlock {
    ExpirationMilestoneIndexFeatureBlock::new(rand_milestone_index())
}

/// Generates a random [`ExpirationUnixFeatureBlock`].
pub fn rand_expiration_unix_feature_block() -> ExpirationUnixFeatureBlock {
    ExpirationUnixFeatureBlock::new(rand_number())
}

/// Generates a random [`MetadataFeatureBlock`].
pub fn rand_metadata_feature_block() -> MetadataFeatureBlock {
    let bytes = rand_bytes(rand_number_range(MetadataFeatureBlock::LENGTH_RANGE) as usize);
    MetadataFeatureBlock::new(bytes).unwrap()
}

/// Generates a random [`IndexationFeatureBlock`].
pub fn rand_indexation_feature_block() -> IndexationFeatureBlock {
    let bytes = rand_bytes(rand_number_range(IndexationFeatureBlock::LENGTH_RANGE) as usize);
    IndexationFeatureBlock::new(bytes).unwrap()
}

fn all_feature_blocks() -> Vec<FeatureBlock> {
    vec![
        FeatureBlock::Sender(rand_sender_feature_block()),
        FeatureBlock::Issuer(rand_issuer_feature_block()),
        FeatureBlock::DustDepositReturn(rand_dust_deposit_return_feature_block()),
        FeatureBlock::TimelockMilestoneIndex(rand_timelock_milestone_index_feature_block()),
        FeatureBlock::TimelockUnix(rand_timelock_unix_feature_block()),
        FeatureBlock::ExpirationMilestoneIndex(rand_expiration_milestone_index_feature_block()),
        FeatureBlock::ExpirationUnix(rand_expiration_unix_feature_block()),
        FeatureBlock::Metadata(rand_metadata_feature_block()),
        FeatureBlock::Indexation(rand_indexation_feature_block()),
    ]
}

/// Generates a [`Vec`] of random [`FeatureBlock`]s given a set of allowed [`FeatureBlockFlags`].
pub fn rand_allowed_feature_blocks(allowed_feature_blocks: FeatureBlockFlags) -> Vec<FeatureBlock> {
    let mut all_feature_blocks = all_feature_blocks();
    all_feature_blocks.retain(|feature_block| allowed_feature_blocks.contains(feature_block.flag()));
    all_feature_blocks
}

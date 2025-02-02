// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    address::Address,
    output::{
        feature_block::{verify_allowed_feature_blocks, FeatureBlock, FeatureBlockFlags, FeatureBlocks},
        unlock_condition::{
            verify_allowed_unlock_conditions, AddressUnlockCondition, UnlockCondition, UnlockConditionFlags,
            UnlockConditions,
        },
        NativeToken, NativeTokens, OutputAmount,
    },
    Error,
};

use packable::Packable;

use alloc::vec::Vec;

///
#[must_use]
pub struct BasicOutputBuilder {
    amount: OutputAmount,
    native_tokens: Vec<NativeToken>,
    unlock_conditions: Vec<UnlockCondition>,
    feature_blocks: Vec<FeatureBlock>,
}

impl BasicOutputBuilder {
    ///
    #[inline(always)]
    pub fn new(amount: u64) -> Result<Self, Error> {
        Ok(Self {
            amount: amount.try_into().map_err(Error::InvalidOutputAmount)?,
            native_tokens: Vec::new(),
            unlock_conditions: Vec::new(),
            feature_blocks: Vec::new(),
        })
    }

    ///
    #[inline(always)]
    pub fn add_native_token(mut self, native_token: NativeToken) -> Self {
        self.native_tokens.push(native_token);
        self
    }

    ///
    #[inline(always)]
    pub fn with_native_tokens(mut self, native_tokens: impl IntoIterator<Item = NativeToken>) -> Self {
        self.native_tokens = native_tokens.into_iter().collect();
        self
    }

    ///
    #[inline(always)]
    pub fn add_unlock_condition(mut self, unlock_condition: UnlockCondition) -> Self {
        self.unlock_conditions.push(unlock_condition);
        self
    }

    ///
    #[inline(always)]
    pub fn with_unlock_conditions(mut self, unlock_conditions: impl IntoIterator<Item = UnlockCondition>) -> Self {
        self.unlock_conditions = unlock_conditions.into_iter().collect();
        self
    }

    ///
    #[inline(always)]
    pub fn add_feature_block(mut self, feature_block: FeatureBlock) -> Self {
        self.feature_blocks.push(feature_block);
        self
    }

    ///
    #[inline(always)]
    pub fn with_feature_blocks(mut self, feature_blocks: impl IntoIterator<Item = FeatureBlock>) -> Self {
        self.feature_blocks = feature_blocks.into_iter().collect();
        self
    }

    ///
    pub fn finish(self) -> Result<BasicOutput, Error> {
        let unlock_conditions = UnlockConditions::new(self.unlock_conditions)?;

        verify_unlock_conditions::<true>(&unlock_conditions)?;

        let feature_blocks = FeatureBlocks::new(self.feature_blocks)?;

        verify_feature_blocks::<true>(&feature_blocks)?;

        Ok(BasicOutput {
            amount: self.amount,
            native_tokens: NativeTokens::new(self.native_tokens)?,
            unlock_conditions,
            feature_blocks,
        })
    }
}

/// Describes a basic output with optional features.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[packable(unpack_error = Error)]
pub struct BasicOutput {
    // Amount of IOTA tokens held by the output.
    #[packable(unpack_error_with = Error::InvalidOutputAmount)]
    amount: OutputAmount,
    // Native tokens held by the output.
    native_tokens: NativeTokens,
    #[packable(verify_with = verify_unlock_conditions)]
    unlock_conditions: UnlockConditions,
    #[packable(verify_with = verify_feature_blocks)]
    feature_blocks: FeatureBlocks,
}

impl BasicOutput {
    /// The [`Output`](crate::output::Output) kind of an [`BasicOutput`].
    pub const KIND: u8 = 3;

    /// The set of allowed [`UnlockCondition`]s for an [`BasicOutput`].
    const ALLOWED_UNLOCK_CONDITIONS: UnlockConditionFlags = UnlockConditionFlags::ADDRESS
        .union(UnlockConditionFlags::DUST_DEPOSIT_RETURN)
        .union(UnlockConditionFlags::TIMELOCK)
        .union(UnlockConditionFlags::EXPIRATION);
    /// The set of allowed [`FeatureBlock`]s for an [`BasicOutput`].
    pub const ALLOWED_FEATURE_BLOCKS: FeatureBlockFlags = FeatureBlockFlags::SENDER
        .union(FeatureBlockFlags::METADATA)
        .union(FeatureBlockFlags::TAG);

    /// Creates a new [`BasicOutput`].
    #[inline(always)]
    pub fn new(amount: u64) -> Result<Self, Error> {
        BasicOutputBuilder::new(amount)?.finish()
    }

    /// Creates a new [`BasicOutputBuilder`].
    #[inline(always)]
    pub fn build(amount: u64) -> Result<BasicOutputBuilder, Error> {
        BasicOutputBuilder::new(amount)
    }

    ///
    #[inline(always)]
    pub fn address(&self) -> &Address {
        // An BasicOutput must have an AddressUnlockCondition.
        if let UnlockCondition::Address(address) = self.unlock_conditions.get(AddressUnlockCondition::KIND).unwrap() {
            address.address()
        } else {
            unreachable!();
        }
    }

    ///
    #[inline(always)]
    pub fn amount(&self) -> u64 {
        self.amount.get()
    }

    ///
    #[inline(always)]
    pub fn native_tokens(&self) -> &[NativeToken] {
        &self.native_tokens
    }

    ///
    #[inline(always)]
    pub fn unlock_conditions(&self) -> &[UnlockCondition] {
        &self.unlock_conditions
    }

    ///
    #[inline(always)]
    pub fn feature_blocks(&self) -> &[FeatureBlock] {
        &self.feature_blocks
    }
}

fn verify_unlock_conditions<const VERIFY: bool>(unlock_conditions: &UnlockConditions) -> Result<(), Error> {
    if VERIFY {
        if unlock_conditions.get(AddressUnlockCondition::KIND).is_none() {
            Err(Error::MissingAddressUnlockCondition)
        } else {
            verify_allowed_unlock_conditions(unlock_conditions, BasicOutput::ALLOWED_UNLOCK_CONDITIONS)
        }
    } else {
        Ok(())
    }
}

fn verify_feature_blocks<const VERIFY: bool>(blocks: &FeatureBlocks) -> Result<(), Error> {
    if VERIFY {
        verify_allowed_feature_blocks(blocks, BasicOutput::ALLOWED_FEATURE_BLOCKS)
    } else {
        Ok(())
    }
}

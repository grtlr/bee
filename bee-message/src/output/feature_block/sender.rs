// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::address::Address;

use derive_more::From;

/// Identifies the validated sender of an output.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, From, packable::Packable)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct SenderFeatureBlock(Address);

impl SenderFeatureBlock {
    /// The [`FeatureBlock`](crate::output::FeatureBlock) kind of a [`SenderFeatureBlock`].
    pub const KIND: u8 = 0;

    /// Creates a new [`SenderFeatureBlock`].
    #[inline(always)]
    pub fn new(address: Address) -> Self {
        Self(address)
    }

    /// Returns the sender [`Address`].
    #[inline(always)]
    pub fn address(&self) -> &Address {
        &self.0
    }
}

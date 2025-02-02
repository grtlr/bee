// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{unlock_block::UnlockBlockIndex, Error};

/// An [`UnlockBlock`](crate::unlock_block::UnlockBlock) that refers to another unlock block.
#[derive(Clone, Debug, Eq, PartialEq, Hash, packable::Packable)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[packable(unpack_error = Error, with = Error::InvalidReferenceIndex)]
pub struct ReferenceUnlockBlock(UnlockBlockIndex);

impl TryFrom<u16> for ReferenceUnlockBlock {
    type Error = Error;

    fn try_from(index: u16) -> Result<Self, Self::Error> {
        Self::new(index)
    }
}

impl ReferenceUnlockBlock {
    /// The [`UnlockBlock`](crate::unlock_block::UnlockBlock) kind of a [`ReferenceUnlockBlock`].
    pub const KIND: u8 = 1;

    /// Creates a new [`ReferenceUnlockBlock`].
    #[inline(always)]
    pub fn new(index: u16) -> Result<Self, Error> {
        index.try_into().map(Self).map_err(Error::InvalidReferenceIndex)
    }

    /// Return the index of a [`ReferenceUnlockBlock`].
    #[inline(always)]
    pub fn index(&self) -> u16 {
        self.0.get()
    }
}

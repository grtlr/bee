// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod alias;
mod ed25519;
mod nft;

pub use alias::AliasAddress;
pub use ed25519::Ed25519Address;
pub use nft::NftAddress;

use crate::Error;

use bech32::{self, FromBase32, ToBase32, Variant};
use derive_more::From;
use packable::PackableExt;

use alloc::{str::FromStr, string::String, vec::Vec};

/// A generic address supporting different address kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
#[cfg_attr(
    feature = "serde1",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", content = "data")
)]
#[packable(tag_type = u8, with_error = Error::InvalidAddressKind)]
#[packable(unpack_error = Error)]
pub enum Address {
    /// An Ed25519 address.
    #[packable(tag = Ed25519Address::KIND)]
    Ed25519(Ed25519Address),
    /// An alias address.
    #[packable(tag = AliasAddress::KIND)]
    Alias(AliasAddress),
    /// An NFT address.
    #[packable(tag = NftAddress::KIND)]
    Nft(NftAddress),
}

impl Address {
    /// Returns the address kind of an `Address`.
    pub fn kind(&self) -> u8 {
        match self {
            Self::Ed25519(_) => Ed25519Address::KIND,
            Self::Alias(_) => AliasAddress::KIND,
            Self::Nft(_) => NftAddress::KIND,
        }
    }

    /// Tries to create an `Address` from a Bech32 encoded string.
    pub fn try_from_bech32(addr: &str) -> Result<Self, Error> {
        match bech32::decode(addr) {
            Ok((_hrp, data, _)) => {
                let bytes = Vec::<u8>::from_base32(&data).map_err(|_| Error::InvalidAddress)?;
                Self::unpack_verified(&mut bytes.as_slice()).map_err(|_| Error::InvalidAddress)
            }
            Err(_) => Err(Error::InvalidAddress),
        }
    }

    /// Encodes this address to a Bech32 string with the hrp (human readable part) argument as prefix.
    #[allow(clippy::wrong_self_convention)]
    pub fn to_bech32(&self, hrp: &str) -> String {
        bech32::encode(hrp, self.pack_to_vec().to_base32(), Variant::Bech32).expect("Invalid address.")
    }
}

impl FromStr for Address {
    type Err = Error;

    fn from_str(address: &str) -> Result<Self, Self::Err> {
        Address::try_from_bech32(address)
    }
}

impl TryFrom<String> for Address {
    type Error = Error;

    fn try_from(address: String) -> Result<Self, Self::Error> {
        Address::from_str(&address)
    }
}

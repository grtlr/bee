# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- ## Unreleased - YYYY-MM-DD

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security -->

## 0.2.0 - 2022-XX-XX

### Added

- Full implementation of the [Node REST API](https://github.com/iotaledger/tips/blob/main/tips/TIP-0013/tip-0013.md);
- `BasicOutputDto`;
- `AliasOutputDto`;
- `FoundryOutputDto`;
- `NftOutputDto`;
- `AliasAddressDto`;
- `NftAddressDto`;
- `AliasUnlockBlockDto`;
- `NftUnlockBlockDto`;
- `NativeTokenDto`;
- `TokenIdDto`;
- `U256Dto`;
- `FeatureBlockDto`;
- `SenderFeatureBlockDto`;
- `IssuerFeatureBlockDto`;
- `DustDepositReturnFeatureBlockDto`;
- `TimelockFeatureBlockDto`;
- `ExpirationFeatureBlockDto`;
- `IndexationFeatureBlockDto`;
- `MetadataFeatureBlockDto`;
- `AliasIdDto`;
- `TokenSchemeDto`;
- `NftIdDto`;

### Changed

- Serialize and deserialize the `DTO` types using `packable` instead of `bee-common::packable`;
- Changed `OutputResponse` type: added `milestone_index`, added `milestone_timestamp`, removed `dust_allowed`;
- Changed `OutputsAddressResponse` type: removed `address_type`, removed `address`;
- Merge `InvalidSyntaxField` with `InvalidSemanticField` to `InvalidField`;
- Renamed `IndexationPayloadDto` to `TaggedDataPayloadDto`;

### Removed

- `OutputDto::SignatureLockedSingle`;
- `OutputDto::SignatureLockedDustAllowance`;
- `SignatureLockedSingleOutputDto`;
- `SignatureLockedDustAllowanceOutputDto`;

## 0.1.7 - 2021-12-08

### Changed

- Update `bee-ledger` dependency;

## 0.1.6 - 2021-12-06

### Fixed

- Add `receipts` field to `ReceiptsResponse`;

## 0.1.5 - 2021-12-06

### Added

- `Deserialize` to `SubmitMessageResponse`;

## 0.1.4 - 2021-12-03

### Added

- `Deserialize` to `SuccessBody`;

## 0.1.3 - 2021-11-03

### Removed

- `RelationDto::Discovered` variant;

### Added

- `RelationDto::Autopeered` variant;

## 0.1.2 - 2021-10-18

### Added

- `peer` feature;

## 0.1.1 - 2021-08-26

### Added

- `ledger_index` field to;
  - `OutputResponse`;
  - `BalanceAddressResponse`;
  - `OutputsAddressResponse`;

## 0.1.0 - 2021-04-20

### Added

- Types;
  - `BodyInner` type;
  - `SuccessBody` type;
  - `ErrorBody` type;
  - `DefaultErrorResponse` type;
  - `MessageDto` type;
  - `PayloadDto` type;
  - `TransactionPayloadDto` type;
  - `EssenceDto` type;
  - `RegularEssenceDto` type;
  - `InputDto` type;
  - `UtxoInputDto` type;
  - `TreasuryInputDto` type;
  - `OutputDto` type;
  - `SignatureLockedSingleOutputDto` type;
  - `SignatureLockedDustAllowanceOutputDto` type;
  - `AddressDto` type;
  - `Ed25519AddressDto` type;
  - `TreasuryOutputDto` type;
  - `UnlockBlockDto` type;
  - `SignatureUnlockDto` type;
  - `SignatureDto` type;
  - `Ed25519SignatureDto` type;
  - `ReferenceUnlockDto` type;
  - `MilestonePayloadDto` type;
  - `IndexationPayloadDto` type;
  - `ReceiptPayloadDto` type;
  - `MigratedFundsEntryDto` type;
  - `TreasuryTransactionPayloadDto` type;
  - `PeerDto` type;
  - `GossipDto` type;
  - `RelationDto` type;
  - `HeartbeatDto` type;
  - `MetricsDto` type;
  - `ReceiptDto` type;
  - `LedgerInclusionStateDto` type;
  - `Error` type;
  - `InfoResponse` type;
  - `TipsResponse` type;
  - `SubmitMessageResponse` type;
  - `MessagesFindResponse` type;
  - `MessageResponse` type;
  - `MessageMetadataResponse` type;
  - `MessageChildrenResponse` type;
  - `OutputResponse` type;
  - `BalanceAddressResponse` type;
  - `OutputsAddressResponse` type;
  - `ReceiptsResponse` type;
  - `TreasuryResponse` type;
  - `MilestoneResponse` type;
  - `UtxoChangesResponse` type;
  - `PeersResponse` type;
  - `AddPeerResponse` type;
  - `PeerResponse` type;
  - `WhiteFlagResponse` type;

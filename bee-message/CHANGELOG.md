# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- ## Unreleased - YYYY-MM-DD

### Added

- Example that counts the number of allocations while scoring PoWs

### Changed

### Deprecated

### Removed

### Fixed

### Security -->

## 0.2.0 - 2022-XX-XX

### Changed

- Serialize and deserialize all the types using `packable` instead of `bee-common::packable`;
- Renamed `IndexationPayload` to `TaggedDataPayload`;
- Update dependencies;

### Removed

- `SignatureLockedSingleOutput`;
- `SignatureLockedDustAllowanceOutput`;
- `PaddedIndex`;

## 0.1.6 - 2021-12-07

### Changed

 - Update dependencies;

## 0.1.5 - 2021-12-05

### Removed

- All enum `#[non_exhaustive]` attributes;

## 0.1.4 - 2021-05-04

### Added

- `DUST_ALLOWANCE_DIVISOR` constant;
- `DUST_OUTPUTS_MAX` constant;
- `dust_outputs_max` function;

## 0.1.3 - 2021-04-22

### Changed

- `HashedIndex` replaced by `PaddedIndex`;
- `IndexationPayload::hash()` replaced by `IndexationPayload::padded_index()`;

## 0.1.2 - 2021-04-18

### Removed

- `ConsumedOutput`;
- `CreatedOutput`;

## 0.1.1 - 2021-04-18

### Fixed

- `MilestonePayload` unpacking;

## 0.1.0 - 2021-04-16

### Added

- First implementation of the Message RFC;
  - `address` module;
  - `input` module;
  - `milestone` module;
  - `output` module;
  - `payload` module;
  - `signature` module;
  - `unlock` module;
  - `message` module;
  - `parents` module;

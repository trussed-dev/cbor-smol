# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased][]

[Unreleased]: https://github.com/trussed-dev/cbor-smol/compare/0.5.0...HEAD

-

## [0.5.0][] - 2024-10-21

[0.5.0]: https://github.com/trussed-dev/cbor-smol/compare/0.4.1...0.5.0

### Changed

- Mark `Error` as non-exhaustive ([#11](https://github.com/trussed-dev/cbor-smol/issues/11))
- Add support for multiple `heapless` and `heapless-bytes` versions ([#13](https://github.com/trussed-dev/cbor-smol/pull/13)):
  - Move existing support for `heapless` 0.7 and `heapless-bytes` 0.3 behind features
  - Add support for `heapless` 0.8 and `heapless-bytes` 0.4
  - Remove `cbor_serialize_bytes` and `cbor_serialize_extending_bytes` (use `cbor_serialize_to` instead)

## [0.4.1][] - 2024-10-08

[0.4.1]: https://github.com/trussed-dev/cbor-smol/compare/0.4.0...0.4.1

### Added

- Add support for `deserialize_ignored_any` ([#6](https://github.com/trussed-dev/cbor-smol/pull/6))
- Accept arrays of integers in `deserialize_bytes` ([#7](https://github.com/trussed-dev/cbor-smol/pull/7))
- Add support for all types expected by serde in `deserialize_identifier` ([#8](https://github.com/trussed-dev/cbor-smol/pull/8))
- Add `?Sized` trait bound to `cbor_serialize`, `cbor_serialize_extending_bytes` and `cbor_serialize_bytes` ([#10](https://github.com/trussed-dev/cbor-smol/pull/10))

### Fixed

- Fix deserialization of enums with struct variants ([#4](https://github.com/trussed-dev/cbor-smol/pull/4))
- Never inline map and tuple deserialization ([#15](https://github.com/trussed-dev/cbor-smol/pull/15))

## [0.4.0][] - 2021-06-10

[0.4.0]: https://github.com/trussed-dev/cbor-smol/compare/0.3.1...0.4.0

### Changed

- Update `heapless` to 0.7 and `heapless-bytes` to 0.3

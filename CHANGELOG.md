

# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog and this project adheres to Semantic Versioning.

---

## [0.1.1] - 2026-02-21

### Added

- Extensive additional unit tests for `decode`, `decode_in_place`, and SIMD paths.
- Explicit tests for all error variants:
  - `OddLength`
  - `OutputTooSmall`
  - `InvalidByte` (beginning / middle / end / tail / SIMD chunk boundary)
- Edge-case coverage:
  - Empty input
  - Odd-length input
  - Invalid characters (space, high ASCII, null byte, mixed invalid sequences)
  - Chunk boundary +/- 1 for SIMD
  - Tail-only paths (no full SIMD chunk)
- Roundtrip validation tests: `encode → decode → encode`
- Exhaustive single-byte validation tests (0x00..=0xFF)
- Strict CI configuration with `clippy -D warnings`
- README improvements:
  - Coverage section (~99% line coverage)
  - Comparison section vs `hex`, `faster-hex`, `const-hex`
  - Security & correctness philosophy section

### Changed

- Removed dead code in SIMD fallback branch.
- Replaced unreachable fallback return with `unreachable!()` for correctness clarity.
- Refactored minor error index edge-case handling in SIMD validation.
- Improved internal error-index precision tests.
- Adjusted documentation examples and public re-exports.

### Fixed

- SIMD error index off-by-one edge case (space character handling).
- Minor clippy violations under `-D warnings`.
- Removed unused imports and dead code in test modules.

### Quality

- ~99% line coverage
- 100% function coverage
- Scalar + SIMD tested under `--all-features`
- Cross-architecture tested (ARM64 + x86_64)
- `no_std` default verified

---

## [0.1.0] - 2026-02-21

### Initial release

- Ultra-fast hex encode/decode implementation
- `#![no_std]` by default
- Zero allocations (except optional `encode_to_string`)
- Precise error reporting with byte index
- `decode_to_slice`, `decode_to_array`, `decode_in_place`
- Optional SIMD acceleration
- Comprehensive benchmark suite (scalar vs SIMD vs `hex` crate)

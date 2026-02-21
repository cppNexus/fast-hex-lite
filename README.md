# fast-hex-lite

Ultra-fast hex encoding/decoding in Rust with **zero allocations** and `#![no_std]` support.

[![Crates.io](https://img.shields.io/crates/v/fast-hex-lite.svg)](https://crates.io/crates/fast-hex-lite)
[![Docs.rs](https://docs.rs/fast-hex-lite/badge.svg)](https://docs.rs/fast-hex-lite)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

---

## Features

| Feature   | Default | Description                                            |
|-----------|:-------:|--------------------------------------------------------|
| _(none)_  | ✓       | `no_std` + alloc-free scalar encoder/decoder           |
| `std`     |         | Implements `std::error::Error` on `Error`              |
| `simd`    |         | SIMD-accelerated decoder via `std::simd` (implies `std`, Rust 1.88+) |

## Installation

```toml
# Default (no_std, scalar only)
[dependencies]
fast-hex-lite = "0.1"

# With SIMD acceleration (requires std, nightly not required on 1.88+)
[dependencies]
fast-hex-lite = { version = "0.1", features = ["simd"] }
```

## Zero-alloc usage

### `decode_to_slice` — hex bytes → raw bytes

```rust
use fast_hex_lite::decode_to_slice;

let hex = b"deadbeef";
let mut buf = [0u8; 4];
let n = decode_to_slice(hex, &mut buf).unwrap();
assert_eq!(&buf[..n], &[0xde, 0xad, 0xbe, 0xef]);

// Accepts uppercase and mixed-case too:
let mut buf2 = [0u8; 4];
decode_to_slice(b"DEADBEEF", &mut buf2).unwrap();
decode_to_slice(b"DeAdBeEf", &mut buf2).unwrap();
```

### `decode_in_place` — decode into the same buffer

Useful when the hex string is in a mutable buffer and you want to avoid
even a second stack allocation.

```rust
use fast_hex_lite::decode_in_place;

let mut buf = *b"deadbeef";  // 8 bytes of ASCII hex
let n = decode_in_place(&mut buf).unwrap(); // writes to buf[0..4]
assert_eq!(&buf[..n], &[0xde, 0xad, 0xbe, 0xef]);
```

### `decode_to_array` — fixed-size decode

```rust
use fast_hex_lite::decode_to_array;

let bytes: [u8; 4] = decode_to_array(b"deadbeef").unwrap();
assert_eq!(bytes, [0xde, 0xad, 0xbe, 0xef]);
```

### `encode_to_slice` — raw bytes → hex bytes

```rust
use fast_hex_lite::encode_to_slice;

let src = [0xde, 0xad, 0xbe, 0xef];
let mut out = [0u8; 8];

// Lowercase
let n = encode_to_slice(&src, &mut out, true).unwrap();
assert_eq!(&out[..n], b"deadbeef");

// Uppercase
encode_to_slice(&src, &mut out, false).unwrap();
assert_eq!(&out[..8], b"DEADBEEF");
```

### Helper functions

```rust
use fast_hex_lite::{decoded_len, encoded_len};

assert_eq!(decoded_len(8).unwrap(), 4);   // 8 hex chars → 4 bytes
assert_eq!(encoded_len(4), 8);            // 4 bytes → 8 hex chars
```

## Error handling

```rust
use fast_hex_lite::{decode_to_slice, Error};

let mut buf = [0u8; 4];

// Odd length
assert_eq!(decode_to_slice(b"abc", &mut buf), Err(Error::OddLength));

// Output buffer too small
assert_eq!(decode_to_slice(b"deadbeef", &mut buf[..1]), Err(Error::OutputTooSmall));

// Invalid character
let err = decode_to_slice(b"deXd", &mut buf).unwrap_err();
assert!(matches!(err, Error::InvalidByte { index: 2, byte: b'X' }));
```

## SIMD acceleration

Enable the `simd` feature for a SIMD-accelerated decoder (processes 32 hex bytes per loop
iteration using `std::simd::u8x32`):

```toml
fast-hex-lite = { version = "0.1", features = ["simd"] }
```

The SIMD path is **fully transparent**: same public API, same error semantics including
exact error-byte index reporting. A scalar fallback handles any remaining tail bytes.

Requirements:
- Rust 1.88+ (stable portable SIMD)
- Feature implies `std`

## Running benchmarks

```bash
# Scalar (default)
cargo bench

# With SIMD
cargo bench --features simd
```

### Sample results (placeholder — run on your machine)

| Benchmark                 | 256 B      | 4 KB       | 64 KB      | 1 MB       |
|---------------------------|-----------|-----------|-----------|-----------|
| decode fast-hex-lite      | ~200 MB/s | ~600 MB/s | ~700 MB/s | ~720 MB/s |
| decode fast-hex-lite+simd | ~400 MB/s | ~1.2 GB/s | ~1.4 GB/s | ~1.5 GB/s |
| decode hex-crate          | ~120 MB/s | ~200 MB/s | ~220 MB/s | ~225 MB/s |
| encode fast-hex-lite      | ~800 MB/s | ~2 GB/s   | ~2.1 GB/s | ~2.1 GB/s |
| encode hex-crate          | ~200 MB/s | ~400 MB/s | ~420 MB/s | ~430 MB/s |

> _Numbers are illustrative. Run `cargo bench --features simd` for real figures._

## `no_std` support

The crate is `#![no_std]` by default. No heap allocator is needed either.
All APIs work on caller-provided slices/arrays.

```toml
# In your no_std crate:
[dependencies]
fast-hex-lite = { version = "0.1", default-features = false }
```

## Codebase overview

```
src/
  lib.rs      — public API, Error type, feature gates
  decode.rs   — scalar decoder + unhex LUT + in-place decode
  encode.rs   — scalar encoder
  simd.rs     — SIMD decoder (compiled only with feature `simd`)
benches/
  bench.rs    — Criterion benchmarks vs `hex` crate
```

## MSRV

Rust **1.88** (edition 2021). Stable only — no nightly features required.

## License

Licensed under either of

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.

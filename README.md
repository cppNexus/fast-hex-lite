# fast-hex-lite

Ultra-fast hex encoding/decoding in Rust with zero allocations and `#![no_std]` support.

[![Crates.io](https://img.shields.io/crates/v/fast-hex-lite.svg)](https://crates.io/crates/fast-hex-lite)
[![Docs.rs](https://img.shields.io/docsrs/fast-hex-lite/badge.svg)](https://docs.rs/fast-hex-lite)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](#license)
[![CI](https://github.com/cppNexus/fast-hex-lite/actions/workflows/ci.yml/badge.svg)](https://github.com/cppNexus/fast-hex-lite/actions/workflows/ci.yml)

Designed for performance-critical systems such as cryptography,
networking stacks, blockchain infrastructure, and embedded environments
where `no_std` and zero heap usage are mandatory.

---

## Features

| Feature   | Default | Description                                              |
|-----------|:-------:|----------------------------------------------------------|
| _(none)_  | yes     | `no_std`, alloc-free scalar encoder/decoder              |
| `std`     |         | Implements `std::error::Error` for `Error`               |
| `simd`    |         | SIMD-accelerated decoder via `std::simd` (implies `std`) |

---

## Installation

```toml
# Default: no_std, scalar only
[dependencies]
fast-hex-lite = "0.1"

# With SIMD acceleration
[dependencies]
fast-hex-lite = { version = "0.1", features = ["simd"] }

# Explicit no_std (same as default)
[dependencies]
fast-hex-lite = { version = "0.1", default-features = false }
```

---

## Usage

All APIs operate on caller-provided buffers. No heap allocations occur.

### Decode hex to bytes

```rust
use fast_hex_lite::decode_to_slice;

let hex = b"deadbeef";
let mut buf = [0u8; 4];
let n = decode_to_slice(hex, &mut buf).unwrap();
assert_eq!(&buf[..n], &[0xde, 0xad, 0xbe, 0xef]);

// Uppercase and mixed-case are accepted
decode_to_slice(b"DEADBEEF", &mut buf).unwrap();
decode_to_slice(b"DeAdBeEf", &mut buf).unwrap();
```

### Decode in-place

Decodes ASCII hex in a mutable buffer into its own first half. No secondary buffer required.

```rust
use fast_hex_lite::decode_in_place;

let mut buf = *b"deadbeef";
let n = decode_in_place(&mut buf).unwrap();
assert_eq!(&buf[..n], &[0xde, 0xad, 0xbe, 0xef]);
```

### Decode into a fixed-size array

```rust
use fast_hex_lite::decode_to_array;

let bytes: [u8; 4] = decode_to_array(b"deadbeef").unwrap();
assert_eq!(bytes, [0xde, 0xad, 0xbe, 0xef]);
```

### Encode bytes to hex

```rust
use fast_hex_lite::encode_to_slice;

let src = [0xde, 0xad, 0xbe, 0xef];
let mut out = [0u8; 8];

encode_to_slice(&src, &mut out, true).unwrap();   // lowercase
assert_eq!(&out, b"deadbeef");

encode_to_slice(&src, &mut out, false).unwrap();  // uppercase
assert_eq!(&out, b"DEADBEEF");
```

### Length helpers

```rust
use fast_hex_lite::{decoded_len, encoded_len};

assert_eq!(decoded_len(8).unwrap(), 4);  // 8 hex chars -> 4 bytes
assert_eq!(encoded_len(4), 8);           // 4 bytes -> 8 hex chars
```

---

## Error handling

```rust
use fast_hex_lite::{decode_to_slice, Error};

let mut buf = [0u8; 4];

// Odd-length input
assert_eq!(decode_to_slice(b"abc", &mut buf), Err(Error::OddLength));

// Output buffer too small
assert_eq!(decode_to_slice(b"deadbeef", &mut buf[..1]), Err(Error::OutputTooSmall));

// Invalid character: exact byte index reported
let err = decode_to_slice(b"deXd", &mut buf).unwrap_err();
assert!(matches!(err, Error::InvalidByte { index: 2, byte: b'X' }));
```

All errors include precise context. `InvalidByte` reports the zero-based index of the
first invalid byte in the source slice.

---

## SIMD acceleration

Enable the `simd` feature to use a SIMD-accelerated decoder built on `std::simd`:

```toml
fast-hex-lite = { version = "0.1", features = ["simd"] }
```

The SIMD path processes 32 hex bytes per iteration using `Simd<u8, 32>`. It is fully
transparent: the public API, error types, and error index semantics are identical to the
scalar path. Remaining tail bytes fall back to scalar automatically.

Requirements: Rust 1.88+, stable (no nightly features used).

---

## Benchmarks

Measured on Apple M3 Pro (macOS, `cargo bench --features simd`).

Numbers are median Criterion throughput values.

Throughput is over **decoded output bytes** for decode, **input bytes** for encode and
validate, and **decoded output bytes** for decode_in_place.

### Decode: scalar (hex to bytes)

| Input | fast-hex-lite lower | fast-hex-lite mixed | hex crate lower | hex crate mixed |
|-------|:-------------------:|:-------------------:|:---------------:|:---------------:|
| 32 B  | 1.67 GiB/s          | 1.66 GiB/s          | 663 MiB/s       | 696 MiB/s       |
| 256 B | 1.57 GiB/s          | 1.58 GiB/s          | 636 MiB/s       | 700 MiB/s       |
| 4 KB  | 1.70 GiB/s          | 1.70 GiB/s          | 597 MiB/s       | 621 MiB/s       |
| 64 KB | 1.67 GiB/s          | 1.68 GiB/s          | 357 MiB/s       | 370 MiB/s       |
| 1 MB  | 1.67 GiB/s          | 1.71 GiB/s          | 207 MiB/s       | 215 MiB/s       |

### Decode: SIMD (hex to bytes)

| Input | fast-hex-lite lower | fast-hex-lite mixed | hex crate lower | hex crate mixed |
|-------|:-------------------:|:-------------------:|:---------------:|:---------------:|
| 32 B  | 5.51 GiB/s          | 5.49 GiB/s          | 628 MiB/s       | 681 MiB/s       |
| 256 B | 6.10 GiB/s          | 6.09 GiB/s          | 608 MiB/s       | 659 MiB/s       |
| 4 KB  | 6.03 GiB/s          | 6.04 GiB/s          | 584 MiB/s       | 617 MiB/s       |
| 64 KB | 6.14 GiB/s          | 6.15 GiB/s          | 390 MiB/s       | 391 MiB/s       |
| 1 MB  | 6.09 GiB/s          | 6.15 GiB/s          | 201 MiB/s       | 202 MiB/s       |

### Encode (bytes to hex)

| Input | fast-hex-lite lower | fast-hex-lite upper | hex crate lower |
|-------|:-------------------:|:-------------------:|:---------------:|
| 32 B  | 2.50 GiB/s          | 2.20 GiB/s          | 2.03 GiB/s      |
| 256 B | 2.50 GiB/s          | 2.48 GiB/s          | 2.01 GiB/s      |
| 4 KB  | 2.61 GiB/s          | 2.59 GiB/s          | 2.06 GiB/s      |
| 64 KB | 2.60 GiB/s          | 2.60 GiB/s          | 2.09 GiB/s      |
| 1 MB  | 2.59 GiB/s          | 2.59 GiB/s          | 2.09 GiB/s      |

### decode_in_place

| Input | scalar     | simd       |
|-------|:----------:|:----------:|
| 32 B  | 655 MiB/s  | 650 MiB/s  |
| 256 B | 717 MiB/s  | 709 MiB/s  |
| 4 KB  | 764 MiB/s  | 775 MiB/s  |
| 64 KB | 765 MiB/s  | 770 MiB/s  |
| 1 MB  | 780 MiB/s  | 785 MiB/s  |

Mixed-case input carries zero overhead versus lowercase. Decode throughput is stable
across all input sizes. The SIMD path delivers ~3.5-3.7x uplift over scalar for decode
at large inputs.

---

## no_std support

The crate is `#![no_std]` by default. No allocator is required. All APIs work on
caller-provided stack arrays or static buffers.

```toml
fast-hex-lite = { version = "0.1", default-features = false }
```

---

## Code structure

```
src/
  lib.rs      -- public API, Error type, feature gates
  decode.rs   -- scalar decoder, 256-entry compile-time LUT, in-place decode
  encode.rs   -- scalar encoder
  simd.rs     -- SIMD decoder (compiled only with feature `simd`)
benches/
  bench.rs    -- Criterion benchmarks vs hex crate
```

---

## MSRV

Rust 1.88, edition 2021. Stable only, no nightly features required.

---

## License

- [Apache License, Version 2.0](LICENSE)

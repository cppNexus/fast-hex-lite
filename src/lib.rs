//! # fast-hex-lite
//!
//! Ultra-fast hex encoding/decoding with **zero allocations** and `#![no_std]` support.
//!
//! ## Features
//!
//! | Feature | Description |
//! |---------|-------------|
//! | _(default)_ | `no_std` scalar decoder + encoder |
//! | `std` | Enables `std::error::Error` impl on [`Error`] |
//! | `simd` | SIMD-accelerated decoder (implies `std`; requires Rust 1.88+) |
//!
//! ## Quick start
//!
//! ### Decode hex → bytes
//!
//! ```rust
//! use fast_hex_lite::decode_to_slice;
//!
//! let hex = b"deadbeef";
//! let mut buf = [0u8; 4];
//! let n = decode_to_slice(hex, &mut buf).unwrap();
//! assert_eq!(&buf[..n], &[0xde, 0xad, 0xbe, 0xef]);
//! ```
//!
//! ### Decode in-place
//!
//! ```rust
//! use fast_hex_lite::decode_in_place;
//!
//! let mut buf = *b"deadbeef";
//! let n = decode_in_place(&mut buf).unwrap();
//! assert_eq!(&buf[..n], &[0xde, 0xad, 0xbe, 0xef]);
//! ```
//!
//! ### Encode bytes → hex
//!
//! ```rust
//! use fast_hex_lite::encode_to_slice;
//!
//! let src = [0xde, 0xad, 0xbe, 0xef];
//! let mut out = [0u8; 8];
//! let n = encode_to_slice(&src, &mut out, true).unwrap();
//! assert_eq!(&out[..n], b"deadbeef");
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs, clippy::all)]

mod decode;
mod encode;

#[cfg(feature = "simd")]
mod simd;

pub use decode::{decode_in_place, decode_to_array, decode_to_slice, decoded_len};
pub use encode::{encode_to_slice, encoded_len};

/// Errors that can occur during hex encoding or decoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The input length is odd; hex strings must have even number of bytes.
    OddLength,
    /// The output buffer is too small.
    OutputTooSmall,
    /// An invalid byte was encountered at the given position.
    InvalidByte {
        /// Zero-based index into the source slice.
        index: usize,
        /// The offending byte value.
        byte: u8,
    },
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::OddLength => f.write_str("hex string has odd length"),
            Error::OutputTooSmall => f.write_str("output buffer is too small"),
            Error::InvalidByte { index, byte } => {
                write!(
                    f,
                    "invalid hex byte 0x{:02x} ('{}') at index {}",
                    byte,
                    if byte.is_ascii_graphic() {
                        *byte as char
                    } else {
                        '?'
                    },
                    index
                )
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

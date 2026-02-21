//! Fast, allocation-free hex encoding/decoding.
//!
//! - `no_std` by default (scalar path)
//! - Optional `std` support
//! - Optional `simd` accelerated decode/validate on supported targets
//!
//! ## API
//!
//! - Decode: [`decode_to_slice`], [`decode_to_array`], [`decode_in_place`]
//! - Encode: [`encode_to_slice`]
//!
//! ## Examples
//!
//! Decode hex → bytes:
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
//! Encode bytes → hex (lowercase):
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
#![warn(missing_docs, clippy::all, clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::must_use_candidate
)]

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
                let b = *byte;
                write!(
                    f,
                    "invalid hex byte 0x{:02x} ('{}') at index {}",
                    b,
                    if b.is_ascii_graphic() {
                        b as char
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

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

// `encode_to_string` requires allocation (String), so it is only available with `std`.
#[cfg(feature = "std")]
pub use encode::encode_to_string;

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
                    if b.is_ascii_graphic() { b as char } else { '?' },
                    index
                )
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use std::prelude::v1::*;

    // ── Error: PartialEq / Clone ───────────────────────────────────────────

    #[test]
    fn test_error_eq_odd_length() {
        assert_eq!(Error::OddLength, Error::OddLength);
    }

    #[test]
    fn test_error_eq_output_too_small() {
        assert_eq!(Error::OutputTooSmall, Error::OutputTooSmall);
    }

    #[test]
    fn test_error_eq_invalid_byte() {
        assert_eq!(
            Error::InvalidByte {
                index: 3,
                byte: b'X'
            },
            Error::InvalidByte {
                index: 3,
                byte: b'X'
            },
        );
    }

    #[test]
    fn test_error_ne_different_variants() {
        assert_ne!(Error::OddLength, Error::OutputTooSmall);
        assert_ne!(Error::OddLength, Error::InvalidByte { index: 0, byte: 0 },);
    }

    #[test]
    fn test_error_ne_different_index() {
        assert_ne!(
            Error::InvalidByte {
                index: 0,
                byte: b'X'
            },
            Error::InvalidByte {
                index: 1,
                byte: b'X'
            },
        );
    }

    #[test]
    fn test_error_ne_different_byte() {
        assert_ne!(
            Error::InvalidByte {
                index: 0,
                byte: b'X'
            },
            Error::InvalidByte {
                index: 0,
                byte: b'Y'
            },
        );
    }

    #[test]
    fn test_error_clone() {
        let e = Error::InvalidByte {
            index: 7,
            byte: 0xAB,
        };
        assert_eq!(e.clone(), e);

        assert_eq!(Error::OddLength.clone(), Error::OddLength);
        assert_eq!(Error::OutputTooSmall.clone(), Error::OutputTooSmall);
    }

    // ── Error: Display ─────────────────────────────────────────────────────

    #[test]
    fn test_display_odd_length() {
        let s = std::format!("{}", Error::OddLength);
        assert_eq!(s, "hex string has odd length");
    }

    #[test]
    fn test_display_output_too_small() {
        let s = std::format!("{}", Error::OutputTooSmall);
        assert_eq!(s, "output buffer is too small");
    }

    #[test]
    fn test_display_invalid_byte_graphic() {
        // 'X' is ASCII graphic → должна подставиться как символ
        let s = std::format!(
            "{}",
            Error::InvalidByte {
                index: 4,
                byte: b'X'
            }
        );
        assert!(s.contains("0x58"), "hex value missing: {s}");
        assert!(s.contains('X'), "char missing: {s}");
        assert!(s.contains('4'), "index missing: {s}");
    }

    #[test]
    fn test_display_invalid_byte_non_graphic() {
        // 0x01 — не ASCII graphic → должен подставиться '?'
        let s = std::format!(
            "{}",
            Error::InvalidByte {
                index: 0,
                byte: 0x01
            }
        );
        assert!(s.contains("0x01"), "hex value missing: {s}");
        assert!(s.contains('?'), "fallback char missing: {s}");
        assert!(s.contains('0'), "index missing: {s}");
    }

    #[test]
    fn test_display_invalid_byte_null() {
        let s = std::format!(
            "{}",
            Error::InvalidByte {
                index: 0,
                byte: 0x00
            }
        );
        assert!(s.contains("0x00"));
        assert!(s.contains('?'));
    }

    #[test]
    fn test_display_invalid_byte_high_ascii() {
        let s = std::format!(
            "{}",
            Error::InvalidByte {
                index: 10,
                byte: 0xFF
            }
        );
        assert!(s.contains("0xff"));
        assert!(s.contains('?'));
        assert!(s.contains("10"));
    }

    #[test]
    fn test_display_invalid_byte_space() {
        // ' ' (0x20) is_ascii_graphic() == false → '?'
        let s = std::format!(
            "{}",
            Error::InvalidByte {
                index: 1,
                byte: b' '
            }
        );
        assert!(s.contains("0x20"));
        assert!(s.contains('?'));
    }

    // ── Error: Debug ───────────────────────────────────────────────────────

    #[test]
    fn test_debug_odd_length() {
        let s = std::format!("{:?}", Error::OddLength);
        assert_eq!(s, "OddLength");
    }

    #[test]
    fn test_debug_output_too_small() {
        let s = std::format!("{:?}", Error::OutputTooSmall);
        assert_eq!(s, "OutputTooSmall");
    }

    #[test]
    fn test_debug_invalid_byte() {
        let s = std::format!(
            "{:?}",
            Error::InvalidByte {
                index: 2,
                byte: 0xAB
            }
        );
        assert!(s.contains("InvalidByte"));
        assert!(s.contains("index: 2"));
        assert!(s.contains("byte: 171")); // 0xAB == 171
    }

    // ── std::error::Error ──────────────────────────────────────────────────

    #[cfg(feature = "std")]
    #[test]
    fn test_std_error_trait_odd_length() {
        let e: &dyn std::error::Error = &Error::OddLength;
        assert!(e.source().is_none());
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_std_error_trait_invalid_byte() {
        let e: &dyn std::error::Error = &Error::InvalidByte { index: 0, byte: 0 };
        assert!(e.source().is_none());
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_std_error_display_matches_fmt() {
        let errors = [
            Error::OddLength,
            Error::OutputTooSmall,
            Error::InvalidByte {
                index: 5,
                byte: b'Z',
            },
        ];
        for e in &errors {
            let via_display = std::format!("{e}");
            let via_error: &dyn std::error::Error = e;
            assert_eq!(via_display, std::format!("{via_error}"));
        }
    }

    // ── публичный API: re-exports доступны из корня ────────────────────────

    #[test]
    fn test_public_api_decode_to_slice() {
        let mut buf = [0u8; 4];
        let n = decode_to_slice(b"deadbeef", &mut buf).unwrap();
        assert_eq!(n, 4);
        assert_eq!(&buf, &[0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn test_public_api_decode_to_array() {
        let arr = decode_to_array::<4>(b"deadbeef").unwrap();
        assert_eq!(arr, [0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn test_public_api_decode_in_place() {
        let mut buf = *b"deadbeef";
        let n = decode_in_place(&mut buf).unwrap();
        assert_eq!(&buf[..n], &[0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn test_public_api_decoded_len() {
        assert_eq!(decoded_len(8).unwrap(), 4);
        assert_eq!(decoded_len(1), Err(Error::OddLength));
    }

    #[test]
    fn test_public_api_encode_to_slice() {
        let mut out = [0u8; 8];
        let n = encode_to_slice(&[0xde, 0xad, 0xbe, 0xef], &mut out, true).unwrap();
        assert_eq!(n, 8);
        assert_eq!(&out, b"deadbeef");
    }

    #[test]
    fn test_public_api_encoded_len() {
        assert_eq!(encoded_len(4), 8);
        assert_eq!(encoded_len(0), 0);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_public_api_encode_to_string() {
        let s = encode_to_string(&[0xde, 0xad, 0xbe, 0xef], true);
        assert_eq!(s, "deadbeef");
    }

    // ── сквозной round-trip через публичный API ────────────────────────────

    #[test]
    fn test_roundtrip_encode_decode() {
        let src: std::vec::Vec<u8> = (0u8..=255).collect();
        let mut hex = std::vec![0u8; src.len() * 2];
        encode_to_slice(&src, &mut hex, true).unwrap();
        let mut dst = std::vec![0u8; src.len()];
        decode_to_slice(&hex, &mut dst).unwrap();
        assert_eq!(dst, src);
    }

    #[test]
    fn test_roundtrip_encode_decode_uppercase() {
        let src: std::vec::Vec<u8> = (0u8..=255).collect();
        let mut hex = std::vec![0u8; src.len() * 2];
        encode_to_slice(&src, &mut hex, false).unwrap();
        let mut dst = std::vec![0u8; src.len()];
        decode_to_slice(&hex, &mut dst).unwrap();
        assert_eq!(dst, src);
    }
}

//! Scalar hex decoder.

use crate::Error;

/// Returns the number of bytes produced from a hex string of `hex_len` bytes.
///
/// Returns [`Error::OddLength`] if `hex_len` is odd.
///
/// # Examples
/// ```
/// assert_eq!(fast_hex_lite::decoded_len(8).unwrap(), 4);
/// ```
#[inline]
pub fn decoded_len(hex_len: usize) -> Result<usize, Error> {
    if hex_len % 2 != 0 {
        Err(Error::OddLength)
    } else {
        Ok(hex_len / 2)
    }
}

/// Decode ASCII-hex bytes `src_hex` into `dst`.
///
/// `src_hex` must contain an even number of bytes, all valid hex characters
/// (`0-9`, `a-f`, `A-F`). `dst` must be at least `src_hex.len() / 2` bytes.
///
/// Returns the number of bytes written.
#[inline]
pub fn decode_to_slice(src_hex: &[u8], dst: &mut [u8]) -> Result<usize, Error> {
    let out_len = decoded_len(src_hex.len())?;
    if dst.len() < out_len {
        return Err(Error::OutputTooSmall);
    }
    #[cfg(feature = "simd")]
    {
        crate::simd::decode_to_slice_simd(src_hex, &mut dst[..out_len])
    }
    #[cfg(not(feature = "simd"))]
    {
        decode_scalar(src_hex, &mut dst[..out_len])
    }
}

/// Decode exactly `N` bytes from a hex string of length `2*N`.
///
/// Returns [`Error::OutputTooSmall`] if `src_hex.len() / 2 != N`.
pub fn decode_to_array<const N: usize>(src_hex: &[u8]) -> Result<[u8; N], Error> {
    let out_len = decoded_len(src_hex.len())?;
    if out_len != N {
        return Err(Error::OutputTooSmall);
    }
    let mut arr = [0u8; N];
    decode_to_slice(src_hex, &mut arr)?;
    Ok(arr)
}

/// Decode hex bytes in-place: `buf` initially contains ASCII hex; after
/// decoding, the first `buf.len() / 2` bytes hold the result.
///
/// Returns the number of bytes written.
#[inline]
pub fn decode_in_place(buf: &mut [u8]) -> Result<usize, Error> {
    let out_len = decoded_len(buf.len())?;

    // Pass 1: validate without writing, so on error the buffer is unchanged.
    // Also lets us keep the fast decode loop branch-free.
    for i in 0..out_len {
        let hi = buf[2 * i];
        let lo = buf[2 * i + 1];

        if unhex_byte(hi).is_none() {
            return Err(Error::InvalidByte {
                index: 2 * i,
                byte: hi,
            });
        }
        if unhex_byte(lo).is_none() {
            return Err(Error::InvalidByte {
                index: 2 * i + 1,
                byte: lo,
            });
        }
    }

    // Pass 2: decode. Safe to write now.
    for i in 0..out_len {
        let hi = buf[2 * i];
        let lo = buf[2 * i + 1];
        // Validation above guarantees this is a valid entry.
        buf[i] = decode_pair(hi, lo) as u8;
    }

    Ok(out_len)
}

// ── Scalar decoder ─────────────────────────────────────────────────────────

#[inline(always)]
pub(crate) fn decode_scalar(src_hex: &[u8], dst: &mut [u8]) -> Result<usize, Error> {
    // `src_hex` is already even-length checked by the caller.
    // `dst` is already sized-checked by the caller.
    let out_len = src_hex.len() >> 1;

    // Hot loop: single 16-bit table lookup per output byte.
    // Use a tight index-based loop so LLVM can eliminate bounds checks.
    let mut j = 0usize;
    for i in 0..out_len {
        let hi = src_hex[j];
        let lo = src_hex[j + 1];

        let v = decode_pair(hi, lo);
        if (v & 0x0100) != 0 {
            // Slow-path only on error: identify which byte is invalid so we
            // can report the correct index/byte.
            if unhex_byte(hi).is_none() {
                return Err(Error::InvalidByte { index: j, byte: hi });
            }
            return Err(Error::InvalidByte {
                index: j + 1,
                byte: lo,
            });
        }

        dst[i] = v as u8;
        j += 2;
    }

    Ok(out_len)
}

/// Map a single ASCII hex digit to its nibble value (0..=15).
/// Returns `None` for non-hex bytes.
///
/// Fast table lookup.
#[inline(always)]
pub(crate) fn unhex_byte(b: u8) -> Option<u8> {
    let v = UNHEX_TABLE[b as usize];
    if v == 0xFF { None } else { Some(v) }
}

// 256-entry nibble table (0..=15) or 0xFF for invalid.
const UNHEX_TABLE: [u8; 256] = make_unhex_table();

// 65_536-entry pair table. Each entry encodes either:
// - valid: 0x0000..=0x00FF (decoded byte)
// - invalid: 0x0100 (flag set)
//
// This lets the scalar decoder process 2 input bytes per iteration with a
// single table lookup.
const HEXPAIR_TABLE: [u16; 65536] = make_hexpair_table();

#[inline(always)]
fn decode_pair(hi: u8, lo: u8) -> u16 {
    // Index is the two ASCII bytes.
    let idx = ((hi as usize) << 8) | (lo as usize);
    HEXPAIR_TABLE[idx]
}

const fn make_unhex_table() -> [u8; 256] {
    let mut t = [0xFFu8; 256];
    let mut i = 0u16;
    while i < 256 {
        let b = i as u8;
        t[i as usize] = if b >= b'0' && b <= b'9' {
            b - b'0'
        } else if b >= b'a' && b <= b'f' {
            b - b'a' + 10
        } else if b >= b'A' && b <= b'F' {
            b - b'A' + 10
        } else {
            0xFF
        };
        i += 1;
    }
    t
}

const fn make_hexpair_table() -> [u16; 65536] {
    let mut t = [0x0100u16; 65536];
    let unhex = make_unhex_table();

    let mut hi = 0u32;
    while hi < 256 {
        let mut lo = 0u32;
        while lo < 256 {
            let hn = unhex[hi as usize];
            let ln = unhex[lo as usize];
            if hn != 0xFF && ln != 0xFF {
                let out = ((hn as u16) << 4) | (ln as u16);
                t[((hi as usize) << 8) | (lo as usize)] = out;
            }
            lo += 1;
        }
        hi += 1;
    }

    t
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    extern crate std;
    use std::prelude::v1::*;
    use super::*;

    #[test]
    fn test_decoded_len() {
        assert_eq!(decoded_len(0).unwrap(), 0);
        assert_eq!(decoded_len(2).unwrap(), 1);
        assert_eq!(decoded_len(8).unwrap(), 4);
        assert_eq!(decoded_len(1), Err(Error::OddLength));
        assert_eq!(decoded_len(3), Err(Error::OddLength));
    }

    #[test]
    fn test_empty() {
        let mut buf = [];
        assert_eq!(decode_to_slice(b"", &mut buf).unwrap(), 0);
    }

    #[test]
    fn test_single_byte() {
        let mut buf = [0u8; 1];
        assert_eq!(decode_to_slice(b"0f", &mut buf).unwrap(), 1);
        assert_eq!(buf[0], 0x0f);
    }

    #[test]
    fn test_known_vectors() {
        let cases: &[(&[u8], &[u8])] = &[
            (b"deadbeef", &[0xde, 0xad, 0xbe, 0xef]),
            (b"DEADBEEF", &[0xde, 0xad, 0xbe, 0xef]),
            (b"DeAdBeEf", &[0xde, 0xad, 0xbe, 0xef]),
            (b"00ff", &[0x00, 0xff]),
            (b"0001020304050607", &[0, 1, 2, 3, 4, 5, 6, 7]),
        ];
        for (hex, expected) in cases {
            let mut buf = std::vec![0u8; expected.len()];
            let n = decode_to_slice(hex, &mut buf).unwrap();
            assert_eq!(&buf[..n], *expected, "hex: {:?}", hex);
        }
    }

    #[test]
    fn test_invalid_byte_at_zero() {
        let mut buf = [0u8; 1];
        let err = decode_to_slice(b"gz", &mut buf).unwrap_err();
        assert_eq!(err, Error::InvalidByte { index: 0, byte: b'g' });
    }

    #[test]
    fn test_invalid_byte_in_middle() {
        let mut buf = [0u8; 3];
        let err = decode_to_slice(b"aabbXX", &mut buf).unwrap_err();
        // 'X' is at index 4
        assert_eq!(err, Error::InvalidByte { index: 4, byte: b'X' });
    }

    #[test]
    fn test_invalid_byte_at_last() {
        let mut buf = [0u8; 2];
        let err = decode_to_slice(b"aabX", &mut buf).unwrap_err();
        assert_eq!(err, Error::InvalidByte { index: 3, byte: b'X' });
    }

    #[test]
    fn test_odd_length() {
        let mut buf = [0u8; 2];
        assert_eq!(decode_to_slice(b"abc", &mut buf).unwrap_err(), Error::OddLength);
    }

    #[test]
    fn test_output_too_small() {
        let mut buf = [0u8; 1];
        assert_eq!(
            decode_to_slice(b"aabbcc", &mut buf).unwrap_err(),
            Error::OutputTooSmall
        );
    }

    #[test]
    fn test_decode_to_array() {
        let result = decode_to_array::<4>(b"deadbeef").unwrap();
        assert_eq!(result, [0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn test_decode_to_array_wrong_size() {
        let err = decode_to_array::<3>(b"deadbeef").unwrap_err();
        assert_eq!(err, Error::OutputTooSmall);
    }

    #[test]
    fn test_decode_in_place() {
        let mut buf = *b"deadbeef";
        let n = decode_in_place(&mut buf).unwrap();
        assert_eq!(n, 4);
        assert_eq!(&buf[..n], &[0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn test_decode_in_place_invalid_no_partial_write() {
        let mut buf = *b"deadXXef";
        let err = decode_in_place(&mut buf).unwrap_err();
        // buf must not have been partially modified
        assert_eq!(err, Error::InvalidByte { index: 4, byte: b'X' });
        // buffer must be fully unmodified on error
        assert_eq!(&buf, b"deadXXef");
    }

    #[test]
    fn test_all_valid_hex_chars() {
        // 24 hex chars (even length) covering all valid characters
        let hex = b"0123456789abcdefABCDEF00";
        let mut buf = [0u8; 12];
        decode_to_slice(hex, &mut buf).unwrap();
    }

    #[test]
    fn test_roundtrip() {
        // deterministic: use a fixed sequence of bytes
        let src: Vec<u8> = (0u8..=255).collect();
        let mut hex_buf = std::vec![0u8; src.len() * 2];
        let n_enc = crate::encode_to_slice(&src, &mut hex_buf, true).unwrap();
        let mut decoded = std::vec![0u8; src.len()];
        let n_dec = decode_to_slice(&hex_buf[..n_enc], &mut decoded).unwrap();
        assert_eq!(n_dec, src.len());
        assert_eq!(decoded, src);
    }
}
//! SIMD-accelerated hex decoder (feature `simd`).
//!
//! Uses `std::simd` (portable SIMD, stabilised in Rust 1.86) to process
//! 32 hex bytes (→ 16 output bytes) per iteration, then falls back to
//! scalar for the tail.
//!
//! # Strategy (per 32-byte chunk)
//! 1. Load 32 ASCII hex bytes into a `Simd<u8, 32>` lane vector.
//! 2. Validate each lane: must belong to `'0'..='9'`, `'a'..='f'`, or `'A'..='F'`.
//! 3. Map lanes to nibble values 0–15 branchlessly via arithmetic + mask select.
//! 4. Extract nibble array; combine even/odd pairs: `(hi << 4) | lo`.
//! 5. Scalar tail handles remaining `< 32` hex bytes.

use std::simd::{
    cmp::SimdPartialOrd,
    Mask,
    Simd,
};

use crate::{
    decode::decode_scalar,
    Error,
};

type U8x32 = Simd<u8, 32>;
type Mask32 = Mask<i8, 32>;

/// Entry-point called by [`crate::decode_to_slice`] when feature `simd` is active.
///
/// Preconditions (enforced by caller):
/// - `src_hex.len()` is even.
/// - `dst.len() == src_hex.len() / 2`.
pub(crate) fn decode_to_slice_simd(src_hex: &[u8], dst: &mut [u8]) -> Result<usize, Error> {
    let out_len = dst.len();

    const CHUNK_HEX: usize = 32; // 32 hex chars → 16 decoded bytes
    const CHUNK_OUT: usize = 16;

    let simd_iters = out_len / CHUNK_OUT;
    let tail_hex_start = simd_iters * CHUNK_HEX;

    // ── SIMD path ────────────────────────────────────────────────────────────
    for i in 0..simd_iters {
        let hex_offset = i * CHUNK_HEX;
        let out_offset = i * CHUNK_OUT;

        decode_chunk32(
            &src_hex[hex_offset..hex_offset + CHUNK_HEX],
            &mut dst[out_offset..out_offset + CHUNK_OUT],
            hex_offset,
        )?;
    }

    // ── Scalar tail ──────────────────────────────────────────────────────────
    let tail_hex = &src_hex[tail_hex_start..];
    if !tail_hex.is_empty() {
        let tail_dst = &mut dst[simd_iters * CHUNK_OUT..];
        decode_scalar(tail_hex, tail_dst)?;
    }

    Ok(out_len)
}

/// Decode exactly 32 ASCII-hex bytes → 16 raw bytes.
///
/// `hex_base` = absolute offset of `src32[0]` in the original input buffer;
/// used for accurate [`Error::InvalidByte`] index reporting.
#[inline(always)]
fn decode_chunk32(src32: &[u8], dst16: &mut [u8], hex_base: usize) -> Result<(), Error> {
    debug_assert_eq!(src32.len(), 32);
    debug_assert_eq!(dst16.len(), 16);

    let v = U8x32::from_slice(src32);

    // ── Classify lanes ───────────────────────────────────────────────────────
    let is_digit: Mask32 = v.simd_ge(U8x32::splat(b'0')) & v.simd_le(U8x32::splat(b'9'));
    let is_lower: Mask32 = v.simd_ge(U8x32::splat(b'a')) & v.simd_le(U8x32::splat(b'f'));
    let is_upper: Mask32 = v.simd_ge(U8x32::splat(b'A')) & v.simd_le(U8x32::splat(b'F'));
    let valid: Mask32 = is_digit | is_lower | is_upper;

    // ── Validate ─────────────────────────────────────────────────────────────
    if !valid.all() {
        // Fast path to locate the first invalid lane.
        // `to_bitmask()` packs lane truth values into a u32; bit i corresponds to lane i.
        let m: u32 = valid.to_bitmask();
        let invalid: u32 = !m;
        debug_assert!(invalid != 0);
        let lane = invalid.trailing_zeros() as usize;
        return Err(Error::InvalidByte {
            index: hex_base + lane,
            byte: src32[lane],
        });
    }

    // ── Nibble mapping ───────────────────────────────────────────────────────
    // digit_val = v - '0'         (correct for '0'..='9')
    // lower_val = v - 'a' + 10   (correct for 'a'..='f')
    // upper_val = v - 'A' + 10   (correct for 'A'..='F')
    let digit_val = v - U8x32::splat(b'0');
    let lower_val = v - U8x32::splat(b'a') + U8x32::splat(10);
    let upper_val = v - U8x32::splat(b'A') + U8x32::splat(10);

    // Branchless select: digit → lower → upper.
    // Mask::select(self, true_val, false_val)
    let nibbles: U8x32 = is_digit.select(digit_val, is_lower.select(lower_val, upper_val));

    // ── Combine pairs ─────────────────────────────────────────────────────────
    // nibbles[2*j]   = hi nibble of dst16[j]
    // nibbles[2*j+1] = lo nibble of dst16[j]
    let nibble_arr: [u8; 32] = nibbles.to_array();

    // Combine pairs (hi, lo). With fixed-size arrays and constant bounds,
    // LLVM usually removes bounds checks in optimized builds.
    for j in 0..16 {
        let hi = nibble_arr[2 * j];
        let lo = nibble_arr[2 * j + 1];
        dst16[j] = (hi << 4) | lo;
    }

    Ok(())
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    extern crate std;
    use std::prelude::v1::*;
    use crate::{decode_to_slice, Error};

    fn make_hex(n: usize) -> (Vec<u8>, Vec<u8>) {
        let src: Vec<u8> = (0..n).map(|i| i as u8).collect();
        let mut hex = vec![0u8; n * 2];
        crate::encode_to_slice(&src, &mut hex, true).unwrap();
        (src, hex)
    }

    #[test]
    fn test_simd_small() {
        let (src, hex) = make_hex(4);
        let mut dst = vec![0u8; 4];
        assert_eq!(decode_to_slice(&hex, &mut dst).unwrap(), 4);
        assert_eq!(dst, src);
    }

    #[test]
    fn test_simd_exact_chunk() {
        // 16 output bytes = exactly one 32-hex-char SIMD chunk, no tail
        let (src, hex) = make_hex(16);
        let mut dst = vec![0u8; 16];
        assert_eq!(decode_to_slice(&hex, &mut dst).unwrap(), 16);
        assert_eq!(dst, src);
    }

    #[test]
    fn test_simd_with_tail() {
        // 18 output bytes = one chunk (16) + 2-byte tail
        let (src, hex) = make_hex(18);
        let mut dst = vec![0u8; 18];
        assert_eq!(decode_to_slice(&hex, &mut dst).unwrap(), 18);
        assert_eq!(dst, src);
    }

    #[test]
    fn test_simd_large() {
        for &n in &[64usize, 256, 1024, 4096] {
            let (src, hex) = make_hex(n);
            let mut dst = vec![0u8; n];
            let w = decode_to_slice(&hex, &mut dst).unwrap();
            assert_eq!(w, n);
            assert_eq!(dst, src, "n={n}");
        }
    }

    #[test]
    fn test_simd_mixed_case() {
        let hex: &[u8] = b"DeAdBeEfDeAdBeEfDeAdBeEfDeAdBeEf";
        let expected = [0xde, 0xad, 0xbe, 0xef].repeat(4);
        let mut dst = vec![0u8; 16];
        assert_eq!(decode_to_slice(hex, &mut dst).unwrap(), 16);
        assert_eq!(dst, expected);
    }

    #[test]
    fn test_simd_invalid_in_chunk() {
        let mut hex = b"deadbeefdeadbeefdeadbeefdeadbeef".to_vec();
        hex[5] = b'X';
        let mut dst = vec![0u8; 16];
        assert_eq!(
            decode_to_slice(&hex, &mut dst).unwrap_err(),
            Error::InvalidByte { index: 5, byte: b'X' }
        );
    }

    #[test]
    fn test_simd_invalid_in_tail() {
        // 34 hex chars: 32 valid + "Xf" tail
        let hex: Vec<u8> = b"deadbeefdeadbeefdeadbeefdeadbeefXf".to_vec();
        let mut dst = vec![0u8; 17];
        assert_eq!(
            decode_to_slice(&hex, &mut dst).unwrap_err(),
            Error::InvalidByte { index: 32, byte: b'X' }
        );
    }

    #[test]
    fn test_simd_invalid_second_chunk() {
        // 64 hex bytes with bad byte at index 35 (in second chunk)
        let src: Vec<u8> = (0..32u8).collect();
        let mut hex = vec![0u8; 64];
        crate::encode_to_slice(&src, &mut hex, true).unwrap();
        hex[35] = b'Z';
        let mut dst = vec![0u8; 32];
        assert_eq!(
            decode_to_slice(&hex, &mut dst).unwrap_err(),
            Error::InvalidByte { index: 35, byte: b'Z' }
        );
    }
}
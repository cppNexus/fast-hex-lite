//! SIMD-accelerated hex decoder (feature `simd`) — STABLE (`core::arch`)
//!
//! Goals (in order):
//! - **Correctness** (no partial writes on error)
//! - **Stability** (stable Rust, no `portable_simd`)
//! - **Universality** (`x86_64` + aarch64 fast paths, scalar fallback)
//! - **Speed**
//!
//! Strategy:
//! - Validate & map 16 ASCII hex chars -> 16 nibbles (0..15)
//! - Store nibbles to a small stack array and pack pairs into bytes
//! - Scalar tail for the remainder

use crate::{decode::decode_scalar, Error};

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{
    __m128i, _mm_add_epi8, _mm_and_si128, _mm_cmpgt_epi8, _mm_cmplt_epi8, _mm_loadu_si128,
    _mm_movemask_epi8, _mm_or_si128, _mm_packus_epi16, _mm_set1_epi16, _mm_set1_epi8,
    _mm_setzero_si128, _mm_slli_epi16, _mm_srli_epi16, _mm_storel_epi64,
};

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::{
    uint16x8_t, uint8x16_t, uint8x8_t, vaddq_u8, vandq_u16, vandq_u8, vbslq_u8, vcgeq_u8, vcleq_u8,
    vdupq_n_u16, vdupq_n_u8, vld1q_u8, vminvq_u8, vmovn_u16, vorrq_u16, vorrq_u8,
    vreinterpretq_u16_u8, vshlq_n_u16, vshrq_n_u16, vst1_u8, vst1q_u8, vsubq_u8,
};

const CHUNK_HEX: usize = 16;
const CHUNK_OUT: usize = 8;

pub(crate) fn decode_to_slice_simd(src_hex: &[u8], dst: &mut [u8]) -> Result<usize, Error> {
    let out_len = dst.len();
    debug_assert_eq!(src_hex.len(), out_len * 2);

    let iters = out_len / CHUNK_OUT;
    let tail_hex_start = iters * CHUNK_HEX;

    // --- PASS 1: validate (no writes) ---
    #[cfg(target_arch = "x86_64")]
    unsafe {
        for i in 0..iters {
            let hex_off = i * CHUNK_HEX;
            validate_chunk16_sse2(&src_hex[hex_off..hex_off + CHUNK_HEX], hex_off)?;
        }
    }

    #[cfg(target_arch = "aarch64")]
    unsafe {
        for i in 0..iters {
            let hex_off = i * CHUNK_HEX;
            validate_chunk16_neon(&src_hex[hex_off..hex_off + CHUNK_HEX], hex_off)?;
        }
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        // No SIMD backend for this arch → full scalar fallback.
        return decode_scalar(src_hex, dst);
    }

    // Validate tail (scalar, no writes).
    let tail_hex = &src_hex[tail_hex_start..];
    if !tail_hex.is_empty() {
        validate_hex_scalar(tail_hex, tail_hex_start)?;
    }

    // --- PASS 2: decode (writes) ---
    #[cfg(target_arch = "x86_64")]
    {
        for i in 0..iters {
            let hex_off = i * CHUNK_HEX;
            let out_off = i * CHUNK_OUT;

            // SAFETY:
            // - Slices are exactly 16 / 8 bytes long.
            // - SSE2 is baseline on x86_64.
            unsafe {
                decode_chunk16_sse2(
                    &src_hex[hex_off..hex_off + CHUNK_HEX],
                    &mut dst[out_off..out_off + CHUNK_OUT],
                );
            }
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        for i in 0..iters {
            let hex_off = i * CHUNK_HEX;
            let out_off = i * CHUNK_OUT;

            // SAFETY:
            // - Slices are exactly 16 / 8 bytes long.
            // - NEON is baseline on aarch64.
            unsafe {
                decode_chunk16_neon(
                    &src_hex[hex_off..hex_off + CHUNK_HEX],
                    &mut dst[out_off..out_off + CHUNK_OUT],
                );
            }
        }
    }

    if !tail_hex.is_empty() {
        let tail_dst = &mut dst[iters * CHUNK_OUT..];
        decode_scalar(tail_hex, tail_dst)?;
    }

    Ok(out_len)
}

#[inline]
fn is_hex_ascii(b: u8) -> bool {
    b.is_ascii_hexdigit()
}

fn validate_hex_scalar(src_hex: &[u8], hex_base: usize) -> Result<(), Error> {
    for (i, &b) in src_hex.iter().enumerate() {
        if !is_hex_ascii(b) {
            return Err(Error::InvalidByte {
                index: hex_base + i,
                byte: b,
            });
        }
    }
    Ok(())
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
unsafe fn validate_chunk16_sse2(src16: &[u8], hex_base: usize) -> Result<(), Error> {
    debug_assert_eq!(src16.len(), 16);

    #[allow(clippy::cast_ptr_alignment)]
    let v = _mm_loadu_si128(src16.as_ptr().cast::<__m128i>());

    let ge_0 = _mm_cmpgt_epi8(v, _mm_set1_epi8((b'0' - 1).cast_signed()));
    let le_9 = _mm_cmplt_epi8(v, _mm_set1_epi8((b'9' + 1).cast_signed()));
    let is_digit = _mm_and_si128(ge_0, le_9);

    let lower = _mm_or_si128(v, _mm_set1_epi8(0x20u8.cast_signed()));

    let ge_a = _mm_cmpgt_epi8(lower, _mm_set1_epi8((b'a' - 1).cast_signed()));
    let le_f = _mm_cmplt_epi8(lower, _mm_set1_epi8((b'f' + 1).cast_signed()));
    let is_alpha = _mm_and_si128(ge_a, le_f);

    let valid = _mm_or_si128(is_digit, is_alpha);
    let mask = _mm_movemask_epi8(valid);

    if mask != -1 {
        let bad_lane = (!mask.cast_unsigned()).trailing_zeros() as usize;
        return Err(Error::InvalidByte {
            index: hex_base + bad_lane,
            byte: src16[bad_lane],
        });
    }

    Ok(())
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
unsafe fn decode_chunk16_sse2(src16: &[u8], dst8: &mut [u8]) {
    debug_assert_eq!(src16.len(), 16);
    debug_assert_eq!(dst8.len(), 8);

    // SAFETY:
    // - Caller guarantees 16-byte input and 8-byte output slices.
    // - All pointer casts are to properly aligned stack memory or unaligned loads
    //   via *_loadu_* intrinsics which support unaligned access.

    #[allow(clippy::cast_ptr_alignment)]
    let v = _mm_loadu_si128(src16.as_ptr().cast::<__m128i>());

    // lower = v | 0x20 (ASCII case fold)
    let lower = _mm_or_si128(v, _mm_set1_epi8(0x20u8.cast_signed()));

    let ge_a = _mm_cmpgt_epi8(lower, _mm_set1_epi8((b'a' - 1).cast_signed()));
    let le_f = _mm_cmplt_epi8(lower, _mm_set1_epi8((b'f' + 1).cast_signed()));
    let is_alpha = _mm_and_si128(ge_a, le_f);

    // nibble = (v & 0x0F) + (is_alpha ? 9 : 0)
    let low_nibble = _mm_and_si128(v, _mm_set1_epi8(0x0Fu8.cast_signed()));
    // add = is_alpha & 9
    let add = _mm_and_si128(is_alpha, _mm_set1_epi8(9i8));
    let nibbles = _mm_add_epi8(low_nibble, add);

    // Pack pairs (0,1)->byte0 ... (14,15)->byte7 without a temporary array.
    //
    // Layout in `nibbles` is bytes: n0 n1 n2 n3 ... n14 n15.
    // Treat as 8 little-endian u16 words: (n0 | (n1<<8)), (n2 | (n3<<8)), ...
    // Then compute ((low_byte<<4) | high_byte) per word and pack low bytes.
    let w = nibbles;
    let low = _mm_and_si128(w, _mm_set1_epi16(0x00FFu16.cast_signed()));
    let high = _mm_srli_epi16(w, 8);
    let packed_words = _mm_or_si128(_mm_slli_epi16(low, 4), high);
    let packed_bytes = _mm_packus_epi16(packed_words, _mm_setzero_si128());
    #[allow(clippy::cast_ptr_alignment)]
    _mm_storel_epi64(dst8.as_mut_ptr().cast::<__m128i>(), packed_bytes);
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn validate_chunk16_neon(src16: &[u8], hex_base: usize) -> Result<(), Error> {
    debug_assert_eq!(src16.len(), 16);

    let v: uint8x16_t = vld1q_u8(src16.as_ptr());

    let ge_0: uint8x16_t = vcgeq_u8(v, vdupq_n_u8(b'0'));
    let le_9: uint8x16_t = vcleq_u8(v, vdupq_n_u8(b'9'));
    let is_digit: uint8x16_t = vandq_u8(ge_0, le_9);

    let lower: uint8x16_t = vorrq_u8(v, vdupq_n_u8(0x20));

    let ge_a: uint8x16_t = vcgeq_u8(lower, vdupq_n_u8(b'a'));
    let le_f: uint8x16_t = vcleq_u8(lower, vdupq_n_u8(b'f'));
    let is_alpha: uint8x16_t = vandq_u8(ge_a, le_f);

    let valid: uint8x16_t = vorrq_u8(is_digit, is_alpha);
    let min_lane: u8 = vminvq_u8(valid);
    if min_lane != 0xFF {
        let mut valid_bytes = [0u8; 16];
        vst1q_u8(valid_bytes.as_mut_ptr(), valid);
        for lane in 0..16 {
            if valid_bytes[lane] != 0xFF {
                return Err(Error::InvalidByte {
                    index: hex_base + lane,
                    byte: src16[lane],
                });
            }
        }
        unreachable!("NEON validate invariant violated: min_lane != 0xFF but all lanes == 0xFF");
    }

    Ok(())
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn decode_chunk16_neon(src16: &[u8], dst8: &mut [u8]) {
    debug_assert_eq!(src16.len(), 16);
    debug_assert_eq!(dst8.len(), 8);

    // SAFETY:
    // - Caller guarantees 16-byte input and 8-byte output slices.
    // - NEON loads/stores are used with valid pointers to stack / slice memory.

    // Load 16 bytes.
    let v: uint8x16_t = vld1q_u8(src16.as_ptr());

    // is_digit: '0'..'9'
    let ge_0: uint8x16_t = vcgeq_u8(v, vdupq_n_u8(b'0'));
    let le_9: uint8x16_t = vcleq_u8(v, vdupq_n_u8(b'9'));
    let is_digit: uint8x16_t = vandq_u8(ge_0, le_9);

    // lower = v | 0x20 (ASCII case fold)
    let lower: uint8x16_t = vorrq_u8(v, vdupq_n_u8(0x20));

    // NOTE: We rely on PASS 1 validation (no partial writes) to guarantee `v` is valid hex here.

    // digit_val = v - '0'
    let digit_val: uint8x16_t = vsubq_u8(v, vdupq_n_u8(b'0'));

    // alpha_val = (lower - 'a') + 10
    let alpha_val: uint8x16_t = vaddq_u8(vsubq_u8(lower, vdupq_n_u8(b'a')), vdupq_n_u8(10));

    // Select: if digit -> digit_val else alpha_val
    let nibbles: uint8x16_t = vbslq_u8(is_digit, digit_val, alpha_val);

    // Pack pairs (0,1)->byte0 ... (14,15)->byte7 without a temporary array.
    //
    // Reinterpret as 8 little-endian u16 words: (n0 | (n1<<8)), ...
    let w: uint16x8_t = vreinterpretq_u16_u8(nibbles);
    let low: uint16x8_t = vandq_u16(w, vdupq_n_u16(0x00FF));
    let high: uint16x8_t = vshrq_n_u16(w, 8);
    let packed_words: uint16x8_t = vorrq_u16(vshlq_n_u16(low, 4), high);
    let packed_bytes: uint8x8_t = vmovn_u16(packed_words);
    vst1_u8(dst8.as_mut_ptr(), packed_bytes);
}
#[cfg(all(test, feature = "simd"))]
#[path = "simd/tests.rs"]
mod tests;

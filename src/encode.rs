use crate::Error;

/// Returns the required output length (in bytes) for encoding `n` bytes to hex.
#[inline]
pub const fn encoded_len(n: usize) -> usize {
    n * 2
}

/// Encode bytes into hex (lowercase or uppercase) into the provided output slice.
///
/// Returns the number of bytes written on success.
///
/// # Errors
///
/// Returns [`Error::OutputTooSmall`] if `dst_hex` is not large enough.
#[inline]
pub fn encode_to_slice(src: &[u8], dst_hex: &mut [u8], lowercase: bool) -> Result<usize, Error> {
    let out_len = encoded_len(src.len());
    if dst_hex.len() < out_len {
        return Err(Error::OutputTooSmall);
    }

    let alphabet = if lowercase {
        b"0123456789abcdef"
    } else {
        b"0123456789ABCDEF"
    };

    // SAFETY-FREE hot loop:
    // - dst_hex length already validated
    // - we write exactly 2 bytes per input byte
    for (byte, out_pair) in src
        .iter()
        .copied()
        .zip(dst_hex[..out_len].chunks_exact_mut(2))
    {
        out_pair[0] = alphabet[(byte >> 4) as usize];
        out_pair[1] = alphabet[(byte & 0x0f) as usize];
    }

    Ok(out_len)
}

/// Encode into a newly allocated `String`.
///
/// Available only with the `std` feature.
#[cfg(feature = "std")]
#[inline]
pub fn encode_to_string(src: &[u8], lowercase: bool) -> String {
    let mut out = vec![0u8; encoded_len(src.len())];
    // infallible because buffer is pre-sized
    let _ = encode_to_slice(src, &mut out, lowercase);

    // Always valid UTF-8 because we only write ASCII hex characters.
    String::from_utf8(out).expect("hex output is always valid UTF-8")
}

#[cfg(test)]
mod tests {
    extern crate std; // ← обязательно для no_std крейта
    use super::*;
    use std::prelude::v1::*; // Vec, String, format!, etc.

    #[test]
    fn test_encode_empty() {
        let mut out = [0u8; 0];
        assert_eq!(encode_to_slice(&[], &mut out, true).unwrap(), 0);
    }

    #[test]
    fn test_encoded_len() {
        assert_eq!(encoded_len(0), 0);
        assert_eq!(encoded_len(1), 2);
        assert_eq!(encoded_len(4), 8);
        assert_eq!(encoded_len(128), 256);
    }

    #[test]
    fn test_encode_lowercase() {
        let mut out = [0u8; 8];
        encode_to_slice(&[0xde, 0xad, 0xbe, 0xef], &mut out, true).unwrap();
        assert_eq!(&out, b"deadbeef");
    }

    #[test]
    fn test_encode_uppercase() {
        let mut out = [0u8; 8];
        encode_to_slice(&[0xde, 0xad, 0xbe, 0xef], &mut out, false).unwrap();
        assert_eq!(&out, b"DEADBEEF");
    }

    #[test]
    fn test_encode_boundary_bytes() {
        let mut out = [0u8; 4];
        encode_to_slice(&[0x00, 0xff], &mut out, true).unwrap();
        assert_eq!(&out, b"00ff");

        encode_to_slice(&[0x00, 0xff], &mut out, false).unwrap();
        assert_eq!(&out, b"00FF");
    }

    #[test]
    fn test_encode_nibble_boundaries() {
        let mut out = [0u8; 4];
        encode_to_slice(&[0x0f, 0xf0], &mut out, true).unwrap();
        assert_eq!(&out, b"0ff0");

        encode_to_slice(&[0x0f, 0xf0], &mut out, false).unwrap();
        assert_eq!(&out, b"0FF0");
    }

    #[test]
    fn test_encode_output_larger_than_needed() {
        let mut out = [0xAAu8; 10];
        let n = encode_to_slice(&[0xde, 0xad], &mut out, true).unwrap();
        assert_eq!(n, 4);
        assert_eq!(&out[..4], b"dead");
        assert_eq!(&out[4..], &[0xAAu8; 6]); // хвост не тронут
    }

    #[test]
    fn test_encode_output_exact_size() {
        let mut out = [0u8; 4];
        let n = encode_to_slice(&[0xca, 0xfe], &mut out, true).unwrap();
        assert_eq!(n, 4);
        assert_eq!(&out, b"cafe");
    }

    #[test]
    fn test_encode_output_too_small() {
        let mut out = [0u8; 2];
        assert_eq!(
            encode_to_slice(&[0xde, 0xad], &mut out, true).unwrap_err(),
            Error::OutputTooSmall
        );
    }

    #[test]
    fn test_encode_output_too_small_by_one() {
        let mut out = [0u8; 3];
        assert_eq!(
            encode_to_slice(&[0xde, 0xad], &mut out, true).unwrap_err(),
            Error::OutputTooSmall
        );
    }

    #[test]
    fn test_encode_returns_written_length() {
        let mut out = [0u8; 6];
        assert_eq!(
            encode_to_slice(&[0x01, 0x02, 0x03], &mut out, true).unwrap(),
            6
        );
    }

    #[test]
    fn test_encode_idempotent() {
        let src = [0xde, 0xad, 0xbe, 0xef];
        let mut out1 = [0u8; 8];
        let mut out2 = [0u8; 8];
        encode_to_slice(&src, &mut out1, true).unwrap();
        encode_to_slice(&src, &mut out2, true).unwrap();
        assert_eq!(out1, out2);
    }

    #[test]
    fn test_encode_all_bytes_lower() {
        for byte in 0u8..=255 {
            let mut out = [0u8; 2];
            encode_to_slice(&[byte], &mut out, true).unwrap();
            let expected = std::format!("{byte:02x}");
            assert_eq!(&out, expected.as_bytes(), "failed for 0x{byte:02x}");
        }
    }

    #[test]
    fn test_encode_all_bytes_upper() {
        for byte in 0u8..=255 {
            let mut out = [0u8; 2];
            encode_to_slice(&[byte], &mut out, false).unwrap();
            let expected = std::format!("{byte:02X}");
            assert_eq!(&out, expected.as_bytes(), "failed for 0x{byte:02X}");
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_encode_to_string_lowercase() {
        assert_eq!(
            encode_to_string(&[0xde, 0xad, 0xbe, 0xef], true),
            "deadbeef"
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_encode_to_string_uppercase() {
        assert_eq!(
            encode_to_string(&[0xde, 0xad, 0xbe, 0xef], false),
            "DEADBEEF"
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_encode_to_string_empty() {
        assert_eq!(encode_to_string(&[], true), "");
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_encode_to_string_all_bytes_are_ascii() {
        let src: Vec<u8> = (0u8..=255).collect();
        let s = encode_to_string(&src, true);
        assert!(s.is_ascii());
        assert_eq!(s.len(), 512);
    }
}

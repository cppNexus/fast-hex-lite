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
#[allow(dead_code)]
pub fn encode_to_string(src: &[u8], lowercase: bool) -> String {
    let mut out = vec![0u8; encoded_len(src.len())];
    // infallible because buffer is pre-sized
    let _ = encode_to_slice(src, &mut out, lowercase);

    // Always valid UTF-8 because we only write ASCII hex characters.
    String::from_utf8(out).expect("hex output is always valid UTF-8")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_empty() {
        let mut out = [0u8; 0];
        let written = encode_to_slice(&[], &mut out, true).unwrap();
        assert_eq!(written, 0);
    }

    #[test]
    fn test_encode_lowercase() {
        let input = [0xde, 0xad, 0xbe, 0xef];
        let mut out = [0u8; 8];
        encode_to_slice(&input, &mut out, true).unwrap();
        assert_eq!(&out, b"deadbeef");
    }

    #[test]
    fn test_encode_uppercase() {
        let input = [0xde, 0xad, 0xbe, 0xef];
        let mut out = [0u8; 8];
        encode_to_slice(&input, &mut out, false).unwrap();
        assert_eq!(&out, b"DEADBEEF");
    }

    #[test]
    fn test_encode_output_too_small() {
        let input = [0xde, 0xad];
        let mut out = [0u8; 2];
        let err = encode_to_slice(&input, &mut out, true).unwrap_err();
        assert_eq!(err, Error::OutputTooSmall);
    }
}

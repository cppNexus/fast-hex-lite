use crate::Error;

/// Returns the required output length (in bytes) for encoding `n` bytes to hex.
#[inline(always)]
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
#[inline(always)]
pub fn encode_to_slice(
    src: &[u8],
    dst_hex: &mut [u8],
    lowercase: bool,
) -> Result<usize, Error> {
    let out_len = encoded_len(src.len());
    if dst_hex.len() < out_len {
        return Err(Error::OutputTooSmall);
    }

    let alphabet = if lowercase {
        b"0123456789abcdef"
    } else {
        b"0123456789ABCDEF"
    };

    let mut src_ptr = src.as_ptr();
    let mut dst_ptr = dst_hex.as_mut_ptr();

    // SAFETY:
    // - dst_hex is validated to be large enough
    // - we advance exactly src.len() * 2 bytes in dst
    // - no overlapping regions
    unsafe {
        for _ in 0..src.len() {
            let byte = *src_ptr;
            *dst_ptr = alphabet[(byte >> 4) as usize];
            *dst_ptr.add(1) = alphabet[(byte & 0x0f) as usize];

            src_ptr = src_ptr.add(1);
            dst_ptr = dst_ptr.add(2);
        }
    }

    Ok(out_len)
}

#[cfg(feature = "std")]
/// Encode into a newly allocated `String`.
pub fn encode_to_string(src: &[u8], lowercase: bool) -> String {
    let mut out = vec![0u8; encoded_len(src.len())];
    // infallible because buffer is pre-sized
    let _ = encode_to_slice(src, &mut out, lowercase);
    unsafe { String::from_utf8_unchecked(out) }
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
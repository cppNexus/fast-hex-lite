extern crate std;
use super::*;
use crate::Error;

// ── Tests ──────────────────────────────────────────────────────────────────

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
        assert_eq!(&buf[..n], *expected, "hex: {hex:?}");
    }
}

#[test]
fn test_invalid_byte_at_zero() {
    let mut buf = [0u8; 1];
    let err = decode_to_slice(b"gz", &mut buf).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 0,
            byte: b'g'
        }
    );
}

#[test]
fn test_invalid_byte_in_middle() {
    let mut buf = [0u8; 3];
    let err = decode_to_slice(b"aabbXX", &mut buf).unwrap_err();
    // 'X' is at index 4
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 4,
            byte: b'X'
        }
    );
}

#[test]
fn test_invalid_byte_at_last() {
    let mut buf = [0u8; 2];
    let err = decode_to_slice(b"aabX", &mut buf).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 3,
            byte: b'X'
        }
    );
}

#[test]
fn test_odd_length() {
    let mut buf = [0u8; 2];
    assert_eq!(
        decode_to_slice(b"abc", &mut buf).unwrap_err(),
        Error::OddLength
    );
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
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 4,
            byte: b'X'
        }
    );
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
    let src: std::vec::Vec<u8> = (0u8..=255).collect();
    let mut hex_buf = std::vec![0u8; src.len() * 2];
    let n_enc = crate::encode_to_slice(&src, &mut hex_buf, true).unwrap();
    let mut decoded = std::vec![0u8; src.len()];
    let n_dec = decode_to_slice(&hex_buf[..n_enc], &mut decoded).unwrap();
    assert_eq!(n_dec, src.len());
    assert_eq!(decoded, src);
}

// ── decoded_len ────────────────────────────────────────────────────────

#[test]
fn test_decoded_len_large_even() {
    assert_eq!(decoded_len(1024).unwrap(), 512);
    assert_eq!(decoded_len(usize::MAX - 1).unwrap(), (usize::MAX - 1) / 2);
}

#[test]
fn test_decoded_len_large_odd() {
    assert_eq!(decoded_len(usize::MAX), Err(Error::OddLength));
    assert_eq!(decoded_len(999), Err(Error::OddLength));
}

// ── unhex_byte ─────────────────────────────────────────────────────────

#[test]
fn test_unhex_byte_digits() {
    for (c, expected) in (b'0'..=b'9').zip(0u8..=9) {
        assert_eq!(unhex_byte(c), Some(expected), "char: {}", c as char);
    }
}

#[test]
fn test_unhex_byte_lowercase() {
    for (c, expected) in (b'a'..=b'f').zip(10u8..=15) {
        assert_eq!(unhex_byte(c), Some(expected), "char: {}", c as char);
    }
}

#[test]
fn test_unhex_byte_uppercase() {
    for (c, expected) in (b'A'..=b'F').zip(10u8..=15) {
        assert_eq!(unhex_byte(c), Some(expected), "char: {}", c as char);
    }
}

#[test]
fn test_unhex_byte_invalid_exhaustive() {
    let valid: std::collections::HashSet<u8> = {
        let mut s = std::collections::HashSet::new();
        s.extend(b'0'..=b'9');
        s.extend(b'a'..=b'f');
        s.extend(b'A'..=b'F');
        s
    };
    for b in 0u8..=255 {
        if valid.contains(&b) {
            assert!(unhex_byte(b).is_some(), "expected Some for 0x{b:02x}");
        } else {
            assert_eq!(
                unhex_byte(b),
                None,
                "expected None for 0x{b:02x} ('{}')",
                b as char
            );
        }
    }
}

// ── decode_to_slice ────────────────────────────────────────────────────

#[test]
fn test_decode_to_slice_exact_output_size() {
    // dst has exactly the right length — should succeed
    let mut buf = [0u8; 2];
    assert_eq!(
        decode_to_slice(b"deadbe", &mut buf[..]),
        Err(Error::OutputTooSmall)
    );
    let mut buf = [0u8; 3];
    assert_eq!(decode_to_slice(b"deadbe", &mut buf).unwrap(), 3);
}

#[test]
fn test_decode_to_slice_larger_output() {
    // dst is larger than needed — excess bytes must stay untouched
    let mut buf = [0xFFu8; 8];
    let n = decode_to_slice(b"cafe", &mut buf).unwrap();
    assert_eq!(n, 2);
    assert_eq!(&buf[..2], &[0xca, 0xfe]);
    assert_eq!(&buf[2..], &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
}

#[test]
fn test_decode_boundary_bytes() {
    // 0x00 and 0xFF
    let mut buf = [0u8; 2];
    decode_to_slice(b"00ff", &mut buf).unwrap();
    assert_eq!(buf, [0x00, 0xff]);

    decode_to_slice(b"FF00", &mut buf).unwrap();
    assert_eq!(buf, [0xff, 0x00]);
}

#[test]
fn test_decode_invalid_hi_byte() {
    let mut buf = [0u8; 1];
    let err = decode_to_slice(b"Zf", &mut buf).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 0,
            byte: b'Z'
        }
    );
}

#[test]
fn test_decode_invalid_lo_byte() {
    let mut buf = [0u8; 1];
    let err = decode_to_slice(b"fZ", &mut buf).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 1,
            byte: b'Z'
        }
    );
}

#[test]
fn test_decode_invalid_second_pair() {
    let mut buf = [0u8; 2];
    let err = decode_to_slice(b"ffZZ", &mut buf).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 2,
            byte: b'Z'
        }
    );
}

#[test]
fn test_decode_space_is_invalid() {
    let mut buf = [0u8; 1];
    let err = decode_to_slice(b" f", &mut buf).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 0,
            byte: b' '
        }
    );
}

#[test]
fn test_decode_newline_is_invalid() {
    let mut buf = [0u8; 1];
    let err = decode_to_slice(b"f\n", &mut buf).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 1,
            byte: b'\n'
        }
    );
}

#[test]
fn test_decode_null_byte_is_invalid() {
    let mut buf = [0u8; 1];
    let err = decode_to_slice(b"\x00f", &mut buf).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 0,
            byte: 0x00
        }
    );
}

#[test]
fn test_decode_g_is_invalid() {
    // 'g' is just past 'f' — common off-by-one
    let mut buf = [0u8; 1];
    let err = decode_to_slice(b"gg", &mut buf).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 0,
            byte: b'g'
        }
    );
}

#[test]
fn test_decode_upper_g_is_invalid() {
    let mut buf = [0u8; 1];
    let err = decode_to_slice(b"GG", &mut buf).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 0,
            byte: b'G'
        }
    );
}

#[test]
fn test_decode_high_ascii_is_invalid() {
    let mut buf = [0u8; 1];
    let err = decode_to_slice(&[0xFFu8, b'0'], &mut buf).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 0,
            byte: 0xFF
        }
    );
}

// ── decode_to_array ────────────────────────────────────────────────────

#[test]
fn test_decode_to_array_zero() {
    let result = decode_to_array::<0>(b"").unwrap();
    assert_eq!(result, []);
}

#[test]
fn test_decode_to_array_one_byte() {
    let result = decode_to_array::<1>(b"ab").unwrap();
    assert_eq!(result, [0xab]);
}

#[test]
fn test_decode_to_array_too_long_input() {
    // src decodes to 3 bytes but N=2
    let err = decode_to_array::<2>(b"aabbcc").unwrap_err();
    assert_eq!(err, Error::OutputTooSmall);
}

#[test]
fn test_decode_to_array_too_short_input() {
    // src decodes to 1 byte but N=2
    let err = decode_to_array::<2>(b"aa").unwrap_err();
    assert_eq!(err, Error::OutputTooSmall);
}

#[test]
fn test_decode_to_array_odd_returns_odd_length() {
    let err = decode_to_array::<1>(b"abc").unwrap_err();
    assert_eq!(err, Error::OddLength);
}

#[test]
fn test_decode_to_array_invalid_byte() {
    let err = decode_to_array::<2>(b"aXbb").unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 1,
            byte: b'X'
        }
    );
}

// ── decode_in_place ────────────────────────────────────────────────────

#[test]
fn test_decode_in_place_empty() {
    let mut buf: [u8; 0] = [];
    assert_eq!(decode_in_place(&mut buf).unwrap(), 0);
}

#[test]
fn test_decode_in_place_uppercase() {
    let mut buf = *b"DEADBEEF";
    let n = decode_in_place(&mut buf).unwrap();
    assert_eq!(n, 4);
    assert_eq!(&buf[..n], &[0xde, 0xad, 0xbe, 0xef]);
}

#[test]
fn test_decode_in_place_mixed_case() {
    let mut buf = *b"DeAdBeEf";
    let n = decode_in_place(&mut buf).unwrap();
    assert_eq!(n, 4);
    assert_eq!(&buf[..n], &[0xde, 0xad, 0xbe, 0xef]);
}

#[test]
fn test_decode_in_place_odd_returns_error() {
    let mut buf = *b"aaa";
    assert_eq!(decode_in_place(&mut buf).unwrap_err(), Error::OddLength);
    // buffer must be unmodified
    assert_eq!(&buf, b"aaa");
}

#[test]
fn test_decode_in_place_invalid_hi_no_partial_write() {
    let mut buf = *b"Xfff";
    let err = decode_in_place(&mut buf).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 0,
            byte: b'X'
        }
    );
    assert_eq!(&buf, b"Xfff");
}

#[test]
fn test_decode_in_place_invalid_lo_no_partial_write() {
    let mut buf = *b"fXff";
    let err = decode_in_place(&mut buf).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 1,
            byte: b'X'
        }
    );
    assert_eq!(&buf, b"fXff");
}

#[test]
fn test_decode_in_place_error_at_last_pair() {
    let mut buf = *b"aabbccXX";
    let err = decode_in_place(&mut buf).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 6,
            byte: b'X'
        }
    );
    assert_eq!(&buf, b"aabbccXX"); // fully unmodified
}

#[test]
fn test_decode_in_place_all_zeros() {
    let mut buf = *b"0000";
    let n = decode_in_place(&mut buf).unwrap();
    assert_eq!(n, 2);
    assert_eq!(&buf[..n], &[0x00, 0x00]);
}

#[test]
fn test_decode_in_place_all_ff() {
    let mut buf = *b"ffff";
    let n = decode_in_place(&mut buf).unwrap();
    assert_eq!(n, 2);
    assert_eq!(&buf[..n], &[0xff, 0xff]);
}

// ── error index precision ──────────────────────────────────────────────

#[test]
fn test_error_index_precision_long_string() {
    // 10 valid pairs then an invalid one at index 20
    let mut hex = std::vec![b'a'; 20];
    hex.push(b'Z');
    hex.push(b'Z');
    let mut buf = std::vec![0u8; 11];
    let err = decode_to_slice(&hex, &mut buf).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 20,
            byte: b'Z'
        }
    );
}

#[test]
fn test_error_index_precision_lo_after_many_valid() {
    let mut hex: std::vec::Vec<u8> = std::vec![b'a'; 20];
    hex.push(b'a'); // valid hi
    hex.push(b'Z'); // invalid lo at index 21
    let mut buf = std::vec![0u8; 11];
    let err = decode_to_slice(&hex, &mut buf).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 21,
            byte: b'Z'
        }
    );
}

// ── all 256 single-byte round-trips ───────────────────────────────────

#[test]
fn test_all_single_bytes_lower() {
    for byte in 0u8..=255 {
        let hex = std::format!("{byte:02x}");
        let mut buf = [0u8; 1];
        let n = decode_to_slice(hex.as_bytes(), &mut buf).unwrap();
        assert_eq!(n, 1);
        assert_eq!(buf[0], byte, "failed for byte 0x{byte:02x}");
    }
}

#[test]
fn test_all_single_bytes_upper() {
    for byte in 0u8..=255 {
        let hex = std::format!("{byte:02X}");
        let mut buf = [0u8; 1];
        let n = decode_to_slice(hex.as_bytes(), &mut buf).unwrap();
        assert_eq!(n, 1);
        assert_eq!(buf[0], byte, "failed for byte 0x{byte:02X}");
    }
}

// ── decode_scalar public contract ─────────────────────────────────────

#[test]
fn test_decode_scalar_direct() {
    let mut dst = [0u8; 4];
    let n = decode_scalar(b"deadbeef", &mut dst).unwrap();
    assert_eq!(n, 4);
    assert_eq!(dst, [0xde, 0xad, 0xbe, 0xef]);
}

#[test]
fn test_decode_scalar_invalid_reports_correct_index() {
    let mut dst = [0u8; 2];
    let err = decode_scalar(b"ffXX", &mut dst).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 2,
            byte: b'X'
        }
    );
}

// ── length-only edge cases ─────────────────────────────────────────────

#[test]
fn test_two_byte_output() {
    let mut buf = [0u8; 2];
    assert_eq!(decode_to_slice(b"0102", &mut buf).unwrap(), 2);
    assert_eq!(buf, [0x01, 0x02]);
}

#[test]
fn test_output_too_small_by_one() {
    let mut buf = [0u8; 1];
    assert_eq!(
        decode_to_slice(b"0102", &mut buf).unwrap_err(),
        Error::OutputTooSmall
    );
}

// ── idempotency: decoding the same data twice ──────────────────────────

#[test]
fn test_decode_idempotent() {
    let src = b"cafebabe";
    let expected = [0xca, 0xfe, 0xba, 0xbe];

    let mut buf1 = [0u8; 4];
    let mut buf2 = [0u8; 4];
    decode_to_slice(src, &mut buf1).unwrap();
    decode_to_slice(src, &mut buf2).unwrap();
    assert_eq!(buf1, expected);
    assert_eq!(buf2, expected);
}

// ── full round-trip with uppercase encoding ───────────────────────────

#[test]
fn test_roundtrip_upper() {
    let src: std::vec::Vec<u8> = (0u8..=255).collect();
    let mut hex_buf = std::vec![0u8; src.len() * 2];
    let n_enc = crate::encode_to_slice(&src, &mut hex_buf, false).unwrap(); // false = uppercase
    let mut decoded = std::vec![0u8; src.len()];
    let n_dec = decode_to_slice(&hex_buf[..n_enc], &mut decoded).unwrap();
    assert_eq!(n_dec, src.len());
    assert_eq!(decoded, src);
}

// ── decode_scalar: прямой вызов, покрывает горячий цикл (lines 117-121) ──

#[test]
fn test_decode_scalar_valid_all_pairs() {
    // Покрывает тело цикла: let lo, decode_pair, проверку флага 0x0100
    let hex = b"0123456789abcdefABCDEF00ff";
    let mut dst = std::vec![0u8; hex.len() / 2];
    let n = decode_scalar(hex, &mut dst).unwrap();
    assert_eq!(n, dst.len());
}

#[test]
fn test_decode_scalar_invalid_hi_in_loop() {
    // Ошибка в hi → ветка `if unhex_byte(hi).is_none()`
    let mut dst = [0u8; 2];
    let err = decode_scalar(b"ffXX", &mut dst).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 2,
            byte: b'X'
        }
    );
}

#[test]
fn test_decode_scalar_invalid_lo_in_loop() {
    // Валидный hi, невалидный lo → ветка else (return lo error)
    let mut dst = [0u8; 2];
    let err = decode_scalar(b"fffX", &mut dst).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 3,
            byte: b'X'
        }
    );
}

#[test]
fn test_decode_scalar_single_pair() {
    let mut dst = [0u8; 1];
    decode_scalar(b"ab", &mut dst).unwrap();
    assert_eq!(dst[0], 0xab);
}

// ── decode_pair (line 164-168): покрывается через decode_in_place Pass 2 ──

#[test]
fn test_decode_pair_via_in_place_all_valid() {
    // Pass 2 вызывает decode_pair для каждой пары
    let mut buf = *b"0f1eff00";
    let n = decode_in_place(&mut buf).unwrap();
    assert_eq!(&buf[..n], &[0x0f, 0x1e, 0xff, 0x00]);
}

// ── make_unhex_table / make_hexpair_table: вызов в runtime (lines 171-210) ─
// const fn можно вызвать без const-контекста — тогда coverage tool видит строки.

#[test]
fn test_make_unhex_table_runtime_digits() {
    let t = make_unhex_table(); // runtime call, не compile-time
    for (i, c) in (b'0'..=b'9').enumerate() {
        let expected = u8::try_from(i).unwrap();
        assert_eq!(t[c as usize], expected, "digit {}", c as char);
    }
}

#[test]
fn test_make_unhex_table_runtime_lowercase() {
    let t = make_unhex_table();
    for (i, c) in (b'a'..=b'f').enumerate() {
        let expected = 10u8 + u8::try_from(i).unwrap();
        assert_eq!(t[c as usize], expected, "char {}", c as char);
    }
}

#[test]
fn test_make_unhex_table_runtime_uppercase() {
    let t = make_unhex_table();
    for (i, c) in (b'A'..=b'F').enumerate() {
        let expected = 10u8 + u8::try_from(i).unwrap();
        assert_eq!(t[c as usize], expected, "char {}", c as char);
    }
}

#[test]
fn test_make_unhex_table_runtime_invalid() {
    let t = make_unhex_table();
    // Несколько заведомо невалидных байт
    for b in [b'g', b'G', b'z', b' ', b'\n', 0x00u8, 0xFFu8] {
        assert_eq!(t[b as usize], 0xFF, "expected 0xFF for 0x{b:02x}");
    }
}

#[test]
fn test_make_hexpair_table_runtime_valid_pair() {
    let t = make_hexpair_table();
    // "de" → 0xDE
    let idx = (b'd' as usize) << 8 | (b'e' as usize);
    assert_eq!(t[idx], 0x00DE);
    // "FF" → 0xFF
    let idx = (b'F' as usize) << 8 | (b'F' as usize);
    assert_eq!(t[idx], 0x00FF);
    // "00" → 0x00
    let idx = (b'0' as usize) << 8 | (b'0' as usize);
    assert_eq!(t[idx], 0x0000);
}

#[test]
fn test_make_hexpair_table_runtime_invalid_pair() {
    let t = make_hexpair_table();
    // Невалидная пара → флаг 0x0100
    let idx = (b'X' as usize) << 8 | (b'0' as usize);
    assert_eq!(t[idx] & 0x0100, 0x0100);
    let idx = (b'0' as usize) << 8 | (b'X' as usize);
    assert_eq!(t[idx] & 0x0100, 0x0100);
}

#[test]
fn test_make_hexpair_table_runtime_all_valid_pairs_no_error_flag() {
    let t = make_hexpair_table();
    let hex_chars: &[u8] = b"0123456789abcdefABCDEF";
    for &hi in hex_chars {
        for &lo in hex_chars {
            let idx = (hi as usize) << 8 | (lo as usize);
            assert_eq!(
                t[idx] & 0x0100,
                0,
                "unexpected error flag for pair ({}, {})",
                hi as char,
                lo as char
            );
        }
    }
}

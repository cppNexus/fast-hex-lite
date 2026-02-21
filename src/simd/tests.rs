extern crate std;
use super::*;
use crate::Error;
use std::prelude::v1::*;

// Все тесты гоняют через decode_to_slice_simd — единственную pub(crate) точку входа.
// На архитектурах без SIMD-бэкенда функция падает в scalar, поэтому тесты
// корректны на любой платформе.

fn decode(hex: &[u8]) -> Result<Vec<u8>, Error> {
    let out_len = hex.len() / 2;
    let mut dst = std::vec![0u8; out_len];
    decode_to_slice_simd(hex, &mut dst)?;
    Ok(dst)
}

// ── базовые векторы ────────────────────────────────────────────────────────

#[test]
fn test_empty() {
    assert_eq!(decode(b"").unwrap(), &[]);
}

#[test]
fn test_single_byte_lower() {
    assert_eq!(decode(b"de").unwrap(), &[0xde]);
}

#[test]
fn test_single_byte_upper() {
    assert_eq!(decode(b"DE").unwrap(), &[0xde]);
}

#[test]
fn test_single_byte_mixed() {
    assert_eq!(decode(b"De").unwrap(), &[0xde]);
}

#[test]
fn test_known_vectors() {
    let cases: &[(&[u8], &[u8])] = &[
        (b"deadbeef", &[0xde, 0xad, 0xbe, 0xef]),
        (b"DEADBEEF", &[0xde, 0xad, 0xbe, 0xef]),
        (b"DeAdBeEf", &[0xde, 0xad, 0xbe, 0xef]),
        (b"00ff", &[0x00, 0xff]),
        (b"FF00", &[0xff, 0x00]),
        (b"0000", &[0x00, 0x00]),
        (b"ffff", &[0xff, 0xff]),
    ];
    for (hex, expected) in cases {
        assert_eq!(&decode(hex).unwrap(), expected, "hex: {hex:?}");
    }
}

// ── границы чанков (CHUNK_HEX = 16 bytes hex = 8 bytes out) ───────────────

#[test]
fn test_exactly_one_chunk() {
    // 16 hex chars → 8 output bytes
    let hex = b"0102030405060708";
    assert_eq!(
        decode(hex).unwrap(),
        &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]
    );
}

#[test]
fn test_exactly_two_chunks() {
    // 32 hex chars → 16 output bytes
    let hex = b"00112233445566778899aabbccddeeff";
    let expected = &[
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee,
        0xff,
    ];
    assert_eq!(&decode(hex).unwrap(), expected);
}

#[test]
fn test_one_chunk_plus_tail() {
    // 18 hex = 8 (SIMD chunk) + 2 (scalar tail) → 9 bytes
    let hex = b"000102030405060708090a"; // 22 hex → 11 bytes
    let expected: Vec<u8> = (0u8..=10).collect();
    assert_eq!(decode(hex).unwrap(), expected);
}

#[test]
fn test_tail_only_no_full_chunk() {
    // < 16 hex chars → only scalar tail, no SIMD iterations
    let hex = b"010203";
    assert_eq!(decode(hex).unwrap(), &[0x01, 0x02, 0x03]);
}

#[test]
fn test_chunk_boundary_minus_one() {
    // 14 hex → 7 bytes (just under one full SIMD chunk)
    let hex = b"01020304050607";
    assert_eq!(
        decode(hex).unwrap(),
        &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]
    );
}

#[test]
fn test_chunk_boundary_plus_one() {
    // 18 hex → 9 bytes (one full SIMD chunk + 1 tail byte)
    let hex = b"010203040506070809";
    assert_eq!(
        decode(hex).unwrap(),
        &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
    );
}

// ── все 256 байт по одному ─────────────────────────────────────────────────

#[test]
fn test_all_single_bytes_lower() {
    for byte in 0u8..=255 {
        let hex = std::format!("{byte:02x}");
        assert_eq!(
            decode(hex.as_bytes()).unwrap(),
            &[byte],
            "failed for 0x{byte:02x}"
        );
    }
}

#[test]
fn test_all_single_bytes_upper() {
    for byte in 0u8..=255 {
        let hex = std::format!("{byte:02X}");
        assert_eq!(
            decode(hex.as_bytes()).unwrap(),
            &[byte],
            "failed for 0x{byte:02X}"
        );
    }
}

// ── полный round-trip (кодирование → SIMD декодирование) ──────────────────

#[test]
fn test_roundtrip_full_256() {
    let src: Vec<u8> = (0u8..=255).collect();
    let mut hex_buf = std::vec![0u8; src.len() * 2];
    let n_enc = crate::encode_to_slice(&src, &mut hex_buf, true).unwrap();
    let decoded = decode(&hex_buf[..n_enc]).unwrap();
    assert_eq!(decoded, src);
}

#[test]
fn test_roundtrip_three_chunks() {
    // 48 hex chars → 3 SIMD chunks, no tail
    let src: Vec<u8> = (0u8..24).collect();
    let mut hex_buf = std::vec![0u8; 48];
    crate::encode_to_slice(&src, &mut hex_buf, true).unwrap();
    assert_eq!(decode(&hex_buf).unwrap(), src);
}

// ── ошибки: индекс и байт ─────────────────────────────────────────────────

#[test]
fn test_invalid_first_byte() {
    let err = decode(b"Xf").unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 0,
            byte: b'X'
        }
    );
}

#[test]
fn test_invalid_second_byte() {
    let err = decode(b"fX").unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 1,
            byte: b'X'
        }
    );
}

#[test]
fn test_invalid_byte_in_simd_chunk() {
    // Ошибка внутри первого SIMD чанка (позиция 4 из 16)
    let hex = *b"0102Xf0405060708";
    let err = decode(&hex).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 4,
            byte: b'X'
        }
    );
}

#[test]
fn test_invalid_byte_in_second_chunk() {
    // Первый чанк валиден, ошибка в начале второго
    let mut hex: Vec<u8> = b"0102030405060708".to_vec(); // chunk 1 OK
    hex.extend_from_slice(b"Xf030405060708"); // chunk 2 bad at +0
    let err = decode(&hex).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 16,
            byte: b'X'
        }
    );
}

#[test]
fn test_invalid_byte_in_tail() {
    // Чанк валиден, ошибка в хвосте
    let mut hex: Vec<u8> = b"0102030405060708".to_vec(); // chunk OK
    hex.extend_from_slice(b"Xf"); // tail bad at +0
    let err = decode(&hex).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 16,
            byte: b'X'
        }
    );
}

#[test]
fn test_invalid_byte_space() {
    // Even-length input, but contains a space in the middle.
    // Expect to fail on the first invalid byte (space at index 2).
    let err = decode(b"ff ff ").unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 2,
            byte: b' ',
        }
    );
}

#[test]
fn test_invalid_g_off_by_one() {
    // 'g' = 'f' + 1, классический off-by-one
    let err = decode(b"gg").unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 0,
            byte: b'g'
        }
    );
}

#[test]
fn test_invalid_g_upper_off_by_one() {
    let err = decode(b"GG").unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 0,
            byte: b'G'
        }
    );
}

#[test]
fn test_invalid_null_byte() {
    let err = decode(&[0x00, b'f']).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 0,
            byte: 0x00
        }
    );
}

#[test]
fn test_invalid_high_ascii() {
    let err = decode(&[0xFF, b'0']).unwrap_err();
    assert_eq!(
        err,
        Error::InvalidByte {
            index: 0,
            byte: 0xFF
        }
    );
}

// ── no partial writes on error ─────────────────────────────────────────────

#[test]
fn test_no_partial_write_on_error_in_chunk() {
    // Ошибка в середине первого SIMD чанка — dst не должен быть изменён
    let hex = b"0102030405Xf0708";
    let mut dst = std::vec![0xAAu8; 8];
    let _ = decode_to_slice_simd(hex, &mut dst);
    assert!(
        dst.iter().all(|&b| b == 0xAA),
        "dst was partially written: {dst:?}"
    );
}

#[test]
fn test_no_partial_write_on_error_in_tail() {
    // Первый чанк OK, хвост плохой — первые 8 байт dst НЕ должны быть записаны
    let mut hex: Vec<u8> = b"0102030405060708".to_vec();
    hex.extend_from_slice(b"ZZ");
    let mut dst = std::vec![0xAAu8; 9];
    let _ = decode_to_slice_simd(&hex, &mut dst);
    assert!(
        dst.iter().all(|&b| b == 0xAA),
        "dst was partially written: {dst:?}"
    );
}

// ── идемпотентность ────────────────────────────────────────────────────────

#[test]
fn test_idempotent() {
    let hex = b"deadbeefcafebabe0011223344556677";
    let first = decode(hex).unwrap();
    let second = decode(hex).unwrap();
    assert_eq!(first, second);
}

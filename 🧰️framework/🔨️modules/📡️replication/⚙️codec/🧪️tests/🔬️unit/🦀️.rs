
use super::*;

// 🔤️ Base64 codec tests moved with the implementation to `semio-framework-io-base64`'s own
// `🦀️.rs` — this crate now only re-exports the functions.

//#region 🔖️Limits
#[test]
fn pack_limits_default_matches_contract() {
    let limits = PackLimits::default();
    assert_eq!(limits.max_file_len, 16 * 1024 * 1024 * 1024);
    assert_eq!(limits.max_segment_len, 256 * 1024 * 1024);
    assert_eq!(limits.max_symbols, 1_000_000);
    assert_eq!(limits.max_depth, 64);
    assert_eq!(limits.max_items, 64_000_000);
    assert_eq!(limits.max_total_alloc, 4 * 1024 * 1024 * 1024);
}
//#endregion 🔖️Limits

//#region 🔖️Varint
#[test]
fn varint_u64_round_trips_boundary_values() {
    let values: &[u64] = &[0, 1, 0x7F, 0x80, 0x3FFF, 0x4000, 0x1F_FFFF, 0x20_0000, u32::MAX as u64, u32::MAX as u64 + 1, u64::MAX / 2, u64::MAX - 1, u64::MAX];
    for &value in values {
        let mut out = Vec::new();
        write_varint_u64(&mut out, value);
        assert!(out.len() <= 10, "varint for {value} exceeded 10 bytes");
        let mut pos = 0usize;
        let decoded = read_varint_u64(&out, &mut pos).unwrap();
        assert_eq!(decoded, value);
        assert_eq!(pos, out.len());
        assert!(is_minimal_varint(&out), "encoding of {value} should be minimal");
    }
}

#[test]
fn varint_u64_max_value_uses_ten_bytes() {
    let mut out = Vec::new();
    write_varint_u64(&mut out, u64::MAX);
    assert_eq!(out.len(), 10);
}

#[test]
fn varint_i64_round_trips_boundary_values() {
    let values: &[i64] = &[0, 1, -1, 63, -64, 64, -65, i32::MIN as i64, i32::MAX as i64, i64::MIN, i64::MAX];
    for &value in values {
        let mut out = Vec::new();
        write_varint_i64(&mut out, value);
        let mut pos = 0usize;
        let decoded = read_varint_i64(&out, &mut pos).unwrap();
        assert_eq!(decoded, value);
        assert_eq!(pos, out.len());
    }
}

#[test]
fn varint_multi_byte_sequence_reads_each_value_in_order() {
    let mut buf = Vec::new();
    write_varint_u64(&mut buf, 300);
    write_varint_u64(&mut buf, 1);
    write_varint_u64(&mut buf, 0x4000);
    let mut pos = 0usize;
    assert_eq!(read_varint_u64(&buf, &mut pos).unwrap(), 300);
    assert_eq!(read_varint_u64(&buf, &mut pos).unwrap(), 1);
    assert_eq!(read_varint_u64(&buf, &mut pos).unwrap(), 0x4000);
    assert_eq!(pos, buf.len());
}

#[test]
fn varint_read_truncated_input_errors_never_panics() {
    let mut pos = 0usize;
    assert_eq!(read_varint_u64(&[], &mut pos), Err(PackError::Truncated(0)));
    pos = 0;
    assert_eq!(read_varint_u64(&[0x80], &mut pos), Err(PackError::Truncated(1)));
    pos = 0;
    assert_eq!(read_varint_u64(&[0x80, 0x80, 0x80], &mut pos), Err(PackError::Truncated(3)));
}

#[test]
fn varint_read_overlong_eleven_bytes_is_malformed() {
    let overlong = [0x80u8; 11];
    let mut pos = 0usize;
    let result = read_varint_u64(&overlong, &mut pos);
    assert!(matches!(result, Err(PackError::Malformed { .. })));
}

#[test]
fn varint_read_tenth_byte_with_extra_bits_is_malformed() {
    let mut bytes = vec![0x80u8; 9];
    bytes.push(0x02);
    let mut pos = 0usize;
    let result = read_varint_u64(&bytes, &mut pos);
    assert!(matches!(result, Err(PackError::Malformed { .. })));
}

#[test]
fn is_minimal_varint_rejects_non_minimal_encoding_of_zero() {
    assert!(is_minimal_varint(&[0x00]));
    assert!(!is_minimal_varint(&[0x80, 0x00]));
}

#[test]
fn is_minimal_varint_rejects_trailing_garbage() {
    let mut out = Vec::new();
    write_varint_u64(&mut out, 5);
    out.push(0xFF);
    assert!(!is_minimal_varint(&out));
}
//#endregion 🔖️Varint

//#region 🔖️Bytes
#[test]
fn byte_reader_writer_round_trip_all_types() {
    let mut writer = ByteWriter::new();
    writer.write_u8(0x42);
    writer.write_u16_le(0x1234);
    writer.write_u32_le(0xDEAD_BEEF);
    writer.write_u64_le(0x0123_4567_89AB_CDEF);
    writer.write_f64_le(std::f64::consts::PI);
    writer.write_varint_u64(300);
    writer.write_varint_i64(-42);
    writer.write_bytes(&[1, 2, 3, 4]);
    let array32: [u8; 32] = [7u8; 32];
    writer.write_bytes(&array32);
    let bytes = writer.into_bytes();

    let mut reader = ByteReader::new(&bytes);
    assert_eq!(reader.position(), 0);
    assert_eq!(reader.read_u8().unwrap(), 0x42);
    assert_eq!(reader.read_u16_le().unwrap(), 0x1234);
    assert_eq!(reader.read_u32_le().unwrap(), 0xDEAD_BEEF);
    assert_eq!(reader.read_u64_le().unwrap(), 0x0123_4567_89AB_CDEF);
    assert_eq!(reader.read_f64_le().unwrap(), std::f64::consts::PI);
    assert_eq!(reader.read_varint_u64().unwrap(), 300);
    assert_eq!(reader.read_varint_i64().unwrap(), -42);
    assert_eq!(reader.read_bytes(4).unwrap(), &[1, 2, 3, 4]);
    assert_eq!(reader.read_array32().unwrap(), array32);
    assert_eq!(reader.remaining(), 0);
    assert_eq!(reader.position(), bytes.len());
}

#[test]
fn byte_reader_bounds_checked_reads_never_panic_on_truncated_input() {
    let bytes = [1u8, 2, 3];
    let mut reader = ByteReader::new(&bytes);
    assert_eq!(reader.read_u32_le(), Err(PackError::Truncated(0)));
    assert_eq!(reader.position(), 0);
    assert!(reader.read_bytes(1).is_ok());
    assert_eq!(reader.read_u64_le(), Err(PackError::Truncated(1)));
    assert_eq!(reader.read_array32(), Err(PackError::Truncated(1)));
    let empty: [u8; 0] = [];
    let mut empty_reader = ByteReader::new(&empty);
    assert_eq!(empty_reader.read_u8(), Err(PackError::Truncated(0)));
    assert_eq!(empty_reader.read_bytes(0).unwrap(), &empty[..]);
}
//#endregion 🔖️Bytes

//#region 🔖️Crc
#[test]
fn crc32c_matches_known_test_vector() {
    assert_eq!(crc32c(b"123456789"), 0xE306_9283);
}

#[test]
fn retained_crc32c_pages_match_the_contiguous_oracle() {
    let bytes = b"a fixed-page CRC cursor must preserve the canonical checksum";
    let mut cursor = Crc32cCursor::new();
    cursor.update_page(&bytes[..17]);
    cursor.update_page(&bytes[17..41]);
    cursor.update_page(&bytes[41..]);
    assert_eq!(cursor.finish(), crc32c(bytes));
}

#[test]
fn crc32c_empty_input_is_zero() {
    assert_eq!(crc32c(b""), 0);
}

#[test]
fn crc32c_differs_for_different_inputs() {
    assert_ne!(crc32c(b"abc"), crc32c(b"abd"));
}
//#endregion 🔖️Crc

//#region 🔖️Codec
#[test]
fn no_compression_round_trips_identity() {
    let codec = NoCompression;
    assert_eq!(codec.id(), CodecId(0));
    let raw = b"the quick brown fox";
    let compressed = codec.compress(raw).unwrap();
    assert_eq!(compressed, raw);
    let decompressed = codec.decompress(&compressed, raw.len() as u64, 1_000_000).unwrap();
    assert_eq!(decompressed, raw);
}

#[test]
fn no_compression_decompress_rejects_raw_len_over_limit_before_allocating() {
    let codec = NoCompression;
    let stored = vec![0u8; 16];
    let result = codec.decompress(&stored, 1_000_000_000, 1_000);
    assert!(matches!(result, Err(PackError::LimitExceeded(_))));
}

#[test]
fn no_compression_decompress_rejects_stored_len_mismatch() {
    let codec = NoCompression;
    let stored = vec![0u8; 4];
    let result = codec.decompress(&stored, 5, 1_000);
    assert!(matches!(result, Err(PackError::Malformed { .. })));
}
//#endregion 🔖️Codec

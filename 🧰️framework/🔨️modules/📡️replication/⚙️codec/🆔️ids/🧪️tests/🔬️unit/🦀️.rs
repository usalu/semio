
use super::*;

//#region 🔖️Ids
#[semio_framework_async_macros::async_test]
async fn content_hash_display_is_lowercase_hex() {
    let mut bytes = [0u8; 32];
    bytes[0] = 0xAB;
    bytes[31] = 0x0F;
    let hash = ContentHash(bytes);
    let text = hash.to_string();
    assert_eq!(text.len(), 64);
    assert!(text.starts_with("ab"));
    assert!(text.ends_with("0f"));
    assert_eq!(text, text.to_lowercase());
}

#[semio_framework_async_macros::async_test]
async fn segment_kind_constants_match_contract() {
    assert_eq!(KIND_END, 0x00);
    assert_eq!(KIND_MANIFEST, 0x01);
    assert_eq!(KIND_SCHEMA, 0x02);
    assert_eq!(KIND_SYMBOLS, 0x03);
    assert_eq!(KIND_DOCUMENT, 0x04);
    assert_eq!(KIND_CHUNK, 0x05);
    assert_eq!(KIND_CHUNK_TABLE, 0x06);
    assert_eq!(KIND_SNAPSHOT, 0x07);
    assert_eq!(KIND_FIELD_INDEX, 0x08);
    assert_eq!(KIND_PADDING, 0x7F);
}
//#endregion 🔖️Ids

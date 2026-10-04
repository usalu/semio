//! 💾️ Operation-specific binary payload codec for insert-array-element/InsertArrayElement.
use super::InsertArrayElementPayload;
pub const BINARY_TAG: u32 = dsl::protocol_record::tag_u32(include_str!("../../💾️binary/📡️.protocol.semio"), "insert-array-element");
pub fn encode_payload(value: &InsertArrayElementPayload) -> Result<Vec<u8>, String> {
    Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(value))).into_bytes())
}
pub fn decode_payload(value: &[u8]) -> Result<InsertArrayElementPayload, String> {
    let parsed = semio_framework_pack_json::parse_bytes(value, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    <InsertArrayElementPayload as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| error.to_string())
}

//! 📝️ Operation-specific text payload codec for insert-array-element.
use super::InsertArrayElementPayload;
pub const TEXT_OPCODE: &str = "insert-array-element";
pub fn encode_payload(value: &InsertArrayElementPayload) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(value))))
}
pub fn decode_payload(value: &str) -> Result<InsertArrayElementPayload, String> {
    let parsed = semio_framework_pack_json::parse(value, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    <InsertArrayElementPayload as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| error.to_string())
}

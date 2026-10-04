//! 📝️ Operation-specific text payload codec for insert-element.
use super::InsertElementPayload;
pub const TEXT_OPCODE: &str = "insert-element";
pub fn encode_payload(value: &InsertElementPayload) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(value))
}
pub fn decode_payload(value: &str) -> Result<InsertElementPayload, String> {
    semio_framework_pack_json::from_json_str(value, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

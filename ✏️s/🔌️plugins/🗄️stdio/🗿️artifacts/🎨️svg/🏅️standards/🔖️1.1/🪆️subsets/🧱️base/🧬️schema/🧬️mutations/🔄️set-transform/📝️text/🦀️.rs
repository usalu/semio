//! 📝️ Operation-specific text payload codec for set-transform.
use super::SetTransformPayload;
pub const TEXT_OPCODE: &str = "set-transform";
pub fn encode_payload(value: &SetTransformPayload) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(value))
}
pub fn decode_payload(value: &str) -> Result<SetTransformPayload, String> {
    semio_framework_pack_json::from_json_str(value, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

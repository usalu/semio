//! 📝️ Operation-specific text payload codec for set-element-name.
use super::SetElementNamePayload;
pub const TEXT_OPCODE: &str = "set-element-name";
pub fn encode_payload(value: &SetElementNamePayload) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(value))
}
pub fn decode_payload(value: &str) -> Result<SetElementNamePayload, String> {
    semio_framework_pack_json::from_json_str(value, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

//! 📝️ Operation-specific text payload codec for set-declaration.
use super::SetDeclarationPayload;
pub const TEXT_OPCODE: &str = "set-declaration";
pub fn encode_payload(value: &SetDeclarationPayload) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(value))
}
pub fn decode_payload(value: &str) -> Result<SetDeclarationPayload, String> {
    semio_framework_pack_json::from_json_str(value, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

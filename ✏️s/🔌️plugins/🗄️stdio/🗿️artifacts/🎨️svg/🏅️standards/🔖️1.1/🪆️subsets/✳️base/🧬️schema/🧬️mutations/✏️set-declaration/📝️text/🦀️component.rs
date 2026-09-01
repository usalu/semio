//! 📝️ Operation-specific text payload codec for set-declaration.
use super::SetDeclarationPayload;
pub const TEXT_OPCODE: &str = "set-declaration";
pub fn encode_payload(value: &SetDeclarationPayload) -> Result<String, String> { serde_json::to_string(value).map_err(|error| error.to_string()) }
pub fn decode_payload(value: &str) -> Result<SetDeclarationPayload, String> { serde_json::from_str(value).map_err(|error| error.to_string()) }

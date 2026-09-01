//! 📝️ Operation-specific text payload codec for set-declaration.
use super::SetDeclarationPayload;
pub const TEXT_OPCODE: &str = "set-declaration";
pub fn encode_payload(value: &SetDeclarationPayload) -> Result<String, String> { Ok(pack::to_json_string(value)) }
pub fn decode_payload(value: &str) -> Result<SetDeclarationPayload, String> { pack::from_json_str(value).map_err(|error| error.to_string()) }

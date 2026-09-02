//! 📝️ Operation-specific text payload codec for set-text.
use super::SetTextPayload;
pub const TEXT_OPCODE: &str = "set-text";
pub fn encode_payload(value: &SetTextPayload) -> Result<String, String> { Ok(pack::to_json_string(value)) }
pub fn decode_payload(value: &str) -> Result<SetTextPayload, String> { pack::from_json_str(value).map_err(|error| error.to_string()) }

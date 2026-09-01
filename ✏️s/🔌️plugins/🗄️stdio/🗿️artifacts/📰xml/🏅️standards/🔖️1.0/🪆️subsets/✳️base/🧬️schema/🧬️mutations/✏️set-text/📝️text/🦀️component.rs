//! 📝️ Operation-specific text payload codec for set-text.
use super::SetTextPayload;
pub const TEXT_OPCODE: &str = "set-text";
pub fn encode_payload(value: &SetTextPayload) -> Result<String, String> { serde_json::to_string(value).map_err(|error| error.to_string()) }
pub fn decode_payload(value: &str) -> Result<SetTextPayload, String> { serde_json::from_str(value).map_err(|error| error.to_string()) }

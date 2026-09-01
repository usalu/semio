//! 📝️ Operation-specific text payload codec for set-doctype.
use super::SetDoctypePayload;
pub const TEXT_OPCODE: &str = "set-doctype";
pub fn encode_payload(value: &SetDoctypePayload) -> Result<String, String> { serde_json::to_string(value).map_err(|error| error.to_string()) }
pub fn decode_payload(value: &str) -> Result<SetDoctypePayload, String> { serde_json::from_str(value).map_err(|error| error.to_string()) }

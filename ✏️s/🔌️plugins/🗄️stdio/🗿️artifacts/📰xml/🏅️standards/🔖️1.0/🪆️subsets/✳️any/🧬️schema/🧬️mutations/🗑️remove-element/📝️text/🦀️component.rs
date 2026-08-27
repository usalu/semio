//! 📝️ Operation-specific text payload codec for remove-element.
use super::RemoveElementPayload;
pub const TEXT_OPCODE: &str = "remove-element";
pub fn encode_payload(value: &RemoveElementPayload) -> Result<String, String> { serde_json::to_string(value).map_err(|error| error.to_string()) }
pub fn decode_payload(value: &str) -> Result<RemoveElementPayload, String> { serde_json::from_str(value).map_err(|error| error.to_string()) }

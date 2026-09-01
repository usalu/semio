//! 📝️ Operation-specific text payload codec for remove-array-element.
use super::RemoveArrayElementPayload;
pub const TEXT_OPCODE: &str = "remove-array-element";
pub fn encode_payload(value: &RemoveArrayElementPayload) -> Result<String, String> { serde_json::to_string(value).map_err(|error| error.to_string()) }
pub fn decode_payload(value: &str) -> Result<RemoveArrayElementPayload, String> { serde_json::from_str(value).map_err(|error| error.to_string()) }

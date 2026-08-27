//! 📝️ Operation-specific text payload codec for insert-array-element.
use super::InsertArrayElementPayload;
pub const TEXT_OPCODE: &str = "insert-array-element";
pub fn encode_payload(value: &InsertArrayElementPayload) -> Result<String, String> { serde_json::to_string(value).map_err(|error| error.to_string()) }
pub fn decode_payload(value: &str) -> Result<InsertArrayElementPayload, String> { serde_json::from_str(value).map_err(|error| error.to_string()) }

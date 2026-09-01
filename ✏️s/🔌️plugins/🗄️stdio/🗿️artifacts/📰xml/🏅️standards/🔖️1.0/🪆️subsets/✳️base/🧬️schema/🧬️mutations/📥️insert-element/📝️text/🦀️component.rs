//! 📝️ Operation-specific text payload codec for insert-element.
use super::InsertElementPayload;
pub const TEXT_OPCODE: &str = "insert-element";
pub fn encode_payload(value: &InsertElementPayload) -> Result<String, String> { serde_json::to_string(value).map_err(|error| error.to_string()) }
pub fn decode_payload(value: &str) -> Result<InsertElementPayload, String> { serde_json::from_str(value).map_err(|error| error.to_string()) }

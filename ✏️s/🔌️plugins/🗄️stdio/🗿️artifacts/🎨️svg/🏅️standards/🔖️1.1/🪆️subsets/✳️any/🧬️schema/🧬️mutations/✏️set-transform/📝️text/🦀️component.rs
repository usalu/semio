//! 📝️ Operation-specific text payload codec for set-transform.
use super::SetTransformPayload;
pub const TEXT_OPCODE: &str = "set-transform";
pub fn encode_payload(value: &SetTransformPayload) -> Result<String, String> { serde_json::to_string(value).map_err(|error| error.to_string()) }
pub fn decode_payload(value: &str) -> Result<SetTransformPayload, String> { serde_json::from_str(value).map_err(|error| error.to_string()) }

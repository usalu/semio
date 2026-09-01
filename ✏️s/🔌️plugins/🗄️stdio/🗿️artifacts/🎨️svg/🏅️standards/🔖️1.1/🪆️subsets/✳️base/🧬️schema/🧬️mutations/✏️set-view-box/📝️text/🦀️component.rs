//! 📝️ Operation-specific text payload codec for set-view-box.
use super::SetViewBoxPayload;
pub const TEXT_OPCODE: &str = "set-view-box";
pub fn encode_payload(value: &SetViewBoxPayload) -> Result<String, String> { serde_json::to_string(value).map_err(|error| error.to_string()) }
pub fn decode_payload(value: &str) -> Result<SetViewBoxPayload, String> { serde_json::from_str(value).map_err(|error| error.to_string()) }

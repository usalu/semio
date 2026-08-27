//! 📝️ Operation-specific text payload codec for set-element-name.
use super::SetElementNamePayload;
pub const TEXT_OPCODE: &str = "set-element-name";
pub fn encode_payload(value: &SetElementNamePayload) -> Result<String, String> { serde_json::to_string(value).map_err(|error| error.to_string()) }
pub fn decode_payload(value: &str) -> Result<SetElementNamePayload, String> { serde_json::from_str(value).map_err(|error| error.to_string()) }

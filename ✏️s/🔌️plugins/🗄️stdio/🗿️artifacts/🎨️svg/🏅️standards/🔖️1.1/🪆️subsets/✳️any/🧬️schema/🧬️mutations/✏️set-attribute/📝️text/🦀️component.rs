//! 📝️ Operation-specific text payload codec for set-attribute.
use super::SetAttributePayload;
pub const TEXT_OPCODE: &str = "set-attribute";
pub fn encode_payload(value: &SetAttributePayload) -> Result<String, String> { serde_json::to_string(value).map_err(|error| error.to_string()) }
pub fn decode_payload(value: &str) -> Result<SetAttributePayload, String> { serde_json::from_str(value).map_err(|error| error.to_string()) }

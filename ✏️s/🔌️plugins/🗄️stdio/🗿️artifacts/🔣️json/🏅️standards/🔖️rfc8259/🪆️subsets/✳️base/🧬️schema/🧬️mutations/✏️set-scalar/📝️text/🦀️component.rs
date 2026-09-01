//! 📝️ Operation-specific text payload codec for set-scalar.
use super::SetScalarPayload;
pub const TEXT_OPCODE: &str = "set-scalar";
pub fn encode_payload(value: &SetScalarPayload) -> Result<String, String> { serde_json::to_string(value).map_err(|error| error.to_string()) }
pub fn decode_payload(value: &str) -> Result<SetScalarPayload, String> { serde_json::from_str(value).map_err(|error| error.to_string()) }

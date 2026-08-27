//! 💾️ Operation-specific binary payload codec for set-attribute/SetAttribute.
use super::SetAttributePayload;
pub const BINARY_TAG: u32 = 5;
pub fn encode_payload(value: &SetAttributePayload) -> Result<Vec<u8>, String> { serde_json::to_vec(value).map_err(|error| error.to_string()) }
pub fn decode_payload(value: &[u8]) -> Result<SetAttributePayload, String> { serde_json::from_slice(value).map_err(|error| error.to_string()) }

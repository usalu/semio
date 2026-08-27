//! 💾️ Operation-specific binary payload codec for remove-array-element/RemoveArrayElement.
use super::RemoveArrayElementPayload;
pub const BINARY_TAG: u32 = 4;
pub fn encode_payload(value: &RemoveArrayElementPayload) -> Result<Vec<u8>, String> { serde_json::to_vec(value).map_err(|error| error.to_string()) }
pub fn decode_payload(value: &[u8]) -> Result<RemoveArrayElementPayload, String> { serde_json::from_slice(value).map_err(|error| error.to_string()) }

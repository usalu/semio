//! 💾️ Operation-specific binary payload codec for remove-element/RemoveElement.
use super::RemoveElementPayload;
pub const BINARY_TAG: u32 = 4;
pub fn encode_payload(value: &RemoveElementPayload) -> Result<Vec<u8>, String> { serde_json::to_vec(value).map_err(|error| error.to_string()) }
pub fn decode_payload(value: &[u8]) -> Result<RemoveElementPayload, String> { serde_json::from_slice(value).map_err(|error| error.to_string()) }

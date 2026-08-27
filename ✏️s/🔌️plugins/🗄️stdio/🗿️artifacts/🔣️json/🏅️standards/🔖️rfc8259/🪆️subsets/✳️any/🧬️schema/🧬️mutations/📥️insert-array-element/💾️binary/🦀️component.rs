//! 💾️ Operation-specific binary payload codec for insert-array-element/InsertArrayElement.
use super::InsertArrayElementPayload;
pub const BINARY_TAG: u32 = 3;
pub fn encode_payload(value: &InsertArrayElementPayload) -> Result<Vec<u8>, String> { serde_json::to_vec(value).map_err(|error| error.to_string()) }
pub fn decode_payload(value: &[u8]) -> Result<InsertArrayElementPayload, String> { serde_json::from_slice(value).map_err(|error| error.to_string()) }

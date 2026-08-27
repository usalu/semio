//! 💾️ Operation-specific binary payload codec for set-transform/SetTransform.
use super::SetTransformPayload;
pub const BINARY_TAG: u32 = 9;
pub fn encode_payload(value: &SetTransformPayload) -> Result<Vec<u8>, String> { serde_json::to_vec(value).map_err(|error| error.to_string()) }
pub fn decode_payload(value: &[u8]) -> Result<SetTransformPayload, String> { serde_json::from_slice(value).map_err(|error| error.to_string()) }

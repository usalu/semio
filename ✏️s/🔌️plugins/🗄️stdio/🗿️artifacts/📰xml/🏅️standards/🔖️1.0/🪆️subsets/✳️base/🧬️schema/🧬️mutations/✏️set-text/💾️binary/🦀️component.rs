//! 💾️ Operation-specific binary payload codec for set-text/SetText.
use super::SetTextPayload;
pub const BINARY_TAG: u32 = 6;
pub fn encode_payload(value: &SetTextPayload) -> Result<Vec<u8>, String> { serde_json::to_vec(value).map_err(|error| error.to_string()) }
pub fn decode_payload(value: &[u8]) -> Result<SetTextPayload, String> { serde_json::from_slice(value).map_err(|error| error.to_string()) }

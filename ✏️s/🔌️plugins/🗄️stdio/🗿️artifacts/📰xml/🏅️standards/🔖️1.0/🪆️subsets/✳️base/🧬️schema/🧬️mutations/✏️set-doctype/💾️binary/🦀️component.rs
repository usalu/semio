//! 💾️ Operation-specific binary payload codec for set-doctype/SetDoctype.
use super::SetDoctypePayload;
pub const BINARY_TAG: u32 = 2;
pub fn encode_payload(value: &SetDoctypePayload) -> Result<Vec<u8>, String> { serde_json::to_vec(value).map_err(|error| error.to_string()) }
pub fn decode_payload(value: &[u8]) -> Result<SetDoctypePayload, String> { serde_json::from_slice(value).map_err(|error| error.to_string()) }

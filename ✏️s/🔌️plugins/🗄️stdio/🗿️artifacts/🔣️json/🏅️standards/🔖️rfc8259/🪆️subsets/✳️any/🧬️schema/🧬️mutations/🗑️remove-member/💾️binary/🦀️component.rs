//! 💾️ Operation-specific binary payload codec for remove-member/RemoveMember.
use super::RemoveMemberPayload;
pub const BINARY_TAG: u32 = 2;
pub fn encode_payload(value: &RemoveMemberPayload) -> Result<Vec<u8>, String> { serde_json::to_vec(value).map_err(|error| error.to_string()) }
pub fn decode_payload(value: &[u8]) -> Result<RemoveMemberPayload, String> { serde_json::from_slice(value).map_err(|error| error.to_string()) }

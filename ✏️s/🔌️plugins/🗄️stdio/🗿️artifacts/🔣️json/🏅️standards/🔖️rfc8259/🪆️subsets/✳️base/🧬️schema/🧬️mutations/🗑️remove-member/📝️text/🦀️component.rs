//! 📝️ Operation-specific text payload codec for remove-member.
use super::RemoveMemberPayload;
pub const TEXT_OPCODE: &str = "remove-member";
pub fn encode_payload(value: &RemoveMemberPayload) -> Result<String, String> { serde_json::to_string(value).map_err(|error| error.to_string()) }
pub fn decode_payload(value: &str) -> Result<RemoveMemberPayload, String> { serde_json::from_str(value).map_err(|error| error.to_string()) }

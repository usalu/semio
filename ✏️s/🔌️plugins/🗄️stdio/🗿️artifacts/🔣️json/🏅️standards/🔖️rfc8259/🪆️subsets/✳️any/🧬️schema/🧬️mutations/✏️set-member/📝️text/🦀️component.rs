//! 📝️ Operation-specific text payload codec for set-member.
use super::SetMemberPayload;
pub const TEXT_OPCODE: &str = "set-member";
pub fn encode_payload(value: &SetMemberPayload) -> Result<String, String> { serde_json::to_string(value).map_err(|error| error.to_string()) }
pub fn decode_payload(value: &str) -> Result<SetMemberPayload, String> { serde_json::from_str(value).map_err(|error| error.to_string()) }

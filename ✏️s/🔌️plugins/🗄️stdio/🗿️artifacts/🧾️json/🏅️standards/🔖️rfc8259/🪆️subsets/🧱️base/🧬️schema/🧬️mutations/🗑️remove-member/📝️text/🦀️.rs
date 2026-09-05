//! 📝️ Operation-specific text payload codec for remove-member.
use super::RemoveMemberPayload;
pub const TEXT_OPCODE: &str = "remove-member";
pub fn encode_payload(value: &RemoveMemberPayload) -> Result<String, String> { Ok(pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(value)))) }
pub fn decode_payload(value: &str) -> Result<RemoveMemberPayload, String> { let parsed = pack::parse_json(value).map_err(|error| error.to_string())?; <RemoveMemberPayload as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| error.to_string()) }

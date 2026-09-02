//! 💾️ Operation-specific binary payload codec for set-member/SetMember.
use super::SetMemberPayload;
pub const BINARY_TAG: u32 = 1;
pub fn encode_payload(value: &SetMemberPayload) -> Result<Vec<u8>, String> { Ok(pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(value))).into_bytes()) }
pub fn decode_payload(value: &[u8]) -> Result<SetMemberPayload, String> { let parsed = pack::parse_json_bytes(value).map_err(|error| error.to_string())?; <SetMemberPayload as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| error.to_string()) }

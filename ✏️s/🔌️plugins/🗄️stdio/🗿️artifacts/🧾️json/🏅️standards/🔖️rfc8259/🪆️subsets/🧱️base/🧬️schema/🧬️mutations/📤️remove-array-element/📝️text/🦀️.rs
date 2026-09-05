//! 📝️ Operation-specific text payload codec for remove-array-element.
use super::RemoveArrayElementPayload;
pub const TEXT_OPCODE: &str = "remove-array-element";
pub fn encode_payload(value: &RemoveArrayElementPayload) -> Result<String, String> { Ok(pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(value)))) }
pub fn decode_payload(value: &str) -> Result<RemoveArrayElementPayload, String> { let parsed = pack::parse_json(value).map_err(|error| error.to_string())?; <RemoveArrayElementPayload as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| error.to_string()) }

//! 📝️ Operation-specific text payload codec for insert-array-element.
use super::InsertArrayElementPayload;
pub const TEXT_OPCODE: &str = "insert-array-element";
pub fn encode_payload(value: &InsertArrayElementPayload) -> Result<String, String> { Ok(pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(value)))) }
pub fn decode_payload(value: &str) -> Result<InsertArrayElementPayload, String> { let parsed = pack::parse_json(value).map_err(|error| error.to_string())?; <InsertArrayElementPayload as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| error.to_string()) }

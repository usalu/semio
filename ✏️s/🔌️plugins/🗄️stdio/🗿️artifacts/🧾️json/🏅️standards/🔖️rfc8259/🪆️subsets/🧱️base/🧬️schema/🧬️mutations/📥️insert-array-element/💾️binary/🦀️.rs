//! 💾️ Operation-specific binary payload codec for insert-array-element/InsertArrayElement.
use super::InsertArrayElementPayload;
pub const BINARY_TAG: u32 = 3;
pub fn encode_payload(value: &InsertArrayElementPayload) -> Result<Vec<u8>, String> { Ok(pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(value))).into_bytes()) }
pub fn decode_payload(value: &[u8]) -> Result<InsertArrayElementPayload, String> { let parsed = pack::parse_json_bytes(value).map_err(|error| error.to_string())?; <InsertArrayElementPayload as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| error.to_string()) }

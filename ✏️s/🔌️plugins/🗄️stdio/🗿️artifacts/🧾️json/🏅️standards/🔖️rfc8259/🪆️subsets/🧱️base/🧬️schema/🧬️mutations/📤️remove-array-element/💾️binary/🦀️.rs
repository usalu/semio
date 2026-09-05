//! 💾️ Operation-specific binary payload codec for remove-array-element/RemoveArrayElement.
use super::RemoveArrayElementPayload;
pub const BINARY_TAG: u32 = 4;
pub fn encode_payload(value: &RemoveArrayElementPayload) -> Result<Vec<u8>, String> { Ok(pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(value))).into_bytes()) }
pub fn decode_payload(value: &[u8]) -> Result<RemoveArrayElementPayload, String> { let parsed = pack::parse_json_bytes(value).map_err(|error| error.to_string())?; <RemoveArrayElementPayload as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| error.to_string()) }

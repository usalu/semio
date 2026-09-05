//! 📝️ Operation-specific text payload codec for set-scalar.
use super::SetScalarPayload;
pub const TEXT_OPCODE: &str = "set-scalar";
pub fn encode_payload(value: &SetScalarPayload) -> Result<String, String> { Ok(pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(value)))) }
pub fn decode_payload(value: &str) -> Result<SetScalarPayload, String> { let parsed = pack::parse_json(value).map_err(|error| error.to_string())?; <SetScalarPayload as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| error.to_string()) }

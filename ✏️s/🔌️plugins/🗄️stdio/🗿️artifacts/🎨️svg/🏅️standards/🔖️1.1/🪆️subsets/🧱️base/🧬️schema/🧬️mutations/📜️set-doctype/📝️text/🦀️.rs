//! 📝️ Operation-specific text payload codec for set-doctype.
use super::SetDoctypePayload;
pub const TEXT_OPCODE: &str = "set-doctype";
pub fn encode_payload(value: &SetDoctypePayload) -> Result<String, String> { Ok(pack::to_json_string(value)) }
pub fn decode_payload(value: &str) -> Result<SetDoctypePayload, String> { pack::from_json_str(value).map_err(|error| error.to_string()) }

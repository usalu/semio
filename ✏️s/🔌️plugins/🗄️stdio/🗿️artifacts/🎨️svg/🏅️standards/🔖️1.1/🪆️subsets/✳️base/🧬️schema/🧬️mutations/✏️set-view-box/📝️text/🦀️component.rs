//! 📝️ Operation-specific text payload codec for set-view-box.
use super::SetViewBoxPayload;
pub const TEXT_OPCODE: &str = "set-view-box";
pub fn encode_payload(value: &SetViewBoxPayload) -> Result<String, String> { Ok(pack::to_json_string(value)) }
pub fn decode_payload(value: &str) -> Result<SetViewBoxPayload, String> { pack::from_json_str(value).map_err(|error| error.to_string()) }

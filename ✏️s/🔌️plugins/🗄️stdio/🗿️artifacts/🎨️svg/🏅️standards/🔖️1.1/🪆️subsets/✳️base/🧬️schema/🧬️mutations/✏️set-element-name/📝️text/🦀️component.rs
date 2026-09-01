//! 📝️ Operation-specific text payload codec for set-element-name.
use super::SetElementNamePayload;
pub const TEXT_OPCODE: &str = "set-element-name";
pub fn encode_payload(value: &SetElementNamePayload) -> Result<String, String> { Ok(pack::to_json_string(value)) }
pub fn decode_payload(value: &str) -> Result<SetElementNamePayload, String> { pack::from_json_str(value).map_err(|error| error.to_string()) }

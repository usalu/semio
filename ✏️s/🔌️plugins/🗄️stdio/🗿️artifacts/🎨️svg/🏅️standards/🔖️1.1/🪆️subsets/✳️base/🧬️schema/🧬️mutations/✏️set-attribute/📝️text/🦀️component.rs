//! 📝️ Operation-specific text payload codec for set-attribute.
use super::SetAttributePayload;
pub const TEXT_OPCODE: &str = "set-attribute";
pub fn encode_payload(value: &SetAttributePayload) -> Result<String, String> { Ok(pack::to_json_string(value)) }
pub fn decode_payload(value: &str) -> Result<SetAttributePayload, String> { pack::from_json_str(value).map_err(|error| error.to_string()) }

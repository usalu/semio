//! 📝️ Operation-specific text payload codec for remove-element.
use super::RemoveElementPayload;
pub const TEXT_OPCODE: &str = "remove-element";
pub fn encode_payload(value: &RemoveElementPayload) -> Result<String, String> { Ok(pack::to_json_string(value)) }
pub fn decode_payload(value: &str) -> Result<RemoveElementPayload, String> { pack::from_json_str(value).map_err(|error| error.to_string()) }

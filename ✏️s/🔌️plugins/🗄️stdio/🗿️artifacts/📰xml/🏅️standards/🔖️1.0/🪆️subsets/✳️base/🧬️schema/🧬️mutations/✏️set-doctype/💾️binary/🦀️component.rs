//! 💾️ Operation-specific binary payload codec for set-doctype/SetDoctype.
use super::SetDoctypePayload;
pub const BINARY_TAG: u32 = 2;
pub fn encode_payload(value: &SetDoctypePayload) -> Result<Vec<u8>, String> { Ok(pack::to_json_string(value).into_bytes()) }
pub fn decode_payload(value: &[u8]) -> Result<SetDoctypePayload, String> { let text = std::str::from_utf8(value).map_err(|error| error.to_string())?; pack::from_json_str(text).map_err(|error| error.to_string()) }

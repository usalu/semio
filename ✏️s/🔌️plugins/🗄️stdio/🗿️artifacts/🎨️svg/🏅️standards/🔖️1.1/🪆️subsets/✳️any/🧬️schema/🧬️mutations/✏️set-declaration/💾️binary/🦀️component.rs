//! 💾️ Operation-specific binary payload codec for set-declaration/SetDeclaration.
use super::SetDeclarationPayload;
pub const BINARY_TAG: u32 = 1;
pub fn encode_payload(value: &SetDeclarationPayload) -> Result<Vec<u8>, String> { serde_json::to_vec(value).map_err(|error| error.to_string()) }
pub fn decode_payload(value: &[u8]) -> Result<SetDeclarationPayload, String> { serde_json::from_slice(value).map_err(|error| error.to_string()) }

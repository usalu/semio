//! 💾️ Direct `replace-model` binary payload codec.

use super::ReplaceModel;

/// 🏷️ Stable binary tag for `ReplaceModel`.
pub const BINARY_TAG: u32 = 0;

/// 📦️ Encodes the direct payload.
pub fn encode_payload(value: &ReplaceModel) -> Result<Vec<u8>, String> {
    serde_json::to_vec(value).map_err(|error| error.to_string())
}

/// 📖️ Decodes the direct payload.
pub fn decode_payload(value: &[u8]) -> Result<ReplaceModel, String> {
    serde_json::from_slice(value).map_err(|error| error.to_string())
}

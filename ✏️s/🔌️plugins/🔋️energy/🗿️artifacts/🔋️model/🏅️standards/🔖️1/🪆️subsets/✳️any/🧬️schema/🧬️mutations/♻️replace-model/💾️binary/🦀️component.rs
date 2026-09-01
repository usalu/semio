//! 💾️ Direct `replace-model` binary payload codec.

use super::ReplaceModel;

/// 🏷️ Stable binary tag for `ReplaceModel`.
pub const BINARY_TAG: u32 = 0;

/// 📦️ Encodes the direct payload.
pub fn encode_payload(value: &ReplaceModel) -> Result<Vec<u8>, String> {
    Ok(pack::json::to_json_string(value).into_bytes())
}

/// 📖️ Decodes the direct payload.
pub fn decode_payload(value: &[u8]) -> Result<ReplaceModel, String> {
    let text = std::str::from_utf8(value).map_err(|error| error.to_string())?;
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

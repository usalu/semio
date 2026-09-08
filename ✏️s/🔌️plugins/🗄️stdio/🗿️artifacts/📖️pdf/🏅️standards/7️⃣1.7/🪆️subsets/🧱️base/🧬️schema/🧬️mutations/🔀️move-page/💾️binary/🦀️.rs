//! 🔀️ Direct binary identity for `move-page`.

pub const TAG: u8 = 13;
pub const BINARY_TAG: u8 = TAG;

use super::MovePage;

/// 📤️ Encodes this direct payload as canonical schema JSON bytes.
pub fn encode(payload: &MovePage) -> Result<Vec<u8>, String> {
    Ok(pack::to_json_string(payload).into_bytes())
}

/// 📥️ Decodes this direct payload from canonical schema JSON bytes.
pub fn decode(bytes: &[u8]) -> Result<MovePage, String> {
    let parsed = pack::parse_json_bytes(bytes).map_err(|error| error.to_string())?;
    dsl::FromValue::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| error.to_string())
}

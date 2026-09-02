//! ✂️ Direct binary identity for `set-page-crop-box`.

pub const TAG: u8 = 3;
pub const BINARY_TAG: u8 = TAG;

use super::SetPageCropBox;

/// 📤️ Encodes this direct payload as canonical schema JSON bytes.
pub fn encode(payload: &SetPageCropBox) -> Result<Vec<u8>, String> {
    Ok(pack::to_json_string(payload).into_bytes())
}

/// 📥️ Decodes this direct payload from canonical schema JSON bytes.
pub fn decode(bytes: &[u8]) -> Result<SetPageCropBox, String> {
    let parsed = pack::parse_json_bytes(bytes).map_err(|error| error.to_string())?;
    dsl::FromValue::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| error.to_string())
}

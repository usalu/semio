//! ✂️ Direct text identity for `set-page-crop-box`.

pub const OPCODE: &str = "set-page-crop-box";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetPageCropBox;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetPageCropBox) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetPageCropBox, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

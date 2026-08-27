//! ✂️ Direct text identity for `set-page-crop-box`.

pub const OPCODE: &str = "set-page-crop-box";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetPageCropBox;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetPageCropBox) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetPageCropBox, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

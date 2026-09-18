//! 🌫️ Direct text identity for `remove-image`.

pub const OPCODE: &str = "remove-image";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveImage;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveImage) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveImage, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

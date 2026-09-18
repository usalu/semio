//! 🎨️ Direct text identity for `remove-color-space`.

pub const OPCODE: &str = "remove-color-space";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveColorSpace;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveColorSpace) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveColorSpace, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

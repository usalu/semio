//! 🔄️ Direct text identity for `set-page-rotation`.

pub const OPCODE: &str = "set-page-rotation";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetPageRotation;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetPageRotation) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetPageRotation, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

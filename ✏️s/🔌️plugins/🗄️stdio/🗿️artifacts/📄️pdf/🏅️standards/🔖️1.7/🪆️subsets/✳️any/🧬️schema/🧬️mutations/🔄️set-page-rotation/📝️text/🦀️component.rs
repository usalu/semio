//! 🔄️ Direct text identity for `set-page-rotation`.

pub const OPCODE: &str = "set-page-rotation";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetPageRotation;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetPageRotation) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetPageRotation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

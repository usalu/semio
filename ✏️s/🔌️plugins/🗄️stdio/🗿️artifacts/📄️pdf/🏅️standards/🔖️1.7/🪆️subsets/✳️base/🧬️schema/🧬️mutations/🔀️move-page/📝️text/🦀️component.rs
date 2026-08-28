//! 🔀️ Direct text identity for `move-page`.

pub const OPCODE: &str = "move-page";
pub const TEXT_OPCODE: &str = OPCODE;

use super::MovePage;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &MovePage) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<MovePage, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

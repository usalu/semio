//! 🗑️ Direct text identity for `remove-page`.

pub const OPCODE: &str = "remove-page";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemovePage;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemovePage) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemovePage, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

//! 🧽️ Direct text identity for `remove-trailer-entry`.

pub const OPCODE: &str = "remove-trailer-entry";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveTrailerEntry;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveTrailerEntry) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveTrailerEntry, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

//! 🧳️ Direct text identity for `set-trailer-entry`.

pub const OPCODE: &str = "set-trailer-entry";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetTrailerEntry;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetTrailerEntry) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetTrailerEntry, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

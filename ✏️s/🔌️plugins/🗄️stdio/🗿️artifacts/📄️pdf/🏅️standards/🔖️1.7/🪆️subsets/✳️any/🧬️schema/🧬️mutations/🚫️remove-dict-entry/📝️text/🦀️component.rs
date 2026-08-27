//! 🚫️ Direct text identity for `remove-dict-entry`.

pub const OPCODE: &str = "remove-dict-entry";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveDictEntry;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveDictEntry) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveDictEntry, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

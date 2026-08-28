//! 🔑️ Direct text identity for `set-dict-entry`.

pub const OPCODE: &str = "set-dict-entry";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetDictEntry;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetDictEntry) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetDictEntry, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

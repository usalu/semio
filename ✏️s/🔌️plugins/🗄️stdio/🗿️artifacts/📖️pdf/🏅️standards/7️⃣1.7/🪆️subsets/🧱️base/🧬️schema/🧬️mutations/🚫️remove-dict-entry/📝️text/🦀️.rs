//! 🚫️ Direct text identity for `remove-dict-entry`.

pub const OPCODE: &str = "remove-dict-entry";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveDictEntry;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveDictEntry) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveDictEntry, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

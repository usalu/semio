//! 🔑️ Direct text identity for `set-dict-entry`.

pub const OPCODE: &str = "set-dict-entry";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetDictEntry;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetDictEntry) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetDictEntry, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

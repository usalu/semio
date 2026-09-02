//! 🧳️ Direct text identity for `set-trailer-entry`.

pub const OPCODE: &str = "set-trailer-entry";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetTrailerEntry;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetTrailerEntry) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetTrailerEntry, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

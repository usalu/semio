//! 🧽️ Direct text identity for `remove-trailer-entry`.

pub const OPCODE: &str = "remove-trailer-entry";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveTrailerEntry;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveTrailerEntry) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveTrailerEntry, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

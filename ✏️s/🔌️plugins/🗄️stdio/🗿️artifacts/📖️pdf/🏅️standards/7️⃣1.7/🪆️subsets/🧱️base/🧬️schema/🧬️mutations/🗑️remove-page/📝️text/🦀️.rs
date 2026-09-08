//! 🗑️ Direct text identity for `remove-page`.

pub const OPCODE: &str = "remove-page";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemovePage;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemovePage) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemovePage, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

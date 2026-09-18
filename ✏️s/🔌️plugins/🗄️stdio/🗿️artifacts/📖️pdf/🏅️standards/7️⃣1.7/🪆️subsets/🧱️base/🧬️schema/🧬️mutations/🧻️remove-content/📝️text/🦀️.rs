//! 🧻️ Direct text identity for `remove-content`.

pub const OPCODE: &str = "remove-content";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveContent;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveContent) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveContent, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

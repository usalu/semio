//! 🪡️ Direct text identity for `remove-pattern`.

pub const OPCODE: &str = "remove-pattern";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemovePattern;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemovePattern) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemovePattern, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

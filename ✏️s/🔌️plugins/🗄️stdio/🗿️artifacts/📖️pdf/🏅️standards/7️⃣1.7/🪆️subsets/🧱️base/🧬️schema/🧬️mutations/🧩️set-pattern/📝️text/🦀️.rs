//! 🧩️ Direct text identity for `set-pattern`.

pub const OPCODE: &str = "set-pattern";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetPattern;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetPattern) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetPattern, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

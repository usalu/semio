//! 🔤️ Direct text identity for `set-font`.

pub const OPCODE: &str = "set-font";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetFont;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetFont) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetFont, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

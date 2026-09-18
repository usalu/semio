//! 🔁️ Direct text identity for `replace-content`.

pub const OPCODE: &str = "replace-content";
pub const TEXT_OPCODE: &str = OPCODE;

use super::ReplaceContent;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &ReplaceContent) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<ReplaceContent, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

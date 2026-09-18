//! 🗣️ Direct text identity for `set-language`.

pub const OPCODE: &str = "set-language";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetLanguage;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetLanguage) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetLanguage, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

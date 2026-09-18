//! 🖥️ Direct text identity for `set-page-mode`.

pub const OPCODE: &str = "set-page-mode";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetPageMode;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetPageMode) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetPageMode, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

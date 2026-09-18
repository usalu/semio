//! 🔢️ Direct text identity for `set-page-labels`.

pub const OPCODE: &str = "set-page-labels";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetPageLabels;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetPageLabels) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetPageLabels, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

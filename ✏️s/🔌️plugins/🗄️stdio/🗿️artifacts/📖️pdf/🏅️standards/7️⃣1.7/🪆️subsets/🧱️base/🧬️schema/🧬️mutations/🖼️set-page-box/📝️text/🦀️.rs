//! 🖼️ Direct text identity for `set-page-box`.

pub const OPCODE: &str = "set-page-box";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetPageBox;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetPageBox) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetPageBox, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

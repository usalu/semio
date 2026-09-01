//! ℹ️ Direct text identity for `set-info`.

pub const OPCODE: &str = "set-info";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetInfo;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetInfo) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetInfo, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

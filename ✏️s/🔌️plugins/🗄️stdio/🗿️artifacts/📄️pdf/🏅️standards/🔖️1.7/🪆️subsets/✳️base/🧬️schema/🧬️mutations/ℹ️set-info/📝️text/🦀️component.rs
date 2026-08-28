//! ℹ️ Direct text identity for `set-info`.

pub const OPCODE: &str = "set-info";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetInfo;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetInfo) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetInfo, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

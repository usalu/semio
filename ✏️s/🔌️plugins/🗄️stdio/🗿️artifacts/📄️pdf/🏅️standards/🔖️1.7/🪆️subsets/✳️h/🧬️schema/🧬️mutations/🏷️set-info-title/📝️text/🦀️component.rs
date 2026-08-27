//! Direct text identity for `set-info-title`.

pub const OPCODE: &str = "set-info-title";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetInfoTitle;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetInfoTitle) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetInfoTitle, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

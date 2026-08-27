//! Direct text identity for `remove-font-file`.

pub const OPCODE: &str = "remove-font-file";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveFontFile;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveFontFile) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveFontFile, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

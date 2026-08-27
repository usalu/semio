//! 🧹️ Direct text identity for `remove-object`.

pub const OPCODE: &str = "remove-object";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveObject;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveObject) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveObject, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

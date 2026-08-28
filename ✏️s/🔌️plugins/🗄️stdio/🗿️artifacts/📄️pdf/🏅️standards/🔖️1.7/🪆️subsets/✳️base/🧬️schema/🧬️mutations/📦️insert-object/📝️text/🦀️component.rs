//! 📦️ Direct text identity for `insert-object`.

pub const OPCODE: &str = "insert-object";
pub const TEXT_OPCODE: &str = OPCODE;

use super::InsertObject;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &InsertObject) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<InsertObject, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

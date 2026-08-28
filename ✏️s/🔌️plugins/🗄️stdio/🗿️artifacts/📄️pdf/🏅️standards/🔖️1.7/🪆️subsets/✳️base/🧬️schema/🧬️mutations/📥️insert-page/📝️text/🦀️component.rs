//! 📥️ Direct text identity for `insert-page`.

pub const OPCODE: &str = "insert-page";
pub const TEXT_OPCODE: &str = OPCODE;

use super::InsertPage;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &InsertPage) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<InsertPage, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

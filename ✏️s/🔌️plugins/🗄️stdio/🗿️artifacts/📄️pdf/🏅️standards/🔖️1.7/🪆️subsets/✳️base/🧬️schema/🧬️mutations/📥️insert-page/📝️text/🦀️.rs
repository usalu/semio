//! 📥️ Direct text identity for `insert-page`.

pub const OPCODE: &str = "insert-page";
pub const TEXT_OPCODE: &str = OPCODE;

use super::InsertPage;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &InsertPage) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<InsertPage, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

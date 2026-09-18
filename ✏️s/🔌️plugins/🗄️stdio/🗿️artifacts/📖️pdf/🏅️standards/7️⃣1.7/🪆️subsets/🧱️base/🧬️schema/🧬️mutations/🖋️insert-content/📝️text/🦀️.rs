//! 🖋️ Direct text identity for `insert-content`.

pub const OPCODE: &str = "insert-content";
pub const TEXT_OPCODE: &str = OPCODE;

use super::InsertContent;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &InsertContent) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<InsertContent, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

//! 📌️ Direct text identity for `insert-annotation`.

pub const OPCODE: &str = "insert-annotation";
pub const TEXT_OPCODE: &str = OPCODE;

use super::InsertAnnotation;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &InsertAnnotation) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<InsertAnnotation, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

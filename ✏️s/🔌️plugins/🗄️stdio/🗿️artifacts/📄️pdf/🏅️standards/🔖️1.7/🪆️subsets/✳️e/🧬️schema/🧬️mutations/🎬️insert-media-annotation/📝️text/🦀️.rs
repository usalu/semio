//! Direct text identity for `insert-media-annotation`.

pub const OPCODE: &str = "insert-media-annotation";
pub const TEXT_OPCODE: &str = OPCODE;

use super::InsertMediaAnnotation;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &InsertMediaAnnotation) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<InsertMediaAnnotation, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

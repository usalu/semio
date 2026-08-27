//! Direct text identity for `insert-media-annotation`.

pub const OPCODE: &str = "insert-media-annotation";
pub const TEXT_OPCODE: &str = OPCODE;

use super::InsertMediaAnnotation;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &InsertMediaAnnotation) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<InsertMediaAnnotation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

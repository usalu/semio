//! Direct text identity for `remove-media-annotation`.

pub const OPCODE: &str = "remove-media-annotation";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveMediaAnnotation;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveMediaAnnotation) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveMediaAnnotation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

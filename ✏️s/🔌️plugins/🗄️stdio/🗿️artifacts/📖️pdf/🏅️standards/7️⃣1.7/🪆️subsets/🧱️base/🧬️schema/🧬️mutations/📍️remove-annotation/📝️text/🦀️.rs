//! 📍️ Direct text identity for `remove-annotation`.

pub const OPCODE: &str = "remove-annotation";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveAnnotation;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveAnnotation) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveAnnotation, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

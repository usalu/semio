//! Direct text identity for `remove-media-annotation`.

pub const OPCODE: &str = "remove-media-annotation";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveMediaAnnotation;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveMediaAnnotation) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveMediaAnnotation, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

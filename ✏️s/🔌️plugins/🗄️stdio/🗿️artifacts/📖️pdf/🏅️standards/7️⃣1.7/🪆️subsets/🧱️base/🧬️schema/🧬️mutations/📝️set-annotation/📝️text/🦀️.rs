//! 📝️ Direct text identity for `set-annotation`.

pub const OPCODE: &str = "set-annotation";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetAnnotation;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetAnnotation) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetAnnotation, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

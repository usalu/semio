//! 🏞️ Direct text identity for `set-image`.

pub const OPCODE: &str = "set-image";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetImage;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetImage) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetImage, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

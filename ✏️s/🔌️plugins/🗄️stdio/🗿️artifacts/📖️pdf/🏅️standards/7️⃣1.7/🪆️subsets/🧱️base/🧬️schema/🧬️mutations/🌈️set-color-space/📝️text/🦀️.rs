//! 🌈️ Direct text identity for `set-color-space`.

pub const OPCODE: &str = "set-color-space";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetColorSpace;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetColorSpace) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetColorSpace, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

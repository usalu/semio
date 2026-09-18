//! 🌅️ Direct text identity for `set-shading`.

pub const OPCODE: &str = "set-shading";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetShading;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetShading) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetShading, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

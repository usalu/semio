//! 🌄️ Direct text identity for `remove-shading`.

pub const OPCODE: &str = "remove-shading";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveShading;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveShading) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveShading, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

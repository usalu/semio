//! 🧹️ Direct text identity for `remove-object`.

pub const OPCODE: &str = "remove-object";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveObject;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveObject) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveObject, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

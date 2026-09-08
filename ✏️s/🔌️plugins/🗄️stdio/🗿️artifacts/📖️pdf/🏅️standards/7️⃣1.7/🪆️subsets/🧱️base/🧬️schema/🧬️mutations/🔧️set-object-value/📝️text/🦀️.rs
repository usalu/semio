//! 🔧️ Direct text identity for `set-object-value`.

pub const OPCODE: &str = "set-object-value";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetObjectValue;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetObjectValue) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetObjectValue, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

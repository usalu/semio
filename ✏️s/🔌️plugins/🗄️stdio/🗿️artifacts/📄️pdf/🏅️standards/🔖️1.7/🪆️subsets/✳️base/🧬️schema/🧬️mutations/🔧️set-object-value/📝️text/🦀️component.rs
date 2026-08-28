//! 🔧️ Direct text identity for `set-object-value`.

pub const OPCODE: &str = "set-object-value";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetObjectValue;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetObjectValue) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetObjectValue, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

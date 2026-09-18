//! 🔖️ Direct text identity for `remove-properties`.

pub const OPCODE: &str = "remove-properties";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveProperties;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveProperties) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveProperties, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

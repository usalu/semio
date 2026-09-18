//! 🏷️ Direct text identity for `set-properties`.

pub const OPCODE: &str = "set-properties";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetProperties;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetProperties) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetProperties, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

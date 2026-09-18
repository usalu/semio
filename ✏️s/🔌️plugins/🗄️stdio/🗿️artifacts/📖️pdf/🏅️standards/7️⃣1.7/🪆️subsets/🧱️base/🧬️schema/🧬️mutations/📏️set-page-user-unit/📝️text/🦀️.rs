//! 📏️ Direct text identity for `set-page-user-unit`.

pub const OPCODE: &str = "set-page-user-unit";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetPageUserUnit;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetPageUserUnit) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetPageUserUnit, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

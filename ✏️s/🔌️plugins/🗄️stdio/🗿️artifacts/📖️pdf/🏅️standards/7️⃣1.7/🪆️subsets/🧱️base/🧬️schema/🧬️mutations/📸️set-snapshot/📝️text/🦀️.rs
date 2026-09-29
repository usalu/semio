//! 📸️ Direct text identity for `set-snapshot`.

pub const OPCODE: &str = "set-snapshot";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetSnapshot;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetSnapshot) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetSnapshot, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

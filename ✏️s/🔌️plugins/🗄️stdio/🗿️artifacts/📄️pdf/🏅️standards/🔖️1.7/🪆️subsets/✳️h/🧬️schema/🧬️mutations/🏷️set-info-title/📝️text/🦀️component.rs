//! Direct text identity for `set-info-title`.

pub const OPCODE: &str = "set-info-title";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetInfoTitle;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetInfoTitle) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetInfoTitle, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

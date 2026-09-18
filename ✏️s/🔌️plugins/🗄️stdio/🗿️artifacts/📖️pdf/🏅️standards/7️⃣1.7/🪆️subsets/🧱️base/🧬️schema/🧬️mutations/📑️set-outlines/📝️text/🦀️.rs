//! 📑️ Direct text identity for `set-outlines`.

pub const OPCODE: &str = "set-outlines";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetOutlines;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetOutlines) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetOutlines, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

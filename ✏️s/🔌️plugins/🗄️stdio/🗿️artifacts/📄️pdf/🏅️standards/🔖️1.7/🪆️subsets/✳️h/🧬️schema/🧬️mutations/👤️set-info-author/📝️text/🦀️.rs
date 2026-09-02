//! Direct text identity for `set-info-author`.

pub const OPCODE: &str = "set-info-author";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetInfoAuthor;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetInfoAuthor) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetInfoAuthor, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

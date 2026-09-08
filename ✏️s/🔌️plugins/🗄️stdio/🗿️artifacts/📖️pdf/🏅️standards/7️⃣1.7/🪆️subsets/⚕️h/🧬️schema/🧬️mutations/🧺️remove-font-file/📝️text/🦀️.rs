//! Direct text identity for `remove-font-file`.

pub const OPCODE: &str = "remove-font-file";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveFontFile;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveFontFile) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveFontFile, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

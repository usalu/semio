//! 📎️ Direct text identity for `set-embedded-file`.

pub const OPCODE: &str = "set-embedded-file";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetEmbeddedFile;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetEmbeddedFile) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetEmbeddedFile, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

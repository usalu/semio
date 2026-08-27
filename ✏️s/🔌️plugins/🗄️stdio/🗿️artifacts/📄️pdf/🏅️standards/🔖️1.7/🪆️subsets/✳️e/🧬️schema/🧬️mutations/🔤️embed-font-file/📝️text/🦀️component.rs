//! Direct text identity for `embed-font-file`.

pub const OPCODE: &str = "embed-font-file";
pub const TEXT_OPCODE: &str = OPCODE;

use super::EmbedFontFile;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &EmbedFontFile) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<EmbedFontFile, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

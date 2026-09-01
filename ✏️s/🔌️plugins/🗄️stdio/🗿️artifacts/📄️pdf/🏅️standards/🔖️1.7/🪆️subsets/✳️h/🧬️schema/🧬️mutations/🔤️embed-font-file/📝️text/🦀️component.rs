//! Direct text identity for `embed-font-file`.

pub const OPCODE: &str = "embed-font-file";
pub const TEXT_OPCODE: &str = OPCODE;

use super::EmbedFontFile;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &EmbedFontFile) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<EmbedFontFile, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

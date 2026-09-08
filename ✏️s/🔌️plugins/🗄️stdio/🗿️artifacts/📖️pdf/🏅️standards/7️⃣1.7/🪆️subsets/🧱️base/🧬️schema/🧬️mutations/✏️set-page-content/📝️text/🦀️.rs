//! ✏️ Direct text identity for `set-page-content`.

pub const OPCODE: &str = "set-page-content";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetPageContent;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetPageContent) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetPageContent, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

//! ✏️ Direct text identity for `set-page-content`.

pub const OPCODE: &str = "set-page-content";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetPageContent;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetPageContent) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetPageContent, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

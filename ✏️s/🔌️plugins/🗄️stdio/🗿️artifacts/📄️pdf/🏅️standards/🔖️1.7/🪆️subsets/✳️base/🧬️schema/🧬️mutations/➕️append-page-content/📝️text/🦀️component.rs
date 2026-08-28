//! ➕️ Direct text identity for `append-page-content`.

pub const OPCODE: &str = "append-page-content";
pub const TEXT_OPCODE: &str = OPCODE;

use super::AppendPageContent;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &AppendPageContent) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<AppendPageContent, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

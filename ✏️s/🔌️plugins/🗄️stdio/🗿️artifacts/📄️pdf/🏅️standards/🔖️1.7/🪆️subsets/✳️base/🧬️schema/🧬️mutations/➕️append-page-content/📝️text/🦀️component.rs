//! ➕️ Direct text identity for `append-page-content`.

pub const OPCODE: &str = "append-page-content";
pub const TEXT_OPCODE: &str = OPCODE;

use super::AppendPageContent;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &AppendPageContent) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<AppendPageContent, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

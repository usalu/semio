//! ➕️ Direct text identity for `append-page-content`.

pub const OPCODE: &str = "append-page-content";
pub const TEXT_OPCODE: &str = OPCODE;

use super::AppendPageContent;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &AppendPageContent) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<AppendPageContent, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

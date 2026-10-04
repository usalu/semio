//! 🗑️ Direct text identity for `remove-page`.

pub const OPCODE: &str = "remove-page";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemovePage;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemovePage) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemovePage, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

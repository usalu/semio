//! 📑️ Direct text identity for `set-outlines`.

pub const OPCODE: &str = "set-outlines";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetOutlines;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetOutlines) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetOutlines, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

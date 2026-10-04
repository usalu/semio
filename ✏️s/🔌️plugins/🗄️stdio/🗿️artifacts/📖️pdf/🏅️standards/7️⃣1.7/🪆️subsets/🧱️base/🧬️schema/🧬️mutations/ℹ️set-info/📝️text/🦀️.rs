//! ℹ️ Direct text identity for `set-info`.

pub const OPCODE: &str = "set-info";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetInfo;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetInfo) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetInfo, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

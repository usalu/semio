//! 🔏️ Direct text identity for `set-mark-info`.

pub const OPCODE: &str = "set-mark-info";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetMarkInfo;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetMarkInfo) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetMarkInfo, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

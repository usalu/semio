//! 🏞️ Direct text identity for `set-image`.

pub const OPCODE: &str = "set-image";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetImage;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetImage) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetImage, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

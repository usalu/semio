//! Direct text identity for `remove-javascript-action`.

pub const OPCODE: &str = "remove-javascript-action";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveJavascriptAction;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveJavascriptAction) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveJavascriptAction, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

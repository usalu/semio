//! Direct text identity for `remove-javascript-action`.

pub const OPCODE: &str = "remove-javascript-action";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveJavascriptAction;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveJavascriptAction) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveJavascriptAction, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

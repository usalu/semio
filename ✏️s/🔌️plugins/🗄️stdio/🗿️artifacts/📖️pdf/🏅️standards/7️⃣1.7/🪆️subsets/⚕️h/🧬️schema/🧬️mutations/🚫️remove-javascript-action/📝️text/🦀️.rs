//! Direct text identity for `remove-javascript-action`.

pub const OPCODE: &str = "remove-javascript-action";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveJavascriptAction;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveJavascriptAction) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveJavascriptAction, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

//! Direct text identity for `insert-javascript-action`.

pub const OPCODE: &str = "insert-javascript-action";
pub const TEXT_OPCODE: &str = OPCODE;

use super::InsertJavascriptAction;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &InsertJavascriptAction) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<InsertJavascriptAction, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

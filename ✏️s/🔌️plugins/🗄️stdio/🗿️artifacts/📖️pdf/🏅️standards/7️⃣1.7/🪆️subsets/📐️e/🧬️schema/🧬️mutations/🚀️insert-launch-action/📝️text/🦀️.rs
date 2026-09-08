//! Direct text identity for `insert-launch-action`.

pub const OPCODE: &str = "insert-launch-action";
pub const TEXT_OPCODE: &str = OPCODE;

use super::InsertLaunchAction;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &InsertLaunchAction) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<InsertLaunchAction, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

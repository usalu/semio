//! Direct text identity for `insert-launch-action`.

pub const OPCODE: &str = "insert-launch-action";
pub const TEXT_OPCODE: &str = OPCODE;

use super::InsertLaunchAction;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &InsertLaunchAction) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<InsertLaunchAction, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

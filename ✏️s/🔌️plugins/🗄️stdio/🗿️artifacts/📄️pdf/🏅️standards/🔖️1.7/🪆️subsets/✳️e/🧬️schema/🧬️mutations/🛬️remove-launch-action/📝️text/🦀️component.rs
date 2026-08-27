//! Direct text identity for `remove-launch-action`.

pub const OPCODE: &str = "remove-launch-action";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveLaunchAction;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveLaunchAction) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveLaunchAction, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

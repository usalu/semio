//! Direct text identity for `remove-launch-action`.

pub const OPCODE: &str = "remove-launch-action";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveLaunchAction;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveLaunchAction) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveLaunchAction, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

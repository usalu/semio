//! Direct text identity for `remove-output-intent`.

pub const OPCODE: &str = "remove-output-intent";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveOutputIntent;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveOutputIntent) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveOutputIntent, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

//! Direct text identity for `remove-output-intent`.

pub const OPCODE: &str = "remove-output-intent";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveOutputIntent;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveOutputIntent) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveOutputIntent, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

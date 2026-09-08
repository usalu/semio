//! Direct text identity for `set-output-intent`.

pub const OPCODE: &str = "set-output-intent";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetOutputIntent;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetOutputIntent) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetOutputIntent, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

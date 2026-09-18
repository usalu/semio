//! 🏳️ Direct text identity for `set-output-intents`.

pub const OPCODE: &str = "set-output-intents";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetOutputIntents;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetOutputIntents) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetOutputIntents, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

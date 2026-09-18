//! 🎯️ Direct text identity for `set-named-destination`.

pub const OPCODE: &str = "set-named-destination";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetNamedDestination;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetNamedDestination) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetNamedDestination, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

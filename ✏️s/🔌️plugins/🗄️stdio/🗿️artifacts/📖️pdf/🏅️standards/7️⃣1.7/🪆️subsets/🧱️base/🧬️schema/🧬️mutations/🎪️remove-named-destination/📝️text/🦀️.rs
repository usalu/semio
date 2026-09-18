//! 🎪️ Direct text identity for `remove-named-destination`.

pub const OPCODE: &str = "remove-named-destination";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveNamedDestination;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveNamedDestination) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveNamedDestination, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

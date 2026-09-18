//! 🎚️ Direct text identity for `remove-ext-g-state`.

pub const OPCODE: &str = "remove-ext-g-state";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveExtGState;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveExtGState) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveExtGState, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

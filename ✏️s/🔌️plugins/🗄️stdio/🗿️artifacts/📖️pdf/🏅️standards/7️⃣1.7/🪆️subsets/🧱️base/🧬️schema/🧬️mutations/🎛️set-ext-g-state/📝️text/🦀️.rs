//! 🎛️ Direct text identity for `set-ext-g-state`.

pub const OPCODE: &str = "set-ext-g-state";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetExtGState;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetExtGState) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetExtGState, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

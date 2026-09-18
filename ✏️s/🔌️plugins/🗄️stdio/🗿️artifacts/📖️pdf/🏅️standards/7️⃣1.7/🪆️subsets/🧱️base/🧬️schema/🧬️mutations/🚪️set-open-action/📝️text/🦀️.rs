//! 🚪️ Direct text identity for `set-open-action`.

pub const OPCODE: &str = "set-open-action";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetOpenAction;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetOpenAction) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetOpenAction, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

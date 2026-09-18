//! 🗞️ Direct text identity for `remove-form`.

pub const OPCODE: &str = "remove-form";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveForm;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveForm) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveForm, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

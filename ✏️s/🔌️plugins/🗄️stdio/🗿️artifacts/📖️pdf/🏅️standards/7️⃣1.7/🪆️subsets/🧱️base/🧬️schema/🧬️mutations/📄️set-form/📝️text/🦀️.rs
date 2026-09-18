//! 📄️ Direct text identity for `set-form`.

pub const OPCODE: &str = "set-form";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetForm;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetForm) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetForm, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

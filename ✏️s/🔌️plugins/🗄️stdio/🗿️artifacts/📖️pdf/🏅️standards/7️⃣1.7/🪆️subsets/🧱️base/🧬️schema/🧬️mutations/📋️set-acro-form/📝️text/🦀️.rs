//! 📋️ Direct text identity for `set-acro-form`.

pub const OPCODE: &str = "set-acro-form";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetAcroForm;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetAcroForm) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetAcroForm, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

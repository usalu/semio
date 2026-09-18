//! 👁️ Direct text identity for `set-optional-content`.

pub const OPCODE: &str = "set-optional-content";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetOptionalContent;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetOptionalContent) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetOptionalContent, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

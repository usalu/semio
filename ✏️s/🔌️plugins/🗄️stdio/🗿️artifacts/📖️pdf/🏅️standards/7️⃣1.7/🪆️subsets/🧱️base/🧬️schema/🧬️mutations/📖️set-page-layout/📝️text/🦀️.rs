//! 📖️ Direct text identity for `set-page-layout`.

pub const OPCODE: &str = "set-page-layout";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetPageLayout;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetPageLayout) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetPageLayout, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

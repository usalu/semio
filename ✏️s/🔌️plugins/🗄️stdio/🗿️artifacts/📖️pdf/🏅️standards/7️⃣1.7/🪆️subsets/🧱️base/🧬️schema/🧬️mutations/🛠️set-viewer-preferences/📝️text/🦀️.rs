//! 🛠️ Direct text identity for `set-viewer-preferences`.

pub const OPCODE: &str = "set-viewer-preferences";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetViewerPreferences;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetViewerPreferences) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetViewerPreferences, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

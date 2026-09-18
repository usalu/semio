//! 🧾️ Direct text identity for `set-metadata`.

pub const OPCODE: &str = "set-metadata";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetMetadata;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetMetadata) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetMetadata, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

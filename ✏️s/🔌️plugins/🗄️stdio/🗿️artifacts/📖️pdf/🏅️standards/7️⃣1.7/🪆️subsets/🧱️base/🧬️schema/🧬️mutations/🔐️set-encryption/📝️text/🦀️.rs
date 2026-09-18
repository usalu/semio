//! 🔐️ Direct text identity for `set-encryption`.

pub const OPCODE: &str = "set-encryption";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetEncryption;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetEncryption) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetEncryption, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

//! Direct text identity for `remove-encryption-dictionary`.

pub const OPCODE: &str = "remove-encryption-dictionary";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveEncryptionDictionary;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveEncryptionDictionary) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveEncryptionDictionary, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

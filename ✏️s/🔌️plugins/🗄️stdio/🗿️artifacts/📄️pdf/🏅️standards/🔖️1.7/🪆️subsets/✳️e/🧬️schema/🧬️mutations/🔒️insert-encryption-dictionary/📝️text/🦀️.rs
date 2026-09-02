//! Direct text identity for `insert-encryption-dictionary`.

pub const OPCODE: &str = "insert-encryption-dictionary";
pub const TEXT_OPCODE: &str = OPCODE;

use super::InsertEncryptionDictionary;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &InsertEncryptionDictionary) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<InsertEncryptionDictionary, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

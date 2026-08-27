//! Direct text identity for `insert-encryption-dictionary`.

pub const OPCODE: &str = "insert-encryption-dictionary";
pub const TEXT_OPCODE: &str = OPCODE;

use super::InsertEncryptionDictionary;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &InsertEncryptionDictionary) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<InsertEncryptionDictionary, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

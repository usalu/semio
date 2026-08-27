//! Direct text identity for `insert-signature-field`.

pub const OPCODE: &str = "insert-signature-field";
pub const TEXT_OPCODE: &str = OPCODE;

use super::InsertSignatureField;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &InsertSignatureField) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<InsertSignatureField, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

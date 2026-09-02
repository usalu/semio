//! Direct text identity for `insert-signature-field`.

pub const OPCODE: &str = "insert-signature-field";
pub const TEXT_OPCODE: &str = OPCODE;

use super::InsertSignatureField;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &InsertSignatureField) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<InsertSignatureField, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

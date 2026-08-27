//! Direct text identity for `remove-signature-field`.

pub const OPCODE: &str = "remove-signature-field";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveSignatureField;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveSignatureField) -> Result<String, String> {
    serde_json::to_string(payload).map_err(|error| error.to_string())
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveSignatureField, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

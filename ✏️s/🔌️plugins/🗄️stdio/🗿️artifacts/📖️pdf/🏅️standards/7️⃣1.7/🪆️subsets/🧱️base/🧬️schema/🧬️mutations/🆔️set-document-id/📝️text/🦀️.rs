//! 🆔️ Direct text identity for `set-document-id`.

pub const OPCODE: &str = "set-document-id";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetDocumentId;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetDocumentId) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetDocumentId, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

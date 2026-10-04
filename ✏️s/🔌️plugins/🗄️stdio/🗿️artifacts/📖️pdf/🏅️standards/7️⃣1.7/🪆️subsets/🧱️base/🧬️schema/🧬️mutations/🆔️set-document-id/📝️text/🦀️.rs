//! 🆔️ Direct text identity for `set-document-id`.

pub const OPCODE: &str = "set-document-id";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetDocumentId;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetDocumentId) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetDocumentId, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

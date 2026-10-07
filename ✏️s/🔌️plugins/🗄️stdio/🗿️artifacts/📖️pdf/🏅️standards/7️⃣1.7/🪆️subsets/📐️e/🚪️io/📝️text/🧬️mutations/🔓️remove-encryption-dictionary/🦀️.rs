//! Direct text identity for `remove-encryption-dictionary`.

pub const OPCODE: &str = "remove-encryption-dictionary";
pub const TEXT_OPCODE: &str = OPCODE;

use crate::standards::v1_7::subsets::e::schema::mutations::RemoveEncryptionDictionary;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveEncryptionDictionary) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveEncryptionDictionary, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

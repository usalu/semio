//! 🗂️ Direct text identity for `set-catalog-entry`.

pub const OPCODE: &str = "set-catalog-entry";
pub const TEXT_OPCODE: &str = OPCODE;

use crate::standards::v1_7::subsets::base::schema::mutations::SetCatalogEntry;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetCatalogEntry) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetCatalogEntry, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

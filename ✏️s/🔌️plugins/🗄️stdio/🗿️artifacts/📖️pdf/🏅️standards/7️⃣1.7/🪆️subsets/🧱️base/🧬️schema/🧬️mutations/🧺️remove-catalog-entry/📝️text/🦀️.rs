//! 🧺️ Direct text identity for `remove-catalog-entry`.

pub const OPCODE: &str = "remove-catalog-entry";
pub const TEXT_OPCODE: &str = OPCODE;

use super::RemoveCatalogEntry;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveCatalogEntry) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveCatalogEntry, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

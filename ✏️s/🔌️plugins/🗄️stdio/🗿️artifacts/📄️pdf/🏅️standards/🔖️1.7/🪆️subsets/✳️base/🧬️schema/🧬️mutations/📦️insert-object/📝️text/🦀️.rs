//! 📦️ Direct text identity for `insert-object`.

pub const OPCODE: &str = "insert-object";
pub const TEXT_OPCODE: &str = OPCODE;

use super::InsertObject;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &InsertObject) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<InsertObject, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

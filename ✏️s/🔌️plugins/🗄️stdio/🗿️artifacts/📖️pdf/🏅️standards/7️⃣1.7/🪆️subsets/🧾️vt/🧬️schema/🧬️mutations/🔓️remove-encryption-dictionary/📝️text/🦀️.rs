//! 🔓️ Direct text codec for `remove-encryption-dictionary`.

use super::RemoveEncryptionDictionary;

//#region 🔖️Identity
pub const OPCODE: &str = "remove-encryption-dictionary";
pub const TEXT_OPCODE: &str = OPCODE;
//#endregion 🔖️Identity

//#region 🔖️Codec
/// 🖨️ Prints the owned payload as schema JSON.
pub fn print(payload: &RemoveEncryptionDictionary) -> Result<String, String> {
    Ok(pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(payload))))
}

/// 📥️ Parses the owned payload from schema JSON.
pub fn parse(text: &str) -> Result<RemoveEncryptionDictionary, String> {
    let parsed = pack::parse_json(text).map_err(|error| error.to_string())?;
    <RemoveEncryptionDictionary as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

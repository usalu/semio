//! 🔓️ Direct text codec for `remove-encryption-dictionary`.

use super::RemoveEncryptionDictionary;

//#region 🔖️Identity
pub const OPCODE: &str = "remove-encryption-dictionary";
pub const TEXT_OPCODE: &str = OPCODE;
//#endregion 🔖️Identity

//#region 🔖️Codec
/// 🖨️ Prints the owned payload as schema JSON.
pub fn print(payload: &RemoveEncryptionDictionary) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(payload))))
}

/// 📥️ Parses the owned payload from schema JSON.
pub fn parse(text: &str) -> Result<RemoveEncryptionDictionary, String> {
    let parsed = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    <RemoveEncryptionDictionary as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

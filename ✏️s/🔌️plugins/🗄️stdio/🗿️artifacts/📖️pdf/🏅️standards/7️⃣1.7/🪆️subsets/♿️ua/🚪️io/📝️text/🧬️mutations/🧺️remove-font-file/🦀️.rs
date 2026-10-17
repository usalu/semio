//! 🧺️ Direct text codec for `remove-font-file`.

use crate::standards::v1_7::subsets::ua::schema::mutations::remove_font_file::RemoveFontFile;

//#region 🔖️Identity
pub const OPCODE: &str = "remove-font-file";
pub const TEXT_OPCODE: &str = OPCODE;
//#endregion 🔖️Identity

//#region 🔖️Codec
/// 🖨️ Prints the owned payload as schema JSON.
pub fn print(payload: &RemoveFontFile) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(payload))
}

/// 📥️ Parses the owned payload from schema JSON.
pub fn parse(text: &str) -> Result<RemoveFontFile, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

//#region 🧪️Tests

//#endregion 🧪️Tests

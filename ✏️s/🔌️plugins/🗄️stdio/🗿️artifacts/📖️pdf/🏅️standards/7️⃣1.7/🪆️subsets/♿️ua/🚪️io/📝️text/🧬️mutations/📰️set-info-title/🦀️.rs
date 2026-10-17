//! 🏷️ Direct text codec for `set-info-title`.

use crate::standards::v1_7::subsets::ua::schema::mutations::set_info_title::SetInfoTitle;

//#region 🔖️Identity
pub const OPCODE: &str = "set-info-title";
pub const TEXT_OPCODE: &str = OPCODE;
//#endregion 🔖️Identity

//#region 🔖️Codec
/// 🖨️ Prints the owned payload as schema JSON.
pub fn print(payload: &SetInfoTitle) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(payload))
}

/// 📥️ Parses the owned payload from schema JSON.
pub fn parse(text: &str) -> Result<SetInfoTitle, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

//#region 🧪️Tests

//#endregion 🧪️Tests

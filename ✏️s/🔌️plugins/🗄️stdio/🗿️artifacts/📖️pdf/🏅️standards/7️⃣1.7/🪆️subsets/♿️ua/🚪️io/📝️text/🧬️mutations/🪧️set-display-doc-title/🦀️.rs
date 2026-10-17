//! 🪧️ Direct text codec for `set-display-doc-title`.

use crate::standards::v1_7::subsets::ua::schema::mutations::set_display_doc_title::SetDisplayDocTitle;

//#region 🔖️Identity
pub const OPCODE: &str = "set-display-doc-title";
pub const TEXT_OPCODE: &str = OPCODE;
//#endregion 🔖️Identity

//#region 🔖️Codec
/// 🖨️ Prints the owned payload as schema JSON.
pub fn print(payload: &SetDisplayDocTitle) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(payload))
}

/// 📥️ Parses the owned payload from schema JSON.
pub fn parse(text: &str) -> Result<SetDisplayDocTitle, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

//#region 🧪️Tests

//#endregion 🧪️Tests

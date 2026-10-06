//! 🗞️ Direct text identity for `remove-form`.

pub const OPCODE: &str = "remove-form";
pub const TEXT_OPCODE: &str = OPCODE;

use crate::standards::v1_7::subsets::base::schema::mutations::RemoveForm;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveForm) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveForm, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

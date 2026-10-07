//! Direct text identity for `insert-launch-action`.

pub const OPCODE: &str = "insert-launch-action";
pub const TEXT_OPCODE: &str = OPCODE;

use crate::standards::v1_7::subsets::e::schema::mutations::InsertLaunchAction;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &InsertLaunchAction) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<InsertLaunchAction, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

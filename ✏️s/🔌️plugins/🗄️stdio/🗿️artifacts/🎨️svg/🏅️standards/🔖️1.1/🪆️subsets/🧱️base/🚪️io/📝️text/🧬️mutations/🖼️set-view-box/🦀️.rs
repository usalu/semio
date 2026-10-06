//! 📝️ Operation-specific text payload codec for set-view-box.
use crate::standards::v1_1::subsets::base::schema::mutations::SetViewBoxPayload;
pub const TEXT_OPCODE: &str = "set-view-box";
pub fn encode_payload(value: &SetViewBoxPayload) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(value))
}
pub fn decode_payload(value: &str) -> Result<SetViewBoxPayload, String> {
    semio_framework_pack_json::from_json_str(value, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

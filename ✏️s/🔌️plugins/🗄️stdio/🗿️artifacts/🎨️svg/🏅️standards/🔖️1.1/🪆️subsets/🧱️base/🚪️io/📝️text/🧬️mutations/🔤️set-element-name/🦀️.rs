//! 📝️ Operation-specific text payload codec for set-element-name.
use crate::standards::v1_1::subsets::base::schema::mutations::SetElementNamePayload;
pub const TEXT_OPCODE: &str = "set-element-name";
pub fn encode_payload(value: &SetElementNamePayload) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(value))
}
pub fn decode_payload(value: &str) -> Result<SetElementNamePayload, String> {
    semio_framework_pack_json::from_json_str(value, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

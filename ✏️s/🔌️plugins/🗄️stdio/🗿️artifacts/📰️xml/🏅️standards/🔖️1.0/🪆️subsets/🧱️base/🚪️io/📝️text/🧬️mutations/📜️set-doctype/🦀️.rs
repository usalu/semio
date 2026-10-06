//! 📝️ Operation-specific text payload codec for set-doctype.
use crate::standards::v1_0::subsets::base::schema::mutations::SetDoctypePayload;
pub const TEXT_OPCODE: &str = "set-doctype";
pub fn encode_payload(value: &SetDoctypePayload) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(value))
}
pub fn decode_payload(value: &str) -> Result<SetDoctypePayload, String> {
    semio_framework_pack_json::from_json_str(value, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

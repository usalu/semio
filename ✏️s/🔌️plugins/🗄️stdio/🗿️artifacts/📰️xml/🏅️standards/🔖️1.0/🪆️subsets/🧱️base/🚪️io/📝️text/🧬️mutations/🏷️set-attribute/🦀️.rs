//! 📝️ Operation-specific text payload codec for set-attribute.
use crate::standards::v1_0::subsets::base::schema::mutations::SetAttributePayload;
pub const TEXT_OPCODE: &str = "set-attribute";
pub fn encode_payload(value: &SetAttributePayload) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(value))
}
pub fn decode_payload(value: &str) -> Result<SetAttributePayload, String> {
    semio_framework_pack_json::from_json_str(value, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

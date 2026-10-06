//! 📝️ Operation-specific text payload codec for remove-array-element.
use crate::standards::v_rfc8259::subsets::base::schema::mutations::RemoveArrayElementPayload;
pub const TEXT_OPCODE: &str = "remove-array-element";
pub fn encode_payload(value: &RemoveArrayElementPayload) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(value))))
}
pub fn decode_payload(value: &str) -> Result<RemoveArrayElementPayload, String> {
    let parsed = semio_framework_pack_json::parse(value, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    <RemoveArrayElementPayload as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| error.to_string())
}

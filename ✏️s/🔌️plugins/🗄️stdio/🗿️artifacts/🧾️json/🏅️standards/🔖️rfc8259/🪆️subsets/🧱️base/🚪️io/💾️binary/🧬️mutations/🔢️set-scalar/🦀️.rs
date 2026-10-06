//! 💾️ Operation-specific binary payload codec for set-scalar/SetScalar.
use crate::standards::v_rfc8259::subsets::base::schema::mutations::SetScalarPayload;
pub const BINARY_TAG: u32 = dsl::protocol_record::tag_u32(include_str!("../📡️.protocol.semio"), "set-scalar");
pub fn encode_payload(value: &SetScalarPayload) -> Result<Vec<u8>, String> {
    Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(value))).into_bytes())
}
pub fn decode_payload(value: &[u8]) -> Result<SetScalarPayload, String> {
    let parsed = semio_framework_pack_json::parse_bytes(value, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    <SetScalarPayload as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| error.to_string())
}

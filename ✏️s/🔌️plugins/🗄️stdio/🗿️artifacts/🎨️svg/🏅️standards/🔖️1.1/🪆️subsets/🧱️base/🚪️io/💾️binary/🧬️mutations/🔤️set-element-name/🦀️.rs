//! 💾️ Operation-specific binary payload codec for set-element-name/SetElementName.
use crate::standards::v1_1::subsets::base::schema::mutations::SetElementNamePayload;
pub const BINARY_TAG: u32 = dsl::protocol_record::tag_u32(include_str!("../📡️.protocol.semio"), "set-element-name");
pub fn encode_payload(value: &SetElementNamePayload) -> Result<Vec<u8>, String> {
    Ok(semio_framework_pack_json::to_json_string(value).into_bytes())
}
pub fn decode_payload(value: &[u8]) -> Result<SetElementNamePayload, String> {
    let text = std::str::from_utf8(value).map_err(|error| error.to_string())?;
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

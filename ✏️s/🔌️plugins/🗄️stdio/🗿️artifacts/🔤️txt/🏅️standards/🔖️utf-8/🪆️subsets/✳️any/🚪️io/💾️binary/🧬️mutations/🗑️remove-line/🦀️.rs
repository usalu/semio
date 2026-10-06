//! 💾️ Canonical binary payload codec for remove-line/RemoveLine.
//#region 💾️PayloadCodec
use crate::standards::v_utf_8::subsets::any::schema::mutations::RemoveLinePayload;
use crate::schema::mutations::TxtMutation;
pub const BINARY_TAG: u32 = dsl::protocol_record::tag_u32(include_str!("../📡️.protocol.semio"), "remove-line");
pub fn encode_payload(value: &RemoveLinePayload) -> Result<Vec<u8>, String> {
    Ok(semio_framework_pack_json::to_json_string(value).into_bytes())
}
pub fn decode_payload(value: &[u8]) -> Result<RemoveLinePayload, String> {
    std::str::from_utf8(value).map_err(|error| error.to_string()).and_then(|text| semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string()))
}
pub fn try_encode(value: &TxtMutation) -> Option<Result<Vec<u8>, String>> {
    match value {
        TxtMutation::RemoveLine(payload) => Some(encode_payload(payload)),
        _ => None,
    }
}
pub fn decode_mutation(value: &[u8]) -> Result<TxtMutation, String> {
    decode_payload(value).map(TxtMutation::RemoveLine)
}
//#endregion 💾️PayloadCodec

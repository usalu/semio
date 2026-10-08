//! 📝️ Canonical text payload codec for splice-text.
//#region 📝️PayloadCodec
use crate::standards::v_utf_8::subsets::any::schema::mutations::SpliceTextPayload;
use crate::schema::mutations::TxtMutation;
pub const TEXT_OPCODE: &str = "splice-text";
pub fn encode_payload(value: &SpliceTextPayload) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(value))
}
pub fn decode_payload(value: &str) -> Result<SpliceTextPayload, String> {
    semio_framework_pack_json::from_json_str(value, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
pub fn try_encode(value: &TxtMutation) -> Option<Result<String, String>> {
    match value {
        TxtMutation::SpliceText(payload) => Some(encode_payload(payload)),
        _ => None,
    }
}
pub fn decode_mutation(value: &str) -> Result<TxtMutation, String> {
    decode_payload(value).map(TxtMutation::SpliceText)
}
//#endregion 📝️PayloadCodec

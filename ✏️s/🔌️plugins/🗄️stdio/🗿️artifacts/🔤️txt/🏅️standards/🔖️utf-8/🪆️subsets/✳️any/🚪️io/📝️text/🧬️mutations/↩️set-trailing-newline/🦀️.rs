//! 📝️ Canonical text payload codec for set-trailing-newline.
//#region 📝️PayloadCodec
use crate::standards::v_utf_8::subsets::any::schema::mutations::SetTrailingNewlinePayload;
use crate::schema::mutations::TxtMutation;
pub const TEXT_OPCODE: &str = "set-trailing-newline";
pub fn encode_payload(value: &SetTrailingNewlinePayload) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(value))
}
pub fn decode_payload(value: &str) -> Result<SetTrailingNewlinePayload, String> {
    semio_framework_pack_json::from_json_str(value, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
pub fn try_encode(value: &TxtMutation) -> Option<Result<String, String>> {
    match value {
        TxtMutation::SetTrailingNewline(payload) => Some(encode_payload(payload)),
        _ => None,
    }
}
pub fn decode_mutation(value: &str) -> Result<TxtMutation, String> {
    decode_payload(value).map(TxtMutation::SetTrailingNewline)
}
//#endregion 📝️PayloadCodec

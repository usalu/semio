//! 📝️ Canonical text payload codec for set-line-ending.
//#region 📝️PayloadCodec
use super::SetLineEndingPayload;
use crate::artifacts::txt::schema::mutations::TxtMutation;
pub const TEXT_OPCODE: &str = "set-line-ending";
pub fn encode_payload(value: &SetLineEndingPayload) -> Result<String, String> {
    serde_json::to_string(value).map_err(|error| error.to_string())
}
pub fn decode_payload(value: &str) -> Result<SetLineEndingPayload, String> {
    serde_json::from_str(value).map_err(|error| error.to_string())
}
pub fn try_encode(value: &TxtMutation) -> Option<Result<String, String>> {
    match value {
        TxtMutation::SetLineEnding(payload) => Some(encode_payload(payload)),
        _ => None,
    }
}
pub fn decode_mutation(value: &str) -> Result<TxtMutation, String> {
    decode_payload(value).map(TxtMutation::SetLineEnding)
}
//#endregion 📝️PayloadCodec

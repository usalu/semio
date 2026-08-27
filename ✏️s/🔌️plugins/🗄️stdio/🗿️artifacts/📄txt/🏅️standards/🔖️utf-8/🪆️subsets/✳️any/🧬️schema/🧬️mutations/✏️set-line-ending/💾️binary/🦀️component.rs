//! 💾️ Operation-specific binary payload codec for set-line-ending/SetLineEnding.
//#region 💾️PayloadCodec
use super::SetLineEndingPayload;
use crate::artifacts::txt::schema::mutations::TxtMutation;
pub const BINARY_TAG: u32 = 2;
pub fn encode_payload(value: &SetLineEndingPayload) -> Result<Vec<u8>, String> {
    serde_json::to_vec(value).map_err(|error| error.to_string())
}
pub fn decode_payload(value: &[u8]) -> Result<SetLineEndingPayload, String> {
    serde_json::from_slice(value).map_err(|error| error.to_string())
}
pub fn try_encode(value: &TxtMutation) -> Option<Result<Vec<u8>, String>> {
    match value {
        TxtMutation::SetLineEnding(payload) => Some(encode_payload(payload)),
        _ => None,
    }
}
pub fn decode_mutation(value: &[u8]) -> Result<TxtMutation, String> {
    decode_payload(value).map(TxtMutation::SetLineEnding)
}
//#endregion 💾️PayloadCodec

//! 💾️ Canonical binary payload codec for insert-line/InsertLine.
//#region 💾️PayloadCodec
use super::InsertLinePayload;
use crate::artifacts::txt::schema::mutations::TxtMutation;
pub const BINARY_TAG: u32 = 3;
pub fn encode_payload(value: &InsertLinePayload) -> Result<Vec<u8>, String> {
    serde_json::to_vec(value).map_err(|error| error.to_string())
}
pub fn decode_payload(value: &[u8]) -> Result<InsertLinePayload, String> {
    serde_json::from_slice(value).map_err(|error| error.to_string())
}
pub fn try_encode(value: &TxtMutation) -> Option<Result<Vec<u8>, String>> {
    match value {
        TxtMutation::InsertLine(payload) => Some(encode_payload(payload)),
        _ => None,
    }
}
pub fn decode_mutation(value: &[u8]) -> Result<TxtMutation, String> {
    decode_payload(value).map(TxtMutation::InsertLine)
}
//#endregion 💾️PayloadCodec

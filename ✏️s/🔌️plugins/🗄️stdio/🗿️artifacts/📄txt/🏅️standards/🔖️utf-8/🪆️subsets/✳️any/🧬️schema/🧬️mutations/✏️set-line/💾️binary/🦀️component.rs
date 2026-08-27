//! 💾️ Operation-specific binary payload codec for set-line/SetLine.
//#region 💾️PayloadCodec
use super::SetLinePayload;
use crate::artifacts::txt::schema::mutations::TxtMutation;
pub const BINARY_TAG: u32 = 5;
pub fn encode_payload(value: &SetLinePayload) -> Result<Vec<u8>, String> {
    serde_json::to_vec(value).map_err(|error| error.to_string())
}
pub fn decode_payload(value: &[u8]) -> Result<SetLinePayload, String> {
    serde_json::from_slice(value).map_err(|error| error.to_string())
}
pub fn try_encode(value: &TxtMutation) -> Option<Result<Vec<u8>, String>> {
    match value {
        TxtMutation::SetLine(payload) => Some(encode_payload(payload)),
        _ => None,
    }
}
pub fn decode_mutation(value: &[u8]) -> Result<TxtMutation, String> {
    decode_payload(value).map(TxtMutation::SetLine)
}
//#endregion 💾️PayloadCodec

//! 💾️ Canonical binary payload codec for insert-line/InsertLine.
//#region 💾️PayloadCodec
use super::InsertLinePayload;
use crate::artifacts::txt::schema::mutations::TxtMutation;
pub const BINARY_TAG: u32 = 3;
pub fn encode_payload(value: &InsertLinePayload) -> Result<Vec<u8>, String> {
    Ok(pack::to_json_string(value).into_bytes())
}
pub fn decode_payload(value: &[u8]) -> Result<InsertLinePayload, String> {
    std::str::from_utf8(value).map_err(|error| error.to_string()).and_then(|text| pack::from_json_str(text).map_err(|error| error.to_string()))
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

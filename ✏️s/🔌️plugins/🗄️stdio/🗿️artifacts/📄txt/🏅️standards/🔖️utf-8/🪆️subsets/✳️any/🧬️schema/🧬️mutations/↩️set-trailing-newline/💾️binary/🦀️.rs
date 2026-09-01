//! 💾️ Canonical binary payload codec for set-trailing-newline/SetTrailingNewline.
//#region 💾️PayloadCodec
use super::SetTrailingNewlinePayload;
use crate::artifacts::txt::schema::mutations::TxtMutation;
pub const BINARY_TAG: u32 = 1;
pub fn encode_payload(value: &SetTrailingNewlinePayload) -> Result<Vec<u8>, String> {
    Ok(pack::to_json_string(value).into_bytes())
}
pub fn decode_payload(value: &[u8]) -> Result<SetTrailingNewlinePayload, String> {
    std::str::from_utf8(value).map_err(|error| error.to_string()).and_then(|text| pack::from_json_str(text).map_err(|error| error.to_string()))
}
pub fn try_encode(value: &TxtMutation) -> Option<Result<Vec<u8>, String>> {
    match value {
        TxtMutation::SetTrailingNewline(payload) => Some(encode_payload(payload)),
        _ => None,
    }
}
pub fn decode_mutation(value: &[u8]) -> Result<TxtMutation, String> {
    decode_payload(value).map(TxtMutation::SetTrailingNewline)
}
//#endregion 💾️PayloadCodec

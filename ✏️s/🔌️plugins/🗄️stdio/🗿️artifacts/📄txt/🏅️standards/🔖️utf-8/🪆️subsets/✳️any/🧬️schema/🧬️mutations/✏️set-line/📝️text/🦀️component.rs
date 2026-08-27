//! 📝️ Operation-specific text payload codec for set-line.
//#region 📝️PayloadCodec
use super::SetLinePayload;
use crate::artifacts::txt::schema::mutations::TxtMutation;
pub const TEXT_OPCODE: &str = "set-line";
pub fn encode_payload(value: &SetLinePayload) -> Result<String, String> {
    serde_json::to_string(value).map_err(|error| error.to_string())
}
pub fn decode_payload(value: &str) -> Result<SetLinePayload, String> {
    serde_json::from_str(value).map_err(|error| error.to_string())
}
pub fn try_encode(value: &TxtMutation) -> Option<Result<String, String>> {
    match value {
        TxtMutation::SetLine(payload) => Some(encode_payload(payload)),
        _ => None,
    }
}
pub fn decode_mutation(value: &str) -> Result<TxtMutation, String> {
    decode_payload(value).map(TxtMutation::SetLine)
}
//#endregion 📝️PayloadCodec

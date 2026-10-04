//! 💾️ Direct patch-pixels binary codec.
use super::*;
use crate::schema::mutations::binary::Entry;
pub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../../💾️binary/📡️.protocol.semio"), "patch-pixels");
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &PngMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let PngMutation::PatchPixels(payload) = value else { return None };
    Some(Ok(semio_framework_pack_json::to_json_string(payload).into_bytes()))
}

pub fn decode(bytes: &[u8]) -> Result<PngMutation, protocol::ProtocolError> {
    let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "patch-pixels", offset: 0, detail: error.to_string() })?;
    let payload = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "patch-pixels", offset: 0, detail: error.to_string() })?;
    Ok(PngMutation::PatchPixels(payload))
}

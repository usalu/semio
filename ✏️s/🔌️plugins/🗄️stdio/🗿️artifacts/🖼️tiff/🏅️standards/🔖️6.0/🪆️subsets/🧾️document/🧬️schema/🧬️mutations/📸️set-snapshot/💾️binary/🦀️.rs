//! 💾️ Direct set-snapshot binary codec.

use super::*;
use crate::schema::mutations::binary::Entry;

pub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../../💾️binary/📡️.protocol.semio"), "set-snapshot");
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };
pub fn encode(value: &TiffMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let TiffMutation::SetSnapshot(SetSnapshot { snapshot }) = value else { return None };
    Some(Ok(semio_framework_pack_json::to_json_string(snapshot).into_bytes()))
}
pub fn decode(bytes: &[u8]) -> Result<TiffMutation, protocol::ProtocolError> {
    let parsed = semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "set-snapshot", offset: 0, detail: error.to_string() })?;
    let snapshot = <TiffSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "set-snapshot", offset: 0, detail: error.to_string() })?;
    Ok(TiffMutation::SetSnapshot(SetSnapshot { snapshot }))
}

//! 💾️ Direct set-snapshot binary codec.

use crate::standards::v1_2::subsets::any::io::binary::mutations::*;
use crate::standards::v1_2::subsets::any::io::binary::mutations::Entry;

pub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../📡️.protocol.semio"), "set-snapshot");
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &PngMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let PngMutation::SetSnapshot(SetSnapshot { snapshot }) = value else { return None };
    Some(Ok(semio_framework_pack_json::to_json_string(snapshot).into_bytes()))
}

pub fn decode(bytes: &[u8]) -> Result<PngMutation, protocol::ProtocolError> {
    let parsed = semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "set-snapshot", offset: 0, detail: error.to_string() })?;
    let snapshot = <PngSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "set-snapshot", offset: 0, detail: error.to_string() })?;
    Ok(PngMutation::SetSnapshot(SetSnapshot { snapshot }))
}

//! 💾️ Direct set-snapshot binary codec.

use super::*;
use crate::schema::mutations::binary::Entry;

pub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../../💾️binary/📡️.protocol.semio"), "set-snapshot");
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };
pub fn encode(value: &BmpMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let BmpMutation::SetSnapshot(SetSnapshot { snapshot }) = value else { return None };
    Some(Ok(pack::to_json_string(snapshot).into_bytes()))
}
pub fn decode(bytes: &[u8]) -> Result<BmpMutation, protocol::ProtocolError> {
    let parsed = pack::parse_json_bytes(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "set-snapshot", offset: 0, detail: error.to_string() })?;
    let snapshot = <BmpSnapshot as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "set-snapshot", offset: 0, detail: error.to_string() })?;
    Ok(BmpMutation::SetSnapshot(SetSnapshot { snapshot }))
}

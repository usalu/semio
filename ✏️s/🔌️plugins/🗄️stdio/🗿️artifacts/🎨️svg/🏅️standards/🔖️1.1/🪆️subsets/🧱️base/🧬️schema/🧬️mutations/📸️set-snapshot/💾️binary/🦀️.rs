//! 💾️ Direct set-snapshot binary codec.

use super::*;

pub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../../💾️binary/📡️.protocol.semio"), "set-snapshot");
pub fn encode(value: &SvgMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let SvgMutation::SetSnapshot(SetSnapshot { snapshot }) = value else { return None };
    Some(Ok(pack::to_json_string(snapshot).into_bytes()))
}
pub fn decode(bytes: &[u8]) -> Result<SvgMutation, protocol::ProtocolError> {
    let parsed = pack::parse_json_bytes(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "set-snapshot", offset: 0, detail: error.to_string() })?;
    let snapshot = <SvgSnapshot as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "set-snapshot", offset: 0, detail: error.to_string() })?;
    Ok(SvgMutation::SetSnapshot(SetSnapshot { snapshot }))
}

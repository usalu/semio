//! 💾️ Direct set-snapshot binary codec.

use crate::standards::v1_1::subsets::base::schema::{mutations::set_snapshot::SetSnapshot, snapshot::SvgSnapshot};
use crate::standards::v1_1::subsets::base::io::binary::mutations::*;

pub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../📡️.protocol.semio"), "set-snapshot");
pub fn encode(value: &SvgMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let SvgMutation::SetSnapshot(SetSnapshot { snapshot }) = value else { return None };
    Some(Ok(semio_framework_pack_json::to_json_string(snapshot).into_bytes()))
}
pub fn decode(bytes: &[u8]) -> Result<SvgMutation, protocol::ProtocolError> {
    let parsed = semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "set-snapshot", offset: 0, detail: error.to_string() })?;
    let snapshot = <SvgSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "set-snapshot", offset: 0, detail: error.to_string() })?;
    Ok(SvgMutation::SetSnapshot(SetSnapshot { snapshot }))
}

//! 💾️ Direct-color region binary codec.
use crate::standards::v_v3::subsets::any::schema::mutations::*;
use crate::standards::v_v3::subsets::any::schema::snapshot::*;
use crate::standards::v_v3::subsets::any::io::binary::diff::*;
use crate::schema::mutations::{BmpMutation, PaintDirectRegion};
use crate::standards::v_v3::subsets::any::io::binary::mutations::Entry;
pub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../📡️.protocol.semio"), "paint-direct-region");
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };
pub fn encode(value: &BmpMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let BmpMutation::PaintDirectRegion(payload) = value else { return None };
    Some(Ok(semio_framework_pack_json::to_json_string(payload).into_bytes()))
}
pub fn decode(bytes: &[u8]) -> Result<BmpMutation, protocol::ProtocolError> {
    let parsed = semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "paint-direct-region", offset: 0, detail: error.to_string() })?;
    let payload = <PaintDirectRegion as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "paint-direct-region", offset: 0, detail: error.to_string() })?;
    Ok(BmpMutation::PaintDirectRegion(payload))
}

//! 💾️ Direct paint-region binary codec.
use crate::standards::v6_0::subsets::document::schema::mutations::*;
use crate::standards::v6_0::subsets::document::schema::snapshot::*;
use crate::standards::v6_0::subsets::document::io::binary::diff::*;
use crate::standards::v6_0::subsets::document::io::binary::mutations::*;
use crate::standards::v6_0::subsets::document::io::binary::mutations::Entry;

pub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../📡️.protocol.semio"), "paint-region");
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &TiffMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let TiffMutation::PaintRegion(payload) = value else { return None };
    Some(Ok(semio_framework_pack_json::to_json_string(payload).into_bytes()))
}

pub fn decode(bytes: &[u8]) -> Result<TiffMutation, protocol::ProtocolError> {
    let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "paint-region", offset: 0, detail: error.to_string() })?;
    let payload = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "paint-region", offset: 0, detail: error.to_string() })?;
    Ok(TiffMutation::PaintRegion(payload))
}

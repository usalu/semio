//! 💾️ Direct remove-ifd binary codec.
use crate::standards::v6_0::subsets::document::schema::mutations::*;
use crate::standards::v6_0::subsets::document::schema::snapshot::*;
use crate::standards::v6_0::subsets::document::io::binary::diff::*;
use crate::standards::v6_0::subsets::document::io::binary::mutations::*;
use crate::standards::v6_0::subsets::document::io::binary::mutations::Entry;
pub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../📡️.protocol.semio"), "remove-ifd");
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &TiffMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let TiffMutation::RemoveIfd(payload) = value else { return None };
    Some(encode_payload(payload))
}
pub fn encode_payload(payload: &RemoveIfdMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    let RemoveIfdMutation { index } = payload;
    let mut out = Vec::new();
    store::pack_rt::write_varint_u64(&mut out, *index as u64);
    Ok(out)
}
pub fn decode(bytes: &[u8]) -> Result<TiffMutation, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let result: Result<TiffMutation, protocol::ProtocolError> = {
        let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
        Ok(TiffMutation::RemoveIfd(RemoveIfdMutation { index }))
    };
    let position = reader.position();
    if position != bytes.len() {
        return Err(protocol::ProtocolError::Malformed { what: "remove-ifd", offset: position as u64, detail: "trailing payload bytes".into() });
    }
    result
}

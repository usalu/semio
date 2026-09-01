//! 💾️ Direct insert-other-segment binary codec.
use super::*;
use crate::artifacts::jpg::schema::diff::{self, *};
use crate::artifacts::jpg::schema::mutations::binary::Entry;
pub const BINARY_TAG: u8 = 8;
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &JpgMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let JpgMutation::InsertOtherSegment(payload) = value else { return None };
    Some(encode_payload(payload))
}
pub fn encode_payload(payload: &InsertOtherSegmentMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    let InsertOtherSegmentMutation { index, segment } = payload;
    let mut out = Vec::new();
    store::pack_rt::write_varint_u64(&mut out, *index as u64);
    diff::enc_segment_bin(segment, &mut out);
    Ok(out)
}
fn op_pack_err(error: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "insert-other-segment", offset: 0, detail: error.to_string() }
}
pub fn decode(bytes: &[u8]) -> Result<JpgMutation, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let result: Result<JpgMutation, protocol::ProtocolError> = {
        let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
        let segment = diff::dec_segment_bin(&mut reader).map_err(|e| malformed("op segment", reader.position(), e))?;
        Ok(JpgMutation::InsertOtherSegment(crate::artifacts::jpg::schema::mutations::InsertOtherSegmentMutation { index, segment }))
    };
    let position = reader.position();
    if position != bytes.len() {
        return Err(protocol::ProtocolError::Malformed { what: "insert-other-segment", offset: position as u64, detail: "trailing payload bytes".into() });
    }
    result
}

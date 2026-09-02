//! 💾️ Direct remove-text-chunk binary codec.
use super::*;
use crate::artifacts::png::schema::diff::{self, *};
use crate::artifacts::png::schema::mutations::binary::Entry;
pub const BINARY_TAG: u8 = 12;
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &PngMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let PngMutation::RemoveTextChunk(payload) = value else { return None };
    Some(encode_payload(payload))
}
pub fn encode_payload(payload: &RemoveTextChunkMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    let RemoveTextChunkMutation { index } = payload;
    let mut w = dsl::ByteWriter::new();
    w.write_varint_u64(*index as u64);
    Ok(w.into_bytes())
}
fn op_pack_err(error: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "remove-text-chunk", offset: 0, detail: error.to_string() }
}
pub fn decode(bytes: &[u8]) -> Result<PngMutation, protocol::ProtocolError> {
    let mut r = dsl::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let result: Result<PngMutation, protocol::ProtocolError> = Ok(PngMutation::RemoveTextChunk(crate::artifacts::png::schema::mutations::RemoveTextChunkMutation { index: r.read_varint_u64().map_err(op_pack_err)? as usize }));
    let position = r.position();
    if position != bytes.len() {
        return Err(protocol::ProtocolError::Malformed { what: "remove-text-chunk", offset: position as u64, detail: "trailing payload bytes".into() });
    }
    result
}

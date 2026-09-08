//! 💾️ Direct replace-text-chunk binary codec.
use super::*;
use crate::artifacts::png::schema::mutations::binary::Entry;
pub const BINARY_TAG: u8 = 13;
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &PngMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let PngMutation::ReplaceTextChunk(payload) = value else { return None };
    Some(encode_payload(payload))
}
pub fn encode_payload(payload: &ReplaceTextChunkMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    let ReplaceTextChunkMutation { index, chunk } = payload;
    let mut w = dsl::ByteWriter::new();
    w.write_varint_u64(*index as u64);
    write_bin_text_chunk(&mut w, chunk);
    Ok(w.into_bytes())
}
fn op_pack_err(error: &dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "replace-text-chunk", offset: 0, detail: error.to_string() }
}
pub fn decode(bytes: &[u8]) -> Result<PngMutation, protocol::ProtocolError> {
    let mut r = dsl::ByteReader::new(bytes);
    let result: Result<PngMutation, protocol::ProtocolError> = Ok({
        let index = r.read_varint_u64().map_err(|error| op_pack_err(&error))? as usize;
        let chunk = read_bin_text_chunk(&mut r).map_err(|error| op_pack_err(&error))?;
        PngMutation::ReplaceTextChunk(ReplaceTextChunkMutation { index, chunk })
    });
    let position = r.position();
    if position != bytes.len() {
        return Err(protocol::ProtocolError::Malformed { what: "replace-text-chunk", offset: position as u64, detail: "trailing payload bytes".into() });
    }
    result
}

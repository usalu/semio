//! 💾️ Direct remove-tag binary codec.
use super::*;
use crate::artifacts::tiff::schema::diff::{self, *};
use crate::artifacts::tiff::schema::mutations::binary::Entry;
pub const BINARY_TAG: u8 = 6;
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &TiffMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let TiffMutation::RemoveTag(payload) = value else { return None };
    Some(encode_payload(payload))
}
pub fn encode_payload(payload: &RemoveTagMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    let RemoveTagMutation { ifd_index, tag } = payload;
    let mut out = Vec::new();

    store::pack_rt::write_varint_u64(&mut out, *ifd_index as u64);
    out.extend_from_slice(&tag.to_le_bytes());
    Ok(out)
}
fn op_pack_err(error: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "remove-tag", offset: 0, detail: error.to_string() }
}
pub fn decode(bytes: &[u8]) -> Result<TiffMutation, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let result: Result<TiffMutation, protocol::ProtocolError> = {
        let ifd_index = reader.read_varint_u64().map_err(|e| malformed("op ifd_index", reader.position(), e.to_string()))? as usize;
        let tag = reader.read_u16_le().map_err(|e| malformed("op tag", reader.position(), e.to_string()))?;
        Ok(TiffMutation::RemoveTag(crate::artifacts::tiff::schema::mutations::RemoveTagMutation { ifd_index, tag }))
    };
    let position = reader.position();
    if position != bytes.len() {
        return Err(protocol::ProtocolError::Malformed { what: "remove-tag", offset: position as u64, detail: "trailing payload bytes".into() });
    }
    result
}

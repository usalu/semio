//! 💾️ Direct remove-ifd binary codec.
use super::*;
use crate::artifacts::tiff::schema::diff::{self, *};
use crate::artifacts::tiff::schema::mutations::binary::Entry;
pub const BINARY_TAG: u8 = 4;
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
fn op_pack_err(error: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "remove-ifd", offset: 0, detail: error.to_string() }
}
pub fn decode(bytes: &[u8]) -> Result<TiffMutation, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let result: Result<TiffMutation, protocol::ProtocolError> = {
        let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
        Ok(TiffMutation::RemoveIfd(crate::artifacts::tiff::schema::mutations::RemoveIfdMutation { index }))
    };
    let position = reader.position();
    if position != bytes.len() {
        return Err(protocol::ProtocolError::Malformed { what: "remove-ifd", offset: position as u64, detail: "trailing payload bytes".into() });
    }
    result
}

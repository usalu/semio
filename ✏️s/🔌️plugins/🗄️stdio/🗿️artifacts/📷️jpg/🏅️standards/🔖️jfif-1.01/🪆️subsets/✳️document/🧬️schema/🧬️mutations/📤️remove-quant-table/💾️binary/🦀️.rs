//! 💾️ Direct remove-quant-table binary codec.
use super::*;
use crate::artifacts::jpg::schema::diff::{self, *};
use crate::artifacts::jpg::schema::mutations::binary::Entry;
pub const BINARY_TAG: u8 = 4;
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &JpgMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let JpgMutation::RemoveQuantTable(payload) = value else { return None };
    Some(encode_payload(payload))
}
pub fn encode_payload(payload: &RemoveQuantTableMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    let RemoveQuantTableMutation { id } = payload;
    let mut out = Vec::new();
    out.push(*id);
    Ok(out)
}
fn op_pack_err(error: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "remove-quant-table", offset: 0, detail: error.to_string() }
}
pub fn decode(bytes: &[u8]) -> Result<JpgMutation, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let result: Result<JpgMutation, protocol::ProtocolError> =
        Ok(JpgMutation::RemoveQuantTable(crate::artifacts::jpg::schema::mutations::RemoveQuantTableMutation { id: reader.read_u8().map_err(|e| malformed("op quant-id", reader.position(), e.to_string()))? }));
    let position = reader.position();
    if position != bytes.len() {
        return Err(protocol::ProtocolError::Malformed { what: "remove-quant-table", offset: position as u64, detail: "trailing payload bytes".into() });
    }
    result
}

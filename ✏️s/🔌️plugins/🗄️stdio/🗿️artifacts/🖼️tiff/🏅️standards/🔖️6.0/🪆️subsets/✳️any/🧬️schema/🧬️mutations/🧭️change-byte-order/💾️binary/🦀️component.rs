//! 💾️ Direct change-byte-order binary codec.
use super::*;
use crate::artifacts::tiff::schema::diff::{self, *};
use crate::artifacts::tiff::schema::mutations::binary::Entry;
pub const BINARY_TAG: u8 = 2;
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &TiffMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let TiffMutation::ChangeByteOrder(payload) = value else { return None };
    Some(encode_payload(payload))
}
pub fn encode_payload(payload: &ChangeByteOrderMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    let ChangeByteOrderMutation { byte_order } = payload;
    let mut out = Vec::new();
    out.push(match byte_order {
        TiffByteOrder::LittleEndian => 0,
        TiffByteOrder::BigEndian => 1,
    });
    Ok(out)
}
fn op_pack_err(error: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "change-byte-order", offset: 0, detail: error.to_string() }
}
pub fn decode(bytes: &[u8]) -> Result<TiffMutation, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let result: Result<TiffMutation, protocol::ProtocolError> = {
        let v = reader.read_u8().map_err(|e| malformed("op byte_order", reader.position(), e.to_string()))?;
        Ok(TiffMutation::ChangeByteOrder(crate::artifacts::tiff::schema::mutations::ChangeByteOrderMutation { byte_order: if v == 0 { TiffByteOrder::LittleEndian } else { TiffByteOrder::BigEndian } }))
    };
    let position = reader.position();
    if position != bytes.len() {
        return Err(protocol::ProtocolError::Malformed { what: "change-byte-order", offset: position as u64, detail: "trailing payload bytes".into() });
    }
    result
}

//! 💾️ Direct change-header binary codec.
use super::*;
use crate::artifacts::png::schema::diff::{self, *};
use crate::artifacts::png::schema::mutations::binary::Entry;
pub const BINARY_TAG: u8 = 2;
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &PngMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let PngMutation::ChangeHeader(payload) = value else { return None };
    Some(encode_payload(payload))
}
pub fn encode_payload(payload: &ChangeHeaderMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    let ChangeHeaderMutation { width, height, bit_depth, color_type, interlace } = payload;
    let mut w = dsl::ByteWriter::new();
    w.write_u32_le(*width);
    w.write_u32_le(*height);
    w.write_u8(*bit_depth);
    w.write_u8(color_type.to_u8());
    w.write_u8(if *interlace { 1 } else { 0 });
    Ok(w.into_bytes())
}
fn op_pack_err(error: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "change-header", offset: 0, detail: error.to_string() }
}
pub fn decode(bytes: &[u8]) -> Result<PngMutation, protocol::ProtocolError> {
    let mut r = dsl::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let result: Result<PngMutation, protocol::ProtocolError> = Ok(PngMutation::ChangeHeader(crate::artifacts::png::schema::mutations::ChangeHeaderMutation {
        width: r.read_u32_le().map_err(op_pack_err)?,
        height: r.read_u32_le().map_err(op_pack_err)?,
        bit_depth: r.read_u8().map_err(op_pack_err)?,
        color_type: PngColorType::from_u8(r.read_u8().map_err(op_pack_err)?).map_err(|e| protocol::ProtocolError::Malformed { what: "png op color type", offset: 0, detail: e })?,
        interlace: r.read_u8().map_err(op_pack_err)? != 0,
    }));
    let position = r.position();
    if position != bytes.len() {
        return Err(protocol::ProtocolError::Malformed { what: "change-header", offset: position as u64, detail: "trailing payload bytes".into() });
    }
    result
}

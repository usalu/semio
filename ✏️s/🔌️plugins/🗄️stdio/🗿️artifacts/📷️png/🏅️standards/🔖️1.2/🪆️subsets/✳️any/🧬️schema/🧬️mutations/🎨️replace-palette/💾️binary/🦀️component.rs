//! 💾️ Direct replace-palette binary codec.
use super::*;
use crate::artifacts::png::schema::diff::{self, *};
use crate::artifacts::png::schema::mutations::binary::Entry;
pub const BINARY_TAG: u8 = 3;
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &PngMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let PngMutation::ReplacePalette(payload) = value else { return None };
    Some(encode_payload(payload))
}
pub fn encode_payload(payload: &ReplacePaletteMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    let ReplacePaletteMutation { plte } = payload;
    let mut w = dsl::ByteWriter::new();
    diff::write_bin_option(&mut w, plte, |w, v: &Vec<PngRgb>| diff::write_bin_vec(w, v, diff::write_bin_rgb));
    Ok(w.into_bytes())
}
fn op_pack_err(error: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "replace-palette", offset: 0, detail: error.to_string() }
}
pub fn decode(bytes: &[u8]) -> Result<PngMutation, protocol::ProtocolError> {
    let mut r = dsl::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let result: Result<PngMutation, protocol::ProtocolError> =
        Ok(PngMutation::ReplacePalette(crate::artifacts::png::schema::mutations::ReplacePaletteMutation { plte: diff::read_bin_option(&mut r, |r| diff::read_bin_vec(r, diff::read_bin_rgb)).map_err(op_pack_err)? }));
    let position = r.position();
    if position != bytes.len() {
        return Err(protocol::ProtocolError::Malformed { what: "replace-palette", offset: position as u64, detail: "trailing payload bytes".into() });
    }
    result
}

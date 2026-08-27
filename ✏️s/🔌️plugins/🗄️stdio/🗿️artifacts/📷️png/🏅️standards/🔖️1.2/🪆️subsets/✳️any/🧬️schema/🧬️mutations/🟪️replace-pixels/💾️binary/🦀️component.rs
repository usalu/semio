//! 💾️ Direct replace-pixels binary codec.
use super::*;
use crate::artifacts::png::schema::diff::{self, *};
use crate::artifacts::png::schema::mutations::binary::Entry;
pub const BINARY_TAG: u8 = 14;
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &PngMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let PngMutation::ReplacePixels(payload) = value else { return None };
    Some(encode_payload(payload))
}
pub fn encode_payload(payload: &ReplacePixelsMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    let ReplacePixelsMutation { pixels } = payload;
    let mut w = dsl::ByteWriter::new();
    diff::write_bin_blob(&mut w, pixels);
    Ok(w.into_bytes())
}
fn op_pack_err(error: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "replace-pixels", offset: 0, detail: error.to_string() }
}
pub fn decode(bytes: &[u8]) -> Result<PngMutation, protocol::ProtocolError> {
    let mut r = dsl::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let result: Result<PngMutation, protocol::ProtocolError> = Ok(PngMutation::ReplacePixels(crate::artifacts::png::schema::mutations::ReplacePixelsMutation { pixels: diff::read_bin_blob(&mut r).map_err(op_pack_err)? }));
    let position = r.position();
    if position != bytes.len() {
        return Err(protocol::ProtocolError::Malformed { what: "replace-pixels", offset: position as u64, detail: "trailing payload bytes".into() });
    }
    result
}

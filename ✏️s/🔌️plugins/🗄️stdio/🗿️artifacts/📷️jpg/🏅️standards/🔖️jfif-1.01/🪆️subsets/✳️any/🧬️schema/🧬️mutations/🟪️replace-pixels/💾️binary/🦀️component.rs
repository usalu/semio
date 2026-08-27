//! 💾️ Direct replace-pixels binary codec.
use super::*;
use crate::artifacts::jpg::schema::diff::{self, *};
use crate::artifacts::jpg::schema::mutations::binary::Entry;
pub const BINARY_TAG: u8 = 10;
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &JpgMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let JpgMutation::ReplacePixels(payload) = value else { return None };
    Some(encode_payload(payload))
}
pub fn encode_payload(payload: &ReplacePixelsMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    let ReplacePixelsMutation { pixels } = payload;
    let mut out = Vec::new();
    diff::write_bytes_lp(&mut out, pixels);
    Ok(out)
}
fn op_pack_err(error: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "replace-pixels", offset: 0, detail: error.to_string() }
}
pub fn decode(bytes: &[u8]) -> Result<JpgMutation, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let result: Result<JpgMutation, protocol::ProtocolError> =
        Ok(JpgMutation::ReplacePixels(crate::artifacts::jpg::schema::mutations::ReplacePixelsMutation { pixels: diff::read_bytes_lp(&mut reader).map_err(|e| malformed("op pixels", reader.position(), e))? }));
    let position = reader.position();
    if position != bytes.len() {
        return Err(protocol::ProtocolError::Malformed { what: "replace-pixels", offset: position as u64, detail: "trailing payload bytes".into() });
    }
    result
}

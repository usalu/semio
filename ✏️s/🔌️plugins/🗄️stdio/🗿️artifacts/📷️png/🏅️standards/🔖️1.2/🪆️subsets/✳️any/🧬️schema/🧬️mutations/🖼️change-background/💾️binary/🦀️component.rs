//! 💾️ Direct change-background binary codec.
use super::*;
use crate::artifacts::png::schema::diff::{self, *};
use crate::artifacts::png::schema::mutations::binary::Entry;
pub const BINARY_TAG: u8 = 10;
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &PngMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let PngMutation::ChangeBackground(payload) = value else { return None };
    Some(encode_payload(payload))
}
pub fn encode_payload(payload: &ChangeBackgroundMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    let ChangeBackgroundMutation { bkgd } = payload;
    let mut w = dsl::ByteWriter::new();
    diff::write_bin_option(&mut w, bkgd, diff::write_bin_background);
    Ok(w.into_bytes())
}
fn op_pack_err(error: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "change-background", offset: 0, detail: error.to_string() }
}
pub fn decode(bytes: &[u8]) -> Result<PngMutation, protocol::ProtocolError> {
    let mut r = dsl::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let result: Result<PngMutation, protocol::ProtocolError> =
        Ok(PngMutation::ChangeBackground(crate::artifacts::png::schema::mutations::ChangeBackgroundMutation { bkgd: diff::read_bin_option(&mut r, diff::read_bin_background).map_err(op_pack_err)? }));
    let position = r.position();
    if position != bytes.len() {
        return Err(protocol::ProtocolError::Malformed { what: "change-background", offset: position as u64, detail: "trailing payload bytes".into() });
    }
    result
}

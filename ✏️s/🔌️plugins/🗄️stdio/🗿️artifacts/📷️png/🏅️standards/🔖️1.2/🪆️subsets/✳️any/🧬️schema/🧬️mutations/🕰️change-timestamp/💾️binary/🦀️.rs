//! 💾️ Direct change-timestamp binary codec.
use super::*;
use crate::artifacts::png::schema::diff::{self, *};
use crate::artifacts::png::schema::mutations::binary::Entry;
pub const BINARY_TAG: u8 = 9;
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &PngMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let PngMutation::ChangeTimestamp(payload) = value else { return None };
    Some(encode_payload(payload))
}
pub fn encode_payload(payload: &ChangeTimestampMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    let ChangeTimestampMutation { time } = payload;
    let mut w = dsl::ByteWriter::new();
    diff::write_bin_option(&mut w, time, diff::write_bin_timestamp);
    Ok(w.into_bytes())
}
fn op_pack_err(error: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "change-timestamp", offset: 0, detail: error.to_string() }
}
pub fn decode(bytes: &[u8]) -> Result<PngMutation, protocol::ProtocolError> {
    let mut r = dsl::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let result: Result<PngMutation, protocol::ProtocolError> =
        Ok(PngMutation::ChangeTimestamp(crate::artifacts::png::schema::mutations::ChangeTimestampMutation { time: diff::read_bin_option(&mut r, diff::read_bin_timestamp).map_err(op_pack_err)? }));
    let position = r.position();
    if position != bytes.len() {
        return Err(protocol::ProtocolError::Malformed { what: "change-timestamp", offset: position as u64, detail: "trailing payload bytes".into() });
    }
    result
}

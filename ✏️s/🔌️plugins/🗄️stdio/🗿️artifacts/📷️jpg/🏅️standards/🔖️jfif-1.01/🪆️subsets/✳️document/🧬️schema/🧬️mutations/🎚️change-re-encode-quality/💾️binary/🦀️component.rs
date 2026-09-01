//! 💾️ Direct change-re-encode-quality binary codec.
use super::*;
use crate::artifacts::jpg::schema::diff::{self, *};
use crate::artifacts::jpg::schema::mutations::binary::Entry;
pub const BINARY_TAG: u8 = 11;
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &JpgMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let JpgMutation::ChangeReEncodeQuality(payload) = value else { return None };
    Some(encode_payload(payload))
}
pub fn encode_payload(payload: &ChangeReEncodeQualityMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    let ChangeReEncodeQualityMutation { quality } = payload;
    let mut out = Vec::new();
    diff::write_opt(&mut out, quality, |v, out| out.push(*v));
    Ok(out)
}
fn op_pack_err(error: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "change-re-encode-quality", offset: 0, detail: error.to_string() }
}
pub fn decode(bytes: &[u8]) -> Result<JpgMutation, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let result: Result<JpgMutation, protocol::ProtocolError> = Ok(JpgMutation::ChangeReEncodeQuality(crate::artifacts::jpg::schema::mutations::ChangeReEncodeQualityMutation {
        quality: diff::read_opt(&mut reader, |r| r.read_u8().map_err(|e| e.to_string())).map_err(|e| malformed("op quality", reader.position(), e))?,
    }));
    let position = reader.position();
    if position != bytes.len() {
        return Err(protocol::ProtocolError::Malformed { what: "change-re-encode-quality", offset: position as u64, detail: "trailing payload bytes".into() });
    }
    result
}

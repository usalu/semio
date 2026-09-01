//! 💾️ Direct change-restart-interval binary codec.
use super::*;
use crate::artifacts::jpg::schema::diff::{self, *};
use crate::artifacts::jpg::schema::mutations::binary::Entry;
pub const BINARY_TAG: u8 = 7;
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &JpgMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let JpgMutation::ChangeRestartInterval(payload) = value else { return None };
    Some(encode_payload(payload))
}
pub fn encode_payload(payload: &ChangeRestartIntervalMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    let ChangeRestartIntervalMutation { restart_interval } = payload;
    let mut out = Vec::new();
    diff::write_opt(&mut out, restart_interval, |v, out| store::pack_rt::write_varint_u64(out, *v as u64));
    Ok(out)
}
fn op_pack_err(error: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "change-restart-interval", offset: 0, detail: error.to_string() }
}
pub fn decode(bytes: &[u8]) -> Result<JpgMutation, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let result: Result<JpgMutation, protocol::ProtocolError> = Ok(JpgMutation::ChangeRestartInterval(crate::artifacts::jpg::schema::mutations::ChangeRestartIntervalMutation {
        restart_interval: diff::read_opt(&mut reader, |r| Ok(r.read_varint_u64().map_err(|e| e.to_string())? as u16)).map_err(|e| malformed("op restart-interval", reader.position(), e))?,
    }));
    let position = reader.position();
    if position != bytes.len() {
        return Err(protocol::ProtocolError::Malformed { what: "change-restart-interval", offset: position as u64, detail: "trailing payload bytes".into() });
    }
    result
}

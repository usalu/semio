//! 💾️ Direct change-restart-interval binary codec.
use crate::standards::v_jfif_1_01::subsets::document::schema::mutations::*;
use crate::standards::v_jfif_1_01::subsets::document::schema::snapshot::*;
use crate::standards::v_jfif_1_01::subsets::document::io::binary::diff::*;
use crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::*;
use crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::Entry;
pub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../📡️.protocol.semio"), "change-restart-interval");
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &JpgMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let JpgMutation::ChangeRestartInterval(payload) = value else { return None };
    Some(encode_payload(payload))
}
pub fn encode_payload(payload: &ChangeRestartIntervalMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    let ChangeRestartIntervalMutation { restart_interval } = payload;
    let mut out = Vec::new();
    write_opt(&mut out, restart_interval, |v, out| store::pack_rt::write_varint_u64(out, *v as u64));
    Ok(out)
}
pub fn decode(bytes: &[u8]) -> Result<JpgMutation, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let result: Result<JpgMutation, protocol::ProtocolError> = Ok(JpgMutation::ChangeRestartInterval(ChangeRestartIntervalMutation {
        restart_interval: read_opt(&mut reader, |r| Ok(r.read_varint_u64().map_err(|e| e.to_string())? as u16)).map_err(|e| malformed("op restart-interval", reader.position(), e))?,
    }));
    let position = reader.position();
    if position != bytes.len() {
        return Err(protocol::ProtocolError::Malformed { what: "change-restart-interval", offset: position as u64, detail: "trailing payload bytes".into() });
    }
    result
}

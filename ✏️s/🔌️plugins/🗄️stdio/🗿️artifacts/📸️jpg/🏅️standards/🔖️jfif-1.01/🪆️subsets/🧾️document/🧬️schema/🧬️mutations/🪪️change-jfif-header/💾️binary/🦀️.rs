//! 💾️ Direct change-jfif-header binary codec.
use super::*;
use crate::artifacts::jpg::schema::diff::{self, *};
use crate::artifacts::jpg::schema::mutations::binary::Entry;
pub const BINARY_TAG: u8 = 2;
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &JpgMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let JpgMutation::ChangeJfifHeader(payload) = value else { return None };
    Some(encode_payload(payload))
}
pub fn encode_payload(payload: &ChangeJfifHeaderMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    let ChangeJfifHeaderMutation { version, density_units, x_density, y_density, thumbnail } = payload;
    let mut out = Vec::new();
    diff::enc_version_bin(version, &mut out);
    diff::enc_density_units_bin(density_units, &mut out);
    store::pack_rt::write_varint_u64(&mut out, *x_density as u64);
    store::pack_rt::write_varint_u64(&mut out, *y_density as u64);
    diff::write_opt(&mut out, thumbnail, |t, out| diff::enc_thumbnail_bin(t, out));
    Ok(out)
}
fn op_pack_err(error: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "change-jfif-header", offset: 0, detail: error.to_string() }
}
pub fn decode(bytes: &[u8]) -> Result<JpgMutation, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let result: Result<JpgMutation, protocol::ProtocolError> = {
        let version = diff::dec_version_bin(&mut reader).map_err(|e| malformed("op version", reader.position(), e))?;
        let density_units = diff::dec_density_units_bin(&mut reader).map_err(|e| malformed("op density-units", reader.position(), e))?;
        let x_density = reader.read_varint_u64().map_err(|e| malformed("op x-density", reader.position(), e.to_string()))? as u16;
        let y_density = reader.read_varint_u64().map_err(|e| malformed("op y-density", reader.position(), e.to_string()))? as u16;
        let thumbnail = diff::read_opt(&mut reader, diff::dec_thumbnail_bin).map_err(|e| malformed("op thumbnail", reader.position(), e))?;
        Ok(JpgMutation::ChangeJfifHeader(crate::artifacts::jpg::schema::mutations::ChangeJfifHeaderMutation { version, density_units, x_density, y_density, thumbnail }))
    };
    let position = reader.position();
    if position != bytes.len() {
        return Err(protocol::ProtocolError::Malformed { what: "change-jfif-header", offset: position as u64, detail: "trailing payload bytes".into() });
    }
    result
}

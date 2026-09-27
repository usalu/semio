//! 💾️ Direct patch-pixels binary codec.
use super::*;
use crate::schema::mutations::binary::Entry;
pub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../../💾️binary/📡️.protocol.semio"), "patch-pixels");
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &PngMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let PngMutation::PatchPixels(payload) = value else { return None };
    Some(encode_payload(payload))
}
pub fn encode_payload(payload: &PatchPixelsMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    let mut writer = dsl::ByteWriter::new();
    writer.write_varint_u64(payload.index);
    writer.write_varint_u64(payload.remove_count);
    write_bin_blob(&mut writer, &payload.pixels);
    writer.write_u8(u8::from(payload.move_to.is_some()));
    if let Some(move_to) = payload.move_to { writer.write_varint_u64(move_to); }
    Ok(writer.into_bytes())
}
fn malformed(error: impl ToString) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "patch-pixels", offset: 0, detail: error.to_string() }
}
pub fn decode(bytes: &[u8]) -> Result<PngMutation, protocol::ProtocolError> {
    let mut reader = dsl::ByteReader::new(bytes);
    let index = reader.read_varint_u64().map_err(malformed)?;
    let remove_count = reader.read_varint_u64().map_err(malformed)?;
    let pixels = read_bin_blob(&mut reader).map_err(malformed)?;
    let move_to = match reader.read_u8().map_err(malformed)? { 0 => None, 1 => Some(reader.read_varint_u64().map_err(malformed)?), value => return Err(malformed(format!("invalid move-to tag {value}"))) };
    let position = reader.position();
    if position != bytes.len() { return Err(protocol::ProtocolError::Malformed { what: "patch-pixels", offset: position as u64, detail: "trailing payload bytes".into() }); }
    Ok(PngMutation::PatchPixels(PatchPixelsMutation { index, remove_count, pixels, move_to }))
}

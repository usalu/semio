//! 💾️ Direct replace-pixels binary codec.
use crate::standards::v_jfif_1_01::subsets::document::schema::mutations::*;
use crate::standards::v_jfif_1_01::subsets::document::schema::snapshot::*;
use crate::standards::v_jfif_1_01::subsets::document::io::binary::diff::*;
use crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::*;
use crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::Entry;
pub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../📡️.protocol.semio"), "replace-pixels");
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &JpgMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let JpgMutation::ReplacePixels(payload) = value else { return None };
    Some(encode_payload(payload))
}
pub fn encode_payload(payload: &ReplacePixelsMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    let ReplacePixelsMutation { pixels } = payload;
    let mut out = Vec::new();
    write_bytes_lp(&mut out, pixels);
    Ok(out)
}
pub fn decode(bytes: &[u8]) -> Result<JpgMutation, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let result: Result<JpgMutation, protocol::ProtocolError> = Ok(JpgMutation::ReplacePixels(ReplacePixelsMutation { pixels: read_bytes_lp(&mut reader).map_err(|e| malformed("op pixels", reader.position(), e))? }));
    let position = reader.position();
    if position != bytes.len() {
        return Err(protocol::ProtocolError::Malformed { what: "replace-pixels", offset: position as u64, detail: "trailing payload bytes".into() });
    }
    result
}

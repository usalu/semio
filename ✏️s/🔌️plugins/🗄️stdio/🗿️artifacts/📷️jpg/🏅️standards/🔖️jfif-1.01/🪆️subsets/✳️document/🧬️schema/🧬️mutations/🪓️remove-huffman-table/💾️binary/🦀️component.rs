//! 💾️ Direct remove-huffman-table binary codec.
use super::*;
use crate::artifacts::jpg::schema::diff::{self, *};
use crate::artifacts::jpg::schema::mutations::binary::Entry;
pub const BINARY_TAG: u8 = 6;
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &JpgMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let JpgMutation::RemoveHuffmanTable(payload) = value else { return None };
    Some(encode_payload(payload))
}
pub fn encode_payload(payload: &RemoveHuffmanTableMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    let RemoveHuffmanTableMutation { key } = payload;
    let mut out = Vec::new();
    diff::enc_huffman_key_bin(key, &mut out);
    Ok(out)
}
fn op_pack_err(error: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "remove-huffman-table", offset: 0, detail: error.to_string() }
}
pub fn decode(bytes: &[u8]) -> Result<JpgMutation, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let result: Result<JpgMutation, protocol::ProtocolError> =
        Ok(JpgMutation::RemoveHuffmanTable(crate::artifacts::jpg::schema::mutations::RemoveHuffmanTableMutation { key: diff::dec_huffman_key_bin(&mut reader).map_err(|e| malformed("op huffman-key", reader.position(), e))? }));
    let position = reader.position();
    if position != bytes.len() {
        return Err(protocol::ProtocolError::Malformed { what: "remove-huffman-table", offset: position as u64, detail: "trailing payload bytes".into() });
    }
    result
}

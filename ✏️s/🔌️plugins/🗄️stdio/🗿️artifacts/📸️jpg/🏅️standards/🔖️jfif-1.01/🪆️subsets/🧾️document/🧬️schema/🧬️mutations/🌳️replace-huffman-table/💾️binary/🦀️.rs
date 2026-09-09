//! 💾️ Direct replace-huffman-table binary codec.
use super::*;
use crate::schema::mutations::binary::Entry;
pub const BINARY_TAG: u8 = 5;
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &JpgMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let JpgMutation::ReplaceHuffmanTable(payload) = value else { return None };
    Some(encode_payload(payload))
}
pub fn encode_payload(payload: &ReplaceHuffmanTableMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    let ReplaceHuffmanTableMutation { table } = payload;
    let mut out = Vec::new();
    enc_huffman_table_bin(table, &mut out);
    Ok(out)
}
pub fn decode(bytes: &[u8]) -> Result<JpgMutation, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let result: Result<JpgMutation, protocol::ProtocolError> = Ok(JpgMutation::ReplaceHuffmanTable(ReplaceHuffmanTableMutation { table: dec_huffman_table_bin(&mut reader).map_err(|e| malformed("op huffman-table", reader.position(), e))? }));
    let position = reader.position();
    if position != bytes.len() {
        return Err(protocol::ProtocolError::Malformed { what: "replace-huffman-table", offset: position as u64, detail: "trailing payload bytes".into() });
    }
    result
}

//! 📦️ Draw artifact — binary document surface + laws (constitutional: pack).

use crate::artifacts::draw::DrawDocument;
use store::PackError;

/// 📦️ Encodes a `DrawDocument` to its binary pack form.
pub fn encode(document: &DrawDocument) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `DrawDocument` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<DrawDocument, PackError> {
    <DrawDocument as store::DocumentPack>::decode_pack(bytes)
}

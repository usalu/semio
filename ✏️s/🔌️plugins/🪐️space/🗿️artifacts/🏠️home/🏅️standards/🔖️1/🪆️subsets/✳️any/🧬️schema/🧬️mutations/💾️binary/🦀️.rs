//! ⚖️ S Home launcher artifact — binary command protocol surface + laws (constitutional: spr).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
use protocol::OpBinary;

pub const BINARY_TAGS: &[(&str, u8)] = &[("ChangeCatalogGeneration", super::change_catalog_generation::binary::BINARY_TAG)];

/// 📦️ Encodes an `SHomeMutation` to its binary command form.
pub fn encode_op(operation: &SHomeMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes an `SHomeMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<SHomeMutation, protocol::ProtocolError> {
    SHomeMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

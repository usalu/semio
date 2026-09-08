//! ⚖️ S Space index artifact — binary command protocol surface + laws (constitutional: spr).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::text::SSpaceMutation;
use protocol::OpBinary;

pub const BINARY_TAG_REGISTRY: &[(&str, u8)] = &[
    ("create-artifact", super::create_artifact::binary::BINARY_TAG),
    ("delete-artifact", super::delete_artifact::binary::BINARY_TAG),
    ("rename-artifact", super::rename_artifact::binary::BINARY_TAG),
    ("touch-artifact", super::touch_artifact::binary::BINARY_TAG),
];

/// 📦️ Encodes an `SSpaceMutation` to its binary command form.
pub fn encode_op(operation: &SSpaceMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes an `SSpaceMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<SSpaceMutation, protocol::ProtocolError> {
    SSpaceMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

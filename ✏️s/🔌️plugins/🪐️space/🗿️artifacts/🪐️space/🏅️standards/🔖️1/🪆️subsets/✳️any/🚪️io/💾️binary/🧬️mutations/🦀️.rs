//! ⚖️ S Space index artifact — binary command protocol surface + laws (constitutional: spr).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use protocol::OpBinary;

pub const BINARY_TAG_REGISTRY: &[(&str, u8)] = &[
    ("create-artifact", create_artifact::BINARY_TAG),
    ("delete-artifact", delete_artifact::BINARY_TAG),
    ("rename-artifact", rename_artifact::BINARY_TAG),
    ("touch-artifact", touch_artifact::BINARY_TAG),
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

pub use semio_s_artifact_space_space_schema::standards::v1::subsets::any::io::binary::mutations::rename_artifact;

pub use semio_s_artifact_space_space_schema::standards::v1::subsets::any::io::binary::mutations::delete_artifact;

pub use semio_s_artifact_space_space_schema::standards::v1::subsets::any::io::binary::mutations::touch_artifact;

pub use semio_s_artifact_space_space_schema::standards::v1::subsets::any::io::binary::mutations::create_artifact;

mod native_codec {
#[cfg(test)] use super::*;
use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;

impl protocol::OpBinary for SSpaceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("📡️.protocol.semio"), bytes)
    }
}
}

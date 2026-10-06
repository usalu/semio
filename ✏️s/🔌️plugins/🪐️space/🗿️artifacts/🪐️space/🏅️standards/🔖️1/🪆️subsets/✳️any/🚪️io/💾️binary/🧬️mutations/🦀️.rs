//! ⚖️ S Space index artifact — binary command protocol surface + laws (constitutional: spr).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use protocol::OpBinary;

pub const BINARY_TAG_REGISTRY: &[(&str, u8)] = &[
    ("create-artifact", crate::standards::v1::subsets::any::schema::mutations::create_artifact::BINARY_TAG),
    ("delete-artifact", crate::standards::v1::subsets::any::schema::mutations::delete_artifact::BINARY_TAG),
    ("rename-artifact", crate::standards::v1::subsets::any::schema::mutations::rename_artifact::BINARY_TAG),
    ("touch-artifact", crate::standards::v1::subsets::any::schema::mutations::touch_artifact::BINARY_TAG),
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

#[path = "🏷️rename-artifact/🦀️.rs"]
pub mod rename_artifact;

#[path = "🗑️delete-artifact/🦀️.rs"]
pub mod delete_artifact;

#[path = "🕒touch-artifact/🦀️.rs"]
pub mod touch_artifact;

#[path = "🌱create-artifact/🦀️.rs"]
pub mod create_artifact;

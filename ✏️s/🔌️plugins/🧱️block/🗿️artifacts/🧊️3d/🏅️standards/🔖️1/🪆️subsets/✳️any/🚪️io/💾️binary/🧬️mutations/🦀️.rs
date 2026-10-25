//! ⚖️ Block 3D artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use semio_s_artifact_block_3d_mutations::Block3dMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `Block3dMutation` to its binary command form.
pub fn encode_op(operation: &Block3dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Block3dMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Block3dMutation, protocol::ProtocolError> {
    Block3dMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

mod native_codec {
#[cfg(test)] use crate::binary_mutations::*;
use semio_s_artifact_block_3d_mutations::Block3dMutation;
#[cfg(test)] use semio_s_artifact_block_3d_mutations::inverse_block3d_mutation;

impl protocol::OpBinary for Block3dMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("📡️.protocol.semio"), bytes)
    }
}
}

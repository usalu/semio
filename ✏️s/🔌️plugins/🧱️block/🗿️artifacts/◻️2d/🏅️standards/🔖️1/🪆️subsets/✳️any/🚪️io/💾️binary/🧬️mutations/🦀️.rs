//! ⚖️ Block 2D artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use semio_s_artifact_block_2d_mutations::Block2dMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `Block2dMutation` to its binary command form.
pub fn encode_op(operation: &Block2dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Block2dMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Block2dMutation, protocol::ProtocolError> {
    Block2dMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

mod native_codec {
#[cfg(test)] use crate::binary_mutations::*;
use semio_s_artifact_block_2d_mutations::Block2dMutation;
#[cfg(test)] use semio_s_artifact_block_2d_mutations::inverse_block2d_mutation;

impl protocol::OpBinary for Block2dMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("📡️.protocol.semio"), bytes)
    }
}
}

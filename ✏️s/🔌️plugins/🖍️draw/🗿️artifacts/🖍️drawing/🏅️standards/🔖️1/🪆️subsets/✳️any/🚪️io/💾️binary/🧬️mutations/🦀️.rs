//! 📡️ Drawing artifact — wire codec (encode_op/decode_op), renamed from the old `protocol` half
//! (constitutional: spr — state patch representation).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::op::DrawingMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `DrawingMutation` to its binary command form.
pub fn encode_op(operation: &DrawingMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `DrawingMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<DrawingMutation, protocol::ProtocolError> {
    DrawingMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

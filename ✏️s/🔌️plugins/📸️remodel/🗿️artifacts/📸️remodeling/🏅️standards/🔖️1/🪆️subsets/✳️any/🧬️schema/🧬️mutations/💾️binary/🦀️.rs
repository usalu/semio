//! ⚖️ Remodeling artifact — the binary operation surface (`spr`) and its laws.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::schema::mutations::text::RemodelingMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `RemodelingMutation` to its binary command form.
pub fn encode_op(operation: &RemodelingMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `RemodelingMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<RemodelingMutation, protocol::ProtocolError> {
    RemodelingMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//! 📡️ `trinity.rewrite.rule` artifact — binary command protocol surface + laws (constitutional:
//! spr, renamed from the old `📡️protocol` — no `📡️protocol` segment survives). `RewriteRuleMutation`
//! already derives `dsl::DslOps` directly (see `🔧️op`), so this file is a pure wrapper, unlike
//! `jack`'s `📡️spr` which needs a full DSL mirror.


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::rewrite::op::RewriteRuleMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `RewriteRuleMutation` to its binary command form.
pub fn encode_op(operation: &RewriteRuleMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `RewriteRuleMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<RewriteRuleMutation, protocol::ProtocolError> {
    RewriteRuleMutation::decode_op(bytes)
}

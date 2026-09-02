//! 📡️ `trinity.rewrite.rule` artifact — binary command protocol surface + laws (constitutional:
//! spr, renamed from the old `📡️protocol` — no `📡️protocol` segment survives). `RewriteRuleMutation`
//! already derives `dsl::DslOps` directly (see `🔧️op`), so this file is a pure wrapper, unlike
//! `jack`'s `📡️spr` which needs a full DSL mirror.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::rewriting::schema::mutations::text::RewriteRuleMutation;
use protocol::OpBinary;

/// 🧾️ Direct-owner binary tags in aggregate declaration order.
pub const BINARY_TAG_REGISTRY: &[(&str, u8)] = &[
    ("EditBeforeFixture", super::edit_before_fixture::binary::BINARY_TAG),
    ("EditLhs", super::edit_lhs::binary::BINARY_TAG),
    ("EditRhs", super::edit_rhs::binary::BINARY_TAG),
    ("ChangeParameterBinding", super::change_parameter_binding::binary::BINARY_TAG),
    ("RemoveParameterBinding", super::remove_parameter_binding::binary::BINARY_TAG),
    ("ChangeRuleLayoutPoint", super::change_rule_layout_point::binary::BINARY_TAG),
    ("RemoveRuleLayoutPoint", super::remove_rule_layout_point::binary::BINARY_TAG),
];

/// 📦️ Encodes a `RewriteRuleMutation` to its binary command form.
pub fn encode_op(operation: &RewriteRuleMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `RewriteRuleMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<RewriteRuleMutation, protocol::ProtocolError> {
    RewriteRuleMutation::decode_op(bytes)
}

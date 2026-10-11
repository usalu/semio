//! 📡️ `trinity.rewrite.rule` artifact — binary command protocol surface + laws (constitutional:
//! spr, renamed from the old `📡️protocol` — no `📡️protocol` segment survives). `RewriteRuleMutation`
//! already derives `dsl::DslOps` directly (see `🔧️op`), so this file is a pure wrapper, unlike
//! `jack`'s `📡️spr` which needs a full DSL mirror.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use protocol::OpBinary;

/// 🧾️ Direct-owner binary tags in aggregate declaration order.
pub const BINARY_TAG_REGISTRY: &[(&str, u8)] = &[
    ("EditWorkingGraph", edit_working_graph::BINARY_TAG),
    ("EditLhs", edit_lhs::BINARY_TAG),
    ("EditRhs", edit_rhs::BINARY_TAG),
    ("ChangeParameterBinding", change_parameter_binding::BINARY_TAG),
    ("RemoveParameterBinding", remove_parameter_binding::BINARY_TAG),
    ("ChangeRuleLayoutPoint", change_rule_layout_point::BINARY_TAG),
    ("RemoveRuleLayoutPoint", remove_rule_layout_point::BINARY_TAG),
    ("DragRuleNodes", drag_rule_nodes::BINARY_TAG),
    ("SetRuleLayoutPoints", set_rule_layout_points::BINARY_TAG),
];

/// 📦️ Encodes a `RewriteRuleMutation` to its binary command form.
pub fn encode_op(operation: &RewriteRuleMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `RewriteRuleMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<RewriteRuleMutation, protocol::ProtocolError> {
    RewriteRuleMutation::decode_op(bytes)
}

#[path = "📐️change-rule-layout/🦀️.rs"]
pub mod change_rule_layout_point;

#[path = "🫳️drag-rule/🦀️.rs"]
pub mod drag_rule_nodes;

#[path = "👉️edit-rhs/🦀️.rs"]
pub mod edit_rhs;

#[path = "🖼️edit-working-graph/🦀️.rs"]
pub mod edit_working_graph;

#[path = "🗑️remove-rule-layout/🦀️.rs"]
pub mod remove_rule_layout_point;

#[path = "🔧️change-parameter/🦀️.rs"]
pub mod change_parameter_binding;

#[path = "👈️edit-lhs/🦀️.rs"]
pub mod edit_lhs;

#[path = "🧹️remove-parameter-binding/🦀️.rs"]
pub mod remove_parameter_binding;

#[path = "📍️set-rule-layout/🦀️.rs"]
pub mod set_rule_layout_points;

mod native_codec {
use super::*;
use crate::RewritingSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::standards::v1::subsets::any::schema::operations::{
    apply_rewrite_rule_mutation, create_rewrite_rule_envelope, dispatch_rewrite_rule_mutations, inverse_rewrite_rule_mutation, RewriteRuleEnvelope, RewriteRuleStore,
};

impl protocol::OpBinary for RewriteRuleMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("📡️.protocol.semio"), bytes)
    }
}
}

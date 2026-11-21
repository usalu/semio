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
use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;
use protocol::OpBinary;

dsl::original_tagged_operation_receiving!(RewriteRuleMutation, COMPONENT_PROTOCOL_SEMIO);

/// 🪝️ Admits native publication and replay owners for the declared mutation protocol.
pub fn build_document_store_owners(
    grant: semio_framework_value::retained_clone::RetainedCloneGrant,
) -> Result<
    (
        semio_os_kernel::os_store::document_store_owners::DocumentStoreOwners<RewritingSnapshot, RewriteRuleMutation>,
        semio_framework_value::retained_clone::RetainedCloneProgress,
    ),
    semio_os_kernel::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<RewritingSnapshot, RewriteRuleMutation>,
> {
    semio_plugin::cold_replay_document_store_owners(grant).map(|(mut owners, progress)| {
        owners.operation_wire.source = prepared_operation_source;
        owners.operation_wire = owners.operation_wire.with_original_binary_receiving();
        (owners, progress)
    })
}

fn prepared_operation_source(operation: &RewriteRuleMutation) -> Option<semio_os_kernel::ArtifactPreparedOperationSource<'_>> {
    use RewriteRuleMutation::*;
    fn borrow<T: semio_framework_dsl_record::DslField + semio_framework_dsl_record::BorrowedDslRecord>(
        value: &T,
        tag: u8,
    ) -> Option<semio_os_kernel::ArtifactPreparedOperationSource<'_>> {
        Some(semio_os_kernel::ArtifactPreparedOperationSource::Pack { tag: u64::from(tag), body: value, spec: T::RECORD })
    }
    match operation {
        EditWorkingGraph(value) => borrow(value, edit_working_graph::BINARY_TAG),
        EditLhs(value) => borrow(value, edit_lhs::BINARY_TAG),
        EditRhs(value) => borrow(value, edit_rhs::BINARY_TAG),
        ChangeParameterBinding(value) => borrow(value, change_parameter_binding::BINARY_TAG),
        RemoveParameterBinding(value) => borrow(value, remove_parameter_binding::BINARY_TAG),
        ChangeRuleLayoutPoint(value) => borrow(value, change_rule_layout_point::BINARY_TAG),
        RemoveRuleLayoutPoint(value) => borrow(value, remove_rule_layout_point::BINARY_TAG),
        DragRuleNodes(value) => borrow(value, drag_rule_nodes::BINARY_TAG),
        SetRuleLayoutPoints(value) => borrow(value, set_rule_layout_points::BINARY_TAG),
    }
}

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

pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::io::binary::mutations::change_rule_layout_point;

pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::io::binary::mutations::drag_rule_nodes;

pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::io::binary::mutations::edit_rhs;

pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::io::binary::mutations::edit_working_graph;

pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::io::binary::mutations::remove_rule_layout_point;

pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::io::binary::mutations::change_parameter_binding;

pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::io::binary::mutations::edit_lhs;

pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::io::binary::mutations::remove_parameter_binding;

pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::io::binary::mutations::set_rule_layout_points;

mod native_codec {
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;

impl protocol::OpBinary for RewriteRuleMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("📡️.protocol.semio"), bytes)
    }
}
}

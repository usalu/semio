//! 🔌️ Block2d mutation — `ChangeHandleKindDefaultWireKind`: a handle-kind catalog row's `defaultWireKind`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_artifact_block_2d_diff::Block2dDiff;

//#region 🔖️Mutation
/// 🔌️ `change-handle-kind-default-wire-kind` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-handle-kind-default-wire-kind")]
pub struct ChangeHandleKindDefaultWireKind {
    pub id: String,
    pub new_default_wire_kind: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_handle_kind_default_wire_kind<Op: From<crate::change_handle_kind_default_wire_kind::ChangeHandleKindDefaultWireKind>>(id: String, new_default_wire_kind: String) -> Op {
    Op::from(ChangeHandleKindDefaultWireKind { id, new_default_wire_kind })
}

impl<Op> protocol::MutationKind<Block2dSnapshot, Op> for ChangeHandleKindDefaultWireKind where Op: protocol::Mutation<Block2dSnapshot, Diff = semio_s_artifact_block_2d_diff::Block2dDiff> + From<crate::change_handle_kind_default_wire_kind::ChangeHandleKindDefaultWireKind> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "handle-kind", kind: "change-handle-kind-default-wire-kind", record: "ChangedHandleKindDefaultWireKind" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change handle kind \"{}\" default wire kind to \"{}\"", self.id, self.new_default_wire_kind), &format!("Standarddrahtart von Griffart \"{}\" auf \"{}\" ändern", self.id, self.new_default_wire_kind))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Block2dSnapshot, Op> for ChangeHandleKindDefaultWireKind where Op: protocol::ColdMutation<Block2dSnapshot, ReferenceDiff = semio_s_artifact_block_2d_diff::Block2dDiff> + From<crate::change_handle_kind_default_wire_kind::ChangeHandleKindDefaultWireKind> {
    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        crate::change_handle_kind_default_wire_kind::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_handle_kind_default_wire_kind::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts

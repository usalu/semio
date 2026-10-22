//! 🖼️ Block2d mutation — `ChangeNodeKindIcon`: the node kind's optional `icon`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_artifact_block_2d_diff::Block2dDiff;

//#region 🔖️Mutation
/// 🖼️ `change-node-kind-icon` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-node-kind-icon")]
pub struct ChangeNodeKindIcon {
    pub new_icon: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_node_kind_icon<Op: From<crate::change_node_kind_icon::ChangeNodeKindIcon>>(new_icon: Option<String>) -> Op {
    Op::from(ChangeNodeKindIcon { new_icon })
}

impl<Op> protocol::MutationKind<Block2dSnapshot, Op> for ChangeNodeKindIcon where Op: protocol::Mutation<Block2dSnapshot, Diff = semio_s_artifact_block_2d_diff::Block2dDiff> + From<crate::change_node_kind_icon::ChangeNodeKindIcon> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node-kind", kind: "change-node-kind-icon", record: "ChangedNodeKindIcon" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change node kind icon to {:?}", self.new_icon), &format!("Knotenartsymbol auf {:?} ändern", self.new_icon))
    }
}

impl<Op> protocol::ColdMutationKind<Block2dSnapshot, Op> for ChangeNodeKindIcon where Op: protocol::ColdMutation<Block2dSnapshot, ReferenceDiff = semio_s_artifact_block_2d_diff::Block2dDiff> + From<crate::change_node_kind_icon::ChangeNodeKindIcon> {
    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        crate::change_node_kind_icon::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_node_kind_icon::inverse::inverse(self, base)?
    
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

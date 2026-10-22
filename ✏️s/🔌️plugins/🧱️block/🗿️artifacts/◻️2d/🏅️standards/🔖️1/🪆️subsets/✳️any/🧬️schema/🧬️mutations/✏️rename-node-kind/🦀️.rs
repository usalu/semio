//! ✏️ Block2d mutation — `RenameNodeKind`: the node kind's identity `name` field.

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_artifact_block_2d_diff::Block2dDiff;

//#region 🔖️Mutation
/// ✏️ `rename-node-kind` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "rename-node-kind")]
pub struct RenameNodeKind {
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rename_node_kind<Op: From<crate::rename_node_kind::RenameNodeKind>>(new_name: String) -> Op {
    Op::from(RenameNodeKind { new_name })
}

impl<Op> protocol::MutationKind<Block2dSnapshot, Op> for RenameNodeKind where Op: protocol::Mutation<Block2dSnapshot, Diff = semio_s_artifact_block_2d_diff::Block2dDiff> + From<crate::rename_node_kind::RenameNodeKind> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "node-kind", kind: "rename-node-kind", record: "RenamedNodeKind" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename node kind to \"{}\"", self.new_name), &format!("Knotenart in \"{}\" umbenennen", self.new_name))
    }
}

impl<Op> protocol::ColdMutationKind<Block2dSnapshot, Op> for RenameNodeKind where Op: protocol::ColdMutation<Block2dSnapshot, ReferenceDiff = semio_s_artifact_block_2d_diff::Block2dDiff> + From<crate::rename_node_kind::RenameNodeKind> {
    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        crate::rename_node_kind::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::rename_node_kind::inverse::inverse(self, base)?
    
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

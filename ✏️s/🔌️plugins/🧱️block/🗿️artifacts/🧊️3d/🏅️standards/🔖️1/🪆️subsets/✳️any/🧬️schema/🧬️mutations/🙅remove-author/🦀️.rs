//! 🙅 Block3d mutation — `RemoveAuthor`: a credited author.

use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_diff::Block3dDiff;

//#region 🔖️Mutation
/// 🙅 `remove-author` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "remove-author")]
pub struct RemoveAuthor {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_author<Op: From<crate::remove_author::RemoveAuthor>>(id: String) -> Op {
    Op::from(RemoveAuthor { id })
}

impl<Op> protocol::MutationKind<Block3dSnapshot, Op> for RemoveAuthor where Op: protocol::Mutation<Block3dSnapshot, Diff = semio_s_artifact_block_3d_diff::Block3dDiff> + From<crate::add_author::AddAuthor> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "author", kind: "remove-author", record: "RemovedAuthor" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove author \"{}\"", self.id), &format!("Autor \"{}\" entfernen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Block3dSnapshot, Op> for RemoveAuthor where Op: protocol::ColdMutation<Block3dSnapshot, ReferenceDiff = semio_s_artifact_block_3d_diff::Block3dDiff> + From<crate::add_author::AddAuthor> {
    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        crate::remove_author::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::remove_author::inverse::inverse(self, base)?
    
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

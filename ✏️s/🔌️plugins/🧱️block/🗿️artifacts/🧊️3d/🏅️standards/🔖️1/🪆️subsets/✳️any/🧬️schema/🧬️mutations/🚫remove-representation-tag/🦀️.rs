//! 🚫 Block3d mutation — `RemoveRepresentationTag`: a member of a representation's `tags` set.

use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_diff::Block3dDiff;

//#region 🔖️Mutation
/// 🚫 `remove-representation-tag` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "remove-representation-tag")]
pub struct RemoveRepresentationTag {
    pub id: String,
    pub tag: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_representation_tag<Op: From<crate::remove_representation_tag::RemoveRepresentationTag>>(id: String, tag: String) -> Op {
    Op::from(RemoveRepresentationTag { id, tag })
}

impl<Op> protocol::MutationKind<Block3dSnapshot, Op> for RemoveRepresentationTag where Op: protocol::Mutation<Block3dSnapshot, Diff = semio_s_artifact_block_3d_diff::Block3dDiff> + From<crate::add_representation_tag::AddRepresentationTag> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "representation-tag", kind: "remove-representation-tag", record: "RemovedRepresentationTag" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove tag \"{}\" from representation \"{}\"", self.tag, self.id), &format!("Tag \"{}\" aus Repräsentation \"{}\" entfernen", self.tag, self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Block3dSnapshot, Op> for RemoveRepresentationTag where Op: protocol::ColdMutation<Block3dSnapshot, ReferenceDiff = semio_s_artifact_block_3d_diff::Block3dDiff> + From<crate::add_representation_tag::AddRepresentationTag> {
    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        crate::remove_representation_tag::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::remove_representation_tag::inverse::inverse(self, base)?
    
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

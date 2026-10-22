//! 📃️ Block3d mutation — `ChangeObjectKindDescription`: the object kind's `description`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_diff::Block3dDiff;

//#region 🔖️Mutation
/// 📃️ `change-object-kind-description` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-object-kind-description")]
pub struct ChangeObjectKindDescription {
    pub new_description: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_object_kind_description<Op: From<crate::change_object_kind_description::ChangeObjectKindDescription>>(new_description: String) -> Op {
    Op::from(ChangeObjectKindDescription { new_description })
}

impl<Op> protocol::MutationKind<Block3dSnapshot, Op> for ChangeObjectKindDescription where Op: protocol::Mutation<Block3dSnapshot, Diff = semio_s_artifact_block_3d_diff::Block3dDiff> + From<crate::change_object_kind_description::ChangeObjectKindDescription> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "object-kind", kind: "change-object-kind-description", record: "ChangedObjectKindDescription" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change object kind description", "Objektartbeschreibung ändern")
    }
}

impl<Op> protocol::ColdMutationKind<Block3dSnapshot, Op> for ChangeObjectKindDescription where Op: protocol::ColdMutation<Block3dSnapshot, ReferenceDiff = semio_s_artifact_block_3d_diff::Block3dDiff> + From<crate::change_object_kind_description::ChangeObjectKindDescription> {
    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        crate::change_object_kind_description::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_object_kind_description::inverse::inverse(self, base)?
    
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

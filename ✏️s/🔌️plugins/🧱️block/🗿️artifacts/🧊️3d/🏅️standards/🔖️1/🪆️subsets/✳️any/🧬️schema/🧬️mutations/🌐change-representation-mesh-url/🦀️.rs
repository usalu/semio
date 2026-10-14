//! 🌐 Block3d mutation — `ChangeRepresentationMeshUrl`: a representation's `meshUrl`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_diff::Block3dDiff;

//#region 🔖️Mutation
/// 🌐 `change-representation-mesh-url` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-representation-mesh-url")]
pub struct ChangeRepresentationMeshUrl {
    pub id: String,
    pub new_mesh_url: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_representation_mesh_url<Op: From<crate::change_representation_mesh_url::ChangeRepresentationMeshUrl>>(id: String, new_mesh_url: Option<String>) -> Op {
    Op::from(ChangeRepresentationMeshUrl { id, new_mesh_url })
}

impl<Op> protocol::MutationKind<Block3dSnapshot, Op> for ChangeRepresentationMeshUrl where Op: protocol::Mutation<Block3dSnapshot, Diff = semio_s_artifact_block_3d_diff::Block3dDiff> + From<crate::change_representation_mesh_url::ChangeRepresentationMeshUrl> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "representation", kind: "change-representation-mesh-url", record: "ChangedRepresentationMeshUrl" };

    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        crate::change_representation_mesh_url::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_representation_mesh_url::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change representation \"{}\" mesh URL", self.id), &format!("Netz-URL von Repräsentation \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts

//! 🌐 Block3d mutation — `ChangeRepresentationMeshUrl`: a representation's `meshUrl`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::diff::{Block3dDiff, Block3dRepresentationsDelta, Block3dRepresentationsPatch, Block3dRepresentationsPatchEntry};
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Mutation
/// 🌐 `change-representation-mesh-url` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
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
pub fn change_representation_mesh_url(id: String, new_mesh_url: Option<String>) -> Block3dMutation {
    Block3dMutation::ChangeRepresentationMeshUrl(ChangeRepresentationMeshUrl { id, new_mesh_url })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for ChangeRepresentationMeshUrl {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "representation", kind: "change-representation-mesh-url", record: "ChangedRepresentationMeshUrl" };

    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change representation \"{}\" mesh URL", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

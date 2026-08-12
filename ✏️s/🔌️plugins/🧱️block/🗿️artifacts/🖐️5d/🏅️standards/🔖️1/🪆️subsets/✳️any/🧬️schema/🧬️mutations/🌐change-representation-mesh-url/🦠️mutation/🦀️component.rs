//! 🌐 Block5d mutation — `ChangeRepresentationMeshUrl`: a representation's `meshUrl`.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌐 `change-representation-mesh-url` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-representation-mesh-url")]
pub struct ChangeRepresentationMeshUrl {
    pub id: String,
    pub new_mesh_url: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_representation_mesh_url(id: String, new_mesh_url: Option<String>) -> Block5dMutation {
    Block5dMutation::ChangeRepresentationMeshUrl(ChangeRepresentationMeshUrl { id, new_mesh_url })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for ChangeRepresentationMeshUrl {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "representation", kind: "change-representation-mesh-url", record: "ChangedRepresentationMeshUrl" };

    fn diff(&self, base: &Block5dSnapshot) -> Block5dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
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

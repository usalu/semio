//! 🌐 Block5d mutation — `ChangeRepresentationMeshUrl`: a representation's `meshUrl`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::{Block5dDiff, Block5dRepresentationsDelta, Block5dRepresentationsPatch, Block5dRepresentationsPatchEntry};
use crate::artifacts::block5d::mutations::Block5dMutation;

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
pub async fn change_representation_mesh_url(id: String, new_mesh_url: Option<String>) -> Block5dMutation {
    Block5dMutation::ChangeRepresentationMeshUrl(ChangeRepresentationMeshUrl { id, new_mesh_url })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for ChangeRepresentationMeshUrl {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "representation", kind: "change-representation-mesh-url", record: "ChangedRepresentationMeshUrl" };

    async fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change representation \"{}\" mesh URL", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

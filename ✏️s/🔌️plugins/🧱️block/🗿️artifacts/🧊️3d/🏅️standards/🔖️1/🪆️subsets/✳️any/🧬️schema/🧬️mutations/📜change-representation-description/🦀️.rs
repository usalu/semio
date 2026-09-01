//! 📜 Block3d mutation — `ChangeRepresentationDescription`: a representation's `description`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::diff::{Block3dDiff, Block3dRepresentationsDelta, Block3dRepresentationsPatch, Block3dRepresentationsPatchEntry};
use crate::artifacts::block3d::mutations::Block3dMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📜 `change-representation-description` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-representation-description")]
pub struct ChangeRepresentationDescription {
    pub id: String,
    pub new_description: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_representation_description(id: String, new_description: String) -> Block3dMutation {
    Block3dMutation::ChangeRepresentationDescription(ChangeRepresentationDescription { id, new_description })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for ChangeRepresentationDescription {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "representation", kind: "change-representation-description", record: "ChangedRepresentationDescription" };

    async fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change representation \"{}\" description", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

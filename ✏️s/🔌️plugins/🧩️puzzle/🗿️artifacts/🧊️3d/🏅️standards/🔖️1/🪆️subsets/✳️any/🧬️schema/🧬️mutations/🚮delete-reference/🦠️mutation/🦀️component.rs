//! Puzzle3d mutation — `DeleteReference`: removes an id-keyed reference.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `delete-reference` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-reference")]
pub struct DeleteReference {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn delete_reference(id: String) -> Puzzle3dMutation {
    Puzzle3dMutation::DeleteReference(DeleteReference { id })
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for DeleteReference {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "reference", kind: "delete-reference", record: "DeletedReference" };

    async fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete reference \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

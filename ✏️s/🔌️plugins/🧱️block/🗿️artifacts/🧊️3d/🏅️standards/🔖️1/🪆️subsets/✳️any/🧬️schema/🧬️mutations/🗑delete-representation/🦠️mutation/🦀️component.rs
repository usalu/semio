//! 🗑 Block3d mutation — `DeleteRepresentation`: a representation.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🗑 `delete-representation` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-representation")]
pub struct DeleteRepresentation {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn delete_representation(id: String) -> Block3dMutation {
    Block3dMutation::DeleteRepresentation(DeleteRepresentation { id })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for DeleteRepresentation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "representation", kind: "delete-representation", record: "DeletedRepresentation" };

    async fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete representation \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

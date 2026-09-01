//! 🕳 Block3d mutation — `DeleteVortex`: a rim-vortex template.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::diff::{Block3dDiff, Block3dVorticesDelta};
use crate::artifacts::block3d::mutations::Block3dMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🕳 `delete-vortex` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-vortex")]
pub struct DeleteVortex {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn delete_vortex(id: String) -> Block3dMutation {
    Block3dMutation::DeleteVortex(DeleteVortex { id })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for DeleteVortex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "vortex", kind: "delete-vortex", record: "DeletedVortex" };

    async fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete vortex \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

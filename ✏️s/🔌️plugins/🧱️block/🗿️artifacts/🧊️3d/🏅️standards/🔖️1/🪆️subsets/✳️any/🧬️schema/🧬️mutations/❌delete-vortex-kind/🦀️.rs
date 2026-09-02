//! ❌ Block3d mutation — `DeleteVortexKind`: a vortex-kind catalog row.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::diff::{Block3dDiff, Block3dVortexKindsDelta};
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Mutation
/// ❌ `delete-vortex-kind` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "delete-vortex-kind")]
pub struct DeleteVortexKind {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn delete_vortex_kind(id: String) -> Block3dMutation {
    Block3dMutation::DeleteVortexKind(DeleteVortexKind { id })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for DeleteVortexKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "vortex-kind", kind: "delete-vortex-kind", record: "DeletedVortexKind" };

    async fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete vortex kind \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

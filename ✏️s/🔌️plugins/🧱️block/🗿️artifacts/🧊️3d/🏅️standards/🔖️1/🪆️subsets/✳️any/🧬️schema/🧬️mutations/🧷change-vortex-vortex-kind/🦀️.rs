//! 🧷 Block3d mutation — `ChangeVortexVortexKind`: a vortex's `vortexKind` catalog reference (rebind).

use crate::artifacts::block3d::{Block3dSnapshot, Block3dVortexTemplate};
use crate::artifacts::block3d::diff::{Block3dDiff, Block3dVorticesDelta, Block3dVorticesPatch, Block3dVorticesPatchEntry};
use crate::artifacts::block3d::mutations::Block3dMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧷 `change-vortex-vortex-kind` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-vortex-vortex-kind")]
pub struct ChangeVortexVortexKind {
    pub id: String,
    pub new_vortex_kind: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_vortex_vortex_kind(id: String, new_vortex_kind: String) -> Block3dMutation {
    Block3dMutation::ChangeVortexVortexKind(ChangeVortexVortexKind { id, new_vortex_kind })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for ChangeVortexVortexKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "vortex", kind: "change-vortex-vortex-kind", record: "ChangedVortexVortexKind" };

    async fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change vortex \"{}\" vortex kind to \"{}\"", self.id, self.new_vortex_kind)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

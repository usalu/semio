//! 🪧 Block3d mutation — `ChangeVortexLabel`: a vortex's optional display `label`.

use crate::artifacts::block3d::{Block3dSnapshot, Block3dVortexTemplate};
use crate::artifacts::block3d::diff::{Block3dDiff, Block3dVorticesDelta, Block3dVorticesPatch, Block3dVorticesPatchEntry};
use crate::artifacts::block3d::mutations::Block3dMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🪧 `change-vortex-label` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-vortex-label")]
pub struct ChangeVortexLabel {
    pub id: String,
    pub new_label: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_vortex_label(id: String, new_label: Option<String>) -> Block3dMutation {
    Block3dMutation::ChangeVortexLabel(ChangeVortexLabel { id, new_label })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for ChangeVortexLabel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "vortex", kind: "change-vortex-label", record: "ChangedVortexLabel" };

    async fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change vortex \"{}\" label", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

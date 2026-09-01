//! 🔌 Block3d mutation — `ChangeVortexKindDefaultCableKind`: a vortex-kind catalog row's `defaultCableKind`.

use crate::artifacts::block3d::{Block3dSnapshot, Block3dVortexKind};
use crate::artifacts::block3d::diff::{Block3dDiff, Block3dVortexKindsDelta, Block3dVortexKindsPatch, Block3dVortexKindsPatchEntry};
use crate::artifacts::block3d::mutations::Block3dMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔌 `change-vortex-kind-default-cable-kind` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-vortex-kind-default-cable-kind")]
pub struct ChangeVortexKindDefaultCableKind {
    pub id: String,
    pub new_default_cable_kind: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_vortex_kind_default_cable_kind(id: String, new_default_cable_kind: String) -> Block3dMutation {
    Block3dMutation::ChangeVortexKindDefaultCableKind(ChangeVortexKindDefaultCableKind { id, new_default_cable_kind })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for ChangeVortexKindDefaultCableKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "vortex-kind", kind: "change-vortex-kind-default-cable-kind", record: "ChangedVortexKindDefaultCableKind" };

    async fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change vortex kind \"{}\" default cable kind to \"{}\"", self.id, self.new_default_cable_kind)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

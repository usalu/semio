//! 🌱 Block3d mutation — `CreateVortexKind`: a new vortex-kind catalog row.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::{Block3dVortexKind};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱 `create-vortex-kind` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-vortex-kind")]
pub struct CreateVortexKind {
    #[dsl(block)]
    pub vortex_kind: Block3dVortexKind,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn create_vortex_kind(vortex_kind: Block3dVortexKind) -> Block3dMutation {
    Block3dMutation::CreateVortexKind(CreateVortexKind { vortex_kind })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for CreateVortexKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "vortex-kind", kind: "create-vortex-kind", record: "CreatedVortexKind" };

    async fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create vortex kind \"{}\"", self.vortex_kind.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.vortex_kind.id.clone()]
    }
}
//#endregion 🔖️Mutation

//! 🌀 Block3d mutation — `CreateVortex`: a new rim-vortex template.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::Block3dVortexTemplate;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌀 `create-vortex` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-vortex")]
pub struct CreateVortex {
    #[dsl(block)]
    pub vortex: Block3dVortexTemplate,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn create_vortex(vortex: Block3dVortexTemplate) -> Block3dMutation {
    Block3dMutation::CreateVortex(CreateVortex { vortex })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for CreateVortex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "vortex", kind: "create-vortex", record: "CreatedVortex" };

    async fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create vortex \"{}\"", self.vortex.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.vortex.id.clone()]
    }
}
//#endregion 🔖️Mutation

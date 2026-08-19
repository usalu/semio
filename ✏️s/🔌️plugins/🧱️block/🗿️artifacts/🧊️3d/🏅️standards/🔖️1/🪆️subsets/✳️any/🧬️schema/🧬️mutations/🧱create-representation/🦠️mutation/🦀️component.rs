//! 🧱 Block3d mutation — `CreateRepresentation`: a new representation (mesh at a LOD/tag combination).
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::{BlockRepresentation};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧱 `create-representation` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-representation")]
pub struct CreateRepresentation {
    #[dsl(block)]
    pub representation: BlockRepresentation,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn create_representation(representation: BlockRepresentation) -> Block3dMutation {
    Block3dMutation::CreateRepresentation(CreateRepresentation { representation })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for CreateRepresentation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "representation", kind: "create-representation", record: "CreatedRepresentation" };

    async fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create representation \"{}\"", self.representation.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.representation.id.clone()]
    }
}
//#endregion 🔖️Mutation

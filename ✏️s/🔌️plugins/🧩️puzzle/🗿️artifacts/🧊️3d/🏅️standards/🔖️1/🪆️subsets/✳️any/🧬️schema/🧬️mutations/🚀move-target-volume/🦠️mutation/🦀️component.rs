//! Puzzle3d mutation — `MoveTargetVolume`: absolute reposition of a target volume's origin.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `move-target-volume` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "move-target-volume")]
pub struct MoveTargetVolume {
    pub id: String,
    pub new_origin: [f64; 3],
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for MoveTargetVolume {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "target-volume", kind: "move-target-volume", record: "MovedTargetVolume" };

    async fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Move target volume \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn move_target_volume(id: String, new_origin: [f64; 3]) -> Puzzle3dMutation {
    Puzzle3dMutation::MoveTargetVolume(MoveTargetVolume { id, new_origin })
}

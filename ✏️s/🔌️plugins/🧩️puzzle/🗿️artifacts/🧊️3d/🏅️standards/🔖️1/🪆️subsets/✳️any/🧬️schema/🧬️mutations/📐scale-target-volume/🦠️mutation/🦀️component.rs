//! Puzzle3d mutation — `ScaleTargetVolume`: changes a target volume's freeform pose scale.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `scale-target-volume` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "scale-target-volume")]
pub struct ScaleTargetVolume {
    pub id: String,
    pub new_scale: Option<crate::artifacts::puzzle3d::Puzzle3dScale>,
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for ScaleTargetVolume {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "target-volume", kind: "scale-target-volume", record: "ScaledTargetVolume" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Scale target volume \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn scale_target_volume(id: String, new_scale: Option<crate::artifacts::puzzle3d::Puzzle3dScale>) -> Puzzle3dMutation {
    Puzzle3dMutation::ScaleTargetVolume(ScaleTargetVolume { id, new_scale })
}

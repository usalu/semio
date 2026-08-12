//! Puzzle3d mutation — `RotateTargetVolume`: changes a target volume's orientation quaternion.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `rotate-target-volume` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "rotate-target-volume")]
pub struct RotateTargetVolume {
    pub id: String,
    pub new_orientation: Option<[f64; 4]>,
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for RotateTargetVolume {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rotate", entity: "target-volume", kind: "rotate-target-volume", record: "RotatedTargetVolume" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> Puzzle3dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Rotate target volume \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rotate_target_volume(id: String, new_orientation: Option<[f64; 4]>) -> Puzzle3dMutation {
    Puzzle3dMutation::RotateTargetVolume(RotateTargetVolume { id, new_orientation })
}

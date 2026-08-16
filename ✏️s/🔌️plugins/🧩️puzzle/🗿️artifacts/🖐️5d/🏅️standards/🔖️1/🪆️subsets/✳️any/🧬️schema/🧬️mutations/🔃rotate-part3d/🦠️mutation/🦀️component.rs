//! Puzzle5d mutation — `RotatePart3d`: changes a part's 3D-projection orientation quaternion.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `rotate-part3d` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "rotate-part3d")]
pub struct RotatePart3d {
    pub id: String,
    pub new_orientation: Option<[f64; 4]>,
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for RotatePart3d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rotate", entity: "part", kind: "rotate-part3d", record: "RotatedPart3d" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Rotate part \"{}\" (3d)", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rotate_part_3d(id: String, new_orientation: Option<[f64; 4]>) -> Puzzle5dMutation {
    Puzzle5dMutation::RotatePart3d(RotatePart3d { id, new_orientation })
}

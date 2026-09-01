//! Puzzle3d mutation — `ReplaceAttractionGeometry`: whole-value swap of an attraction's pose-solver connection pose.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `replace-attraction-geometry` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-attraction-geometry")]
pub struct ReplaceAttractionGeometry {
    pub id: String,
    pub new_gap: f64,
    pub new_shift: f64,
    pub new_rise: f64,
    pub new_rotation: f64,
    pub new_turn: f64,
    pub new_tilt: f64,
    pub new_x: f64,
    pub new_y: f64,
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for ReplaceAttractionGeometry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "attraction", kind: "replace-attraction-geometry", record: "ReplacedAttractionGeometry" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace attraction \"{}\" geometry", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_attraction_geometry(id: String, new_gap: f64, new_shift: f64, new_rise: f64, new_rotation: f64, new_turn: f64, new_tilt: f64, new_x: f64, new_y: f64) -> Puzzle3dMutation {
    Puzzle3dMutation::ReplaceAttractionGeometry(ReplaceAttractionGeometry { id, new_gap, new_shift, new_rise, new_rotation, new_turn, new_tilt, new_x, new_y })
}

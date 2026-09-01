//! Puzzle5d mutation — `ReplaceFastenerGeometry`: whole-value swap of a fastener's pose-solver connection pose.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `replace-fastener-geometry` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-fastener-geometry")]
pub struct ReplaceFastenerGeometry {
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

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ReplaceFastenerGeometry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "fastener", kind: "replace-fastener-geometry", record: "ReplacedFastenerGeometry" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace fastener \"{}\" geometry", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_fastener_geometry(id: String, new_gap: f64, new_shift: f64, new_rise: f64, new_rotation: f64, new_turn: f64, new_tilt: f64, new_x: f64, new_y: f64) -> Puzzle5dMutation {
    Puzzle5dMutation::ReplaceFastenerGeometry(ReplaceFastenerGeometry { id, new_gap, new_shift, new_rise, new_rotation, new_turn, new_tilt, new_x, new_y })
}

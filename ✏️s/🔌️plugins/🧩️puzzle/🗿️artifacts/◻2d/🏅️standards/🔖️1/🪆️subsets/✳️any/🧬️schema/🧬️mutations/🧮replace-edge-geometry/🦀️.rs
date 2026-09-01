//! 🧮 Puzzle2d mutation — `ReplaceEdgeGeometry`: whole-value swap of an edge's connection-pose —
//! `gap`+`shift`+`rise`+`rotation`+`turn`+`tilt`+`x`+`y` together are the one connection geometry.

use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧮 `replace-edge-geometry` payload — new connection pose.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-edge-geometry")]
pub struct ReplaceEdgeGeometry {
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

/// 🏗️ Builder — wraps the payload in its dispatch variant.
#[allow(clippy::too_many_arguments)]
pub fn replace_edge_geometry(id: String, new_gap: f64, new_shift: f64, new_rise: f64, new_rotation: f64, new_turn: f64, new_tilt: f64, new_x: f64, new_y: f64) -> Puzzle2dMutation {
    Puzzle2dMutation::ReplaceEdgeGeometry(ReplaceEdgeGeometry { id, new_gap, new_shift, new_rise, new_rotation, new_turn, new_tilt, new_x, new_y })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for ReplaceEdgeGeometry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "edge", kind: "replace-edge-geometry", record: "ReplacedEdgeGeometry" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace edge \"{}\" geometry", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

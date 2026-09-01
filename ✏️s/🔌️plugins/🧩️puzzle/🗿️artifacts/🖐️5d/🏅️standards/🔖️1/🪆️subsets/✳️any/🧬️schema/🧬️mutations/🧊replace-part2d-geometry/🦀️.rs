//! Puzzle5d mutation — `ReplacePart2dGeometry`: whole-value swap of a part's 2D shape/extent.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `replace-part2d-geometry` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-part2d-geometry")]
pub struct ReplacePart2dGeometry {
    pub id: String,
    pub new_shape: Option<String>,
    pub new_radius: Option<f64>,
    pub new_width: Option<f64>,
    pub new_height: Option<f64>,
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ReplacePart2dGeometry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "part", kind: "replace-part2d-geometry", record: "ReplacedPart2dGeometry" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace part \"{}\" 2d geometry", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_part_2d_geometry(id: String, new_shape: Option<String>, new_radius: Option<f64>, new_width: Option<f64>, new_height: Option<f64>) -> Puzzle5dMutation {
    Puzzle5dMutation::ReplacePart2dGeometry(ReplacePart2dGeometry { id, new_shape, new_radius, new_width, new_height })
}

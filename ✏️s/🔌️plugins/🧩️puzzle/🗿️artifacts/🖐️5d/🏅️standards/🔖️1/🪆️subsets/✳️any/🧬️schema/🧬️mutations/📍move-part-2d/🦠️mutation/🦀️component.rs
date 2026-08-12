//! Puzzle5d mutation — `MovePart2d`: absolute reposition of a part's 2D-projection anchor point.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `move-part-2d` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "move-part-2d")]
pub struct MovePart2d {
    pub id: String,
    pub new_x: f64,
    pub new_y: f64,
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for MovePart2d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "part", kind: "move-part-2d", record: "MovedPart2d" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> Puzzle5dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Move part \"{}\" (2d)", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_part_2d(id: String, new_x: f64, new_y: f64) -> Puzzle5dMutation {
    Puzzle5dMutation::MovePart2d(MovePart2d { id, new_x, new_y })
}

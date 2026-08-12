//! Puzzle5d mutation — `ChangePart2dIcon`: changes a part's 2D-projection icon.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `change-part2d-icon` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-part2d-icon")]
pub struct ChangePart2dIcon {
    pub id: String,
    pub new_icon_kind: Option<String>,
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ChangePart2dIcon {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "part", kind: "change-part2d-icon", record: "ChangedPart2dIcon" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> Puzzle5dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change part \"{}\" 2d icon", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_part_2d_icon(id: String, new_icon_kind: Option<String>) -> Puzzle5dMutation {
    Puzzle5dMutation::ChangePart2dIcon(ChangePart2dIcon { id, new_icon_kind })
}

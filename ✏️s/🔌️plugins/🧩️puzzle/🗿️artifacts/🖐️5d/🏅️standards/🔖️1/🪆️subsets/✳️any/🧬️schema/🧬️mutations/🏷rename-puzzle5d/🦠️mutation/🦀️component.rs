//! 🏷 Puzzle5d mutation — `RenamePuzzle5d`: changes the document's display label (the closest
//! thing this fixture has to an identity field).
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🏷 `rename-puzzle5d` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "rename-puzzle5d")]
pub struct RenamePuzzle5d {
    pub new_label: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rename_puzzle5d(new_label: Option<String>) -> Puzzle5dMutation {
    Puzzle5dMutation::RenamePuzzle5d(RenamePuzzle5d { new_label })
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for RenamePuzzle5d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "puzzle5d", kind: "rename-puzzle5d", record: "RenamedPuzzle5d" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> Puzzle5dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Rename puzzle5d".to_string()
    }
}
//#endregion 🔖️Mutation

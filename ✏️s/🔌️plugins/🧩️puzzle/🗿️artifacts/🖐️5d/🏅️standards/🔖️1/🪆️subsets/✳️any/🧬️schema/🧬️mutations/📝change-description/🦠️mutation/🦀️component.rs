//! 📝 Puzzle5d mutation — `ChangeDescription`: changes the document's free-text scene description.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📝 `change-description` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-description")]
pub struct ChangeDescription {
    pub new_description: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_description(new_description: String) -> Puzzle5dMutation {
    Puzzle5dMutation::ChangeDescription(ChangeDescription { new_description })
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ChangeDescription {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "description", kind: "change-description", record: "ChangedDescription" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> Puzzle5dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Change description".to_string()
    }
}
//#endregion 🔖️Mutation

//! 🔀️ `reorder-runs` — repositions one run within the sequence (never spatial — `SemioTextRun`
//! carries no position of its own, only sequence order).

use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::SemioTextMutation;
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReorderRuns {
    pub from: usize,
    pub to: usize,
}

impl protocol::MutationKind<SemioTextSnapshot, SemioTextMutation> for ReorderRuns {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "runs", kind: "reorder-runs", record: "ReorderedRuns" };

    async fn diff(&self, base: &SemioTextSnapshot) -> protocol::MutationOutcome<<SemioTextMutation as protocol::Mutation<SemioTextSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SemioTextSnapshot) -> Vec<SemioTextMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Move run #{} to #{}", self.from, self.to)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.from.to_string()]
    }
}
//#endregion 🔖️Payload

//! Puzzle5d mutation — `ChangeFastenerKind`: changes a fastener's `fastener_kind` catalog reference.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `change-fastener-kind` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-fastener-kind")]
pub struct ChangeFastenerKind {
    pub id: String,
    pub new_fastener_kind: Option<String>,
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ChangeFastenerKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fastener", kind: "change-fastener-kind", record: "ChangedFastenerKind" };

    async fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change fastener \"{}\" kind", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_fastener_kind(id: String, new_fastener_kind: Option<String>) -> Puzzle5dMutation {
    Puzzle5dMutation::ChangeFastenerKind(ChangeFastenerKind { id, new_fastener_kind })
}

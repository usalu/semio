//! 💔 Puzzle5d mutation — `DisconnectKindCompatibility`: revokes one grip-kind-id pair's fasten
//! allowance.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 💔 `disconnect-kind-compatibility` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "disconnect-kind-compatibility")]
pub struct DisconnectKindCompatibility {
    pub source: String,
    pub target: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn disconnect_kind_compatibility(source: String, target: String) -> Puzzle5dMutation {
    Puzzle5dMutation::DisconnectKindCompatibility(DisconnectKindCompatibility { source, target })
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for DisconnectKindCompatibility {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "kind-compatibility", kind: "disconnect-kind-compatibility", record: "DisconnectedKindCompatibility" };

    async fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Disconnect kind compatibility \"{}\" -> \"{}\"", self.source, self.target)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.source.clone(), self.target.clone()]
    }
}
//#endregion 🔖️Mutation

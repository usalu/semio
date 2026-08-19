//! 💔 Puzzle2d mutation — `DisconnectKindCompatibility`: revokes one kind-id pair's link allowance.
use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
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
pub async fn disconnect_kind_compatibility(source: String, target: String) -> Puzzle2dMutation {
    Puzzle2dMutation::DisconnectKindCompatibility(DisconnectKindCompatibility { source, target })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for DisconnectKindCompatibility {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "kind-compatibility", kind: "disconnect-kind-compatibility", record: "DisconnectedKindCompatibility" };

    async fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
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

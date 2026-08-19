//! ✂️ Puzzle5d mutation — `DisconnectGrips`: removes a fastener between two grips.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ✂️ `disconnect-grips` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "disconnect-grips")]
pub struct DisconnectGrips {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn disconnect_grips(id: String) -> Puzzle5dMutation {
    Puzzle5dMutation::DisconnectGrips(DisconnectGrips { id })
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for DisconnectGrips {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "grips", kind: "disconnect-grips", record: "DisconnectedGrips" };

    async fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Disconnect \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

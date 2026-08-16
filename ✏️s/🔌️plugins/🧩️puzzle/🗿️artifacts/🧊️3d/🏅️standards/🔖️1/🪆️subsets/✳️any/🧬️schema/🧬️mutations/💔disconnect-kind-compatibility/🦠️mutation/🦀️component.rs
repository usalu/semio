//! 💔 Puzzle3d mutation — `DisconnectKindCompatibility`: revokes one vortex-kind-id pair's
//! attraction allowance.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
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
pub fn disconnect_kind_compatibility(source: String, target: String) -> Puzzle3dMutation {
    Puzzle3dMutation::DisconnectKindCompatibility(DisconnectKindCompatibility { source, target })
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for DisconnectKindCompatibility {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "kind-compatibility", kind: "disconnect-kind-compatibility", record: "DisconnectedKindCompatibility" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Disconnect kind compatibility \"{}\" -> \"{}\"", self.source, self.target)
    }
    fn target(&self) -> Vec<String> {
        vec![self.source.clone(), self.target.clone()]
    }
}
//#endregion 🔖️Mutation

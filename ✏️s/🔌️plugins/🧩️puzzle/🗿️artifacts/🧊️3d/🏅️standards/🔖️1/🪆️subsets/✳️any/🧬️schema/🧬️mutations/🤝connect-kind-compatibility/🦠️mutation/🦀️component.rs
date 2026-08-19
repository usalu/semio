//! 🤝 Puzzle3d mutation — `ConnectKindCompatibility`: allows one vortex-kind-id pair to attract.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::{Puzzle3dCompatSpecificity, Puzzle3dSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🤝 `connect-kind-compatibility` payload. A duplicate `(source, target)` pair is a no-op.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "connect-kind-compatibility")]
pub struct ConnectKindCompatibility {
    pub source: String,
    pub target: String,
    pub bidirectional: bool,
    pub important: bool,
    pub specificity: Puzzle3dCompatSpecificity,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn connect_kind_compatibility(source: String, target: String, bidirectional: bool, important: bool, specificity: Puzzle3dCompatSpecificity) -> Puzzle3dMutation {
    Puzzle3dMutation::ConnectKindCompatibility(ConnectKindCompatibility { source, target, bidirectional, important, specificity })
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for ConnectKindCompatibility {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "kind-compatibility", kind: "connect-kind-compatibility", record: "ConnectedKindCompatibility" };

    async fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Connect kind compatibility \"{}\" -> \"{}\"", self.source, self.target)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.source.clone(), self.target.clone()]
    }
}
//#endregion 🔖️Mutation

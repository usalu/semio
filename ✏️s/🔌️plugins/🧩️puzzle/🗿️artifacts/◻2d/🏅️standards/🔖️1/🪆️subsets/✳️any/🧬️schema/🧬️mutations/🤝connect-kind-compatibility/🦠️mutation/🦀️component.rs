//! 🤝 Puzzle2d mutation — `ConnectKindCompatibility`: allows one kind-id pair to link.
use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::{Puzzle2dCompatSpecificity, Puzzle2dSnapshot};
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
    pub specificity: Puzzle2dCompatSpecificity,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn connect_kind_compatibility(source: String, target: String, bidirectional: bool, important: bool, specificity: Puzzle2dCompatSpecificity) -> Puzzle2dMutation {
    Puzzle2dMutation::ConnectKindCompatibility(ConnectKindCompatibility { source, target, bidirectional, important, specificity })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for ConnectKindCompatibility {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "kind-compatibility", kind: "connect-kind-compatibility", record: "ConnectedKindCompatibility" };

    async fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
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

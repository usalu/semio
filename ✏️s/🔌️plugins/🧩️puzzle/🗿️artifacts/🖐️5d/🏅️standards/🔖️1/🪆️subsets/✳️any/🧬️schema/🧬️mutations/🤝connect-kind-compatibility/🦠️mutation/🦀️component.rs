//! 🤝 Puzzle5d mutation — `ConnectKindCompatibility`: allows one grip-kind-id pair to fasten.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::{Puzzle5dCompatSpecificity, Puzzle5dSnapshot};
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
    pub specificity: Puzzle5dCompatSpecificity,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn connect_kind_compatibility(source: String, target: String, bidirectional: bool, important: bool, specificity: Puzzle5dCompatSpecificity) -> Puzzle5dMutation {
    Puzzle5dMutation::ConnectKindCompatibility(ConnectKindCompatibility { source, target, bidirectional, important, specificity })
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ConnectKindCompatibility {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "kind-compatibility", kind: "connect-kind-compatibility", record: "ConnectedKindCompatibility" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Connect kind compatibility \"{}\" -> \"{}\"", self.source, self.target)
    }
    fn target(&self) -> Vec<String> {
        vec![self.source.clone(), self.target.clone()]
    }
}
//#endregion 🔖️Mutation

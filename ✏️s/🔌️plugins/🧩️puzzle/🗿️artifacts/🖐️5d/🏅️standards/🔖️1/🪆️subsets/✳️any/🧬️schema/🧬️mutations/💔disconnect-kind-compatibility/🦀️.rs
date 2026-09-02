//! 💔 Puzzle5d mutation — `DisconnectKindCompatibility`: revokes one grip-kind-id pair's fasten
//! allowance.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// 💔 `disconnect-kind-compatibility` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "disconnect-kind-compatibility")]
pub struct DisconnectKindCompatibility {
    pub source: String,
    pub target: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn disconnect_kind_compatibility(source: String, target: String) -> Puzzle5dMutation {
    Puzzle5dMutation::DisconnectKindCompatibility(DisconnectKindCompatibility { source, target })
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for DisconnectKindCompatibility {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "kind-compatibility", kind: "disconnect-kind-compatibility", record: "DisconnectedKindCompatibility" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
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

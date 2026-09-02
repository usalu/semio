//! Puzzle5d mutation — `ChangeFastenerKind`: changes a fastener's `fastener_kind` catalog reference.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// `change-fastener-kind` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-fastener-kind")]
pub struct ChangeFastenerKind {
    pub id: String,
    pub new_fastener_kind: Option<String>,
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ChangeFastenerKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fastener", kind: "change-fastener-kind", record: "ChangedFastenerKind" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change fastener \"{}\" kind", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_fastener_kind(id: String, new_fastener_kind: Option<String>) -> Puzzle5dMutation {
    Puzzle5dMutation::ChangeFastenerKind(ChangeFastenerKind { id, new_fastener_kind })
}

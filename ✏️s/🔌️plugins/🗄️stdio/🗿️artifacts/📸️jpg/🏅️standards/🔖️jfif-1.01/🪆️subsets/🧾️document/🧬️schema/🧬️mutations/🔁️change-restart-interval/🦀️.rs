//! 🧬️ Authoritative change-restart-interval mutation.
use crate::artifacts::jpg::schema::diff::{self, *};
use crate::artifacts::jpg::schema::mutations::JpgMutation;
use crate::artifacts::jpg::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangeRestartIntervalMutation {
    pub restart_interval: Option<u16>,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<JpgSnapshot, JpgMutation> for ChangeRestartIntervalMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "restart-interval", kind: "change-restart-interval", record: "ChangeRestartInterval" };
    fn diff(&self, base: &JpgSnapshot) -> protocol::MutationOutcome<JpgDiff> {
        let Self { restart_interval } = self;
        protocol::MutationOutcome::new(contribute(base, *restart_interval))
    }
    fn inverse(&self, base: &JpgSnapshot) -> Vec<JpgMutation> {
        let Self { restart_interval } = self;
        let outcome = <Self as protocol::MutationKind<JpgSnapshot, JpgMutation>>::diff(self, base);
        if <JpgDiff as protocol::DiffAlgebra<JpgSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        vec![JpgMutation::ChangeRestartInterval(crate::artifacts::jpg::schema::mutations::ChangeRestartIntervalMutation { restart_interval: base.restart_interval })]
    }
    fn label(&self) -> String {
        "change restart interval".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["change-restart-interval".into()]
    }
}
pub fn contribute(base: &JpgSnapshot, restart_interval: Option<u16>) -> JpgDiff {
    JpgDiff { restart_interval: (base.restart_interval != restart_interval).then_some(restart_interval), ..Default::default() }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> JpgMutation {
    serde_json::from_str(include_str!("🧪️tests/🎯️direct-behavior/🦠️mutation/🔣️.json")).expect("committed change-restart-interval payload")
}
#[cfg(test)]
#[path = "🧪️tests/🎯️direct-behavior/🦀️.rs"]
mod tests_direct_behavior;

//! ⚡️ Sourcing curate artifact — the operation type + laws (constitutional: op).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::curate::diff::{diff_set_snapshot, CurateDiff};
use crate::artifacts::curate::CurateSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Operations
/// 🛒️ Curate document operation — currently a wholesale snapshot swap from app commands.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SourcingMutation {
    SetSnapshot {
        #[dsl(block)]
        snapshot: CurateSnapshot,
    },
}

impl Mutation<CurateSnapshot> for SourcingMutation {
    type Diff = CurateDiff;

    fn diff(&self, _snapshot: &CurateSnapshot) -> Self::Diff {
        match self {
            SourcingMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &CurateSnapshot) -> Vec<Self> {
        match self {
            SourcingMutation::SetSnapshot { .. } => vec![SourcingMutation::SetSnapshot { snapshot: snapshot.clone() }],
        }
    }
}
//#endregion 🔖️Operations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;
    use store::os_store::test_support;

    fn sample_snapshot() -> CurateSnapshot {
        CurateSnapshot {
            stock: crate::artifacts::curate::engine::sourcing_modules().iter().flat_map(|module| module.demo_kinds()).collect(),
            ..Default::default()
        }
    }

    #[test]
    fn set_snapshot_op_text_round_trips() {
        test_support::assert_op_text_binary_equivalence(&SourcingMutation::SetSnapshot { snapshot: sample_snapshot() });
        test_support::assert_op_text_binary_equivalence(&SourcingMutation::SetSnapshot { snapshot: CurateSnapshot::default() });
    }

    #[test]
    fn set_snapshot_backwards_restores_the_base_snapshot() {
        let base = sample_snapshot();
        let operation = SourcingMutation::SetSnapshot { snapshot: CurateSnapshot::default() };
        let forward = operation.diff(&base).apply(&base);
        assert_eq!(forward, CurateSnapshot::default());
        let restored = operation.inverse(&base).iter().fold(forward, |snapshot, inverse| inverse.diff(&snapshot).apply(&snapshot));
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests

pub fn apply_sourcing_mutation(snapshot: &mut CurateSnapshot, mutation: &SourcingMutation) {
    *snapshot = vcs::apply_mutation(snapshot, mutation);
}

pub fn inverse_sourcing_mutation(snapshot: &CurateSnapshot, mutation: &SourcingMutation) -> Vec<SourcingMutation> {
    mutation.inverse(snapshot)
}

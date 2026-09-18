//! 🎚️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-ext-g-state`.

use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{
    diff::{self, PdfDiff},
    snapshot::*,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveExtGState {
    pub id: String,
}

impl MutationKind<PdfSnapshot, PdfMutation> for RemoveExtGState {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "ext-g-state", kind: "remove-ext-g-state", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_remove_ext_g_state(base, &self.id))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        base.ext_g_states.iter().find(|item| item.id == self.id).map(|item| PdfMutation::SetExtGState(super::set_ext_g_state::SetExtGState { state: item.clone() })).into_iter().collect()
    }

    fn label(&self) -> String {
        format!("Remove ext-g-state {}", self.id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;

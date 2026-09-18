//! 🎛️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-ext-g-state`.

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
pub struct SetExtGState {
    pub state: PdfExtGState,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetExtGState {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "ext-g-state", kind: "set-ext-g-state", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_ext_g_state(base, self.state.clone()))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        match base.ext_g_states.iter().find(|item| item.id == self.state.id) { Some(previous) => vec![PdfMutation::SetExtGState(SetExtGState { state: previous.clone() })], None => vec![PdfMutation::RemoveExtGState(super::remove_ext_g_state::RemoveExtGState { id: self.state.id.clone() })] }
    }

    fn label(&self) -> String {
        format!("Set ext-g-state {}", self.state.id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.state.id.clone()]
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

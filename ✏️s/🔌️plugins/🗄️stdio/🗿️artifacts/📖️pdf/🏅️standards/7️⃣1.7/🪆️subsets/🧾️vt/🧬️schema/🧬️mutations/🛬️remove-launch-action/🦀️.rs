//! 🛬️ Authoritative PDF/VT mutation for removing a matching launch action.

use super::insert_launch_action::InsertLaunchAction;
use super::PdfVtMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveLaunchAction {
    pub target: String,
}

impl MutationKind<PdfSnapshot, PdfVtMutation> for RemoveLaunchAction {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "launch-action", kind: "remove-launch-action", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        if let Some(id) = support::action_with(&next, "Launch", "F", &self.target) {
            support::remove_object(&mut next, id);
        }
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfVtMutation> {
        support::action_with(base, "Launch", "F", &self.target).map(|_| PdfVtMutation::InsertLaunchAction(InsertLaunchAction { target: self.target.clone() })).into_iter().collect()
    }

    fn label(&self) -> String {
        "Remove launch action".to_string()
    }

    fn target(&self) -> Vec<String> {
        vec![self.target.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion 🔖️Facets

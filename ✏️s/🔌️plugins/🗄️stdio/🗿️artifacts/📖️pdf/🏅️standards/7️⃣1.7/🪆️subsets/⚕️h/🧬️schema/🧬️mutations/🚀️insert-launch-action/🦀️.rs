//! 🚀️ Authoritative PDF/H mutation for inserting a launch action.

use super::remove_launch_action::RemoveLaunchAction;
use super::PdfHMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertLaunchAction {
    pub target: String,
}

impl MutationKind<PdfSnapshot, PdfHMutation> for InsertLaunchAction {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "launch-action", kind: "insert-launch-action", record: "Insert" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        support::insert_object(&mut next, support::action_object("Launch", "F", &self.target));
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, _base: &PdfSnapshot) -> Vec<PdfHMutation> {
        vec![PdfHMutation::RemoveLaunchAction(RemoveLaunchAction { target: self.target.clone() })]
    }

    fn label(&self) -> String {
        "Insert launch action".to_string()
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

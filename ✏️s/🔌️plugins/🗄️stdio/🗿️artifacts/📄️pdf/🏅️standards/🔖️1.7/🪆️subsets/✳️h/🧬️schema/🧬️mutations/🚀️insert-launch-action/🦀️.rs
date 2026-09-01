//! 🚀️ Authoritative PDF/H mutation for inserting a launch action.

use super::remove_launch_action::RemoveLaunchAction;
use super::PdfHMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
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
mod tests {
    use super::*;
    use protocol::MutationDiff;

    #[test]
    fn inserts_the_launch_target() {
        let base = PdfSnapshot::default();
        let mutation = InsertLaunchAction { target: "render.bat".to_string() };
        let outcome = <InsertLaunchAction as MutationKind<PdfSnapshot, PdfHMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert!(support::action_with(&next, "Launch", "F", &mutation.target).is_some());
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Facets

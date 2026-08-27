//! 🛬️ Authoritative PDF/X mutation for removing a matching launch action.

use super::insert_launch_action::InsertLaunchAction;
use super::PdfXMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveLaunchAction {
    pub target: String,
}

impl MutationKind<PdfSnapshot, PdfXMutation> for RemoveLaunchAction {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "launch-action", kind: "remove-launch-action", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        if let Some(id) = support::action_with(&next, "Launch", "F", &self.target) {
            support::remove_object(&mut next, id);
        }
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfXMutation> {
        support::action_with(base, "Launch", "F", &self.target)
            .map(|_| PdfXMutation::InsertLaunchAction(InsertLaunchAction { target: self.target.clone() }))
            .into_iter()
            .collect()
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
mod tests {
    use super::*;
    use protocol::MutationDiff;

    #[test]
    fn removes_the_matching_launch_target() {
        let mut base = PdfSnapshot::default();
        support::insert_object(&mut base, support::action_object("Launch", "F", "render.bat"));
        let mutation = RemoveLaunchAction { target: "render.bat".to_string() };
        let outcome = <RemoveLaunchAction as MutationKind<PdfSnapshot, PdfXMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert!(support::action_with(&next, "Launch", "F", "render.bat").is_none());
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Facets

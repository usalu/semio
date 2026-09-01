//! 🚫️ Authoritative PDF/A mutation for removing a matching JavaScript action.

use super::insert_javascript_action::InsertJavascriptAction;
use super::PdfAMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct RemoveJavascriptAction {
    pub script: String,
}

impl MutationKind<PdfSnapshot, PdfAMutation> for RemoveJavascriptAction {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "javascript-action", kind: "remove-javascript-action", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        if let Some(id) = support::action_with(&next, "JavaScript", "JS", &self.script) {
            support::remove_object(&mut next, id);
        }
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfAMutation> {
        support::action_with(base, "JavaScript", "JS", &self.script)
            .map(|_| PdfAMutation::InsertJavascriptAction(InsertJavascriptAction { script: self.script.clone() }))
            .into_iter()
            .collect()
    }

    fn label(&self) -> String {
        "Remove JavaScript action".to_string()
    }

    fn target(&self) -> Vec<String> {
        vec![self.script.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;

    #[test]
    fn removes_the_matching_script_action() {
        let mut base = PdfSnapshot::default();
        support::insert_object(&mut base, support::action_object("JavaScript", "JS", "audit"));
        let mutation = RemoveJavascriptAction { script: "audit".to_string() };
        let outcome = <RemoveJavascriptAction as MutationKind<PdfSnapshot, PdfAMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert!(support::action_with(&next, "JavaScript", "JS", "audit").is_none());
    }
}
//#endregion 🧪️Tests

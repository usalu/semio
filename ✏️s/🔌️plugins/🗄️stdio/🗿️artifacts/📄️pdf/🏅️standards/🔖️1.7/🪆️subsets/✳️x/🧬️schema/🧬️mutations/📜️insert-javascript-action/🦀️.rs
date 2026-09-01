//! 📜️ Authoritative PDF/X mutation for inserting a JavaScript action.

use super::remove_javascript_action::RemoveJavascriptAction;
use super::PdfXMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct InsertJavascriptAction {
    pub script: String,
}

impl MutationKind<PdfSnapshot, PdfXMutation> for InsertJavascriptAction {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "javascript-action", kind: "insert-javascript-action", record: "Insert" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        support::insert_object(&mut next, support::action_object("JavaScript", "JS", &self.script));
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, _base: &PdfSnapshot) -> Vec<PdfXMutation> {
        vec![PdfXMutation::RemoveJavascriptAction(RemoveJavascriptAction { script: self.script.clone() })]
    }

    fn label(&self) -> String {
        "Insert JavaScript action".to_string()
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
    fn inserts_the_script_action() {
        let base = PdfSnapshot::default();
        let mutation = InsertJavascriptAction { script: "app.alert('audit');".to_string() };
        let outcome = <InsertJavascriptAction as MutationKind<PdfSnapshot, PdfXMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert!(support::action_with(&next, "JavaScript", "JS", &mutation.script).is_some());
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Facets

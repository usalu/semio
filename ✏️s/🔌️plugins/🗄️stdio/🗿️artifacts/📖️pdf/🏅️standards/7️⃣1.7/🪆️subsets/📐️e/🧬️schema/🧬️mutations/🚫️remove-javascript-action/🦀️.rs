//! 🚫️ Authoritative PDF/E mutation for removing a matching JavaScript action.

use super::insert_javascript_action::InsertJavascriptAction;
use super::PdfEMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveJavascriptAction {
    pub script: String,
}

impl MutationKind<PdfSnapshot, PdfEMutation> for RemoveJavascriptAction {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "javascript-action", kind: "remove-javascript-action", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        if let Some(id) = support::action_with(&next, "JavaScript", "JS", &self.script) {
            support::remove_object(&mut next, id);
        }
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfEMutation> {
        support::action_with(base, "JavaScript", "JS", &self.script)
            .map(|_| PdfEMutation::InsertJavascriptAction(InsertJavascriptAction { script: self.script.clone() }))
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion 🔖️Facets

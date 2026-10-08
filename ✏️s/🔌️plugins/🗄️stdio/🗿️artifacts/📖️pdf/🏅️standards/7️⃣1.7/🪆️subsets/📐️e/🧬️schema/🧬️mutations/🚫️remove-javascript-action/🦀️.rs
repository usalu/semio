//! 🚫️ Authoritative PDF/E mutation for removing a matching JavaScript action.

use super::insert_javascript_action::InsertJavascriptAction;
use super::PdfEMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::{PdfSnapshot}};
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
        let rows = support::action_with(base, "JavaScript", "JS", &self.script).map_or_else(PdfDiff::default, |id| support::remove_object_rows(base, id));
        MutationOutcome::new(diff::graph_edit(rows))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfEMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        support::action_with(base, "JavaScript", "JS", &self.script).map(|_| PdfEMutation::InsertJavascriptAction(InsertJavascriptAction { script: self.script.clone() })).into_iter().collect()
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove JavaScript action", "JavaScript-Aktion entfernen")
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
//#endregion 🔖️Facets

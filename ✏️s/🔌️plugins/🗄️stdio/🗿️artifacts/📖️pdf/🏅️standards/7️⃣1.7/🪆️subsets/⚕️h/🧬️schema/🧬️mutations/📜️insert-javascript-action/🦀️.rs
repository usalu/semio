//! 📜️ Authoritative PDF/H mutation for inserting a JavaScript action.

use super::remove_javascript_action::RemoveJavascriptAction;
use super::PdfHMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertJavascriptAction {
    pub script: String,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub placements: Vec<support::ObjectPlacement>,
}

impl MutationKind<PdfSnapshot, PdfHMutation> for InsertJavascriptAction {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "javascript-action", kind: "insert-javascript-action", record: "Insert" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let (_, rows) = support::insert_object_rows(base, support::action_object("JavaScript", "JS", &self.script), &self.placements);
        MutationOutcome::new(diff::graph_edit(rows))
    }

    fn inverse(&self, _base: &PdfSnapshot) -> Result<Vec<PdfHMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![PdfHMutation::RemoveJavascriptAction(RemoveJavascriptAction { script: self.script.clone() })]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert JavaScript action", "JavaScript-Aktion einfügen")
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

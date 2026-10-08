//! 🧽️ Authoritative PDF/VT mutation for removing the catalog output intent.

use super::set_output_intent::SetOutputIntent;
use super::PdfVtMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::{PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveOutputIntent {}

impl MutationKind<PdfSnapshot, PdfVtMutation> for RemoveOutputIntent {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "output-intent", kind: "remove-output-intent", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::remove_catalog_entry_owned_rows(base, "OutputIntents")))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfVtMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        support::output_intent_identifier(base).map(|identifier| PdfVtMutation::SetOutputIntent(SetOutputIntent { identifier })).into_iter().collect()
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove PDF/VT output intent", "PDF/VT-Ausgabebedingung entfernen")
    }

    fn target(&self) -> Vec<String> {
        vec!["OutputIntents".to_string()]
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

//! 🚫️ Authoritative PDF/UA mutation for remove display doc title.

use super::set_display_doc_title::SetDisplayDocTitle;
use super::PdfUaMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveDisplayDocTitle {}

impl MutationKind<PdfSnapshot, PdfUaMutation> for RemoveDisplayDocTitle {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "display-doc-title", kind: "remove-display-doc-title", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::remove_catalog_entry_rows(base, "ViewerPreferences")))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfUaMutation>, semio_framework_value::ValueError> {
        Ok({
            support::catalog_flag(base, "ViewerPreferences", "DisplayDocTitle").map(|display| PdfUaMutation::SetDisplayDocTitle(SetDisplayDocTitle { display, entry_index: support::catalog_entry_position(base, "ViewerPreferences") })).into_iter().collect()
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove PDF/UA title display preference", "PDF/UA-Anzeigeeinstellung für den Titel entfernen")
    }

    fn target(&self) -> Vec<String> {
        vec!["ViewerPreferences".to_string()]
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

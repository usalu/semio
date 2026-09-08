//! 🚫️ Authoritative PDF/UA mutation for remove display doc title.

use super::set_display_doc_title::SetDisplayDocTitle;
use super::PdfUaMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
#[cfg(test)]
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfObject;
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveDisplayDocTitle {}

impl MutationKind<PdfSnapshot, PdfUaMutation> for RemoveDisplayDocTitle {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "display-doc-title", kind: "remove-display-doc-title", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        support::remove_catalog_entry(&mut next, "ViewerPreferences");
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfUaMutation> {
        support::catalog_flag(base, "ViewerPreferences", "DisplayDocTitle")
            .map(|display| PdfUaMutation::SetDisplayDocTitle(SetDisplayDocTitle { display }))
            .into_iter()
            .collect()
    }

    fn label(&self) -> String {
        "Remove PDF/UA title display preference".to_string()
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
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion 🔖️Facets

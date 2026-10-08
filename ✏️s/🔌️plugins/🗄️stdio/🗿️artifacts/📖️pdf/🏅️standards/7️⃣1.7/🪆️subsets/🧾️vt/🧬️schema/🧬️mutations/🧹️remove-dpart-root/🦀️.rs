//! 🧹️ Authoritative PDF/VT mutation for remove dpart root.

use super::set_dpart_root::SetDpartRoot;
use super::PdfVtMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::{PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveDpartRoot {}

impl MutationKind<PdfSnapshot, PdfVtMutation> for RemoveDpartRoot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "dpart-root", kind: "remove-dpart-root", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::remove_catalog_entry_owned_rows(base, "DPartRoot")))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfVtMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        support::catalog_entry(base, "DPartRoot").map(|_| PdfVtMutation::SetDpartRoot(SetDpartRoot { job: support::dpart_job(base).unwrap_or_default() })).into_iter().collect()
    })())
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove PDF/VT document partition", "PDF/VT-Dokumentpartition entfernen")
    }

    fn target(&self) -> Vec<String> {
        vec!["DPartRoot".to_string()]
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

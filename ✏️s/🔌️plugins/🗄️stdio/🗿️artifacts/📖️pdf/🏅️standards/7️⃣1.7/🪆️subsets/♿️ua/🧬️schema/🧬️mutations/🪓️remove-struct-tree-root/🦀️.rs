//! 🪓️ Authoritative PDF/UA mutation for remove struct tree root.

use super::set_struct_tree_root::SetStructTreeRoot;
use super::PdfUaMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveStructTreeRoot {}

impl MutationKind<PdfSnapshot, PdfUaMutation> for RemoveStructTreeRoot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "struct-tree-root", kind: "remove-struct-tree-root", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::remove_catalog_entry_owned_rows(base, "StructTreeRoot")))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfUaMutation>, semio_framework_value::ValueError> {
        Ok({
            support::catalog_entry(base, "StructTreeRoot").map(|entry| PdfUaMutation::SetStructTreeRoot(SetStructTreeRoot { placements: entry.as_ref().map(|root| support::placements_of(base, &[root])).unwrap_or_default(), entry_index: support::catalog_entry_position(base, "StructTreeRoot") })).into_iter().collect()
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove PDF/UA structure tree root", "PDF/UA-Strukturbaumwurzel entfernen")
    }

    fn target(&self) -> Vec<String> {
        vec!["StructTreeRoot".to_string()]
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

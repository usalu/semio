//! 🌲️ Authoritative PDF/UA mutation for set struct tree root.

use super::remove_struct_tree_root::RemoveStructTreeRoot;
use super::PdfUaMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetStructTreeRoot {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub placements: Vec<support::ObjectPlacement>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub entry_index: Option<usize>,
}

impl MutationKind<PdfSnapshot, PdfUaMutation> for SetStructTreeRoot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "struct-tree-root", kind: "set-struct-tree-root", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::struct_tree_root_rows(base, &self.placements, self.entry_index)))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfUaMutation>, semio_framework_value::ValueError> {
        Ok({
            match support::catalog_entry(base, "StructTreeRoot") {
                Some(entry) => vec![PdfUaMutation::SetStructTreeRoot(SetStructTreeRoot { placements: entry.as_ref().map(|root| support::placements_of(base, &[root])).unwrap_or_default(), entry_index: None })],
                None => vec![PdfUaMutation::RemoveStructTreeRoot(RemoveStructTreeRoot {})],
            }
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set PDF/UA structure tree root", "PDF/UA-Strukturbaumwurzel setzen")
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

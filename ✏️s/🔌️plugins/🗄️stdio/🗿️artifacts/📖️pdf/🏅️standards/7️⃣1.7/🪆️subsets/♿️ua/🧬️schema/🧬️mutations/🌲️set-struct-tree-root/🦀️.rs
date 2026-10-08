//! 🌲️ Authoritative PDF/UA mutation for set struct tree root.

use super::remove_struct_tree_root::RemoveStructTreeRoot;
use super::PdfUaMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::{PdfObject, PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetStructTreeRoot {}

impl MutationKind<PdfSnapshot, PdfUaMutation> for SetStructTreeRoot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "struct-tree-root", kind: "set-struct-tree-root", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let (root, rows) = support::insert_object_rows(base, support::struct_tree_root_object());
        MutationOutcome::new(diff::graph_edit(diff::sequence(rows, support::set_catalog_entry_rows(base, "StructTreeRoot", PdfObject::Ref(root)))))
    }

    fn inverse(&self, _base: &PdfSnapshot) -> Result<Vec<PdfUaMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![PdfUaMutation::RemoveStructTreeRoot(RemoveStructTreeRoot {})]
    
    })())
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

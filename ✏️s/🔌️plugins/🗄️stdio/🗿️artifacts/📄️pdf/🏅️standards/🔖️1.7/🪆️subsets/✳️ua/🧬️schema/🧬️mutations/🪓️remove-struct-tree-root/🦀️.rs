//! 🪓️ Authoritative PDF/UA mutation for remove struct tree root.

use super::set_struct_tree_root::SetStructTreeRoot;
use super::PdfUaMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::{PdfObject, PdfSnapshot}};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveStructTreeRoot {}

impl MutationKind<PdfSnapshot, PdfUaMutation> for RemoveStructTreeRoot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "struct-tree-root", kind: "remove-struct-tree-root", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        support::remove_catalog_entry(&mut next, "StructTreeRoot");
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, _base: &PdfSnapshot) -> Vec<PdfUaMutation> {
        vec![PdfUaMutation::SetStructTreeRoot(SetStructTreeRoot {})]
    }

    fn label(&self) -> String {
        "Remove PDF/UA structure tree root".to_string()
    }

    fn target(&self) -> Vec<String> {
        vec!["StructTreeRoot".to_string()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;

    #[test]
    fn changes_the_owned_catalog_axis_and_plans_its_inverse() {
        let mut base = PdfSnapshot::default();
        support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("Catalog".to_string()))]));
        let id = support::insert_object(&mut base, support::struct_tree_root_object());
        support::set_catalog_entry(&mut base, "StructTreeRoot", PdfObject::Ref(id));
        let mutation = RemoveStructTreeRoot {};
        let outcome = <RemoveStructTreeRoot as MutationKind<PdfSnapshot, PdfUaMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert!(support::catalog_entry(&next, "StructTreeRoot").is_none());
        assert_eq!(<RemoveStructTreeRoot as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base), vec![PdfUaMutation::SetStructTreeRoot(SetStructTreeRoot {})]);
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Facets

//! 🌲️ Authoritative PDF/UA mutation for set struct tree root.

use super::remove_struct_tree_root::RemoveStructTreeRoot;
use super::PdfUaMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::{PdfObject, PdfSnapshot}};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetStructTreeRoot {}

impl MutationKind<PdfSnapshot, PdfUaMutation> for SetStructTreeRoot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "struct-tree-root", kind: "set-struct-tree-root", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        let id = support::insert_object(&mut next, support::struct_tree_root_object());
        support::set_catalog_entry(&mut next, "StructTreeRoot", PdfObject::Ref(id));
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, _base: &PdfSnapshot) -> Vec<PdfUaMutation> {
        vec![PdfUaMutation::RemoveStructTreeRoot(RemoveStructTreeRoot {})]
    }

    fn label(&self) -> String {
        "Set PDF/UA structure tree root".to_string()
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
        let mutation = SetStructTreeRoot {};
        let outcome = <SetStructTreeRoot as MutationKind<PdfSnapshot, PdfUaMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        let Some(PdfObject::Ref(id)) = support::catalog_entry(&next, "StructTreeRoot") else { panic!("structure tree root reference") };
        assert_eq!(support::object(&next, *id), Some(&support::struct_tree_root_object()));
        assert_eq!(<SetStructTreeRoot as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base), vec![PdfUaMutation::RemoveStructTreeRoot(RemoveStructTreeRoot {})]);
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Facets

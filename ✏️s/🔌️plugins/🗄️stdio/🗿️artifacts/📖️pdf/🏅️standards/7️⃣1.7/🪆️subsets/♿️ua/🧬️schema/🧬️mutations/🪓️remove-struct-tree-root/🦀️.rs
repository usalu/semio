//! 🪓️ Authoritative PDF/UA mutation for remove struct tree root.

use crate::standards::v1_7::subsets::ua::schema::mutations::set_struct_tree_root::SetStructTreeRoot;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveStructTreeRoot {}

impl<Op> MutationKind<PdfSnapshot, Op> for RemoveStructTreeRoot
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemoveStructTreeRoot> + From<crate::standards::v1_7::subsets::ua::schema::mutations::set_struct_tree_root::SetStructTreeRoot>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "struct-tree-root", kind: "remove-struct-tree-root", record: "Remove" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove PDF/UA structure tree root", "PDF/UA-Strukturbaumwurzel entfernen")
    }

    fn target(&self) -> Vec<String> {
        vec!["StructTreeRoot".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for RemoveStructTreeRoot
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<RemoveStructTreeRoot> + From<crate::standards::v1_7::subsets::ua::schema::mutations::set_struct_tree_root::SetStructTreeRoot>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::remove_catalog_entry_owned_rows(base, "StructTreeRoot")))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok({
            support::catalog_entry(base, "StructTreeRoot").map(|entry| Op::from(SetStructTreeRoot { placements: entry.as_ref().map(|root| support::placements_of(base, &[root])).unwrap_or_default(), entry_index: support::catalog_entry_position(base, "StructTreeRoot") })).into_iter().collect()
        })
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets

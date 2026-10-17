//! 🌲️ Authoritative PDF/UA mutation for set struct tree root.

use crate::standards::v1_7::subsets::ua::schema::mutations::remove_struct_tree_root::RemoveStructTreeRoot;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetStructTreeRoot {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub placements: Vec<support::ObjectPlacement>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub entry_index: Option<usize>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetStructTreeRoot
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetStructTreeRoot> + From<crate::standards::v1_7::subsets::ua::schema::mutations::remove_struct_tree_root::RemoveStructTreeRoot>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "struct-tree-root", kind: "set-struct-tree-root", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::struct_tree_root_rows(base, &self.placements, self.entry_index)))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok({
            match support::catalog_entry(base, "StructTreeRoot") {
                Some(entry) => vec![Op::from(SetStructTreeRoot { placements: entry.as_ref().map(|root| support::placements_of(base, &[root])).unwrap_or_default(), entry_index: None })],
                None => vec![Op::from(RemoveStructTreeRoot {})],
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

//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets

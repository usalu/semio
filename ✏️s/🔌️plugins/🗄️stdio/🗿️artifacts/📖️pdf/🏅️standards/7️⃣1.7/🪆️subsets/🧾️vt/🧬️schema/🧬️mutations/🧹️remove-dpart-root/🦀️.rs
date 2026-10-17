//! 🧹️ Authoritative PDF/VT mutation for remove dpart root.

use crate::standards::v1_7::subsets::vt::schema::mutations::set_dpart_root::SetDpartRoot;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveDpartRoot {}

impl<Op> MutationKind<PdfSnapshot, Op> for RemoveDpartRoot
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemoveDpartRoot> + From<crate::standards::v1_7::subsets::vt::schema::mutations::set_dpart_root::SetDpartRoot>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "dpart-root", kind: "remove-dpart-root", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::remove_catalog_entry_owned_rows(base, "DPartRoot")))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok({
            support::catalog_entry(base, "DPartRoot").map(|_| Op::from(SetDpartRoot { job: support::dpart_job(base).unwrap_or_default(), placements: support::placements_of(base, &support::dpart_root_creation_ids(base)), entry_index: support::catalog_entry_position(base, "DPartRoot") })).into_iter().collect()
        })
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

//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets

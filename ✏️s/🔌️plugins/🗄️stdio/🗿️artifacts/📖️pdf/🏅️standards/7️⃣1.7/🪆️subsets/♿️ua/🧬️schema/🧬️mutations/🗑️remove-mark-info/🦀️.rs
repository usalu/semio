//! 🗑️ Authoritative PDF/UA mutation for remove mark info.

use crate::standards::v1_7::subsets::ua::schema::mutations::set_mark_info::SetMarkInfo;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveMarkInfo {}

impl<Op> MutationKind<PdfSnapshot, Op> for RemoveMarkInfo
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemoveMarkInfo> + From<crate::standards::v1_7::subsets::ua::schema::mutations::set_mark_info::SetMarkInfo>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "mark-info", kind: "remove-mark-info", record: "Remove" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove PDF/UA marked flag", "PDF/UA-Markierungskennung entfernen")
    }

    fn target(&self) -> Vec<String> {
        vec!["MarkInfo".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for RemoveMarkInfo
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<RemoveMarkInfo> + From<crate::standards::v1_7::subsets::ua::schema::mutations::set_mark_info::SetMarkInfo>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::remove_catalog_entry_rows(base, "MarkInfo")))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok({
            support::catalog_flag(base, "MarkInfo", "Marked").map(|marked| Op::from(SetMarkInfo { marked, entry_index: support::catalog_entry_position(base, "MarkInfo") })).into_iter().collect()
        })
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets

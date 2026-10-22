//! 🚫️ Authoritative PDF/UA mutation for remove display doc title.

use crate::standards::v1_7::subsets::ua::schema::mutations::set_display_doc_title::SetDisplayDocTitle;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveDisplayDocTitle {}

impl<Op> MutationKind<PdfSnapshot, Op> for RemoveDisplayDocTitle
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemoveDisplayDocTitle> + From<crate::standards::v1_7::subsets::ua::schema::mutations::set_display_doc_title::SetDisplayDocTitle>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "display-doc-title", kind: "remove-display-doc-title", record: "Remove" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove PDF/UA title display preference", "PDF/UA-Anzeigeeinstellung für den Titel entfernen")
    }

    fn target(&self) -> Vec<String> {
        vec!["ViewerPreferences".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for RemoveDisplayDocTitle
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<RemoveDisplayDocTitle> + From<crate::standards::v1_7::subsets::ua::schema::mutations::set_display_doc_title::SetDisplayDocTitle>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::remove_catalog_entry_rows(base, "ViewerPreferences")))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok({
            support::catalog_flag(base, "ViewerPreferences", "DisplayDocTitle").map(|display| Op::from(SetDisplayDocTitle { display, entry_index: support::catalog_entry_position(base, "ViewerPreferences") })).into_iter().collect()
        })
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets

//! 🧽️ Authoritative PDF/VT mutation for remove trim box.

use crate::standards::v1_7::subsets::vt::schema::mutations::set_trim_box::SetTrimBox;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveTrimBox {
    pub page_index: usize,
}

impl<Op> MutationKind<PdfSnapshot, Op> for RemoveTrimBox
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemoveTrimBox> + From<crate::standards::v1_7::subsets::vt::schema::mutations::set_trim_box::SetTrimBox>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "trim-box", kind: "remove-trim-box", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let rows = support::page_objects(base).get(self.page_index).copied().map_or_else(PdfDiff::default, |page| support::remove_entry_rows(base, page, "TrimBox"));
        MutationOutcome::new(diff::graph_edit(rows))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok({
            support::page_objects(base).get(self.page_index).copied().and_then(|page| support::page_box(base, page, "TrimBox").map(|trim_box| Op::from(SetTrimBox { page_index: self.page_index, trim_box, entry_index: support::entry_position(base, page, "TrimBox") }))).into_iter().collect()
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove PDF/VT trim box on page {}", self.page_index), &format!("PDF/VT-TrimBox auf Seite {} entfernen", self.page_index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.page_index.to_string(), "TrimBox".to_string()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets

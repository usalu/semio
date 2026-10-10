//! 🗑️ Authoritative PDF/UA mutation for remove mark info.

use super::set_mark_info::SetMarkInfo;
use super::PdfUaMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveMarkInfo {}

impl MutationKind<PdfSnapshot, PdfUaMutation> for RemoveMarkInfo {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "mark-info", kind: "remove-mark-info", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::remove_catalog_entry_rows(base, "MarkInfo")))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfUaMutation>, semio_framework_value::ValueError> {
        Ok({
            support::catalog_flag(base, "MarkInfo", "Marked").map(|marked| PdfUaMutation::SetMarkInfo(SetMarkInfo { marked, entry_index: support::catalog_entry_position(base, "MarkInfo") })).into_iter().collect()
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove PDF/UA marked flag", "PDF/UA-Markierungskennung entfernen")
    }

    fn target(&self) -> Vec<String> {
        vec!["MarkInfo".to_string()]
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

//! 🗑️ Authoritative PDF/UA mutation for remove mark info.

use super::set_mark_info::SetMarkInfo;
use super::PdfUaMutation;
#[cfg(test)]
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfObject;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveMarkInfo {}

impl MutationKind<PdfSnapshot, PdfUaMutation> for RemoveMarkInfo {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "mark-info", kind: "remove-mark-info", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        support::remove_catalog_entry(&mut next, "MarkInfo");
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfUaMutation> {
        support::catalog_flag(base, "MarkInfo", "Marked").map(|marked| PdfUaMutation::SetMarkInfo(SetMarkInfo { marked })).into_iter().collect()
    }

    fn label(&self) -> String {
        "Remove PDF/UA marked flag".to_string()
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
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion 🔖️Facets

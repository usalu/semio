//! 🧽️ Authoritative PDF/X mutation for remove trim box.

use super::set_trim_box::SetTrimBox;
use super::PdfXMutation;
#[cfg(test)]
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfObject;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveTrimBox {
    pub page_index: usize,
}

impl MutationKind<PdfSnapshot, PdfXMutation> for RemoveTrimBox {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "trim-box", kind: "remove-trim-box", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        if let Some(page) = support::page_objects(&next).get(self.page_index).copied() {
            support::remove_entry(&mut next, page, "TrimBox");
        }
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfXMutation> {
        support::page_objects(base).get(self.page_index).copied().and_then(|page| support::page_box(base, page, "TrimBox")).map(|trim_box| PdfXMutation::SetTrimBox(SetTrimBox { page_index: self.page_index, trim_box })).into_iter().collect()
    }

    fn label(&self) -> String {
        format!("Remove PDF/X trim box on page {}", self.page_index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.page_index.to_string(), "TrimBox".to_string()]
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

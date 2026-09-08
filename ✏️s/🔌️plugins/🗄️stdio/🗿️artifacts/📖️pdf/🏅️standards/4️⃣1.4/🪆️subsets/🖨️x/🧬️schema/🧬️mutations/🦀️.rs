//! 🧬️ Transparent PDF 1.4/X mutation registry and delegation.

use crate::standards::v1_4::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};

//#region 🔖️Leaves
#[path = "📐️set-page-size/🦀️.rs"]
pub mod set_page_size;
pub use set_page_size::SetPageSize;
#[path = "📉️collapse-page-size/🦀️.rs"]
pub mod collapse_page_size;
pub use collapse_page_size::CollapsePageSize;
pub use set_page_size::{CONFORMANT_HEIGHT, CONFORMANT_WIDTH};
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", content = "payload", rename_all = "kebab-case", deny_unknown_fields)]
#[mutations(snapshot = PdfSnapshot, diff = PdfDiff, schema = "s.stdio.pdf.1.4.x")]
pub enum PdfX1Mutation {
    SetPageSize(SetPageSize),
    CollapsePageSize(CollapsePageSize),
}

//#endregion 🔖️Aggregate

//#region 🔖️Delegation
/// ▶️ Applies the authoritative leaf diff.
pub fn apply_x_conformance_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfX1Mutation) -> protocol::MutationOutcome<PdfDiff> {
    use protocol::Mutation;
    mutation.diff(snapshot).apply_to(snapshot)
}

/// ↩️ Returns concrete inverse operations owned by the selected leaf.
pub fn inverse_x_conformance_mutation(mutation: &PdfX1Mutation, base: &PdfSnapshot) -> Vec<PdfX1Mutation> {
    use protocol::Mutation;
    mutation.inverse(base)
}
//#endregion 🔖️Delegation

//#region 🧪️Structure
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Structure

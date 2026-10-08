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
//#endregion 🔖️Delegation

//#region 🧪️Structure
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Structure

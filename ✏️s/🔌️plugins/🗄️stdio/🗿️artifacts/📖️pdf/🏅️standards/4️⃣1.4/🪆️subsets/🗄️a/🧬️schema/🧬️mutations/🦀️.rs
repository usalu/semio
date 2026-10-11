//! 🧬️ Transparent PDF 1.4/A mutation registry and delegation.

use crate::standards::v1_4::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};

//#region 🔖️Leaves
#[path = "📝️set-page-text/🦀️.rs"]
pub mod set_page_text;
pub use set_page_text::SetPageText;
#[path = "🧹️clear-page-text/🦀️.rs"]
pub mod clear_page_text;
pub use clear_page_text::ClearPageText;
pub use set_page_text::CONFORMANT_TEXT;
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", content = "payload", rename_all = "kebab-case", deny_unknown_fields)]
#[mutations(snapshot = PdfSnapshot, diff = PdfDiff, schema = "s.stdio.pdf.1.4.a")]
pub enum PdfA1Mutation {
    SetPageText(SetPageText),
    ClearPageText(ClearPageText),
}

//#endregion 🔖️Aggregate

//#region 🔖️Delegation
//#endregion 🔖️Delegation

//#region 🧪️Structure
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Structure

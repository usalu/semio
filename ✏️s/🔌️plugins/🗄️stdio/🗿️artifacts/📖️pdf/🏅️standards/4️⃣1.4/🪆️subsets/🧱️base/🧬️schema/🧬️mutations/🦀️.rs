//! 🧬️ Transparent PDF 1.4/ANY mutation registry and delegation.

use crate::standards::v1_4::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};

//#region 🔖️Leaves
#[path = "📥️insert-page/🦀️.rs"]
pub mod insert_page;
pub use insert_page::InsertPage;
#[path = "🗑️remove-page/🦀️.rs"]
pub mod remove_page;
pub use remove_page::RemovePage;
#[path = "🔀️move-page/🦀️.rs"]
pub mod move_page;
pub use move_page::MovePage;
#[path = "📐️resize-page/🦀️.rs"]
pub mod resize_page;
pub use resize_page::ResizePage;
#[path = "♻️replace-page-text/🦀️.rs"]
pub mod replace_page_text;
pub use replace_page_text::ReplacePageText;
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", content = "payload", rename_all = "kebab-case", deny_unknown_fields)]
#[mutations(snapshot = PdfSnapshot, diff = PdfDiff, schema = "s.stdio.pdf.1.4")]
pub enum PdfMutation {
    InsertPage(InsertPage),
    RemovePage(RemovePage),
    MovePage(MovePage),
    ResizePage(ResizePage),
    ReplacePageText(ReplacePageText),
}

//#endregion 🔖️Aggregate

//#region 🔖️Delegation
/// ▶️ Applies the authoritative leaf diff.
pub fn apply_pdf_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfMutation) -> protocol::MutationOutcome<PdfDiff> {
    use protocol::Mutation;
    mutation.diff(snapshot).apply_to(snapshot)
}

/// ↩️ Returns concrete inverse operations owned by the selected leaf.
pub fn inverse_pdf_mutation(mutation: &PdfMutation, base: &PdfSnapshot) -> Vec<PdfMutation> {
    use protocol::Mutation;
    mutation.inverse(base)
}
//#endregion 🔖️Delegation

//#region 🔖️Codecs
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion 🔖️Codecs

//#region 🧪️Structure
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Structure

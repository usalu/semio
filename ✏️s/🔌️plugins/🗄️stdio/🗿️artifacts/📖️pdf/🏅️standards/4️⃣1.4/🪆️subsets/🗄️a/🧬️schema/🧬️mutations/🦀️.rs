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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", content = "payload", rename_all = "kebab-case", deny_unknown_fields)]
#[mutations(snapshot = PdfSnapshot, diff = PdfDiff, schema = "s.stdio.pdf.1.4.a")]
pub enum PdfA1Mutation {
    SetPageText(SetPageText),
    ClearPageText(ClearPageText),
}

//#endregion 🔖️Aggregate

//#region 🔖️Delegation
/// ▶️ Applies the authoritative leaf diff.
pub fn apply_a_conformance_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfA1Mutation) -> protocol::MutationOutcome<PdfDiff> {
    use protocol::Mutation;
    mutation.diff(snapshot).apply_to(snapshot)
}

/// ↩️ Returns concrete inverse operations owned by the selected leaf.
pub fn inverse_a_conformance_mutation(mutation: &PdfA1Mutation, base: &PdfSnapshot) -> Vec<PdfA1Mutation> {
    use protocol::Mutation;
    mutation.inverse(base)
}

/// 📨️ Builds the operation of semantic kind `kind` from its editable payload JSON — the leaf wire (`payload_value()`) a
/// `🥒️.feature` row carries — through the derive's generic `from_payload_value`.
pub fn decode_a_conformance_mutation_payload(kind: &str, payload: &str) -> Result<PdfA1Mutation, String> {
    use protocol::Mutation;
    pack::from_json_str(payload).and_then(|value| PdfA1Mutation::from_payload_value(kind, value)).map_err(|error| error.to_string())
}
//#endregion 🔖️Delegation

//#region 🧪️Structure
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Structure

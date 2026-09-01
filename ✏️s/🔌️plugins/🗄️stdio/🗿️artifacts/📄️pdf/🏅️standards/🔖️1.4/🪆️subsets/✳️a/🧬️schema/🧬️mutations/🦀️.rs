//! 🧬️ Transparent PDF 1.4/A mutation registry and delegation.

use crate::artifacts::pdf::standards::v1_4::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};
use serde::{Deserialize, Serialize};

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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", content = "payload", rename_all = "kebab-case", deny_unknown_fields)]
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
//#endregion 🔖️Delegation

//#region 🧪️Structure
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_descriptor_and_catalog_bijection() {
        let kinds: Vec<_> = <PdfA1Mutation as protocol::SemanticMutation<PdfSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
        let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations");
        let catalog: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("../../🧪️oracle/🔣️.json")).unwrap()).unwrap();
        assert_eq!(catalog["mutationCatalogs"][0]["kinds"], serde_json::json!(kinds));
        {
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("📝️set-page-text").join("🔣️component.json")).unwrap()).unwrap();
            assert_eq!(descriptor["semanticKind"], kinds[0]);
            assert!(source.join("📝️set-page-text").join("🦀️.rs").is_file());
        }
        {
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("🧹️clear-page-text").join("🔣️component.json")).unwrap()).unwrap();
            assert_eq!(descriptor["semanticKind"], kinds[1]);
            assert!(source.join("🧹️clear-page-text").join("🦀️.rs").is_file());
        }
    }
}
//#endregion 🧪️Structure

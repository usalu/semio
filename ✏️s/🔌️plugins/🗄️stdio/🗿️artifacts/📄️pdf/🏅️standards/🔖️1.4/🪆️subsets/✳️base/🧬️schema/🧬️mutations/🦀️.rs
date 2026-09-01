//! 🧬️ Transparent PDF 1.4/ANY mutation registry and delegation.

use crate::artifacts::pdf::standards::v1_4::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};
use serde::{Deserialize, Serialize};

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
#[path = "📝️replace-page-text/🦀️.rs"]
pub mod replace_page_text;
pub use replace_page_text::ReplacePageText;
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", content = "payload", rename_all = "kebab-case", deny_unknown_fields)]
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
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Codecs

//#region 🧪️Structure
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_descriptor_and_catalog_bijection() {
        let kinds: Vec<_> = <PdfMutation as protocol::SemanticMutation<PdfSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
        let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/🧬️schema/🧬️mutations");
        let catalog: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("../../🧪️oracle/🔣️.json")).unwrap()).unwrap();
        assert_eq!(catalog["mutationCatalogs"][0]["kinds"], serde_json::json!(kinds));
        {
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("📥️insert-page").join("🔣️component.json")).unwrap()).unwrap();
            assert_eq!(descriptor["semanticKind"], kinds[0]);
            assert!(source.join("📥️insert-page").join("🦀️.rs").is_file());
        }
        {
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("🗑️remove-page").join("🔣️component.json")).unwrap()).unwrap();
            assert_eq!(descriptor["semanticKind"], kinds[1]);
            assert!(source.join("🗑️remove-page").join("🦀️.rs").is_file());
        }
        {
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("🔀️move-page").join("🔣️component.json")).unwrap()).unwrap();
            assert_eq!(descriptor["semanticKind"], kinds[2]);
            assert!(source.join("🔀️move-page").join("🦀️.rs").is_file());
        }
        {
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("📐️resize-page").join("🔣️component.json")).unwrap()).unwrap();
            assert_eq!(descriptor["semanticKind"], kinds[3]);
            assert!(source.join("📐️resize-page").join("🦀️.rs").is_file());
        }
        {
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("📝️replace-page-text").join("🔣️component.json")).unwrap()).unwrap();
            assert_eq!(descriptor["semanticKind"], kinds[4]);
            assert!(source.join("📝️replace-page-text").join("🦀️.rs").is_file());
        }
    }
}
//#endregion 🧪️Structure

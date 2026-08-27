//! 🧬️ Transparent dispatch for the PDF 1.7/A conformance mutation vocabulary. Concrete payloads,
//! graph transforms, inverse plans, and tests live in direct semantic leaves; this root only mounts,
//! re-exports, wraps, delegates, and assembles the generated registry.

use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::PdfDiff;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Leaves
#[path = "🔤️embed-font-file/🦀️component.rs"]
pub mod embed_font_file;
#[path = "📎️insert-embedded-file/🦀️component.rs"]
pub mod insert_embedded_file;
#[path = "🔒️insert-encryption-dictionary/🦀️component.rs"]
pub mod insert_encryption_dictionary;
#[path = "📜️insert-javascript-action/🦀️component.rs"]
pub mod insert_javascript_action;
#[path = "🚀️insert-launch-action/🦀️component.rs"]
pub mod insert_launch_action;
#[path = "✂️remove-af-relationship/🦀️component.rs"]
pub mod remove_af_relationship;
#[path = "🗑️remove-embedded-file/🦀️component.rs"]
pub mod remove_embedded_file;
#[path = "🔓️remove-encryption-dictionary/🦀️component.rs"]
pub mod remove_encryption_dictionary;
#[path = "🧺️remove-font-file/🦀️component.rs"]
pub mod remove_font_file;
#[path = "🚫️remove-javascript-action/🦀️component.rs"]
pub mod remove_javascript_action;
#[path = "🛬️remove-launch-action/🦀️component.rs"]
pub mod remove_launch_action;
#[path = "🧽️remove-output-intent/🦀️component.rs"]
pub mod remove_output_intent;
#[path = "🔗️set-af-relationship/🦀️component.rs"]
pub mod set_af_relationship;
#[path = "🏳️set-output-intent/🦀️component.rs"]
pub mod set_output_intent;

pub use embed_font_file::EmbedFontFile;
pub use insert_embedded_file::InsertEmbeddedFile;
pub use insert_encryption_dictionary::InsertEncryptionDictionary;
pub use insert_javascript_action::InsertJavascriptAction;
pub use insert_launch_action::InsertLaunchAction;
pub use remove_af_relationship::RemoveAfRelationship;
pub use remove_embedded_file::RemoveEmbeddedFile;
pub use remove_encryption_dictionary::RemoveEncryptionDictionary;
pub use remove_font_file::RemoveFontFile;
pub use remove_javascript_action::RemoveJavascriptAction;
pub use remove_launch_action::RemoveLaunchAction;
pub use remove_output_intent::RemoveOutputIntent;
pub use set_af_relationship::SetAfRelationship;
pub use set_output_intent::SetOutputIntent;
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
/// 📐️ Typed PDF/A-2 and PDF/A-3 conformance mutation vocabulary. Every variant directly wraps its
/// authoritative semantic leaf payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = PdfSnapshot, diff = PdfDiff, schema = "s.stdio.pdf.1.7.a")]
pub enum PdfAMutation {
    InsertEncryptionDictionary(InsertEncryptionDictionary),
    RemoveEncryptionDictionary(RemoveEncryptionDictionary),
    InsertJavascriptAction(InsertJavascriptAction),
    RemoveJavascriptAction(RemoveJavascriptAction),
    InsertLaunchAction(InsertLaunchAction),
    RemoveLaunchAction(RemoveLaunchAction),
    InsertEmbeddedFile(InsertEmbeddedFile),
    RemoveEmbeddedFile(RemoveEmbeddedFile),
    SetAfRelationship(SetAfRelationship),
    RemoveAfRelationship(RemoveAfRelationship),
    SetOutputIntent(SetOutputIntent),
    RemoveOutputIntent(RemoveOutputIntent),
    EmbedFontFile(EmbedFontFile),
    RemoveFontFile(RemoveFontFile),
}
//#endregion 🔖️Aggregate

//#region 🔖️Delegation
/// ▶️ Applies one conformance mutation through its leaf-owned diff.
pub fn apply_a_conformance_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfAMutation) -> protocol::MutationOutcome<PdfDiff> {
    use protocol::Mutation;
    let outcome = mutation.diff(snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ Delegates inverse planning to the authoritative leaf.
pub fn inverse_a_conformance_mutation(mutation: &PdfAMutation, base: &PdfSnapshot) -> Vec<PdfAMutation> {
    use protocol::Mutation;
    mutation.inverse(base)
}

/// 🧾️ Returns the derive-owned semantic catalog in declaration order.
pub fn pdf_a_mutation_kinds() -> &'static [protocol::SemanticDescriptor] {
    use protocol::SemanticMutation;
    PdfAMutation::kinds()
}
//#endregion 🔖️Delegation

//#region 🧪️CatalogParity
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derived_catalog_matches_the_language_neutral_oracle_catalog() {
        let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️a/🧬️schema/🧬️mutations");
        let manifest = std::fs::read_to_string(mutation_root.join("../../🧪️oracle/🔣️component.json")).expect("language-neutral oracle catalog");
        let needle = "\"kinds\": [";
        let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
        let end = start + manifest[start..].find(']').expect("kinds array is closed");
        let declared: Vec<String> = manifest[start..end]
            .split(',')
            .map(|entry| entry.trim().trim_matches('"').to_string())
            .filter(|entry| !entry.is_empty())
            .collect();
        let derived: Vec<&str> = pdf_a_mutation_kinds().iter().map(|descriptor| descriptor.kind).collect();
        assert_eq!(declared, derived);
    }
}
//#endregion 🧪️CatalogParity

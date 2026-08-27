//! 🧬️ Transparent PDF 1.7/E conformance mutation dispatch. Every concrete payload, graph
//! transform, inverse, codec, schema, and test is owned by its direct semantic leaf.

use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::{diff::PdfDiff, snapshot::PdfSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Leaves
#[path = "🔒️insert-encryption-dictionary/🦀️component.rs"]
pub mod insert_encryption_dictionary;
#[path = "🔓️remove-encryption-dictionary/🦀️component.rs"]
pub mod remove_encryption_dictionary;
#[path = "📜️insert-javascript-action/🦀️component.rs"]
pub mod insert_javascript_action;
#[path = "🚫️remove-javascript-action/🦀️component.rs"]
pub mod remove_javascript_action;
#[path = "🚀️insert-launch-action/🦀️component.rs"]
pub mod insert_launch_action;
#[path = "🛬️remove-launch-action/🦀️component.rs"]
pub mod remove_launch_action;
#[path = "🎬️insert-media-annotation/🦀️component.rs"]
pub mod insert_media_annotation;
#[path = "⏹️remove-media-annotation/🦀️component.rs"]
pub mod remove_media_annotation;
#[path = "🏳️set-output-intent/🦀️component.rs"]
pub mod set_output_intent;
#[path = "🧽️remove-output-intent/🦀️component.rs"]
pub mod remove_output_intent;
#[path = "🔤️embed-font-file/🦀️component.rs"]
pub mod embed_font_file;
#[path = "🧺️remove-font-file/🦀️component.rs"]
pub mod remove_font_file;

pub use embed_font_file::EmbedFontFile;
pub use insert_encryption_dictionary::InsertEncryptionDictionary;
pub use insert_javascript_action::InsertJavascriptAction;
pub use insert_launch_action::InsertLaunchAction;
pub use insert_media_annotation::InsertMediaAnnotation;
pub use remove_encryption_dictionary::RemoveEncryptionDictionary;
pub use remove_font_file::RemoveFontFile;
pub use remove_javascript_action::RemoveJavascriptAction;
pub use remove_launch_action::RemoveLaunchAction;
pub use remove_media_annotation::RemoveMediaAnnotation;
pub use remove_output_intent::RemoveOutputIntent;
pub use set_output_intent::SetOutputIntent;
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
/// 📐️ Typed PDF/E-1 conformance vocabulary with one direct wrapped variant per semantic operation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = PdfSnapshot, diff = PdfDiff, schema = "s.stdio.pdf.1.7.e")]
pub enum PdfEMutation {
    InsertEncryptionDictionary(InsertEncryptionDictionary),
    RemoveEncryptionDictionary(RemoveEncryptionDictionary),
    InsertJavascriptAction(InsertJavascriptAction),
    RemoveJavascriptAction(RemoveJavascriptAction),
    InsertLaunchAction(InsertLaunchAction),
    RemoveLaunchAction(RemoveLaunchAction),
    InsertMediaAnnotation(InsertMediaAnnotation),
    RemoveMediaAnnotation(RemoveMediaAnnotation),
    SetOutputIntent(SetOutputIntent),
    RemoveOutputIntent(RemoveOutputIntent),
    EmbedFontFile(EmbedFontFile),
    RemoveFontFile(RemoveFontFile),
}
//#endregion 🔖️Aggregate

//#region 🔖️Codecs
#[path = "📝️text/🦀️component.rs"]
pub mod text;
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
//#endregion 🔖️Codecs

//#region 🔖️Delegation
/// ▶️ Applies one PDF/E conformance mutation through its leaf-owned diff.
pub fn apply_e_conformance_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfEMutation) -> protocol::MutationOutcome<PdfDiff> {
    use protocol::Mutation;
    let outcome = mutation.diff(snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ Delegates inverse planning to the authoritative leaf.
pub fn inverse_e_conformance_mutation(mutation: &PdfEMutation, base: &PdfSnapshot) -> Vec<PdfEMutation> {
    use protocol::Mutation;
    mutation.inverse(base)
}

/// 🧾️ Returns the derive-owned semantic catalog in declaration and binary-tag order.
pub fn pdf_e_mutation_kinds() -> &'static [protocol::SemanticDescriptor] {
    use protocol::SemanticMutation;
    PdfEMutation::kinds()
}
//#endregion 🔖️Delegation

//#region 🧪️CatalogParity
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derived_catalog_matches_the_language_neutral_oracle_catalog() {
        let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️e/🧬️schema/🧬️mutations");
        let manifest = std::fs::read_to_string(mutation_root.join("../../🧪️oracle/🔣️.json")).expect("language-neutral oracle catalog");
        let needle = "\"kinds\": [";
        let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
        let end = start + manifest[start..].find(']').expect("kinds array is closed");
        let declared: Vec<String> = manifest[start..end]
            .split(',')
            .map(|entry| entry.trim().trim_matches('"').to_string())
            .filter(|entry| !entry.is_empty())
            .collect();
        let derived: Vec<&str> = pdf_e_mutation_kinds().iter().map(|descriptor| descriptor.kind).collect();
        assert_eq!(declared, derived);
    }
}
//#endregion 🧪️CatalogParity

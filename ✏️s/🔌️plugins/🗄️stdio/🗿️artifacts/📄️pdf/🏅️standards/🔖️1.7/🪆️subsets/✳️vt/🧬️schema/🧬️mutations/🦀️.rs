//! 🧬️ Transparent PDF 1.7/VT conformance mutation dispatch. Concrete payloads, graph transforms,
//! inverses, codecs, schemas, and tests live in direct semantic leaves.

use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};

//#region 🔖️Leaves
#[path = "🔒️insert-encryption-dictionary/🦀️.rs"]
pub mod insert_encryption_dictionary;
#[path = "🔓️remove-encryption-dictionary/🦀️.rs"]
pub mod remove_encryption_dictionary;
#[path = "🏳️set-output-intent/🦀️.rs"]
pub mod set_output_intent;
#[path = "🧽️remove-output-intent/🦀️.rs"]
pub mod remove_output_intent;
#[path = "📐️set-trim-box/🦀️.rs"]
pub mod set_trim_box;
#[path = "🧽️remove-trim-box/🦀️.rs"]
pub mod remove_trim_box;
#[path = "🔤️embed-font-file/🦀️.rs"]
pub mod embed_font_file;
#[path = "🧺️remove-font-file/🦀️.rs"]
pub mod remove_font_file;
#[path = "📜️insert-javascript-action/🦀️.rs"]
pub mod insert_javascript_action;
#[path = "🚫️remove-javascript-action/🦀️.rs"]
pub mod remove_javascript_action;
#[path = "🚀️insert-launch-action/🦀️.rs"]
pub mod insert_launch_action;
#[path = "🛬️remove-launch-action/🦀️.rs"]
pub mod remove_launch_action;
#[path = "🎬️insert-media-annotation/🦀️.rs"]
pub mod insert_media_annotation;
#[path = "⏹️remove-media-annotation/🦀️.rs"]
pub mod remove_media_annotation;
#[path = "🗂️set-dpart-root/🦀️.rs"]
pub mod set_dpart_root;
#[path = "🧹️remove-dpart-root/🦀️.rs"]
pub mod remove_dpart_root;
#[path = "🏷️set-dpart-metadata/🦀️.rs"]
pub mod set_dpart_metadata;
#[path = "🗑️remove-dpart-metadata/🦀️.rs"]
pub mod remove_dpart_metadata;

pub use insert_encryption_dictionary::InsertEncryptionDictionary;
pub use remove_encryption_dictionary::RemoveEncryptionDictionary;
pub use set_output_intent::SetOutputIntent;
pub use remove_output_intent::RemoveOutputIntent;
pub use set_trim_box::SetTrimBox;
pub use remove_trim_box::RemoveTrimBox;
pub use embed_font_file::EmbedFontFile;
pub use remove_font_file::RemoveFontFile;
pub use insert_javascript_action::InsertJavascriptAction;
pub use remove_javascript_action::RemoveJavascriptAction;
pub use insert_launch_action::InsertLaunchAction;
pub use remove_launch_action::RemoveLaunchAction;
pub use insert_media_annotation::InsertMediaAnnotation;
pub use remove_media_annotation::RemoveMediaAnnotation;
pub use set_dpart_root::SetDpartRoot;
pub use remove_dpart_root::RemoveDpartRoot;
pub use set_dpart_metadata::SetDpartMetadata;
pub use remove_dpart_metadata::RemoveDpartMetadata;
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
/// 📐️ Typed PDF/VT conformance vocabulary with one direct wrapped variant per semantic operation.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = PdfSnapshot, diff = PdfDiff, schema = "s.stdio.pdf.1.7.vt")]
pub enum PdfVtMutation {
    InsertEncryptionDictionary(InsertEncryptionDictionary),
    RemoveEncryptionDictionary(RemoveEncryptionDictionary),
    SetOutputIntent(SetOutputIntent),
    RemoveOutputIntent(RemoveOutputIntent),
    SetTrimBox(SetTrimBox),
    RemoveTrimBox(RemoveTrimBox),
    EmbedFontFile(EmbedFontFile),
    RemoveFontFile(RemoveFontFile),
    InsertJavascriptAction(InsertJavascriptAction),
    RemoveJavascriptAction(RemoveJavascriptAction),
    InsertLaunchAction(InsertLaunchAction),
    RemoveLaunchAction(RemoveLaunchAction),
    InsertMediaAnnotation(InsertMediaAnnotation),
    RemoveMediaAnnotation(RemoveMediaAnnotation),
    SetDpartRoot(SetDpartRoot),
    RemoveDpartRoot(RemoveDpartRoot),
    SetDpartMetadata(SetDpartMetadata),
    RemoveDpartMetadata(RemoveDpartMetadata),
}
//#endregion 🔖️Aggregate

//#region 🔖️Codecs
#[path = "📝️text/🦀️.rs"]
pub mod text;
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
//#endregion 🔖️Codecs

//#region 🔖️Delegation
/// ▶️ Applies one PDF/VT mutation through its leaf-owned diff.
pub fn apply_vt_conformance_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfVtMutation) -> protocol::MutationOutcome<PdfDiff> {
    use protocol::Mutation;
    let outcome = mutation.diff(snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ Delegates inverse planning to the authoritative leaf.
pub fn inverse_vt_conformance_mutation(mutation: &PdfVtMutation, base: &PdfSnapshot) -> Vec<PdfVtMutation> {
    use protocol::Mutation;
    mutation.inverse(base)
}

/// 🧾️ Returns the derive-owned semantic catalog.
pub fn pdf_vt_mutation_kinds() -> &'static [protocol::SemanticDescriptor] {
    use protocol::SemanticMutation;
    PdfVtMutation::kinds()
}
//#endregion 🔖️Delegation

//#region 🧪️CatalogParity
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derived_catalog_matches_the_language_neutral_oracle_catalog() {
        let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️vt/🧬️schema/🧬️mutations");
        let manifest = std::fs::read_to_string(mutation_root.join("../../🔣️oracle.json")).expect("language-neutral oracle catalog");
        let needle = "\"kinds\": [";
        let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
        let end = start + manifest[start..].find(']').expect("kinds array is closed");
        let declared: Vec<String> = manifest[start..end]
            .split(',')
            .map(|entry| entry.trim().trim_matches('"').to_string())
            .filter(|entry| !entry.is_empty())
            .collect();
        let derived: Vec<&str> = pdf_vt_mutation_kinds().iter().map(|descriptor| descriptor.kind).collect();
        assert_eq!(declared, derived);
        assert_eq!(text::TEXT_OPCODE_REGISTRY.iter().map(|(_, kind)| *kind).collect::<Vec<_>>(), derived);
        assert_eq!(binary::BINARY_TAG_REGISTRY.iter().map(|(_, _, tag)| *tag).collect::<Vec<_>>(), (0..derived.len() as u8).collect::<Vec<_>>());
    }
}
//#endregion 🧪️CatalogParity

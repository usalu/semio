//! 🧬️ Transparent PDF 1.7/H conformance mutation dispatch. Concrete payloads, graph transforms,
//! inverses, codecs, schemas, and tests live in direct semantic leaves.

use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Leaves
#[path = "🏷️set-info-title/🦀️.rs"]
pub mod set_info_title;
#[path = "👤️set-info-author/🦀️.rs"]
pub mod set_info_author;
#[path = "📜️insert-javascript-action/🦀️.rs"]
pub mod insert_javascript_action;
#[path = "🚫️remove-javascript-action/🦀️.rs"]
pub mod remove_javascript_action;
#[path = "🚀️insert-launch-action/🦀️.rs"]
pub mod insert_launch_action;
#[path = "🛬️remove-launch-action/🦀️.rs"]
pub mod remove_launch_action;
#[path = "✒️insert-signature-field/🦀️.rs"]
pub mod insert_signature_field;
#[path = "✂️remove-signature-field/🦀️.rs"]
pub mod remove_signature_field;
#[path = "🔤️embed-font-file/🦀️.rs"]
pub mod embed_font_file;
#[path = "🧺️remove-font-file/🦀️.rs"]
pub mod remove_font_file;

pub use embed_font_file::EmbedFontFile;
pub use insert_javascript_action::InsertJavascriptAction;
pub use insert_launch_action::InsertLaunchAction;
pub use insert_signature_field::InsertSignatureField;
pub use remove_font_file::RemoveFontFile;
pub use remove_javascript_action::RemoveJavascriptAction;
pub use remove_launch_action::RemoveLaunchAction;
pub use remove_signature_field::RemoveSignatureField;
pub use set_info_author::SetInfoAuthor;
pub use set_info_title::SetInfoTitle;
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
/// 📐️ Typed PDF/H conformance vocabulary with one direct wrapped variant per semantic operation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = PdfSnapshot, diff = PdfDiff, schema = "s.stdio.pdf.1.7.h")]
pub enum PdfHMutation {
    SetInfoTitle(SetInfoTitle),
    SetInfoAuthor(SetInfoAuthor),
    InsertJavascriptAction(InsertJavascriptAction),
    RemoveJavascriptAction(RemoveJavascriptAction),
    InsertLaunchAction(InsertLaunchAction),
    RemoveLaunchAction(RemoveLaunchAction),
    InsertSignatureField(InsertSignatureField),
    RemoveSignatureField(RemoveSignatureField),
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
/// ▶️ Applies one PDF/H mutation through its leaf-owned diff.
pub fn apply_h_conformance_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfHMutation) -> protocol::MutationOutcome<PdfDiff> {
    use protocol::Mutation;
    let outcome = mutation.diff(snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ Delegates inverse planning to the authoritative leaf.
pub fn inverse_h_conformance_mutation(mutation: &PdfHMutation, base: &PdfSnapshot) -> Vec<PdfHMutation> {
    use protocol::Mutation;
    mutation.inverse(base)
}

/// 🧾️ Returns the derive-owned semantic catalog.
pub fn pdf_h_mutation_kinds() -> &'static [protocol::SemanticDescriptor] {
    use protocol::SemanticMutation;
    PdfHMutation::kinds()
}
//#endregion 🔖️Delegation

//#region 🧪️CatalogParity
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derived_catalog_matches_the_language_neutral_oracle_catalog() {
        let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️h/🧬️schema/🧬️mutations");
        let manifest = std::fs::read_to_string(mutation_root.join("../../🧪️oracle/🔣️.json")).expect("language-neutral oracle catalog");
        let needle = "\"kinds\": [";
        let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
        let end = start + manifest[start..].find(']').expect("kinds array is closed");
        let declared: Vec<String> = manifest[start..end]
            .split(',')
            .map(|entry| entry.trim().trim_matches('"').to_string())
            .filter(|entry| !entry.is_empty())
            .collect();
        let derived: Vec<&str> = pdf_h_mutation_kinds().iter().map(|descriptor| descriptor.kind).collect();
        assert_eq!(declared, derived);
    }
}
//#endregion 🧪️CatalogParity

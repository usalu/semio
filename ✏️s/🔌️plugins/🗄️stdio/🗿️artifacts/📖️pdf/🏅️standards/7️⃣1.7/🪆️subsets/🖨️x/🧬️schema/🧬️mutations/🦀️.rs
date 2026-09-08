//! 🧬️ Transparent PDF 1.7/X conformance mutation dispatch. Concrete payloads, graph transforms,
//! inverses, codecs, schemas, and tests live in direct semantic leaves.

use crate::standards::v1_7::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};

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
#[path = "✂️remove-trim-box/🦀️.rs"]
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
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
/// 📐️ Typed PDF/X conformance vocabulary with one direct wrapped variant per semantic operation.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = PdfSnapshot, diff = PdfDiff, schema = "s.stdio.pdf.1.7.x")]
pub enum PdfXMutation {
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
}
//#endregion 🔖️Aggregate

//#region 🔖️Codecs
#[path = "📝️text/🦀️.rs"]
pub mod text;
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
//#endregion 🔖️Codecs

//#region 🔖️Delegation
/// ▶️ Applies one PDF/X mutation through its leaf-owned diff.
pub fn apply_x_conformance_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfXMutation) -> protocol::MutationOutcome<PdfDiff> {
    use protocol::Mutation;
    let outcome = mutation.diff(snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ Delegates inverse planning to the authoritative leaf.
pub fn inverse_x_conformance_mutation(mutation: &PdfXMutation, base: &PdfSnapshot) -> Vec<PdfXMutation> {
    use protocol::Mutation;
    mutation.inverse(base)
}

/// 🧾️ Returns the derive-owned semantic catalog.
pub fn pdf_x_mutation_kinds() -> &'static [protocol::SemanticDescriptor] {
    use protocol::SemanticMutation;
    PdfXMutation::kinds()
}
//#endregion 🔖️Delegation

//#region 🧪️CatalogParity
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️CatalogParity

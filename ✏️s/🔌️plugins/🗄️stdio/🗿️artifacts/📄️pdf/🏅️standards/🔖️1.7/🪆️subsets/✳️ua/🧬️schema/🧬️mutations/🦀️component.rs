//! 🧬️ Transparent PDF 1.7/UA conformance mutation dispatch. Concrete payloads, graph transforms,
//! inverses, codecs, schemas, and tests live in direct semantic leaves.

use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::{diff::PdfDiff, snapshot::PdfSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Leaves
#[path = "🏷️set-mark-info/🦀️component.rs"]
pub mod set_mark_info;
#[path = "🗑️remove-mark-info/🦀️component.rs"]
pub mod remove_mark_info;
#[path = "🌲️set-struct-tree-root/🦀️component.rs"]
pub mod set_struct_tree_root;
#[path = "🪓️remove-struct-tree-root/🦀️component.rs"]
pub mod remove_struct_tree_root;
#[path = "🗣️set-lang/🦀️component.rs"]
pub mod set_lang;
#[path = "🤐️remove-lang/🦀️component.rs"]
pub mod remove_lang;
#[path = "🪧️set-display-doc-title/🦀️component.rs"]
pub mod set_display_doc_title;
#[path = "🚫️remove-display-doc-title/🦀️component.rs"]
pub mod remove_display_doc_title;
#[path = "🏷️set-info-title/🦀️component.rs"]
pub mod set_info_title;
#[path = "🔤️embed-font-file/🦀️component.rs"]
pub mod embed_font_file;
#[path = "🧺️remove-font-file/🦀️component.rs"]
pub mod remove_font_file;

pub use set_mark_info::SetMarkInfo;
pub use remove_mark_info::RemoveMarkInfo;
pub use set_struct_tree_root::SetStructTreeRoot;
pub use remove_struct_tree_root::RemoveStructTreeRoot;
pub use set_lang::SetLang;
pub use remove_lang::RemoveLang;
pub use set_display_doc_title::SetDisplayDocTitle;
pub use remove_display_doc_title::RemoveDisplayDocTitle;
pub use set_info_title::SetInfoTitle;
pub use embed_font_file::EmbedFontFile;
pub use remove_font_file::RemoveFontFile;
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
/// 📐️ Typed PDF/UA conformance vocabulary with one direct wrapped variant per semantic operation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = PdfSnapshot, diff = PdfDiff, schema = "s.stdio.pdf.1.7.ua")]
pub enum PdfUaMutation {
    SetMarkInfo(SetMarkInfo),
    RemoveMarkInfo(RemoveMarkInfo),
    SetStructTreeRoot(SetStructTreeRoot),
    RemoveStructTreeRoot(RemoveStructTreeRoot),
    SetLang(SetLang),
    RemoveLang(RemoveLang),
    SetDisplayDocTitle(SetDisplayDocTitle),
    RemoveDisplayDocTitle(RemoveDisplayDocTitle),
    SetInfoTitle(SetInfoTitle),
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
/// ▶️ Applies one PDF/UA mutation through its leaf-owned diff.
pub fn apply_ua_conformance_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfUaMutation) -> protocol::MutationOutcome<PdfDiff> {
    use protocol::Mutation;
    let outcome = mutation.diff(snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ Delegates inverse planning to the authoritative leaf.
pub fn inverse_ua_conformance_mutation(mutation: &PdfUaMutation, base: &PdfSnapshot) -> Vec<PdfUaMutation> {
    use protocol::Mutation;
    mutation.inverse(base)
}

/// 🧾️ Returns the derive-owned semantic catalog.
pub fn pdf_ua_mutation_kinds() -> &'static [protocol::SemanticDescriptor] {
    use protocol::SemanticMutation;
    PdfUaMutation::kinds()
}
//#endregion 🔖️Delegation

//#region 🧪️CatalogParity
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derived_catalog_matches_the_language_neutral_oracle_catalog() {
        let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️ua/🧬️schema/🧬️mutations");
        let manifest = std::fs::read_to_string(mutation_root.join("../../🧪️oracle/🔣️.json")).expect("language-neutral oracle catalog");
        let needle = "\"kinds\": [";
        let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
        let end = start + manifest[start..].find(']').expect("kinds array is closed");
        let declared: Vec<String> = manifest[start..end]
            .split(',')
            .map(|entry| entry.trim().trim_matches('"').to_string())
            .filter(|entry| !entry.is_empty())
            .collect();
        let derived: Vec<&str> = pdf_ua_mutation_kinds().iter().map(|descriptor| descriptor.kind).collect();
        assert_eq!(declared, derived);
        assert_eq!(text::TEXT_OPCODE_REGISTRY.iter().map(|(_, kind)| *kind).collect::<Vec<_>>(), derived);
        assert_eq!(binary::BINARY_TAG_REGISTRY.iter().map(|(_, _, tag)| *tag).collect::<Vec<_>>(), (0..derived.len() as u8).collect::<Vec<_>>());
    }
}
//#endregion 🧪️CatalogParity

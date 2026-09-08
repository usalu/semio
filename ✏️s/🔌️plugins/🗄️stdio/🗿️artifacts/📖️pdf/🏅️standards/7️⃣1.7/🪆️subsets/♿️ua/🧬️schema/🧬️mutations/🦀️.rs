//! 🧬️ Transparent PDF 1.7/UA conformance mutation dispatch. Concrete payloads, graph transforms,
//! inverses, codecs, schemas, and tests live in direct semantic leaves.

use crate::standards::v1_7::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};

//#region 🔖️Leaves
#[path = "✅️set-mark-info/🦀️.rs"]
pub mod set_mark_info;
#[path = "🗑️remove-mark-info/🦀️.rs"]
pub mod remove_mark_info;
#[path = "🌲️set-struct-tree-root/🦀️.rs"]
pub mod set_struct_tree_root;
#[path = "🪓️remove-struct-tree-root/🦀️.rs"]
pub mod remove_struct_tree_root;
#[path = "🗣️set-lang/🦀️.rs"]
pub mod set_lang;
#[path = "🤐️remove-lang/🦀️.rs"]
pub mod remove_lang;
#[path = "🪧️set-display-doc-title/🦀️.rs"]
pub mod set_display_doc_title;
#[path = "🚫️remove-display-doc-title/🦀️.rs"]
pub mod remove_display_doc_title;
#[path = "📰️set-info-title/🦀️.rs"]
pub mod set_info_title;
#[path = "🔤️embed-font-file/🦀️.rs"]
pub mod embed_font_file;
#[path = "🧺️remove-font-file/🦀️.rs"]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
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
#[path = "📝️text/🦀️.rs"]
pub mod text;
#[path = "💾️binary/🦀️.rs"]
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️CatalogParity

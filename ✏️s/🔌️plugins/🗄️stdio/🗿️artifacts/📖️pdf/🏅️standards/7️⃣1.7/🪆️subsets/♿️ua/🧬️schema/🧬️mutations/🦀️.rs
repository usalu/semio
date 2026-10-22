//! 🧬️ Transparent PDF 1.7/UA conformance mutation dispatch. Concrete payloads, graph transforms,
//! inverses, codecs, schemas, and tests live in direct semantic leaves.

use crate::standards::v1_7::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};

//#region 🔖️Leaves
pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::embed_font_file;
pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::remove_display_doc_title;
pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::remove_font_file;
pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::remove_lang;
pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::remove_mark_info;
pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::remove_struct_tree_root;
pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::set_display_doc_title;
pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::ua::schema::mutations::set_info_title;
pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::set_lang;
pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::set_mark_info;
pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::set_struct_tree_root;

pub use embed_font_file::EmbedFontFile;
pub use remove_display_doc_title::RemoveDisplayDocTitle;
pub use remove_font_file::RemoveFontFile;
pub use remove_lang::RemoveLang;
pub use remove_mark_info::RemoveMarkInfo;
pub use remove_struct_tree_root::RemoveStructTreeRoot;
pub use set_display_doc_title::SetDisplayDocTitle;
pub use set_info_title::SetInfoTitle;
pub use set_lang::SetLang;
pub use set_mark_info::SetMarkInfo;
pub use set_struct_tree_root::SetStructTreeRoot;
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
/// 📐️ Typed PDF/UA conformance vocabulary with one direct wrapped variant per semantic operation.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = PdfSnapshot, diff = PdfDiff, schema = "s.stdio.pdf.1.7.ua", reference = PdfSnapshot, reference_diff = PdfDiff)]
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
//#endregion 🔖️Codecs

//#region 🔖️Delegation
/// 🧾️ Returns the derive-owned semantic catalog.
pub fn pdf_ua_mutation_kinds() -> &'static [protocol::SemanticDescriptor] {
    use protocol::SemanticMutation;
    PdfUaMutation::kinds()
}

//#endregion 🔖️Delegation

//#region 🧪️CatalogParity

//#endregion 🧪️CatalogParity

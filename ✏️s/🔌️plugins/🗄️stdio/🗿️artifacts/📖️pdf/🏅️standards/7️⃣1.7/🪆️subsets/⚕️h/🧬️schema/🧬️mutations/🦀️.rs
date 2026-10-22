//! 🧬️ Transparent PDF 1.7/H conformance mutation dispatch. Concrete payloads, graph transforms,
//! inverses, codecs, schemas, and tests live in direct semantic leaves.

use crate::standards::v1_7::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};

//#region 🔖️Leaves
pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::embed_font_file;
pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::insert_javascript_action;
pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::insert_launch_action;
pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::insert_signature_field;
pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::remove_font_file;
pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::remove_javascript_action;
pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::remove_launch_action;
pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::remove_signature_field;
pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::h::schema::mutations::set_info_author;
pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::h::schema::mutations::set_info_title;

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
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = PdfSnapshot, diff = PdfDiff, schema = "s.stdio.pdf.1.7.h", reference = PdfSnapshot, reference_diff = PdfDiff)]
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
//#endregion 🔖️Codecs

//#region 🔖️Delegation
/// 🧾️ Returns the derive-owned semantic catalog.
pub fn pdf_h_mutation_kinds() -> &'static [protocol::SemanticDescriptor] {
    use protocol::SemanticMutation;
    PdfHMutation::kinds()
}

//#endregion 🔖️Delegation

//#region 🧪️CatalogParity

//#endregion 🧪️CatalogParity

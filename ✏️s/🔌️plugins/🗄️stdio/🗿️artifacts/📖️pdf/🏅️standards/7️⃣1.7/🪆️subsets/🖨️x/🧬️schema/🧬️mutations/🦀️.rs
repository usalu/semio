//! 🧬️ Transparent PDF 1.7/X conformance mutation dispatch. Concrete payloads, graph transforms,
//! inverses, codecs, schemas, and tests live in direct semantic leaves.

use crate::standards::v1_7::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};

//#region 🔖️Leaves
pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::embed_font_file;
pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::insert_encryption_dictionary;
pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::insert_javascript_action;
pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::insert_launch_action;
pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::insert_media_annotation;
pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::remove_encryption_dictionary;
pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::remove_font_file;
pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::remove_javascript_action;
pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::remove_launch_action;
pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::remove_media_annotation;
pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::remove_output_intent;
pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::remove_trim_box;
pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::set_output_intent;
pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::set_trim_box;

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
pub use remove_trim_box::RemoveTrimBox;
pub use set_output_intent::SetOutputIntent;
pub use set_trim_box::SetTrimBox;
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
/// 📐️ Typed PDF/X conformance vocabulary with one direct wrapped variant per semantic operation.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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
//#endregion 🔖️Codecs

//#region 🔖️Delegation
/// 🧾️ Returns the derive-owned semantic catalog.
pub fn pdf_x_mutation_kinds() -> &'static [protocol::SemanticDescriptor] {
    use protocol::SemanticMutation;
    PdfXMutation::kinds()
}

//#endregion 🔖️Delegation

//#region 🧪️CatalogParity

//#endregion 🧪️CatalogParity

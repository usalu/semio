//! 🧬️ Transparent dispatch for the PDF 1.7/A conformance mutation vocabulary. Concrete payloads,
//! graph transforms, inverse plans, and tests live in direct semantic leaves; this root only mounts,
//! re-exports, wraps, delegates, and assembles the generated registry.

use crate::standards::v1_7::subsets::base::schema::diff::PdfDiff;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;

//#region 🔖️Leaves
pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::a::schema::mutations::embed_font_file;
pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::a::schema::mutations::insert_embedded_file;
pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::a::schema::mutations::insert_encryption_dictionary;
pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::a::schema::mutations::insert_javascript_action;
pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::a::schema::mutations::insert_launch_action;
pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::a::schema::mutations::remove_af_relationship;
pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::a::schema::mutations::remove_embedded_file;
pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::a::schema::mutations::remove_encryption_dictionary;
pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::a::schema::mutations::remove_font_file;
pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::a::schema::mutations::remove_javascript_action;
pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::a::schema::mutations::remove_launch_action;
pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::a::schema::mutations::remove_output_intent;
pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::a::schema::mutations::set_af_relationship;
pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::a::schema::mutations::set_output_intent;

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
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = PdfSnapshot, diff = PdfDiff, schema = "s.stdio.pdf.1.7.a", reference = PdfSnapshot, reference_diff = PdfDiff)]
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
/// 🧾️ Returns the derive-owned semantic catalog in declaration order.
pub fn pdf_a_mutation_kinds() -> &'static [protocol::SemanticDescriptor] {
    use protocol::SemanticMutation;
    PdfAMutation::kinds()
}

//#endregion 🔖️Delegation

//#region 🧪️CatalogParity

//#endregion 🧪️CatalogParity

//! 🧬️ Transparent PDF 1.7/Any mutation dispatch. Every concrete payload, diff, inverse, codec,
//! schema, and test is owned by its direct semantic folder. Ticket 26/09/18/PDF-ARTIFACT-SPEC-
//! COMPLETE widened the vocabulary from the page/COS edits to the whole typed model: page boxes
//! and user units, content operators and annotations by index, every document collection by id
//! (fonts, images, forms, graphics states, shadings, patterns, colour spaces, property lists,
//! embedded files), outlines, named destinations, page labels, output intents, the interactive
//! form, optional content, viewer settings, metadata, identity and encryption, catalog extras —
//! with the retained COS lanes' object/dict/trailer edits kept as they were.

use crate::standards::v1_7::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};

//#region 🔖️Leaves
#[path = "📥️insert-page/🦀️.rs"]
pub mod insert_page;
#[path = "🗑️remove-page/🦀️.rs"]
pub mod remove_page;
#[path = "📐️set-page-media-box/🦀️.rs"]
pub mod set_page_media_box;
#[path = "✂️set-page-crop-box/🦀️.rs"]
pub mod set_page_crop_box;
#[path = "➕️append-page-content/🦀️.rs"]
pub mod append_page_content;
#[path = "ℹ️set-info/🦀️.rs"]
pub mod set_info;
#[path = "📦️insert-object/🦀️.rs"]
pub mod insert_object;
#[path = "🧹️remove-object/🦀️.rs"]
pub mod remove_object;
#[path = "🔧️set-object-value/🦀️.rs"]
pub mod set_object_value;
#[path = "🔑️set-dict-entry/🦀️.rs"]
pub mod set_dict_entry;
#[path = "🚫️remove-dict-entry/🦀️.rs"]
pub mod remove_dict_entry;
#[path = "🧳️set-trailer-entry/🦀️.rs"]
pub mod set_trailer_entry;
#[path = "🧽️remove-trailer-entry/🦀️.rs"]
pub mod remove_trailer_entry;
#[path = "🔀️move-page/🦀️.rs"]
pub mod move_page;
#[path = "✏️set-page-content/🦀️.rs"]
pub mod set_page_content;
#[path = "🔄️set-page-rotation/🦀️.rs"]
pub mod set_page_rotation;
#[path = "🖼️set-page-box/🦀️.rs"]
pub mod set_page_box;
#[path = "📏️set-page-user-unit/🦀️.rs"]
pub mod set_page_user_unit;
#[path = "🖋️insert-content/🦀️.rs"]
pub mod insert_content;
#[path = "🧻️remove-content/🦀️.rs"]
pub mod remove_content;
#[path = "🔁️replace-content/🦀️.rs"]
pub mod replace_content;
#[path = "📌️insert-annotation/🦀️.rs"]
pub mod insert_annotation;
#[path = "📍️remove-annotation/🦀️.rs"]
pub mod remove_annotation;
#[path = "📝️set-annotation/🦀️.rs"]
pub mod set_annotation;
#[path = "🔤️set-font/🦀️.rs"]
pub mod set_font;
#[path = "🅾️remove-font/🦀️.rs"]
pub mod remove_font;
#[path = "🏞️set-image/🦀️.rs"]
pub mod set_image;
#[path = "🌫️remove-image/🦀️.rs"]
pub mod remove_image;
#[path = "📄️set-form/🦀️.rs"]
pub mod set_form;
#[path = "🗞️remove-form/🦀️.rs"]
pub mod remove_form;
#[path = "🎛️set-ext-g-state/🦀️.rs"]
pub mod set_ext_g_state;
#[path = "🎚️remove-ext-g-state/🦀️.rs"]
pub mod remove_ext_g_state;
#[path = "🌅️set-shading/🦀️.rs"]
pub mod set_shading;
#[path = "🌄️remove-shading/🦀️.rs"]
pub mod remove_shading;
#[path = "🧩️set-pattern/🦀️.rs"]
pub mod set_pattern;
#[path = "🪡️remove-pattern/🦀️.rs"]
pub mod remove_pattern;
#[path = "🌈️set-color-space/🦀️.rs"]
pub mod set_color_space;
#[path = "🎨️remove-color-space/🦀️.rs"]
pub mod remove_color_space;
#[path = "🏷️set-properties/🦀️.rs"]
pub mod set_properties;
#[path = "🔖️remove-properties/🦀️.rs"]
pub mod remove_properties;
#[path = "📎️set-embedded-file/🦀️.rs"]
pub mod set_embedded_file;
#[path = "🗃️remove-embedded-file/🦀️.rs"]
pub mod remove_embedded_file;
#[path = "📑️set-outlines/🦀️.rs"]
pub mod set_outlines;
#[path = "🎯️set-named-destination/🦀️.rs"]
pub mod set_named_destination;
#[path = "🎪️remove-named-destination/🦀️.rs"]
pub mod remove_named_destination;
#[path = "🔢️set-page-labels/🦀️.rs"]
pub mod set_page_labels;
#[path = "🏳️set-output-intents/🦀️.rs"]
pub mod set_output_intents;
#[path = "📋️set-acro-form/🦀️.rs"]
pub mod set_acro_form;
#[path = "👁️set-optional-content/🦀️.rs"]
pub mod set_optional_content;
#[path = "📖️set-page-layout/🦀️.rs"]
pub mod set_page_layout;
#[path = "🖥️set-page-mode/🦀️.rs"]
pub mod set_page_mode;
#[path = "🛠️set-viewer-preferences/🦀️.rs"]
pub mod set_viewer_preferences;
#[path = "🚪️set-open-action/🦀️.rs"]
pub mod set_open_action;
#[path = "🗣️set-language/🦀️.rs"]
pub mod set_language;
#[path = "🔏️set-mark-info/🦀️.rs"]
pub mod set_mark_info;
#[path = "🧾️set-metadata/🦀️.rs"]
pub mod set_metadata;
#[path = "🆔️set-document-id/🦀️.rs"]
pub mod set_document_id;
#[path = "🔐️set-encryption/🦀️.rs"]
pub mod set_encryption;
#[path = "🗂️set-catalog-entry/🦀️.rs"]
pub mod set_catalog_entry;
#[path = "🧺️remove-catalog-entry/🦀️.rs"]
pub mod remove_catalog_entry;

pub use insert_page::InsertPage;
pub use remove_page::RemovePage;
pub use set_page_media_box::SetPageMediaBox;
pub use set_page_crop_box::SetPageCropBox;
pub use append_page_content::AppendPageContent;
pub use set_info::SetInfo;
pub use insert_object::InsertObject;
pub use remove_object::RemoveObject;
pub use set_object_value::SetObjectValue;
pub use set_dict_entry::SetDictEntry;
pub use remove_dict_entry::RemoveDictEntry;
pub use set_trailer_entry::SetTrailerEntry;
pub use remove_trailer_entry::RemoveTrailerEntry;
pub use move_page::MovePage;
pub use set_page_content::SetPageContent;
pub use set_page_rotation::SetPageRotation;
pub use set_page_box::SetPageBox;
pub use set_page_user_unit::SetPageUserUnit;
pub use insert_content::InsertContent;
pub use remove_content::RemoveContent;
pub use replace_content::ReplaceContent;
pub use insert_annotation::InsertAnnotation;
pub use remove_annotation::RemoveAnnotation;
pub use set_annotation::SetAnnotation;
pub use set_font::SetFont;
pub use remove_font::RemoveFont;
pub use set_image::SetImage;
pub use remove_image::RemoveImage;
pub use set_form::SetForm;
pub use remove_form::RemoveForm;
pub use set_ext_g_state::SetExtGState;
pub use remove_ext_g_state::RemoveExtGState;
pub use set_shading::SetShading;
pub use remove_shading::RemoveShading;
pub use set_pattern::SetPattern;
pub use remove_pattern::RemovePattern;
pub use set_color_space::SetColorSpace;
pub use remove_color_space::RemoveColorSpace;
pub use set_properties::SetProperties;
pub use remove_properties::RemoveProperties;
pub use set_embedded_file::SetEmbeddedFile;
pub use remove_embedded_file::RemoveEmbeddedFile;
pub use set_outlines::SetOutlines;
pub use set_named_destination::SetNamedDestination;
pub use remove_named_destination::RemoveNamedDestination;
pub use set_page_labels::SetPageLabels;
pub use set_output_intents::SetOutputIntents;
pub use set_acro_form::SetAcroForm;
pub use set_optional_content::SetOptionalContent;
pub use set_page_layout::SetPageLayout;
pub use set_page_mode::SetPageMode;
pub use set_viewer_preferences::SetViewerPreferences;
pub use set_open_action::SetOpenAction;
pub use set_language::SetLanguage;
pub use set_mark_info::SetMarkInfo;
pub use set_metadata::SetMetadata;
pub use set_document_id::SetDocumentId;
pub use set_encryption::SetEncryption;
pub use set_catalog_entry::SetCatalogEntry;
pub use remove_catalog_entry::RemoveCatalogEntry;
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = PdfSnapshot, diff = PdfDiff, schema = "s.stdio.pdf.1.7")]
pub enum PdfMutation {
    InsertPage(InsertPage),
    RemovePage(RemovePage),
    SetPageMediaBox(SetPageMediaBox),
    SetPageCropBox(SetPageCropBox),
    AppendPageContent(AppendPageContent),
    SetInfo(SetInfo),
    InsertObject(InsertObject),
    RemoveObject(RemoveObject),
    SetObjectValue(SetObjectValue),
    SetDictEntry(SetDictEntry),
    RemoveDictEntry(RemoveDictEntry),
    SetTrailerEntry(SetTrailerEntry),
    RemoveTrailerEntry(RemoveTrailerEntry),
    MovePage(MovePage),
    SetPageContent(SetPageContent),
    SetPageRotation(SetPageRotation),
    SetPageBox(SetPageBox),
    SetPageUserUnit(SetPageUserUnit),
    InsertContent(InsertContent),
    RemoveContent(RemoveContent),
    ReplaceContent(ReplaceContent),
    InsertAnnotation(InsertAnnotation),
    RemoveAnnotation(RemoveAnnotation),
    SetAnnotation(SetAnnotation),
    SetFont(SetFont),
    RemoveFont(RemoveFont),
    SetImage(SetImage),
    RemoveImage(RemoveImage),
    SetForm(SetForm),
    RemoveForm(RemoveForm),
    SetExtGState(SetExtGState),
    RemoveExtGState(RemoveExtGState),
    SetShading(SetShading),
    RemoveShading(RemoveShading),
    SetPattern(SetPattern),
    RemovePattern(RemovePattern),
    SetColorSpace(SetColorSpace),
    RemoveColorSpace(RemoveColorSpace),
    SetProperties(SetProperties),
    RemoveProperties(RemoveProperties),
    SetEmbeddedFile(SetEmbeddedFile),
    RemoveEmbeddedFile(RemoveEmbeddedFile),
    SetOutlines(SetOutlines),
    SetNamedDestination(SetNamedDestination),
    RemoveNamedDestination(RemoveNamedDestination),
    SetPageLabels(SetPageLabels),
    SetOutputIntents(SetOutputIntents),
    SetAcroForm(SetAcroForm),
    SetOptionalContent(SetOptionalContent),
    SetPageLayout(SetPageLayout),
    SetPageMode(SetPageMode),
    SetViewerPreferences(SetViewerPreferences),
    SetOpenAction(SetOpenAction),
    SetLanguage(SetLanguage),
    SetMarkInfo(SetMarkInfo),
    SetMetadata(SetMetadata),
    SetDocumentId(SetDocumentId),
    SetEncryption(SetEncryption),
    SetCatalogEntry(SetCatalogEntry),
    RemoveCatalogEntry(RemoveCatalogEntry),
}
//#endregion 🔖️Aggregate

//#region 🔖️Delegation
/// ▶️ Applies one mutation through its leaf-owned diff.
pub fn apply_pdf_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfMutation) -> protocol::MutationOutcome<PdfDiff> {
    use protocol::Mutation;
    let outcome = mutation.diff(snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ Delegates inverse planning to the authoritative leaf.
pub fn inverse_pdf_mutation(mutation: &PdfMutation, base: &PdfSnapshot) -> Vec<PdfMutation> {
    use protocol::Mutation;
    mutation.inverse(base)
}

/// 🧾️ Returns the derive-owned identity table in declaration and binary-tag order.
pub fn pdf_mutation_kinds() -> &'static [protocol::SemanticDescriptor] {
    use protocol::SemanticMutation;
    PdfMutation::kinds()
}
//#endregion 🔖️Delegation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//! 📝️ Generic text framing and direct-owner registry for the visible PDF mutation aggregate.

use crate::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
use protocol::OpText;

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🧾️DerivedRegistry
/// 🧾️ Direct-owner text opcodes in aggregate declaration order.
pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[
    ("InsertPage", crate::standards::v1_7::subsets::base::schema::mutations::insert_page::TEXT_OPCODE),
    ("RemovePage", crate::standards::v1_7::subsets::base::schema::mutations::remove_page::TEXT_OPCODE),
    ("SetPageMediaBox", crate::standards::v1_7::subsets::base::schema::mutations::set_page_media_box::TEXT_OPCODE),
    ("SetPageCropBox", crate::standards::v1_7::subsets::base::schema::mutations::set_page_crop_box::TEXT_OPCODE),
    ("AppendPageContent", crate::standards::v1_7::subsets::base::schema::mutations::append_page_content::TEXT_OPCODE),
    ("SetInfo", crate::standards::v1_7::subsets::base::schema::mutations::set_info::TEXT_OPCODE),
    ("InsertObject", crate::standards::v1_7::subsets::base::schema::mutations::insert_object::TEXT_OPCODE),
    ("RemoveObject", crate::standards::v1_7::subsets::base::schema::mutations::remove_object::TEXT_OPCODE),
    ("SetObjectValue", crate::standards::v1_7::subsets::base::schema::mutations::set_object_value::TEXT_OPCODE),
    ("SetDictEntry", crate::standards::v1_7::subsets::base::schema::mutations::set_dict_entry::TEXT_OPCODE),
    ("RemoveDictEntry", crate::standards::v1_7::subsets::base::schema::mutations::remove_dict_entry::TEXT_OPCODE),
    ("SetTrailerEntry", crate::standards::v1_7::subsets::base::schema::mutations::set_trailer_entry::TEXT_OPCODE),
    ("RemoveTrailerEntry", crate::standards::v1_7::subsets::base::schema::mutations::remove_trailer_entry::TEXT_OPCODE),
    ("MovePage", crate::standards::v1_7::subsets::base::schema::mutations::move_page::TEXT_OPCODE),
    ("SetPageContent", crate::standards::v1_7::subsets::base::schema::mutations::set_page_content::TEXT_OPCODE),
    ("SetPageRotation", crate::standards::v1_7::subsets::base::schema::mutations::set_page_rotation::TEXT_OPCODE),
    ("SetPageBox", crate::standards::v1_7::subsets::base::schema::mutations::set_page_box::TEXT_OPCODE),
    ("SetPageUserUnit", crate::standards::v1_7::subsets::base::schema::mutations::set_page_user_unit::TEXT_OPCODE),
    ("InsertContent", crate::standards::v1_7::subsets::base::schema::mutations::insert_content::TEXT_OPCODE),
    ("RemoveContent", crate::standards::v1_7::subsets::base::schema::mutations::remove_content::TEXT_OPCODE),
    ("ReplaceContent", crate::standards::v1_7::subsets::base::schema::mutations::replace_content::TEXT_OPCODE),
    ("InsertAnnotation", crate::standards::v1_7::subsets::base::schema::mutations::insert_annotation::TEXT_OPCODE),
    ("RemoveAnnotation", crate::standards::v1_7::subsets::base::schema::mutations::remove_annotation::TEXT_OPCODE),
    ("SetAnnotation", crate::standards::v1_7::subsets::base::schema::mutations::set_annotation::TEXT_OPCODE),
    ("SetFont", crate::standards::v1_7::subsets::base::schema::mutations::set_font::TEXT_OPCODE),
    ("RemoveFont", crate::standards::v1_7::subsets::base::schema::mutations::remove_font::TEXT_OPCODE),
    ("SetImage", crate::standards::v1_7::subsets::base::schema::mutations::set_image::TEXT_OPCODE),
    ("RemoveImage", crate::standards::v1_7::subsets::base::schema::mutations::remove_image::TEXT_OPCODE),
    ("SetForm", crate::standards::v1_7::subsets::base::schema::mutations::set_form::TEXT_OPCODE),
    ("RemoveForm", crate::standards::v1_7::subsets::base::schema::mutations::remove_form::TEXT_OPCODE),
    ("SetExtGState", crate::standards::v1_7::subsets::base::schema::mutations::set_ext_g_state::TEXT_OPCODE),
    ("RemoveExtGState", crate::standards::v1_7::subsets::base::schema::mutations::remove_ext_g_state::TEXT_OPCODE),
    ("SetShading", crate::standards::v1_7::subsets::base::schema::mutations::set_shading::TEXT_OPCODE),
    ("RemoveShading", crate::standards::v1_7::subsets::base::schema::mutations::remove_shading::TEXT_OPCODE),
    ("SetPattern", crate::standards::v1_7::subsets::base::schema::mutations::set_pattern::TEXT_OPCODE),
    ("RemovePattern", crate::standards::v1_7::subsets::base::schema::mutations::remove_pattern::TEXT_OPCODE),
    ("SetColorSpace", crate::standards::v1_7::subsets::base::schema::mutations::set_color_space::TEXT_OPCODE),
    ("RemoveColorSpace", crate::standards::v1_7::subsets::base::schema::mutations::remove_color_space::TEXT_OPCODE),
    ("SetProperties", crate::standards::v1_7::subsets::base::schema::mutations::set_properties::TEXT_OPCODE),
    ("RemoveProperties", crate::standards::v1_7::subsets::base::schema::mutations::remove_properties::TEXT_OPCODE),
    ("SetEmbeddedFile", crate::standards::v1_7::subsets::base::schema::mutations::set_embedded_file::TEXT_OPCODE),
    ("RemoveEmbeddedFile", crate::standards::v1_7::subsets::base::schema::mutations::remove_embedded_file::TEXT_OPCODE),
    ("SetOutlines", crate::standards::v1_7::subsets::base::schema::mutations::set_outlines::TEXT_OPCODE),
    ("SetNamedDestination", crate::standards::v1_7::subsets::base::schema::mutations::set_named_destination::TEXT_OPCODE),
    ("RemoveNamedDestination", crate::standards::v1_7::subsets::base::schema::mutations::remove_named_destination::TEXT_OPCODE),
    ("SetPageLabels", crate::standards::v1_7::subsets::base::schema::mutations::set_page_labels::TEXT_OPCODE),
    ("SetOutputIntents", crate::standards::v1_7::subsets::base::schema::mutations::set_output_intents::TEXT_OPCODE),
    ("SetAcroForm", crate::standards::v1_7::subsets::base::schema::mutations::set_acro_form::TEXT_OPCODE),
    ("SetOptionalContent", crate::standards::v1_7::subsets::base::schema::mutations::set_optional_content::TEXT_OPCODE),
    ("SetPageLayout", crate::standards::v1_7::subsets::base::schema::mutations::set_page_layout::TEXT_OPCODE),
    ("SetPageMode", crate::standards::v1_7::subsets::base::schema::mutations::set_page_mode::TEXT_OPCODE),
    ("SetViewerPreferences", crate::standards::v1_7::subsets::base::schema::mutations::set_viewer_preferences::TEXT_OPCODE),
    ("SetOpenAction", crate::standards::v1_7::subsets::base::schema::mutations::set_open_action::TEXT_OPCODE),
    ("SetLanguage", crate::standards::v1_7::subsets::base::schema::mutations::set_language::TEXT_OPCODE),
    ("SetMarkInfo", crate::standards::v1_7::subsets::base::schema::mutations::set_mark_info::TEXT_OPCODE),
    ("SetMetadata", crate::standards::v1_7::subsets::base::schema::mutations::set_metadata::TEXT_OPCODE),
    ("SetDocumentId", crate::standards::v1_7::subsets::base::schema::mutations::set_document_id::TEXT_OPCODE),
    ("SetEncryption", crate::standards::v1_7::subsets::base::schema::mutations::set_encryption::TEXT_OPCODE),
    ("SetCatalogEntry", crate::standards::v1_7::subsets::base::schema::mutations::set_catalog_entry::TEXT_OPCODE),
    ("RemoveCatalogEntry", crate::standards::v1_7::subsets::base::schema::mutations::remove_catalog_entry::TEXT_OPCODE),
    ("SetSnapshot", crate::standards::v1_7::subsets::base::schema::mutations::set_snapshot::TEXT_OPCODE),
    ("PatchSnapshot", crate::standards::v1_7::subsets::base::schema::mutations::patch_snapshot::TEXT_OPCODE),
];
//#endregion 🧾️DerivedRegistry

//#region 🧱️Framing
const MAX_PAYLOAD_BYTES: usize = 64 * 1024 * 1024;

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut text = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        text.push(HEX[(byte >> 4) as usize] as char);
        text.push(HEX[(byte & 0x0f) as usize] as char);
    }
    text
}

fn decode_hex(value: &str) -> Result<Vec<u8>, String> {
    if !value.len().is_multiple_of(2) || value.len() > MAX_PAYLOAD_BYTES * 2 {
        return Err("PDF mutation text payload exceeds its budget".into());
    }
    fn nibble(value: u8) -> Option<u8> {
        if value.is_ascii_digit() {
            return Some(value - b'0');
        }
        (b'a'..=b'f').contains(&value).then_some(value - b'a' + 10)
    }
    value
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            let high = nibble(pair[0]).ok_or_else(|| "PDF mutation payload must be lowercase hexadecimal".to_string())?;
            let low = nibble(pair[1]).ok_or_else(|| "PDF mutation payload must be lowercase hexadecimal".to_string())?;
            Ok((high << 4) | low)
        })
        .collect()
}

fn text_error(detail: impl Into<String>) -> semio_framework_diagnostic::TextError {
    semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail.into(), semio_framework_diagnostic::TextSpan::at(1, 1))
}

impl OpText for PdfMutation {
    fn print_op(&self) -> String {
        let payload = semio_framework_pack_json::to_json_string(self).into_bytes();
        format!("pdf-mutation payload={}", encode_hex(&payload))
    }

    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let payload = line.strip_prefix("pdf-mutation payload=").ok_or_else(|| text_error("expected canonical PDF mutation aggregate"))?;
        let bytes = decode_hex(payload).map_err(text_error)?;
        let parsed = semio_framework_pack_json::parse_bytes(&bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| text_error(error.to_string()))?;
        semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| text_error(error.to_string()))
    }
}
//#endregion 🧱️Framing

#[path = "🌈️set-color-space/🦀️.rs"]
pub mod set_color_space;

#[path = "📐️set-page-media-box/🦀️.rs"]
pub mod set_page_media_box;

#[path = "🧩️set-pattern/🦀️.rs"]
pub mod set_pattern;

#[path = "🌄️remove-shading/🦀️.rs"]
pub mod remove_shading;

#[path = "🔄️set-page-rotation/🦀️.rs"]
pub mod set_page_rotation;

#[path = "🔀️move-page/🦀️.rs"]
pub mod move_page;

#[path = "🚪️set-open-action/🦀️.rs"]
pub mod set_open_action;

#[path = "🪡️remove-pattern/🦀️.rs"]
pub mod remove_pattern;

#[path = "🗑️remove-page/🦀️.rs"]
pub mod remove_page;

#[path = "📦️insert-object/🦀️.rs"]
pub mod insert_object;

#[path = "🛠️set-viewer-preferences/🦀️.rs"]
pub mod set_viewer_preferences;

#[path = "📏️set-page-user-unit/🦀️.rs"]
pub mod set_page_user_unit;

#[path = "🗣️set-language/🦀️.rs"]
pub mod set_language;

#[path = "✂️set-page-crop-box/🦀️.rs"]
pub mod set_page_crop_box;

#[path = "🏞️set-image/🦀️.rs"]
pub mod set_image;

#[path = "🎯️set-named-destination/🦀️.rs"]
pub mod set_named_destination;

#[path = "📄️set-form/🦀️.rs"]
pub mod set_form;

#[path = "🗂️set-catalog-entry/🦀️.rs"]
pub mod set_catalog_entry;

#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;

#[path = "🔏️set-mark-info/🦀️.rs"]
pub mod set_mark_info;

#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;

#[path = "🎚️remove-ext-g-state/🦀️.rs"]
pub mod remove_ext_g_state;

#[path = "📝️set-annotation/🦀️.rs"]
pub mod set_annotation;

#[path = "📖️set-page-layout/🦀️.rs"]
pub mod set_page_layout;

#[path = "🏳️set-output-intents/🦀️.rs"]
pub mod set_output_intents;

#[path = "📋️set-acro-form/🦀️.rs"]
pub mod set_acro_form;

#[path = "📍️remove-annotation/🦀️.rs"]
pub mod remove_annotation;

#[path = "📎️set-embedded-file/🦀️.rs"]
pub mod set_embedded_file;

#[path = "🧻️remove-content/🦀️.rs"]
pub mod remove_content;

#[path = "🧾️set-metadata/🦀️.rs"]
pub mod set_metadata;

#[path = "📌️insert-annotation/🦀️.rs"]
pub mod insert_annotation;

#[path = "🗃️remove-embedded-file/🦀️.rs"]
pub mod remove_embedded_file;

#[path = "🚫️remove-dict-entry/🦀️.rs"]
pub mod remove_dict_entry;

#[path = "🎛️set-ext-g-state/🦀️.rs"]
pub mod set_ext_g_state;

#[path = "🌅️set-shading/🦀️.rs"]
pub mod set_shading;

#[path = "➕️append-page-content/🦀️.rs"]
pub mod append_page_content;

#[path = "🗞️remove-form/🦀️.rs"]
pub mod remove_form;

#[path = "🔤️set-font/🦀️.rs"]
pub mod set_font;

#[path = "🧹️remove-object/🦀️.rs"]
pub mod remove_object;

#[path = "🏷️set-properties/🦀️.rs"]
pub mod set_properties;

#[path = "🧳️set-trailer-entry/🦀️.rs"]
pub mod set_trailer_entry;

#[path = "🎪️remove-named-destination/🦀️.rs"]
pub mod remove_named_destination;

#[path = "📑️set-outlines/🦀️.rs"]
pub mod set_outlines;

#[path = "🔑️set-dict-entry/🦀️.rs"]
pub mod set_dict_entry;

#[path = "🧺️remove-catalog-entry/🦀️.rs"]
pub mod remove_catalog_entry;

#[path = "🧽️remove-trailer-entry/🦀️.rs"]
pub mod remove_trailer_entry;

#[path = "📥️insert-page/🦀️.rs"]
pub mod insert_page;

#[path = "🔁️replace-content/🦀️.rs"]
pub mod replace_content;

#[path = "🔐️set-encryption/🦀️.rs"]
pub mod set_encryption;

#[path = "🔧️set-object-value/🦀️.rs"]
pub mod set_object_value;

#[path = "🌫️remove-image/🦀️.rs"]
pub mod remove_image;

#[path = "👁️set-optional-content/🦀️.rs"]
pub mod set_optional_content;

#[path = "🔢️set-page-labels/🦀️.rs"]
pub mod set_page_labels;

#[path = "🔖️remove-properties/🦀️.rs"]
pub mod remove_properties;

#[path = "✏️set-page-content/🦀️.rs"]
pub mod set_page_content;

#[path = "🖼️set-page-box/🦀️.rs"]
pub mod set_page_box;

#[path = "🅾️remove-font/🦀️.rs"]
pub mod remove_font;

#[path = "🎨️remove-color-space/🦀️.rs"]
pub mod remove_color_space;

#[path = "🖥️set-page-mode/🦀️.rs"]
pub mod set_page_mode;

#[path = "🖋️insert-content/🦀️.rs"]
pub mod insert_content;

#[path = "ℹ️set-info/🦀️.rs"]
pub mod set_info;

#[path = "🆔️set-document-id/🦀️.rs"]
pub mod set_document_id;

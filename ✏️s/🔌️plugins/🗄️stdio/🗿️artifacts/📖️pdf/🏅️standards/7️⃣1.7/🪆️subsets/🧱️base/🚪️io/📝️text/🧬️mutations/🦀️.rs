//! 📝️ Generic text framing and direct-owner registry for the visible PDF mutation aggregate.

use crate::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
use protocol::OpText;

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🧾️DerivedRegistry
/// 🧾️ Direct-owner text opcodes in aggregate declaration order.
pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[
    ("InsertPage", insert_page::TEXT_OPCODE),
    ("RemovePage", remove_page::TEXT_OPCODE),
    ("SetPageMediaBox", set_page_media_box::TEXT_OPCODE),
    ("SetPageCropBox", set_page_crop_box::TEXT_OPCODE),
    ("AppendPageContent", append_page_content::TEXT_OPCODE),
    ("SetInfo", set_info::TEXT_OPCODE),
    ("InsertObject", insert_object::TEXT_OPCODE),
    ("RemoveObject", remove_object::TEXT_OPCODE),
    ("SetObjectValue", set_object_value::TEXT_OPCODE),
    ("SetDictEntry", set_dict_entry::TEXT_OPCODE),
    ("RemoveDictEntry", remove_dict_entry::TEXT_OPCODE),
    ("SetTrailerEntry", set_trailer_entry::TEXT_OPCODE),
    ("RemoveTrailerEntry", remove_trailer_entry::TEXT_OPCODE),
    ("MovePage", move_page::TEXT_OPCODE),
    ("SetPageContent", set_page_content::TEXT_OPCODE),
    ("SetPageRotation", set_page_rotation::TEXT_OPCODE),
    ("SetPageBox", set_page_box::TEXT_OPCODE),
    ("SetPageUserUnit", set_page_user_unit::TEXT_OPCODE),
    ("InsertContent", insert_content::TEXT_OPCODE),
    ("RemoveContent", remove_content::TEXT_OPCODE),
    ("ReplaceContent", replace_content::TEXT_OPCODE),
    ("InsertAnnotation", insert_annotation::TEXT_OPCODE),
    ("RemoveAnnotation", remove_annotation::TEXT_OPCODE),
    ("SetAnnotation", set_annotation::TEXT_OPCODE),
    ("SetFont", set_font::TEXT_OPCODE),
    ("RemoveFont", remove_font::TEXT_OPCODE),
    ("SetImage", set_image::TEXT_OPCODE),
    ("RemoveImage", remove_image::TEXT_OPCODE),
    ("SetForm", set_form::TEXT_OPCODE),
    ("RemoveForm", remove_form::TEXT_OPCODE),
    ("SetExtGState", set_ext_g_state::TEXT_OPCODE),
    ("RemoveExtGState", remove_ext_g_state::TEXT_OPCODE),
    ("SetShading", set_shading::TEXT_OPCODE),
    ("RemoveShading", remove_shading::TEXT_OPCODE),
    ("SetPattern", set_pattern::TEXT_OPCODE),
    ("RemovePattern", remove_pattern::TEXT_OPCODE),
    ("SetColorSpace", set_color_space::TEXT_OPCODE),
    ("RemoveColorSpace", remove_color_space::TEXT_OPCODE),
    ("SetProperties", set_properties::TEXT_OPCODE),
    ("RemoveProperties", remove_properties::TEXT_OPCODE),
    ("SetEmbeddedFile", set_embedded_file::TEXT_OPCODE),
    ("RemoveEmbeddedFile", remove_embedded_file::TEXT_OPCODE),
    ("SetOutlines", set_outlines::TEXT_OPCODE),
    ("SetNamedDestination", set_named_destination::TEXT_OPCODE),
    ("RemoveNamedDestination", remove_named_destination::TEXT_OPCODE),
    ("SetPageLabels", set_page_labels::TEXT_OPCODE),
    ("SetOutputIntents", set_output_intents::TEXT_OPCODE),
    ("SetAcroForm", set_acro_form::TEXT_OPCODE),
    ("SetOptionalContent", set_optional_content::TEXT_OPCODE),
    ("SetPageLayout", set_page_layout::TEXT_OPCODE),
    ("SetPageMode", set_page_mode::TEXT_OPCODE),
    ("SetViewerPreferences", set_viewer_preferences::TEXT_OPCODE),
    ("SetOpenAction", set_open_action::TEXT_OPCODE),
    ("SetLanguage", set_language::TEXT_OPCODE),
    ("SetMarkInfo", set_mark_info::TEXT_OPCODE),
    ("SetMetadata", set_metadata::TEXT_OPCODE),
    ("SetDocumentId", set_document_id::TEXT_OPCODE),
    ("SetEncryption", set_encryption::TEXT_OPCODE),
    ("SetCatalogEntry", set_catalog_entry::TEXT_OPCODE),
    ("RemoveCatalogEntry", remove_catalog_entry::TEXT_OPCODE),
    ("ReplacePage", replace_page::TEXT_OPCODE),
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

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::text::mutations::set_color_space;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::text::mutations::set_page_media_box;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::text::mutations::set_pattern;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::text::mutations::remove_shading;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::text::mutations::set_page_rotation;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::text::mutations::move_page;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::text::mutations::set_open_action;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::text::mutations::remove_pattern;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::text::mutations::remove_page;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::text::mutations::insert_object;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::text::mutations::set_viewer_preferences;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::text::mutations::set_page_user_unit;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::text::mutations::set_language;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::text::mutations::set_page_crop_box;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::set_image;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::set_named_destination;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::set_form;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::set_catalog_entry;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::set_mark_info;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::remove_ext_g_state;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::set_annotation;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::set_page_layout;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::set_output_intents;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::set_acro_form;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::remove_annotation;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::set_embedded_file;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::remove_content;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::set_metadata;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::insert_annotation;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::remove_embedded_file;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::remove_dict_entry;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::set_ext_g_state;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::set_shading;

pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::base::io::text::mutations::append_page_content;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::remove_form;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::set_font;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::remove_object;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::set_properties;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::set_trailer_entry;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::remove_named_destination;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::set_outlines;

pub use semio_s_artifact_stdio_pdf_mut_base_7::standards::v1_7::subsets::base::io::text::mutations::set_dict_entry;

pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::base::io::text::mutations::remove_catalog_entry;

pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::base::io::text::mutations::remove_trailer_entry;

pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::base::io::text::mutations::insert_page;

pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::base::io::text::mutations::replace_content;

pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::base::io::text::mutations::set_encryption;

pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::base::io::text::mutations::set_object_value;

pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::base::io::text::mutations::remove_image;

pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::base::io::text::mutations::set_optional_content;

pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::base::io::text::mutations::set_page_labels;

pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::base::io::text::mutations::remove_properties;

pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::base::io::text::mutations::set_page_content;

pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::base::io::text::mutations::set_page_box;

pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::base::io::text::mutations::remove_font;

pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::base::io::text::mutations::remove_color_space;

pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::base::io::text::mutations::set_page_mode;

pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::base::io::text::mutations::insert_content;

pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::base::io::text::mutations::set_info;

pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::base::io::text::mutations::set_document_id;

pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::base::io::text::mutations::replace_page;

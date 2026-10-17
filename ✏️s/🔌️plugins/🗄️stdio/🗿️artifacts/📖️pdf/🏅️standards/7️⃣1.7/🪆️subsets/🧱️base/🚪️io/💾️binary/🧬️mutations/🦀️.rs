//! 💾️ Generic binary framing and direct-owner registry for the visible PDF mutation aggregate.

use crate::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
use protocol::OpBinary;

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

//#region 🧾️DerivedRegistry
/// 🧾️ Direct-owner identities and binary tags in aggregate declaration order.
pub const BINARY_TAG_REGISTRY: &[(&str, &str, u8)] = &[
    ("InsertPage", "insertPage", insert_page::BINARY_TAG),
    ("RemovePage", "removePage", remove_page::BINARY_TAG),
    ("SetPageMediaBox", "setPageMediaBox", set_page_media_box::BINARY_TAG),
    ("SetPageCropBox", "setPageCropBox", set_page_crop_box::BINARY_TAG),
    ("AppendPageContent", "appendPageContent", append_page_content::BINARY_TAG),
    ("SetInfo", "setInfo", set_info::BINARY_TAG),
    ("InsertObject", "insertObject", insert_object::BINARY_TAG),
    ("RemoveObject", "removeObject", remove_object::BINARY_TAG),
    ("SetObjectValue", "setObjectValue", set_object_value::BINARY_TAG),
    ("SetDictEntry", "setDictEntry", set_dict_entry::BINARY_TAG),
    ("RemoveDictEntry", "removeDictEntry", remove_dict_entry::BINARY_TAG),
    ("SetTrailerEntry", "setTrailerEntry", set_trailer_entry::BINARY_TAG),
    ("RemoveTrailerEntry", "removeTrailerEntry", remove_trailer_entry::BINARY_TAG),
    ("MovePage", "movePage", move_page::BINARY_TAG),
    ("SetPageContent", "setPageContent", set_page_content::BINARY_TAG),
    ("SetPageRotation", "setPageRotation", set_page_rotation::BINARY_TAG),
    ("SetPageBox", "setPageBox", set_page_box::BINARY_TAG),
    ("SetPageUserUnit", "setPageUserUnit", set_page_user_unit::BINARY_TAG),
    ("InsertContent", "insertContent", insert_content::BINARY_TAG),
    ("RemoveContent", "removeContent", remove_content::BINARY_TAG),
    ("ReplaceContent", "replaceContent", replace_content::BINARY_TAG),
    ("InsertAnnotation", "insertAnnotation", insert_annotation::BINARY_TAG),
    ("RemoveAnnotation", "removeAnnotation", remove_annotation::BINARY_TAG),
    ("SetAnnotation", "setAnnotation", set_annotation::BINARY_TAG),
    ("SetFont", "setFont", set_font::BINARY_TAG),
    ("RemoveFont", "removeFont", remove_font::BINARY_TAG),
    ("SetImage", "setImage", set_image::BINARY_TAG),
    ("RemoveImage", "removeImage", remove_image::BINARY_TAG),
    ("SetForm", "setForm", set_form::BINARY_TAG),
    ("RemoveForm", "removeForm", remove_form::BINARY_TAG),
    ("SetExtGState", "setExtGState", set_ext_g_state::BINARY_TAG),
    ("RemoveExtGState", "removeExtGState", remove_ext_g_state::BINARY_TAG),
    ("SetShading", "setShading", set_shading::BINARY_TAG),
    ("RemoveShading", "removeShading", remove_shading::BINARY_TAG),
    ("SetPattern", "setPattern", set_pattern::BINARY_TAG),
    ("RemovePattern", "removePattern", remove_pattern::BINARY_TAG),
    ("SetColorSpace", "setColorSpace", set_color_space::BINARY_TAG),
    ("RemoveColorSpace", "removeColorSpace", remove_color_space::BINARY_TAG),
    ("SetProperties", "setProperties", set_properties::BINARY_TAG),
    ("RemoveProperties", "removeProperties", remove_properties::BINARY_TAG),
    ("SetEmbeddedFile", "setEmbeddedFile", set_embedded_file::BINARY_TAG),
    ("RemoveEmbeddedFile", "removeEmbeddedFile", remove_embedded_file::BINARY_TAG),
    ("SetOutlines", "setOutlines", set_outlines::BINARY_TAG),
    ("SetNamedDestination", "setNamedDestination", set_named_destination::BINARY_TAG),
    ("RemoveNamedDestination", "removeNamedDestination", remove_named_destination::BINARY_TAG),
    ("SetPageLabels", "setPageLabels", set_page_labels::BINARY_TAG),
    ("SetOutputIntents", "setOutputIntents", set_output_intents::BINARY_TAG),
    ("SetAcroForm", "setAcroForm", set_acro_form::BINARY_TAG),
    ("SetOptionalContent", "setOptionalContent", set_optional_content::BINARY_TAG),
    ("SetPageLayout", "setPageLayout", set_page_layout::BINARY_TAG),
    ("SetPageMode", "setPageMode", set_page_mode::BINARY_TAG),
    ("SetViewerPreferences", "setViewerPreferences", set_viewer_preferences::BINARY_TAG),
    ("SetOpenAction", "setOpenAction", set_open_action::BINARY_TAG),
    ("SetLanguage", "setLanguage", set_language::BINARY_TAG),
    ("SetMarkInfo", "setMarkInfo", set_mark_info::BINARY_TAG),
    ("SetMetadata", "setMetadata", set_metadata::BINARY_TAG),
    ("SetDocumentId", "setDocumentId", set_document_id::BINARY_TAG),
    ("SetEncryption", "setEncryption", set_encryption::BINARY_TAG),
    ("SetCatalogEntry", "setCatalogEntry", set_catalog_entry::BINARY_TAG),
    ("RemoveCatalogEntry", "removeCatalogEntry", remove_catalog_entry::BINARY_TAG),
    ("ReplacePage", "replacePage", replace_page::BINARY_TAG),
];
//#endregion 🧾️DerivedRegistry

//#region 🧱️Framing
const MAX_PAYLOAD_BYTES: usize = 64 * 1024 * 1024;

fn malformed(offset: u64, detail: impl Into<String>) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "PDF mutation aggregate", offset, detail: detail.into() }
}

fn identity(payload: &[u8]) -> Result<String, protocol::ProtocolError> {
    let value = semio_framework_pack_json::parse_bytes(payload, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| malformed(2, error.to_string()))?;
    value.get("mutation").and_then(semio_framework_pack_json::Value::as_str).map(str::to_owned).ok_or_else(|| malformed(2, "missing mutation identity"))
}

impl OpBinary for PdfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let payload = semio_framework_pack_json::to_json_string(self).into_bytes();
        if payload.len() > MAX_PAYLOAD_BYTES {
            return Err(protocol::ProtocolError::LimitExceeded("PDF mutation payload"));
        }
        let identity = identity(&payload)?;
        let tag = BINARY_TAG_REGISTRY.iter().find(|(_, json, _)| *json == identity).map(|(_, _, tag)| *tag).ok_or_else(|| malformed(1, format!("unknown mutation identity {identity:?}")))?;
        let mut writer = dsl::ByteWriter::new();
        writer.write_u8(store::pack_rt::OP_BINARY_FORMAT);
        writer.write_u8(tag);
        writer.write_varint_u64(payload.len() as u64);
        writer.write_bytes(&payload);
        Ok(writer.into_bytes())
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = dsl::ByteReader::new(bytes);
        let format = reader.read_u8().map_err(|error| malformed(0, error.to_string()))?;
        if format != store::pack_rt::OP_BINARY_FORMAT {
            return Err(malformed(0, format!("expected format {}, got {format}", store::pack_rt::OP_BINARY_FORMAT)));
        }
        let tag = reader.read_u8().map_err(|error| malformed(1, error.to_string()))?;
        let (_, expected_identity, _) = BINARY_TAG_REGISTRY.iter().find(|(_, _, candidate)| *candidate == tag).ok_or_else(|| malformed(1, format!("unknown mutation tag {tag}")))?;
        let payload_len = usize::try_from(reader.read_varint_u64().map_err(|error| malformed(2, error.to_string()))?).map_err(|_| malformed(2, "payload length exceeds usize"))?;
        if payload_len > MAX_PAYLOAD_BYTES {
            return Err(protocol::ProtocolError::LimitExceeded("PDF mutation payload"));
        }
        let payload = reader.read_bytes(payload_len).map_err(|error| malformed(2, error.to_string()))?;
        if reader.remaining() != 0 {
            return Err(malformed((bytes.len() - reader.remaining()) as u64, "trailing bytes"));
        }
        let actual_identity = identity(payload)?;
        if actual_identity != *expected_identity {
            return Err(malformed(2, format!("tag {tag} declares {expected_identity}, payload declares {actual_identity}")));
        }
        let parsed = semio_framework_pack_json::parse_bytes(payload, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| malformed(2, error.to_string()))?;
        semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| malformed(2, error.to_string()))
    }
}
//#endregion 🧱️Framing

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_color_space;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_page_media_box;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_pattern;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::remove_shading;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_page_rotation;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::move_page;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_open_action;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::remove_pattern;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::remove_page;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::insert_object;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_viewer_preferences;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_page_user_unit;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_language;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_page_crop_box;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_image;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_named_destination;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_form;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_catalog_entry;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_mark_info;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::remove_ext_g_state;

pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::io::binary::mutations::set_annotation;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_page_layout;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_output_intents;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_acro_form;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::remove_annotation;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_embedded_file;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::remove_content;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_metadata;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::insert_annotation;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::remove_embedded_file;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::remove_dict_entry;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_ext_g_state;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_shading;

pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::base::io::binary::mutations::append_page_content;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::remove_form;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_font;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::remove_object;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_properties;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_trailer_entry;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::remove_named_destination;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_outlines;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_dict_entry;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::remove_catalog_entry;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::remove_trailer_entry;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::insert_page;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::replace_content;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::set_encryption;

pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::base::io::binary::mutations::set_object_value;

pub use semio_s_artifact_stdio_pdf_mut_base_5::standards::v1_7::subsets::base::io::binary::mutations::remove_image;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::set_optional_content;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::set_page_labels;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::remove_properties;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::set_page_content;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::set_page_box;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::remove_font;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::remove_color_space;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::set_page_mode;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::insert_content;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::set_info;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::set_document_id;

pub use semio_s_artifact_stdio_pdf_mut_base_6::standards::v1_7::subsets::base::io::binary::mutations::replace_page;

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

#[path = "🔏️set-mark-info/🦀️.rs"]
pub mod set_mark_info;

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

#[path = "🪄️replace-page/🦀️.rs"]
pub mod replace_page;

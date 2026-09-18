//! 💾️ Generic binary framing and direct-owner registry for the visible PDF mutation aggregate.

use super::PdfMutation;
use protocol::OpBinary;

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

//#region 🧾️DerivedRegistry
/// 🧾️ Direct-owner identities and binary tags in aggregate declaration order.
pub const BINARY_TAG_REGISTRY: &[(&str, &str, u8)] = &[
    ("InsertPage", "insertPage", super::insert_page::binary::BINARY_TAG),
    ("RemovePage", "removePage", super::remove_page::binary::BINARY_TAG),
    ("SetPageMediaBox", "setPageMediaBox", super::set_page_media_box::binary::BINARY_TAG),
    ("SetPageCropBox", "setPageCropBox", super::set_page_crop_box::binary::BINARY_TAG),
    ("AppendPageContent", "appendPageContent", super::append_page_content::binary::BINARY_TAG),
    ("SetInfo", "setInfo", super::set_info::binary::BINARY_TAG),
    ("InsertObject", "insertObject", super::insert_object::binary::BINARY_TAG),
    ("RemoveObject", "removeObject", super::remove_object::binary::BINARY_TAG),
    ("SetObjectValue", "setObjectValue", super::set_object_value::binary::BINARY_TAG),
    ("SetDictEntry", "setDictEntry", super::set_dict_entry::binary::BINARY_TAG),
    ("RemoveDictEntry", "removeDictEntry", super::remove_dict_entry::binary::BINARY_TAG),
    ("SetTrailerEntry", "setTrailerEntry", super::set_trailer_entry::binary::BINARY_TAG),
    ("RemoveTrailerEntry", "removeTrailerEntry", super::remove_trailer_entry::binary::BINARY_TAG),
    ("MovePage", "movePage", super::move_page::binary::BINARY_TAG),
    ("SetPageContent", "setPageContent", super::set_page_content::binary::BINARY_TAG),
    ("SetPageRotation", "setPageRotation", super::set_page_rotation::binary::BINARY_TAG),
    ("SetPageBox", "setPageBox", super::set_page_box::binary::BINARY_TAG),
    ("SetPageUserUnit", "setPageUserUnit", super::set_page_user_unit::binary::BINARY_TAG),
    ("InsertContent", "insertContent", super::insert_content::binary::BINARY_TAG),
    ("RemoveContent", "removeContent", super::remove_content::binary::BINARY_TAG),
    ("ReplaceContent", "replaceContent", super::replace_content::binary::BINARY_TAG),
    ("InsertAnnotation", "insertAnnotation", super::insert_annotation::binary::BINARY_TAG),
    ("RemoveAnnotation", "removeAnnotation", super::remove_annotation::binary::BINARY_TAG),
    ("SetAnnotation", "setAnnotation", super::set_annotation::binary::BINARY_TAG),
    ("SetFont", "setFont", super::set_font::binary::BINARY_TAG),
    ("RemoveFont", "removeFont", super::remove_font::binary::BINARY_TAG),
    ("SetImage", "setImage", super::set_image::binary::BINARY_TAG),
    ("RemoveImage", "removeImage", super::remove_image::binary::BINARY_TAG),
    ("SetForm", "setForm", super::set_form::binary::BINARY_TAG),
    ("RemoveForm", "removeForm", super::remove_form::binary::BINARY_TAG),
    ("SetExtGState", "setExtGState", super::set_ext_g_state::binary::BINARY_TAG),
    ("RemoveExtGState", "removeExtGState", super::remove_ext_g_state::binary::BINARY_TAG),
    ("SetShading", "setShading", super::set_shading::binary::BINARY_TAG),
    ("RemoveShading", "removeShading", super::remove_shading::binary::BINARY_TAG),
    ("SetPattern", "setPattern", super::set_pattern::binary::BINARY_TAG),
    ("RemovePattern", "removePattern", super::remove_pattern::binary::BINARY_TAG),
    ("SetColorSpace", "setColorSpace", super::set_color_space::binary::BINARY_TAG),
    ("RemoveColorSpace", "removeColorSpace", super::remove_color_space::binary::BINARY_TAG),
    ("SetProperties", "setProperties", super::set_properties::binary::BINARY_TAG),
    ("RemoveProperties", "removeProperties", super::remove_properties::binary::BINARY_TAG),
    ("SetEmbeddedFile", "setEmbeddedFile", super::set_embedded_file::binary::BINARY_TAG),
    ("RemoveEmbeddedFile", "removeEmbeddedFile", super::remove_embedded_file::binary::BINARY_TAG),
    ("SetOutlines", "setOutlines", super::set_outlines::binary::BINARY_TAG),
    ("SetNamedDestination", "setNamedDestination", super::set_named_destination::binary::BINARY_TAG),
    ("RemoveNamedDestination", "removeNamedDestination", super::remove_named_destination::binary::BINARY_TAG),
    ("SetPageLabels", "setPageLabels", super::set_page_labels::binary::BINARY_TAG),
    ("SetOutputIntents", "setOutputIntents", super::set_output_intents::binary::BINARY_TAG),
    ("SetAcroForm", "setAcroForm", super::set_acro_form::binary::BINARY_TAG),
    ("SetOptionalContent", "setOptionalContent", super::set_optional_content::binary::BINARY_TAG),
    ("SetPageLayout", "setPageLayout", super::set_page_layout::binary::BINARY_TAG),
    ("SetPageMode", "setPageMode", super::set_page_mode::binary::BINARY_TAG),
    ("SetViewerPreferences", "setViewerPreferences", super::set_viewer_preferences::binary::BINARY_TAG),
    ("SetOpenAction", "setOpenAction", super::set_open_action::binary::BINARY_TAG),
    ("SetLanguage", "setLanguage", super::set_language::binary::BINARY_TAG),
    ("SetMarkInfo", "setMarkInfo", super::set_mark_info::binary::BINARY_TAG),
    ("SetMetadata", "setMetadata", super::set_metadata::binary::BINARY_TAG),
    ("SetDocumentId", "setDocumentId", super::set_document_id::binary::BINARY_TAG),
    ("SetEncryption", "setEncryption", super::set_encryption::binary::BINARY_TAG),
    ("SetCatalogEntry", "setCatalogEntry", super::set_catalog_entry::binary::BINARY_TAG),
    ("RemoveCatalogEntry", "removeCatalogEntry", super::remove_catalog_entry::binary::BINARY_TAG),
];
//#endregion 🧾️DerivedRegistry

//#region 🧱️Framing
const MAX_PAYLOAD_BYTES: usize = 64 * 1024 * 1024;

fn malformed(offset: u64, detail: impl Into<String>) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "PDF mutation aggregate", offset, detail: detail.into() }
}

fn identity(payload: &[u8]) -> Result<String, protocol::ProtocolError> {
    let value = pack::parse_json_bytes(payload).map_err(|error| malformed(2, error.to_string()))?;
    value.get("mutation").and_then(pack::JsonValue::as_str).map(str::to_owned).ok_or_else(|| malformed(2, "missing mutation identity"))
}

impl OpBinary for PdfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let payload = pack::to_json_string(self).into_bytes();
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
        let parsed = pack::parse_json_bytes(payload).map_err(|error| malformed(2, error.to_string()))?;
        dsl::FromValue::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| malformed(2, error.to_string()))
    }
}
//#endregion 🧱️Framing

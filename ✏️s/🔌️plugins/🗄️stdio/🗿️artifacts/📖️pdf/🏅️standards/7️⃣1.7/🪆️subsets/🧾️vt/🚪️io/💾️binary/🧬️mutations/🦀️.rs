//! 💾️ Generic binary framing and direct-owner registry for the visible PDF/VT mutation aggregate.

use crate::standards::v1_7::subsets::vt::schema::mutations::PdfVtMutation;
use protocol::OpBinary;

//#region 🧾️DerivedRegistry
pub const BINARY_TAG_REGISTRY: &[(&str, &str, u8)] = &[
    ("InsertEncryptionDictionary", "insertEncryptionDictionary", insert_encryption_dictionary::BINARY_TAG),
    ("RemoveEncryptionDictionary", "removeEncryptionDictionary", remove_encryption_dictionary::BINARY_TAG),
    ("SetOutputIntent", "setOutputIntent", set_output_intent::BINARY_TAG),
    ("RemoveOutputIntent", "removeOutputIntent", remove_output_intent::BINARY_TAG),
    ("SetTrimBox", "setTrimBox", set_trim_box::BINARY_TAG),
    ("RemoveTrimBox", "removeTrimBox", remove_trim_box::BINARY_TAG),
    ("EmbedFontFile", "embedFontFile", embed_font_file::BINARY_TAG),
    ("RemoveFontFile", "removeFontFile", remove_font_file::BINARY_TAG),
    ("InsertJavascriptAction", "insertJavascriptAction", insert_javascript_action::BINARY_TAG),
    ("RemoveJavascriptAction", "removeJavascriptAction", remove_javascript_action::BINARY_TAG),
    ("InsertLaunchAction", "insertLaunchAction", insert_launch_action::BINARY_TAG),
    ("RemoveLaunchAction", "removeLaunchAction", remove_launch_action::BINARY_TAG),
    ("InsertMediaAnnotation", "insertMediaAnnotation", insert_media_annotation::BINARY_TAG),
    ("RemoveMediaAnnotation", "removeMediaAnnotation", remove_media_annotation::BINARY_TAG),
    ("SetDpartRoot", "setDpartRoot", set_dpart_root::BINARY_TAG),
    ("RemoveDpartRoot", "removeDpartRoot", remove_dpart_root::BINARY_TAG),
    ("SetDpartMetadata", "setDpartMetadata", set_dpart_metadata::BINARY_TAG),
    ("RemoveDpartMetadata", "removeDpartMetadata", remove_dpart_metadata::BINARY_TAG),
];
//#endregion 🧾️DerivedRegistry

//#region 🧱️Framing
const MAX_PAYLOAD_BYTES: usize = 256 * 1024;

fn malformed(offset: u64, detail: impl Into<String>) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "PDF/VT mutation aggregate", offset, detail: detail.into() }
}

fn identity(payload: &[u8]) -> Result<String, protocol::ProtocolError> {
    let value = semio_framework_pack_json::parse_bytes(payload, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| malformed(2, error.to_string()))?;
    value.get("mutation").and_then(semio_framework_pack_json::Value::as_str).map(str::to_owned).ok_or_else(|| malformed(2, "missing mutation identity"))
}

impl OpBinary for PdfVtMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let payload = semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(self))).into_bytes();
        if payload.len() > MAX_PAYLOAD_BYTES {
            return Err(protocol::ProtocolError::LimitExceeded("PDF/VT mutation payload"));
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
            return Err(protocol::ProtocolError::LimitExceeded("PDF/VT mutation payload"));
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
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| malformed(2, error.to_string()))
    }
}
//#endregion 🧱️Framing

//#region 🧪️Tests

//#endregion 🧪️Tests

pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::vt::io::binary::mutations::set_output_intent;

pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::vt::io::binary::mutations::remove_dpart_root;

pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::vt::io::binary::mutations::remove_media_annotation;

pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::vt::io::binary::mutations::remove_launch_action;

pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::vt::io::binary::mutations::remove_dpart_metadata;

pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::vt::io::binary::mutations::insert_media_annotation;

pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::vt::io::binary::mutations::remove_trim_box;

pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::vt::io::binary::mutations::set_dpart_metadata;

pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::binary::mutations::remove_javascript_action;

pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::binary::mutations::insert_encryption_dictionary;

pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::binary::mutations::remove_font_file;

pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::binary::mutations::insert_javascript_action;

pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::binary::mutations::remove_encryption_dictionary;

pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::binary::mutations::set_trim_box;

pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::binary::mutations::embed_font_file;

pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::binary::mutations::remove_output_intent;

pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::binary::mutations::insert_launch_action;

pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::binary::mutations::set_dpart_root;

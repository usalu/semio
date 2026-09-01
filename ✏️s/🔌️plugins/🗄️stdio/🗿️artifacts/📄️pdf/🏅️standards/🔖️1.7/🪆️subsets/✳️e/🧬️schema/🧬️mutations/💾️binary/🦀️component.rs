//! 💾️ Generic binary framing and direct-owner registry for the visible PDF/E mutation aggregate.

use super::PdfEMutation;
use protocol::OpBinary;

//#region 🧾️DerivedRegistry
/// 🧾️ Direct-owner identities and binary tags in aggregate declaration order.
pub const BINARY_TAG_REGISTRY: &[(&str, &str, u8)] = &[
    ("InsertEncryptionDictionary", "insertEncryptionDictionary", super::insert_encryption_dictionary::binary::BINARY_TAG),
    ("RemoveEncryptionDictionary", "removeEncryptionDictionary", super::remove_encryption_dictionary::binary::BINARY_TAG),
    ("InsertJavascriptAction", "insertJavascriptAction", super::insert_javascript_action::binary::BINARY_TAG),
    ("RemoveJavascriptAction", "removeJavascriptAction", super::remove_javascript_action::binary::BINARY_TAG),
    ("InsertLaunchAction", "insertLaunchAction", super::insert_launch_action::binary::BINARY_TAG),
    ("RemoveLaunchAction", "removeLaunchAction", super::remove_launch_action::binary::BINARY_TAG),
    ("InsertMediaAnnotation", "insertMediaAnnotation", super::insert_media_annotation::binary::BINARY_TAG),
    ("RemoveMediaAnnotation", "removeMediaAnnotation", super::remove_media_annotation::binary::BINARY_TAG),
    ("SetOutputIntent", "setOutputIntent", super::set_output_intent::binary::BINARY_TAG),
    ("RemoveOutputIntent", "removeOutputIntent", super::remove_output_intent::binary::BINARY_TAG),
    ("EmbedFontFile", "embedFontFile", super::embed_font_file::binary::BINARY_TAG),
    ("RemoveFontFile", "removeFontFile", super::remove_font_file::binary::BINARY_TAG),
];
//#endregion 🧾️DerivedRegistry

//#region 🧱️Framing
const MAX_PAYLOAD_BYTES: usize = 256 * 1024;

fn malformed(offset: u64, detail: impl Into<String>) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "PDF/E mutation aggregate", offset, detail: detail.into() }
}

fn identity(payload: &[u8]) -> Result<String, protocol::ProtocolError> {
    let value = pack::parse_json_bytes(payload).map_err(|error| malformed(2, error.to_string()))?;
    value.get("mutation").and_then(pack::JsonValue::as_str).map(str::to_owned).ok_or_else(|| malformed(2, "missing mutation identity"))
}

impl OpBinary for PdfEMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let payload = pack::to_json_string(self).into_bytes();
        if payload.len() > MAX_PAYLOAD_BYTES {
            return Err(protocol::ProtocolError::LimitExceeded("PDF/E mutation payload"));
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
            return Err(protocol::ProtocolError::LimitExceeded("PDF/E mutation payload"));
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

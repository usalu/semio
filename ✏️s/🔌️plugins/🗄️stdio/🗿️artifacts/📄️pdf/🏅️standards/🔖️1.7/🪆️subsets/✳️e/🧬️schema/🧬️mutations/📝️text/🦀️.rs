//! 📝️ Generic text framing and direct-owner registry for the visible PDF/E mutation aggregate.

use super::PdfEMutation;
use protocol::OpText;

//#region 🧾️DerivedRegistry
/// 🧾️ Direct-owner text opcodes in aggregate declaration order.
pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[
    ("InsertEncryptionDictionary", super::insert_encryption_dictionary::text::TEXT_OPCODE),
    ("RemoveEncryptionDictionary", super::remove_encryption_dictionary::text::TEXT_OPCODE),
    ("InsertJavascriptAction", super::insert_javascript_action::text::TEXT_OPCODE),
    ("RemoveJavascriptAction", super::remove_javascript_action::text::TEXT_OPCODE),
    ("InsertLaunchAction", super::insert_launch_action::text::TEXT_OPCODE),
    ("RemoveLaunchAction", super::remove_launch_action::text::TEXT_OPCODE),
    ("InsertMediaAnnotation", super::insert_media_annotation::text::TEXT_OPCODE),
    ("RemoveMediaAnnotation", super::remove_media_annotation::text::TEXT_OPCODE),
    ("SetOutputIntent", super::set_output_intent::text::TEXT_OPCODE),
    ("RemoveOutputIntent", super::remove_output_intent::text::TEXT_OPCODE),
    ("EmbedFontFile", super::embed_font_file::text::TEXT_OPCODE),
    ("RemoveFontFile", super::remove_font_file::text::TEXT_OPCODE),
];
//#endregion 🧾️DerivedRegistry

//#region 🧱️Framing
const MAX_PAYLOAD_BYTES: usize = 256 * 1024;

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
    if value.len() % 2 != 0 || value.len() > MAX_PAYLOAD_BYTES * 2 {
        return Err("PDF/E mutation text payload exceeds its budget".into());
    }
    fn nibble(value: u8) -> Option<u8> {
        if value.is_ascii_digit() {
            return Some(value - b'0');
        }
        (b'a'..=b'f').contains(&value).then_some(value - b'a' + 10)
    }
    value.as_bytes().chunks_exact(2).map(|pair| {
        let high = nibble(pair[0]).ok_or_else(|| "PDF/E mutation payload must be lowercase hexadecimal".to_string())?;
        let low = nibble(pair[1]).ok_or_else(|| "PDF/E mutation payload must be lowercase hexadecimal".to_string())?;
        Ok((high << 4) | low)
    }).collect()
}

fn text_error(detail: impl Into<String>) -> store::TextError {
    store::TextError::new(detail.into(), dsl::TextSpan::at(1, 1))
}

impl OpText for PdfEMutation {
    fn print_op(&self) -> String {
        let payload = pack::to_json_string(self).into_bytes();
        format!("pdf-e-mutation payload={}", encode_hex(&payload))
    }

    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let payload = line.strip_prefix("pdf-e-mutation payload=").ok_or_else(|| text_error("expected canonical PDF/E mutation aggregate"))?;
        let bytes = decode_hex(payload).map_err(text_error)?;
        let parsed = pack::parse_json_bytes(&bytes).map_err(|error| text_error(error.to_string()))?;
        dsl::FromValue::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| text_error(error.to_string()))
    }
}
//#endregion 🧱️Framing

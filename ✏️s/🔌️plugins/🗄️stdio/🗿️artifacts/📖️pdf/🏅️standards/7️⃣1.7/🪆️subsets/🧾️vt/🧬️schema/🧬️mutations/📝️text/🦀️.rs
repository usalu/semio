//! 📝️ Generic text framing and direct-owner registry for the visible PDF/VT mutation aggregate.

use super::PdfVtMutation;
use protocol::OpText;

//#region 🧾️DerivedRegistry
pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[
    ("InsertEncryptionDictionary", super::insert_encryption_dictionary::text::TEXT_OPCODE),
    ("RemoveEncryptionDictionary", super::remove_encryption_dictionary::text::TEXT_OPCODE),
    ("SetOutputIntent", super::set_output_intent::text::TEXT_OPCODE),
    ("RemoveOutputIntent", super::remove_output_intent::text::TEXT_OPCODE),
    ("SetTrimBox", super::set_trim_box::text::TEXT_OPCODE),
    ("RemoveTrimBox", super::remove_trim_box::text::TEXT_OPCODE),
    ("EmbedFontFile", super::embed_font_file::text::TEXT_OPCODE),
    ("RemoveFontFile", super::remove_font_file::text::TEXT_OPCODE),
    ("InsertJavascriptAction", super::insert_javascript_action::text::TEXT_OPCODE),
    ("RemoveJavascriptAction", super::remove_javascript_action::text::TEXT_OPCODE),
    ("InsertLaunchAction", super::insert_launch_action::text::TEXT_OPCODE),
    ("RemoveLaunchAction", super::remove_launch_action::text::TEXT_OPCODE),
    ("InsertMediaAnnotation", super::insert_media_annotation::text::TEXT_OPCODE),
    ("RemoveMediaAnnotation", super::remove_media_annotation::text::TEXT_OPCODE),
    ("SetDpartRoot", super::set_dpart_root::text::TEXT_OPCODE),
    ("RemoveDpartRoot", super::remove_dpart_root::text::TEXT_OPCODE),
    ("SetDpartMetadata", super::set_dpart_metadata::text::TEXT_OPCODE),
    ("RemoveDpartMetadata", super::remove_dpart_metadata::text::TEXT_OPCODE),
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
    if !value.len().is_multiple_of(2) || value.len() > MAX_PAYLOAD_BYTES * 2 {
        return Err("PDF/VT mutation text payload exceeds its budget".into());
    }
    fn nibble(value: u8) -> Option<u8> {
        if value.is_ascii_digit() {
            return Some(value - b'0');
        }
        if (b'a'..=b'f').contains(&value) {
            Some(value - b'a' + 10)
        } else {
            None
        }
    }
    value
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            let high = nibble(pair[0]).ok_or_else(|| "PDF/VT mutation payload must be lowercase hexadecimal".to_string())?;
            let low = nibble(pair[1]).ok_or_else(|| "PDF/VT mutation payload must be lowercase hexadecimal".to_string())?;
            Ok((high << 4) | low)
        })
        .collect()
}

fn text_error(detail: impl Into<String>) -> semio_framework_diagnostic::TextError {
    semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail.into(), semio_framework_diagnostic::TextSpan::at(1, 1))
}

impl OpText for PdfVtMutation {
    fn print_op(&self) -> String {
        let payload = semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(self))).into_bytes();
        format!("pdf-vt-mutation payload={}", encode_hex(&payload))
    }

    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let payload = line.strip_prefix("pdf-vt-mutation payload=").ok_or_else(|| text_error("expected canonical PDF/VT mutation aggregate"))?;
        let bytes = decode_hex(payload).map_err(text_error)?;
        let parsed = semio_framework_pack_json::parse_bytes(&bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| text_error(error.to_string()))?;
        <PdfVtMutation as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| text_error(error.to_string()))
    }
}
//#endregion 🧱️Framing

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

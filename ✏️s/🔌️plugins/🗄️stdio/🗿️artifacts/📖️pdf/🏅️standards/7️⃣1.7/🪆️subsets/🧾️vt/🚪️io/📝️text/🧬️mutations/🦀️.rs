//! 📝️ Generic text framing and direct-owner registry for the visible PDF/VT mutation aggregate.

use crate::standards::v1_7::subsets::vt::schema::mutations::PdfVtMutation;
use protocol::OpText;

//#region 🧾️DerivedRegistry
pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[
    ("InsertEncryptionDictionary", self::insert_encryption_dictionary::TEXT_OPCODE),
    ("RemoveEncryptionDictionary", self::remove_encryption_dictionary::TEXT_OPCODE),
    ("SetOutputIntent", self::set_output_intent::TEXT_OPCODE),
    ("RemoveOutputIntent", self::remove_output_intent::TEXT_OPCODE),
    ("SetTrimBox", self::set_trim_box::TEXT_OPCODE),
    ("RemoveTrimBox", self::remove_trim_box::TEXT_OPCODE),
    ("EmbedFontFile", self::embed_font_file::TEXT_OPCODE),
    ("RemoveFontFile", self::remove_font_file::TEXT_OPCODE),
    ("InsertJavascriptAction", self::insert_javascript_action::TEXT_OPCODE),
    ("RemoveJavascriptAction", self::remove_javascript_action::TEXT_OPCODE),
    ("InsertLaunchAction", self::insert_launch_action::TEXT_OPCODE),
    ("RemoveLaunchAction", self::remove_launch_action::TEXT_OPCODE),
    ("InsertMediaAnnotation", self::insert_media_annotation::TEXT_OPCODE),
    ("RemoveMediaAnnotation", self::remove_media_annotation::TEXT_OPCODE),
    ("SetDpartRoot", self::set_dpart_root::TEXT_OPCODE),
    ("RemoveDpartRoot", self::remove_dpart_root::TEXT_OPCODE),
    ("SetDpartMetadata", self::set_dpart_metadata::TEXT_OPCODE),
    ("RemoveDpartMetadata", self::remove_dpart_metadata::TEXT_OPCODE),
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

#[path = "🏳️set-output-intent/🦀️.rs"]
pub mod set_output_intent;

#[path = "🧹️remove-dpart-root/🦀️.rs"]
pub mod remove_dpart_root;

#[path = "⏹️remove-media-annotation/🦀️.rs"]
pub mod remove_media_annotation;

#[path = "🛬️remove-launch-action/🦀️.rs"]
pub mod remove_launch_action;

#[path = "🗑️remove-dpart-metadata/🦀️.rs"]
pub mod remove_dpart_metadata;

#[path = "🎬️insert-media-annotation/🦀️.rs"]
pub mod insert_media_annotation;

#[path = "✂️remove-trim-box/🦀️.rs"]
pub mod remove_trim_box;

#[path = "🏷️set-dpart-metadata/🦀️.rs"]
pub mod set_dpart_metadata;

#[path = "🚫️remove-javascript-action/🦀️.rs"]
pub mod remove_javascript_action;

#[path = "🔒️insert-encryption-dictionary/🦀️.rs"]
pub mod insert_encryption_dictionary;

#[path = "🧺️remove-font-file/🦀️.rs"]
pub mod remove_font_file;

#[path = "📜️insert-javascript-action/🦀️.rs"]
pub mod insert_javascript_action;

#[path = "🔓️remove-encryption-dictionary/🦀️.rs"]
pub mod remove_encryption_dictionary;

#[path = "📐️set-trim-box/🦀️.rs"]
pub mod set_trim_box;

#[path = "🔤️embed-font-file/🦀️.rs"]
pub mod embed_font_file;

#[path = "🧽️remove-output-intent/🦀️.rs"]
pub mod remove_output_intent;

#[path = "🚀️insert-launch-action/🦀️.rs"]
pub mod insert_launch_action;

#[path = "🗂️set-dpart-root/🦀️.rs"]
pub mod set_dpart_root;

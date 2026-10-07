//! 📝️ Generic text framing and direct-owner registry for the visible PDF/UA mutation aggregate.

use crate::standards::v1_7::subsets::ua::schema::mutations::PdfUaMutation;
use protocol::OpText;

//#region 🧾️DerivedRegistry
pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[
    ("SetMarkInfo", self::set_mark_info::TEXT_OPCODE),
    ("RemoveMarkInfo", self::remove_mark_info::TEXT_OPCODE),
    ("SetStructTreeRoot", self::set_struct_tree_root::TEXT_OPCODE),
    ("RemoveStructTreeRoot", self::remove_struct_tree_root::TEXT_OPCODE),
    ("SetLang", self::set_lang::TEXT_OPCODE),
    ("RemoveLang", self::remove_lang::TEXT_OPCODE),
    ("SetDisplayDocTitle", self::set_display_doc_title::TEXT_OPCODE),
    ("RemoveDisplayDocTitle", self::remove_display_doc_title::TEXT_OPCODE),
    ("SetInfoTitle", self::set_info_title::TEXT_OPCODE),
    ("EmbedFontFile", self::embed_font_file::TEXT_OPCODE),
    ("RemoveFontFile", self::remove_font_file::TEXT_OPCODE),
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
        return Err("PDF/UA mutation text payload exceeds its budget".into());
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
            let high = nibble(pair[0]).ok_or_else(|| "PDF/UA mutation payload must be lowercase hexadecimal".to_string())?;
            let low = nibble(pair[1]).ok_or_else(|| "PDF/UA mutation payload must be lowercase hexadecimal".to_string())?;
            Ok((high << 4) | low)
        })
        .collect()
}

fn text_error(detail: impl Into<String>) -> semio_framework_diagnostic::TextError {
    semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail.into(), semio_framework_diagnostic::TextSpan::at(1, 1))
}

impl OpText for PdfUaMutation {
    fn print_op(&self) -> String {
        let payload = semio_framework_pack_json::to_json_string(self).into_bytes();
        format!("pdf-ua-mutation payload={}", encode_hex(&payload))
    }

    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let payload = line.strip_prefix("pdf-ua-mutation payload=").ok_or_else(|| text_error("expected canonical PDF/UA mutation aggregate"))?;
        let bytes = decode_hex(payload).map_err(text_error)?;
        let parsed = semio_framework_pack_json::parse_bytes(&bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| text_error(error.to_string()))?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| text_error(error.to_string()))
    }
}
//#endregion 🧱️Framing

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "🪓️remove-struct-tree-root/🦀️.rs"]
pub mod remove_struct_tree_root;

#[path = "🚫️remove-display-doc-title/🦀️.rs"]
pub mod remove_display_doc_title;

#[path = "📰️set-info-title/🦀️.rs"]
pub mod set_info_title;

#[path = "🗑️remove-mark-info/🦀️.rs"]
pub mod remove_mark_info;

#[path = "🗣️set-lang/🦀️.rs"]
pub mod set_lang;

#[path = "🧺️remove-font-file/🦀️.rs"]
pub mod remove_font_file;

#[path = "🤐️remove-lang/🦀️.rs"]
pub mod remove_lang;

#[path = "✅️set-mark-info/🦀️.rs"]
pub mod set_mark_info;

#[path = "🔤️embed-font-file/🦀️.rs"]
pub mod embed_font_file;

#[path = "🌲️set-struct-tree-root/🦀️.rs"]
pub mod set_struct_tree_root;

#[path = "🪧️set-display-doc-title/🦀️.rs"]
pub mod set_display_doc_title;

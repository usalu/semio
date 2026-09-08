//! 📝️ Generic text framing and direct-owner registry for the visible PDF mutation aggregate.

use super::PdfMutation;
use protocol::OpText;

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🧾️DerivedRegistry
/// 🧾️ Direct-owner text opcodes in aggregate declaration order.
pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[
    ("InsertPage", super::insert_page::text::TEXT_OPCODE),
    ("RemovePage", super::remove_page::text::TEXT_OPCODE),
    ("SetPageMediaBox", super::set_page_media_box::text::TEXT_OPCODE),
    ("SetPageCropBox", super::set_page_crop_box::text::TEXT_OPCODE),
    ("AppendPageContent", super::append_page_content::text::TEXT_OPCODE),
    ("SetInfo", super::set_info::text::TEXT_OPCODE),
    ("InsertObject", super::insert_object::text::TEXT_OPCODE),
    ("RemoveObject", super::remove_object::text::TEXT_OPCODE),
    ("SetObjectValue", super::set_object_value::text::TEXT_OPCODE),
    ("SetDictEntry", super::set_dict_entry::text::TEXT_OPCODE),
    ("RemoveDictEntry", super::remove_dict_entry::text::TEXT_OPCODE),
    ("SetTrailerEntry", super::set_trailer_entry::text::TEXT_OPCODE),
    ("RemoveTrailerEntry", super::remove_trailer_entry::text::TEXT_OPCODE),
    ("MovePage", super::move_page::text::TEXT_OPCODE),
    ("SetPageContent", super::set_page_content::text::TEXT_OPCODE),
    ("SetPageRotation", super::set_page_rotation::text::TEXT_OPCODE),
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
        .chunks_exact(2)
        .map(|pair| {
            let high = nibble(pair[0]).ok_or_else(|| "PDF mutation payload must be lowercase hexadecimal".to_string())?;
            let low = nibble(pair[1]).ok_or_else(|| "PDF mutation payload must be lowercase hexadecimal".to_string())?;
            Ok((high << 4) | low)
        })
        .collect()
}

fn text_error(detail: impl Into<String>) -> store::TextError {
    store::TextError::new(detail.into(), dsl::TextSpan::at(1, 1))
}

impl OpText for PdfMutation {
    fn print_op(&self) -> String {
        let payload = pack::to_json_string(self).into_bytes();
        format!("pdf-mutation payload={}", encode_hex(&payload))
    }

    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let payload = line.strip_prefix("pdf-mutation payload=").ok_or_else(|| text_error("expected canonical PDF mutation aggregate"))?;
        let bytes = decode_hex(payload).map_err(text_error)?;
        let parsed = pack::parse_json_bytes(&bytes).map_err(|error| text_error(error.to_string()))?;
        dsl::FromValue::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| text_error(error.to_string()))
    }
}
//#endregion 🧱️Framing

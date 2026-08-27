//! 📝️ Generic text framing and direct-owner registry for the visible PDF/UA mutation aggregate.

use super::PdfUaMutation;
use protocol::OpText;

//#region 🧾️DerivedRegistry
pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[
    ("SetMarkInfo", super::set_mark_info::text::TEXT_OPCODE),
    ("RemoveMarkInfo", super::remove_mark_info::text::TEXT_OPCODE),
    ("SetStructTreeRoot", super::set_struct_tree_root::text::TEXT_OPCODE),
    ("RemoveStructTreeRoot", super::remove_struct_tree_root::text::TEXT_OPCODE),
    ("SetLang", super::set_lang::text::TEXT_OPCODE),
    ("RemoveLang", super::remove_lang::text::TEXT_OPCODE),
    ("SetDisplayDocTitle", super::set_display_doc_title::text::TEXT_OPCODE),
    ("RemoveDisplayDocTitle", super::remove_display_doc_title::text::TEXT_OPCODE),
    ("SetInfoTitle", super::set_info_title::text::TEXT_OPCODE),
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
        return Err("PDF/UA mutation text payload exceeds its budget".into());
    }
    fn nibble(value: u8) -> Option<u8> {
        if value.is_ascii_digit() { return Some(value - b'0'); }
        if (b'a'..=b'f').contains(&value) { Some(value - b'a' + 10) } else { None }
    }
    value.as_bytes().chunks_exact(2).map(|pair| {
        let high = nibble(pair[0]).ok_or_else(|| "PDF/UA mutation payload must be lowercase hexadecimal".to_string())?;
        let low = nibble(pair[1]).ok_or_else(|| "PDF/UA mutation payload must be lowercase hexadecimal".to_string())?;
        Ok((high << 4) | low)
    }).collect()
}

fn text_error(detail: impl Into<String>) -> store::TextError {
    store::TextError::new(detail.into(), dsl::TextSpan::at(1, 1))
}

impl OpText for PdfUaMutation {
    fn print_op(&self) -> String {
        let payload = serde_json::to_vec(self).expect("PdfUaMutation serialization is infallible");
        format!("pdf-ua-mutation payload={}", encode_hex(&payload))
    }

    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let payload = line.strip_prefix("pdf-ua-mutation payload=").ok_or_else(|| text_error("expected canonical PDF/UA mutation aggregate"))?;
        let bytes = decode_hex(payload).map_err(text_error)?;
        serde_json::from_slice(&bytes).map_err(|error| text_error(error.to_string()))
    }
}
//#endregion 🧱️Framing

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use super::super::SetLang;

    #[test]
    fn framed_payload_round_trips_and_rejects_non_hex() {
        let mutation = PdfUaMutation::SetLang(SetLang { lang: "de-DE".to_string() });
        assert_eq!(PdfUaMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert!(PdfUaMutation::parse_op("pdf-ua-mutation payload=!!").is_err());
    }
}
//#endregion 🧪️Tests

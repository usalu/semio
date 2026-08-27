//! 📝️ Generic framing and descriptor roster for the transparent TxtMutation.
//#region 🔖️Registry
use crate::artifacts::txt::schema::mutations::{TxtMutation, insert_line, remove_line, set_line, set_line_ending, set_trailing_newline};
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
struct TextCodec {
    opcode: &'static str,
    try_encode: fn(&TxtMutation) -> Option<Result<String, String>>,
    decode: fn(&str) -> Result<TxtMutation, String>,
}
const TEXT_CODECS: &[TextCodec] = &[
    TextCodec { opcode: set_trailing_newline::text::TEXT_OPCODE, try_encode: set_trailing_newline::text::try_encode, decode: set_trailing_newline::text::decode_mutation },
    TextCodec { opcode: set_line_ending::text::TEXT_OPCODE, try_encode: set_line_ending::text::try_encode, decode: set_line_ending::text::decode_mutation },
    TextCodec { opcode: insert_line::text::TEXT_OPCODE, try_encode: insert_line::text::try_encode, decode: insert_line::text::decode_mutation },
    TextCodec { opcode: remove_line::text::TEXT_OPCODE, try_encode: remove_line::text::try_encode, decode: remove_line::text::decode_mutation },
    TextCodec { opcode: set_line::text::TEXT_OPCODE, try_encode: set_line::text::try_encode, decode: set_line::text::decode_mutation },
];
pub const TEXT_OPCODES: &[&str] = &[set_trailing_newline::text::TEXT_OPCODE, set_line_ending::text::TEXT_OPCODE, insert_line::text::TEXT_OPCODE, remove_line::text::TEXT_OPCODE, set_line::text::TEXT_OPCODE];
//#endregion 🔖️Registry

//#region 🔖️Framing
fn error(detail: impl Into<String>) -> store::TextError {
    store::TextError::new(detail.into(), dsl::TextSpan::at(1, 1))
}
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
    fn nibble(value: u8) -> Option<u8> {
        if value.is_ascii_digit() {
            return Some(value - b'0');
        }
        match value {
            b'a'..=b'f' => Some(value - b'a' + 10),
            _ => None,
        }
    }
    if value.len() % 2 != 0 {
        return Err("payload must be lowercase hexadecimal".to_string());
    }
    value.as_bytes().chunks_exact(2).map(|pair| Ok((nibble(pair[0]).ok_or_else(|| "invalid hexadecimal".to_string())? << 4) | nibble(pair[1]).ok_or_else(|| "invalid hexadecimal".to_string())?)).collect()
}
//#endregion 🔖️Framing

//#region ⚙️Codec
impl protocol::OpText for TxtMutation {
    fn print_op(&self) -> String {
        let (opcode, payload) = TEXT_CODECS.iter().find_map(|codec| (codec.try_encode)(self).map(|payload| (codec.opcode, payload))).expect("txt mutation registry covers every variant");
        format!("txt-mutation {opcode} payload={}", encode_hex(payload.expect("leaf text payload serialization").as_bytes()))
    }

    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let frame = line.strip_prefix("txt-mutation ").ok_or_else(|| error("expected txt mutation frame"))?;
        let (opcode, payload) = frame.split_once(" payload=").ok_or_else(|| error("expected opcode and payload"))?;
        let bytes = decode_hex(payload).map_err(error)?;
        let payload = std::str::from_utf8(&bytes).map_err(|cause| error(cause.to_string()))?;
        let codec = TEXT_CODECS.iter().find(|codec| codec.opcode == opcode).ok_or_else(|| error("unknown txt mutation opcode"))?;
        (codec.decode)(payload).map_err(error)
    }
}
//#endregion ⚙️Codec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::OpText;

    #[test]
    fn generic_framing_refuses_malformed_text_without_panicking() {
        for frame in ["txt-mutation unknown payload=0", "txt-mutation unknown payload=!!", "txt-mutation unknown payload=ff", "txt-mutation unknown payload=7b7d"] {
            assert!(TxtMutation::parse_op(frame).is_err(), "{frame}");
        }
    }
}
//#endregion 🧪️Tests

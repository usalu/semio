//! 📝️ Generic framing and descriptor roster for the transparent TxtMutation.
//#region 🔖️Registry
use crate::schema::mutations::TxtMutation;
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
struct TextCodec {
    opcode: &'static str,
    try_encode: fn(&TxtMutation) -> Option<Result<String, String>>,
    decode: fn(&str) -> Result<TxtMutation, String>,
}
const TEXT_CODECS: &[TextCodec] = &[
    TextCodec { opcode: set_snapshot::TEXT_OPCODE, try_encode: set_snapshot::try_encode, decode: set_snapshot::decode_mutation },
    TextCodec { opcode: set_trailing_newline::TEXT_OPCODE, try_encode: set_trailing_newline::try_encode, decode: set_trailing_newline::decode_mutation },
    TextCodec { opcode: set_line_ending::TEXT_OPCODE, try_encode: set_line_ending::try_encode, decode: set_line_ending::decode_mutation },
    TextCodec { opcode: insert_line::TEXT_OPCODE, try_encode: insert_line::try_encode, decode: insert_line::decode_mutation },
    TextCodec { opcode: remove_line::TEXT_OPCODE, try_encode: remove_line::try_encode, decode: remove_line::decode_mutation },
    TextCodec { opcode: set_line::TEXT_OPCODE, try_encode: set_line::try_encode, decode: set_line::decode_mutation },
];
pub const TEXT_OPCODES: &[&str] = &[set_snapshot::TEXT_OPCODE, set_trailing_newline::TEXT_OPCODE, set_line_ending::TEXT_OPCODE, insert_line::TEXT_OPCODE, remove_line::TEXT_OPCODE, set_line::TEXT_OPCODE];
//#endregion 🔖️Registry

//#region 🔖️Framing
fn error(detail: impl Into<String>) -> semio_framework_diagnostic::TextError {
    semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail.into(), semio_framework_diagnostic::TextSpan::at(1, 1))
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
    if !value.len().is_multiple_of(2) {
        return Err("payload must be lowercase hexadecimal".to_string());
    }
    value.as_bytes().as_chunks::<2>().0.iter().map(|pair| Ok((nibble(pair[0]).ok_or_else(|| "invalid hexadecimal".to_string())? << 4) | nibble(pair[1]).ok_or_else(|| "invalid hexadecimal".to_string())?)).collect()
}
//#endregion 🔖️Framing

//#region ⚙️Codec
impl protocol::OpText for TxtMutation {
    fn print_op(&self) -> String {
        let (opcode, payload) = TEXT_CODECS.iter().find_map(|codec| (codec.try_encode)(self).map(|payload| (codec.opcode, payload))).expect("txt mutation registry covers every variant");
        format!("txt-mutation {opcode} payload={}", encode_hex(payload.expect("leaf text payload serialization").as_bytes()))
    }

    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "📥️insert-line/🦀️.rs"]
pub mod insert_line;

#[path = "✏️set-line/🦀️.rs"]
pub mod set_line;

#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;

#[path = "🔚️set-line-ending/🦀️.rs"]
pub mod set_line_ending;

#[path = "🗑️remove-line/🦀️.rs"]
pub mod remove_line;

#[path = "↩️set-trailing-newline/🦀️.rs"]
pub mod set_trailing_newline;

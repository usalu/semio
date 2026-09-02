//! 📝️ Generic framing and descriptor roster for the transparent JsonMutation.
use crate::artifacts::json::schema::mutations::JsonMutation;
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
pub const TEXT_OPCODES: &[&str] = &["set-member", "remove-member", "insert-array-element", "remove-array-element", "set-scalar"];
fn error(detail: impl Into<String>) -> store::TextError { store::TextError::new(detail.into(), dsl::TextSpan::at(1, 1)) }
fn encode_hex(bytes: &[u8]) -> String { const HEX: &[u8; 16] = b"0123456789abcdef"; let mut text = String::with_capacity(bytes.len() * 2); for byte in bytes { text.push(HEX[(byte >> 4) as usize] as char); text.push(HEX[(byte & 0x0f) as usize] as char); } text }
fn decode_hex(value: &str) -> Result<Vec<u8>, String> { fn nibble(value: u8) -> Option<u8> { if value.is_ascii_digit() { return Some(value - b'0'); } (b'a'..=b'f').contains(&value).then_some(value - b'a' + 10) } if value.len() % 2 != 0 { return Err("payload must be lowercase hexadecimal".to_string()); } value.as_bytes().chunks_exact(2).map(|pair| Ok((nibble(pair[0]).ok_or_else(|| "invalid hexadecimal".to_string())? << 4) | nibble(pair[1]).ok_or_else(|| "invalid hexadecimal".to_string())?)).collect() }
impl protocol::OpText for JsonMutation {
    fn print_op(&self) -> String {
        format!("json-mutation payload={}", encode_hex(pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(self))).as_bytes()))
    }

    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let value = line.strip_prefix("json-mutation payload=").ok_or_else(|| error("expected aggregate payload"))?;
        let bytes = decode_hex(value).map_err(error)?;
        let text = std::str::from_utf8(&bytes).map_err(|cause| error(cause.to_string()))?;
        let parsed = pack::parse_json(text).map_err(|cause| error(cause.to_string()))?;
        <Self as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|cause| error(cause.to_string()))
    }
}

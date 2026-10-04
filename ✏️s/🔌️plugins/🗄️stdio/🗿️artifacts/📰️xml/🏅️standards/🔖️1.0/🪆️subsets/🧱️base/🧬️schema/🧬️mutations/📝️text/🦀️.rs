//! 📝️ Generic framing and descriptor roster for the transparent XmlMutation.
use crate::schema::mutations::XmlMutation;
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
pub const TEXT_OPCODES: &[&str] = &["set-declaration", "set-doctype", "insert-element", "remove-element", "set-attribute", "set-text", "set-snapshot", "patch-snapshot"];
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
        (b'a'..=b'f').contains(&value).then_some(value - b'a' + 10)
    }
    if !value.len().is_multiple_of(2) {
        return Err("payload must be lowercase hexadecimal".to_string());
    }
    value.as_bytes().as_chunks::<2>().0.iter().map(|pair| Ok((nibble(pair[0]).ok_or_else(|| "invalid hexadecimal".to_string())? << 4) | nibble(pair[1]).ok_or_else(|| "invalid hexadecimal".to_string())?)).collect()
}
impl protocol::OpText for XmlMutation {
    fn print_op(&self) -> String {
        format!("xml-mutation payload={}", encode_hex(semio_framework_pack_json::to_json_string(self).as_bytes()))
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let value = line.strip_prefix("xml-mutation payload=").ok_or_else(|| error("expected aggregate payload"))?;
        let bytes = decode_hex(value).map_err(error)?;
        let text = std::str::from_utf8(&bytes).map_err(|cause| error(cause.to_string()))?;
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|cause| semio_framework_diagnostic::TextError::from_value_error(cause, semio_framework_diagnostic::TextSpan::at(1,1)))
    }
}

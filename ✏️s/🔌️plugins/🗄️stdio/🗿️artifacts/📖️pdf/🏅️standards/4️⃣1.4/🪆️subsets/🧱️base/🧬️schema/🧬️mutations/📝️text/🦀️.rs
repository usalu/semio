//! 📝️ PDF 1.4 mutation text framing and executable direct-leaf registry.

use super::PdfMutation;
use protocol::OpText;

//#region 🔖️Grammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 🔖️Grammar

//#region 🔖️Registry
type Printer = fn(&PdfMutation) -> Option<String>;
type Parser = fn(&str) -> Result<PdfMutation, semio_framework_diagnostic::TextError>;
pub const REGISTRY: &[(&str, Printer, Parser)] = &[
    (super::insert_page::text::OPCODE, super::insert_page::text::print, super::insert_page::text::parse),
    (super::remove_page::text::OPCODE, super::remove_page::text::print, super::remove_page::text::parse),
    (super::move_page::text::OPCODE, super::move_page::text::print, super::move_page::text::parse),
    (super::resize_page::text::OPCODE, super::resize_page::text::print, super::resize_page::text::parse),
    (super::replace_page_text::text::OPCODE, super::replace_page_text::text::print, super::replace_page_text::text::parse),
];
//#endregion 🔖️Registry

//#region 🔖️Framing
pub(super) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub(super) fn unhex(text: &str) -> Result<Vec<u8>, String> {
    if !text.len().is_multiple_of(2) || !text.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err("Invalid hexadecimal payload".into());
    }
    text.as_bytes().as_chunks::<2>().0.iter().map(|pair| u8::from_str_radix(std::str::from_utf8(pair).map_err(|error| error.to_string())?, 16).map_err(|error| error.to_string())).collect()
}

impl OpText for PdfMutation {
    fn print_op(&self) -> String {
        REGISTRY.iter().find_map(|(opcode, print, _)| print(self).map(|payload| format!("{opcode} payload={payload}"))).expect("Every mutation has one direct text owner")
    }

    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let (opcode, payload) = line.split_once(" payload=").ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Expected opcode and payload", semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        let (_, _, parser) = REGISTRY.iter().find(|(identity, _, _)| *identity == opcode).ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Unknown PDF 1.4 mutation opcode", semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        parser(payload)
    }
}
//#endregion 🔖️Framing

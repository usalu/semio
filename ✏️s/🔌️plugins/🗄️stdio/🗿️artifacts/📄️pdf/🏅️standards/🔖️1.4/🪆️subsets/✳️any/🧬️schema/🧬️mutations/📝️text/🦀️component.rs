//! 📝️ PDF 1.4 mutation text framing and executable direct-leaf registry.

use super::PdfMutation;
use protocol::OpText;

//#region 🔖️Grammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 🔖️Grammar

//#region 🔖️Registry
type Printer = fn(&PdfMutation) -> Option<String>;
type Parser = fn(&str) -> Result<PdfMutation, String>;
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
    if text.len() % 2 != 0 || !text.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err("Invalid hexadecimal payload".into());
    }
    text.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).map_err(|error| error.to_string())?, 16).map_err(|error| error.to_string())).collect()
}

impl OpText for PdfMutation {
    fn print_op(&self) -> String {
        REGISTRY.iter().find_map(|(opcode, print, _)| print(self).map(|payload| format!("{opcode} payload={payload}"))).expect("Every mutation has one direct text owner")
    }

    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let parse = || -> Result<Self, String> {
            let (opcode, payload) = line.split_once(" payload=").ok_or("Expected opcode and payload")?;
            let (_, _, parser) = REGISTRY.iter().find(|(identity, _, _)| *identity == opcode).ok_or("Unknown PDF 1.4 mutation opcode")?;
            parser(payload)
        };
        parse().map_err(|error| store::TextError::new(error, dsl::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️Framing

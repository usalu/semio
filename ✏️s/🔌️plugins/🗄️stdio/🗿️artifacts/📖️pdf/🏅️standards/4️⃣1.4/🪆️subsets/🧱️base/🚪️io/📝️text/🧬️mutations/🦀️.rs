//! 📝️ PDF 1.4 mutation text framing and executable direct-leaf registry.

use crate::standards::v1_4::subsets::base::schema::mutations::PdfMutation;
use protocol::OpText;

//#region 🔖️Grammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 🔖️Grammar

//#region 🔖️Registry
type Printer = fn(&PdfMutation) -> Option<String>;
type Parser = fn(&str) -> Result<PdfMutation, semio_framework_diagnostic::TextError>;
pub const REGISTRY: &[(&str, Printer, Parser)] = &[
    (crate::standards::v1_4::subsets::base::schema::mutations::insert_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::insert_page::print, crate::standards::v1_4::subsets::base::schema::mutations::insert_page::parse),
    (crate::standards::v1_4::subsets::base::schema::mutations::remove_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::remove_page::print, crate::standards::v1_4::subsets::base::schema::mutations::remove_page::parse),
    (crate::standards::v1_4::subsets::base::schema::mutations::move_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::move_page::print, crate::standards::v1_4::subsets::base::schema::mutations::move_page::parse),
    (crate::standards::v1_4::subsets::base::schema::mutations::resize_page::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::resize_page::print, crate::standards::v1_4::subsets::base::schema::mutations::resize_page::parse),
    (crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text::print, crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text::parse),
    (crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot::print, crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot::parse),
    (crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::OPCODE, crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::print, crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::parse),
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

#[path = "🔀️move-page/🦀️.rs"]
pub mod move_page;

#[path = "🗑️remove-page/🦀️.rs"]
pub mod remove_page;

#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;

#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;

#[path = "♻️replace-page-text/🦀️.rs"]
pub mod replace_page_text;

#[path = "📥️insert-page/🦀️.rs"]
pub mod insert_page;

#[path = "📐️resize-page/🦀️.rs"]
pub mod resize_page;

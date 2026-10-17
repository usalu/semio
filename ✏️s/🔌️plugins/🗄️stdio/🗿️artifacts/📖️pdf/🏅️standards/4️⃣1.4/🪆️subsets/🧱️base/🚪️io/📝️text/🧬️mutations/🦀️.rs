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
    (insert_page::OPCODE, insert_page::print, insert_page::parse),
    (remove_page::OPCODE, remove_page::print, remove_page::parse),
    (move_page::OPCODE, move_page::print, move_page::parse),
    (resize_page::OPCODE, resize_page::print, resize_page::parse),
    (replace_page_text::OPCODE, replace_page_text::print, replace_page_text::parse),
];
//#endregion 🔖️Registry

//#region 🔖️Framing
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

pub use semio_s_artifact_stdio_pdf_snapshot::standards::v1_4::subsets::base::io::text::mutations::wire;

#[path = "🔀️move-page/🦀️.rs"]
pub mod move_page;

#[path = "🗑️remove-page/🦀️.rs"]
pub mod remove_page;

#[path = "♻️replace-page-text/🦀️.rs"]
pub mod replace_page_text;

#[path = "📥️insert-page/🦀️.rs"]
pub mod insert_page;

#[path = "📐️resize-page/🦀️.rs"]
pub mod resize_page;

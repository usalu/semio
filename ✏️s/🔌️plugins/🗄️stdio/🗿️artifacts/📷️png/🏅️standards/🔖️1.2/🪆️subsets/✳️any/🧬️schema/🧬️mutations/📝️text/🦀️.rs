//! 📝️ Framing and direct codec registry for PngMutation.
use crate::schema::mutations::PngMutation;

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

pub struct Entry {
    pub opcode: &'static str,
    pub print: fn(&PngMutation) -> Option<String>,
    pub parse: fn(&str) -> Result<PngMutation, semio_framework_diagnostic::TextError>,
}

pub const REGISTRY: &[Entry] = &[
    crate::schema::mutations::set_snapshot::text::CODEC,
    crate::schema::mutations::patch_snapshot::text::CODEC,
    crate::schema::mutations::change_gamma::text::CODEC,
    crate::schema::mutations::patch_pixels::text::CODEC,
    crate::schema::mutations::paint_native_samples::text::CODEC,
];

impl protocol::OpText for PngMutation {
    fn print_op(&self) -> String {
        REGISTRY.iter().find_map(|entry| (entry.print)(self)).expect("every PNG mutation has a direct text owner")
    }

    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let opcode = line.split_once(' ').map_or(line, |(opcode, _)| opcode);
        let entry = REGISTRY.iter().find(|entry| entry.opcode == opcode).ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown mutation opcode {opcode}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        (entry.parse)(line)
    }
}

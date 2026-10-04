//! 📝️ Framing and direct codec registry for TiffMutation.
use crate::schema::mutations::TiffMutation;

//#region Registry
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
pub struct Entry {
    pub opcode: &'static str,
    pub print: fn(&TiffMutation) -> Option<String>,
    pub parse: fn(&str) -> Result<TiffMutation, semio_framework_diagnostic::TextError>,
}
pub const REGISTRY: &[Entry] = &[
    crate::schema::mutations::patch_snapshot::text::CODEC,
    crate::schema::mutations::set_snapshot::text::CODEC,
    crate::schema::mutations::change_byte_order::text::CODEC,
    crate::schema::mutations::insert_ifd::text::CODEC,
    crate::schema::mutations::remove_ifd::text::CODEC,
    crate::schema::mutations::replace_tag::text::CODEC,
    crate::schema::mutations::remove_tag::text::CODEC,
    crate::schema::mutations::paint_region::text::CODEC,
];
//#endregion Registry

//#region Framing
impl protocol::OpText for TiffMutation {
    fn print_op(&self) -> String {
        REGISTRY.iter().find_map(|entry| (entry.print)(self)).expect("every aggregate variant has a direct text owner")
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let opcode = line.split_once(' ').map_or(line, |(opcode, _)| opcode);
        let entry = REGISTRY.iter().find(|entry| entry.opcode == opcode).ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown mutation opcode {opcode}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        (entry.parse)(line)
    }
}
//#endregion Framing

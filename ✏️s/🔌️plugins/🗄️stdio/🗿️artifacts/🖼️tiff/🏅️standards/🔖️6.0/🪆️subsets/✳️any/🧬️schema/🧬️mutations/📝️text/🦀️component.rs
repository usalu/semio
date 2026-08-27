//! 📝️ Framing and direct codec registry for TiffMutation.
use crate::artifacts::tiff::schema::mutations::TiffMutation;

//#region Registry
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
pub struct Entry {
    pub opcode: &'static str,
    pub print: fn(&TiffMutation) -> Option<String>,
    pub parse: fn(&str) -> Result<TiffMutation, String>,
}
pub const REGISTRY: &[Entry] = &[
    crate::artifacts::tiff::schema::mutations::change_byte_order::text::CODEC,
    crate::artifacts::tiff::schema::mutations::insert_ifd::text::CODEC,
    crate::artifacts::tiff::schema::mutations::remove_ifd::text::CODEC,
    crate::artifacts::tiff::schema::mutations::replace_tag::text::CODEC,
    crate::artifacts::tiff::schema::mutations::remove_tag::text::CODEC,
    crate::artifacts::tiff::schema::mutations::replace_pixels::text::CODEC,
];
//#endregion Registry

//#region Framing
impl protocol::OpText for TiffMutation {
    fn print_op(&self) -> String {
        REGISTRY.iter().find_map(|entry| (entry.print)(self)).expect("every aggregate variant has a direct text owner")
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let opcode = line.split_once(' ').map_or(line, |(opcode, _)| opcode);
        let entry = REGISTRY.iter().find(|entry| entry.opcode == opcode).ok_or_else(|| store::TextError::new(format!("unknown mutation opcode {opcode}"), dsl::TextSpan::at(1, 1)))?;
        (entry.parse)(line).map_err(|error| store::TextError::new(error, dsl::TextSpan::at(1, 1)))
    }
}
//#endregion Framing

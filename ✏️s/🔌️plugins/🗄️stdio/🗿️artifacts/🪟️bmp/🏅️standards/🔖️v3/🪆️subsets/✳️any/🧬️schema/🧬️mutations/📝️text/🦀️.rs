//! 📝️ Framing and direct codec registry for BmpMutation.
use crate::artifacts::bmp::schema::mutations::BmpMutation;

//#region Registry
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
pub struct Entry {
    pub opcode: &'static str,
    pub print: fn(&BmpMutation) -> Option<String>,
    pub parse: fn(&str) -> Result<BmpMutation, String>,
}
pub const REGISTRY: &[Entry] = &[
    crate::artifacts::bmp::schema::mutations::change_header_fields::text::CODEC,
    crate::artifacts::bmp::schema::mutations::insert_palette_entry::text::CODEC,
    crate::artifacts::bmp::schema::mutations::remove_palette_entry::text::CODEC,
    crate::artifacts::bmp::schema::mutations::replace_palette_entry::text::CODEC,
    crate::artifacts::bmp::schema::mutations::replace_pixel_data::text::CODEC,
];
//#endregion Registry

//#region Framing
impl protocol::OpText for BmpMutation {
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

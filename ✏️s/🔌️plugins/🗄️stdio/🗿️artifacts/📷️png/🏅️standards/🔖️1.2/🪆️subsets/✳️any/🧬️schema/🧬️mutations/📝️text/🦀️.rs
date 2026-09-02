//! 📝️ Framing and direct codec registry for PngMutation.
use crate::artifacts::png::schema::mutations::PngMutation;

//#region Registry
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
pub struct Entry {
    pub opcode: &'static str,
    pub print: fn(&PngMutation) -> Option<String>,
    pub parse: fn(&str) -> Result<PngMutation, String>,
}
pub const REGISTRY: &[Entry] = &[
    crate::artifacts::png::schema::mutations::change_header::text::CODEC,
    crate::artifacts::png::schema::mutations::replace_palette::text::CODEC,
    crate::artifacts::png::schema::mutations::change_transparency::text::CODEC,
    crate::artifacts::png::schema::mutations::change_gamma::text::CODEC,
    crate::artifacts::png::schema::mutations::change_chromaticities::text::CODEC,
    crate::artifacts::png::schema::mutations::change_srgb_intent::text::CODEC,
    crate::artifacts::png::schema::mutations::change_physical_dims::text::CODEC,
    crate::artifacts::png::schema::mutations::change_timestamp::text::CODEC,
    crate::artifacts::png::schema::mutations::change_background::text::CODEC,
    crate::artifacts::png::schema::mutations::insert_text_chunk::text::CODEC,
    crate::artifacts::png::schema::mutations::remove_text_chunk::text::CODEC,
    crate::artifacts::png::schema::mutations::replace_text_chunk::text::CODEC,
    crate::artifacts::png::schema::mutations::replace_pixels::text::CODEC,
    crate::artifacts::png::schema::mutations::insert_unknown_chunk::text::CODEC,
    crate::artifacts::png::schema::mutations::remove_unknown_chunk::text::CODEC,
];
//#endregion Registry

//#region Framing
impl protocol::OpText for PngMutation {
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

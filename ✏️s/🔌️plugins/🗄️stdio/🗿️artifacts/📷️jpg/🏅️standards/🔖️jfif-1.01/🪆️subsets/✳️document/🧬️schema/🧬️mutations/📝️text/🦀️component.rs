//! 📝️ Framing and direct codec registry for JpgMutation.
use crate::artifacts::jpg::schema::mutations::JpgMutation;

//#region Registry
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
pub struct Entry {
    pub opcode: &'static str,
    pub print: fn(&JpgMutation) -> Option<String>,
    pub parse: fn(&str) -> Result<JpgMutation, String>,
}
pub const REGISTRY: &[Entry] = &[
    crate::artifacts::jpg::schema::mutations::change_jfif_header::text::CODEC,
    crate::artifacts::jpg::schema::mutations::replace_quant_table::text::CODEC,
    crate::artifacts::jpg::schema::mutations::remove_quant_table::text::CODEC,
    crate::artifacts::jpg::schema::mutations::replace_huffman_table::text::CODEC,
    crate::artifacts::jpg::schema::mutations::remove_huffman_table::text::CODEC,
    crate::artifacts::jpg::schema::mutations::change_restart_interval::text::CODEC,
    crate::artifacts::jpg::schema::mutations::insert_other_segment::text::CODEC,
    crate::artifacts::jpg::schema::mutations::remove_other_segment::text::CODEC,
    crate::artifacts::jpg::schema::mutations::replace_pixels::text::CODEC,
    crate::artifacts::jpg::schema::mutations::change_re_encode_quality::text::CODEC,
];
//#endregion Registry

//#region Framing
impl protocol::OpText for JpgMutation {
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

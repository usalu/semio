//! 📝️ Framing and direct codec registry for JpgMutation.
use crate::schema::mutations::JpgMutation;

//#region Registry
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
pub struct Entry {
    pub opcode: &'static str,
    pub print: fn(&JpgMutation) -> Option<String>,
    pub parse: fn(&str) -> Result<JpgMutation, semio_framework_diagnostic::TextError>,
}
pub const REGISTRY: &[Entry] = &[
    crate::standards::v_jfif_1_01::subsets::document::io::text::mutations::change_jfif_header::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::io::text::mutations::insert_other_segment::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::io::text::mutations::remove_other_segment::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::io::text::mutations::replace_pixels::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::io::text::mutations::replace_image::CODEC,
];
//#endregion Registry

//#region Framing
impl protocol::OpText for JpgMutation {
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


#[path = "🪪️change-jfif/🦀️.rs"]
pub mod change_jfif_header;



#[path = "🗑️remove-other/🦀️.rs"]
pub mod remove_other_segment;

#[path = "📥️insert-other/🦀️.rs"]
pub mod insert_other_segment;


#[path = "🔲️replace-pixels/🦀️.rs"]
pub mod replace_pixels;




#[path = "🖼️replace-image/🦀️.rs"]
pub mod replace_image;



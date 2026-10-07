//! 📝️ Framing and direct codec registry for BmpMutation.
use crate::standards::v_v3::subsets::any::schema::mutations::BmpMutation;

//#region Registry
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
pub struct Entry {
    pub opcode: &'static str,
    pub print: fn(&BmpMutation) -> Option<String>,
    pub parse: fn(&str) -> Result<BmpMutation, semio_framework_diagnostic::TextError>,
}
pub const REGISTRY: &[Entry] = &[crate::standards::v_v3::subsets::any::io::text::mutations::set_snapshot::CODEC, crate::standards::v_v3::subsets::any::io::text::mutations::patch_snapshot::CODEC, crate::standards::v_v3::subsets::any::io::text::mutations::paint_indexed_region::CODEC, crate::standards::v_v3::subsets::any::io::text::mutations::paint_direct_region::CODEC];
//#endregion Registry

//#region Framing
impl protocol::OpText for BmpMutation {
    fn print_op(&self) -> String {
        REGISTRY.iter().find_map(|entry| (entry.print)(self)).expect("every aggregate variant has a direct text owner")
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let opcode = line.split_once(' ').map_or(line, |(opcode, _)| opcode);
        let entry = REGISTRY
            .iter()
            .find(|entry| entry.opcode == opcode)
            .ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown mutation opcode {opcode}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        (entry.parse)(line)
    }
}
//#endregion Framing

#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;

#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;

#[path = "🖌️paint-direct-region/🦀️.rs"]
pub mod paint_direct_region;

#[path = "🎨️paint-indexed-region/🦀️.rs"]
pub mod paint_indexed_region;

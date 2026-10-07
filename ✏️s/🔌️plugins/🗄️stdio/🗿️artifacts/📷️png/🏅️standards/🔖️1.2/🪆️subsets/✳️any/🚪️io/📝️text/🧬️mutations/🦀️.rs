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
    crate::standards::v1_2::subsets::any::io::text::mutations::set_snapshot::CODEC,
    crate::standards::v1_2::subsets::any::io::text::mutations::patch_snapshot::CODEC,
    crate::standards::v1_2::subsets::any::io::text::mutations::change_gamma::CODEC,
    crate::standards::v1_2::subsets::any::io::text::mutations::patch_pixels::CODEC,
    crate::standards::v1_2::subsets::any::io::text::mutations::paint_native_samples::CODEC,
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

#[path = "🩹️patch-pixels/🦀️.rs"]
pub mod patch_pixels;

#[path = "🌗️change-gamma/🦀️.rs"]
pub mod change_gamma;

#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;

#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;

#[path = "🎨️paint-native-samples/🦀️.rs"]
pub mod paint_native_samples;

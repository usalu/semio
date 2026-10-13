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
    change_gamma::CODEC,
    patch_pixels::CODEC,
    paint_native_samples::CODEC,
    set_gamma::CODEC,
    replace_samples::CODEC,
    replace_image::CODEC,
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

#[path = "🎨️paint-native-samples/🦀️.rs"]
pub mod paint_native_samples;

#[path = "🎚️set-gamma/🦀️.rs"]
pub mod set_gamma;

#[path = "🧩️replace-samples/🦀️.rs"]
pub mod replace_samples;

#[path = "🔄️replace-image/🦀️.rs"]
pub mod replace_image;

//! 🎨️ Pure TIFF tiled-region action metadata shared by typed editing and lower window declarations.

use semio_plugin::{ActionArgDef, ActionDefinition, ActionKind, ArgSchema};
use semio_ui_locale::LocalizedLabel;
use crate::schema::mutations::paint_region::samples::TIFF_MAXIMUM_INTERACTIVE_PAINT_ROWS;

pub const ACTION_ID: &str = "paint-region";
pub const MAXIMUM_INTERACTIVE_ROWS: u32 = TIFF_MAXIMUM_INTERACTIVE_PAINT_ROWS;

fn integer_arg(id: &'static str, en: &'static str, de: &'static str, minimum: f64, maximum: f64, default: u32) -> ActionArgDef {
    let mut argument = ActionArgDef::number(id, LocalizedLabel::native(en, de)).required().default_value(&default);
    if let ArgSchema::Number { min, max, step, integer, .. } = &mut argument.schema {
        *min = Some(minimum);
        *max = Some(maximum);
        *step = Some(1.0);
        *integer = true;
    }
    argument
}

pub fn action() -> ActionDefinition {
    ActionDefinition::bounded_catalog(ACTION_ID, LocalizedLabel::native("Paint Tiled Region", "Kachelbereich malen"), ActionKind::Mutation).with_args(vec![
        integer_arg("x", "Left (x)", "Links (x)", 0.0, u32::MAX as f64, 0),
        integer_arg("y", "Top (y)", "Oben (y)", 0.0, u32::MAX as f64, 0),
        integer_arg("width", "Width", "Breite", 1.0, u32::MAX as f64, 1),
        integer_arg("height", "Height", "Höhe", 1.0, f64::from(MAXIMUM_INTERACTIVE_ROWS), 1),
        integer_arg("red", "Red", "Rot", 0.0, 255.0, 0),
        integer_arg("green", "Green", "Grün", 0.0, 255.0, 0),
        integer_arg("blue", "Blue", "Blau", 0.0, 255.0, 0),
        integer_arg("alpha", "Alpha", "Alpha", 0.0, 255.0, 255),
    ])
}


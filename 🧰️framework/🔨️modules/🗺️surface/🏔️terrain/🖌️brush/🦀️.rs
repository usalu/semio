//! 🖌️ Action metadata that an editor gives a terrain viewport.

use semio_framework_value::{FromValue, ToValue};

/// 🎟️ Typed element of the World3d interaction lane for a terrain sample click.
#[derive(Clone, Debug, PartialEq, FromValue, ToValue)]
#[value(deny_unknown_fields)]
pub struct TerrainBrushAction {
    pub action: String,
    pub arguments: TerrainBrushArguments,
}

/// 📐️ Arguments declared by the editor. The viewport adds the sampled row and column.
#[derive(Clone, Debug, PartialEq, FromValue, ToValue)]
#[value(deny_unknown_fields)]
pub struct TerrainBrushArguments {
    pub operation: String,
    pub radius: u32,
    pub value: f64,
}

impl TerrainBrushAction {
    /// ✅️ Checks descriptor limits before a host borrows or dispatches its fields.
    pub fn is_valid(&self) -> bool {
        let valid_id = |id: &str| !id.is_empty() && id.len() <= 64
            && id.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'));
        let value = self.arguments.value;
        let strength = if self.arguments.operation == "smooth" {
            (0.0..=100.0).contains(&value) && value.fract() == 0.0
        } else {
            (-32768.0..=32767.0).contains(&value) && (value * 1000.0).round() / 1000.0 == value
        };
        valid_id(&self.action) && valid_id(&self.arguments.operation)
            && self.arguments.radius < super::TERRAIN_GRID_RESOLUTION
            && strength
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

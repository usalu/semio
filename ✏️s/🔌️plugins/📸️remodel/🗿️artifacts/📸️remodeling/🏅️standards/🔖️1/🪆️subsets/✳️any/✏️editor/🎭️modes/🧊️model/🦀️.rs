//! 🧊️ Remodeling play app — the `model` mode: the app's default mode, showing the 3D reconstruction
//! result beside the frame filmstrip. Owns the Model window (the one window every mode's layout uses).

use crate::editor::remodeling::modes::capture::windows::frames;
use crate::editor::remodeling::modes::model::windows::model;
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const REMODELING_PLAY_MODE_MODEL: &str = "model";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::remodeling::create_remodeling_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: REMODELING_PLAY_MODE_MODEL.into(), label: LocalizedLabel::native("Model", "Modell"), icon_id: "box".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ This mode is the app's `default_mode_id`, so its layout IS the app-level `default_layout`.
pub fn layout() -> WindowLayout {
    create_default_layout(&[model::REMODELING_PLAY_WINDOW_MAIN.into(), frames::REMODELING_PLAY_WINDOW_FRAMES.into()], "row", Some(&[70.0, 30.0]), Some(&["Model".into(), "Frames".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//! 🎥️ Edit-mode window option — the world projection picker (orthographic/one-two-three-point plus
//! the cardinal/free orientation and its parameters), delegated wholesale to the framework's shared
//! `world3d_projection_measures` builder. Shared by every window instance of the one puzzle3d window
//! kind; each instance keeps its own `Puzzle3dCamera::projection` (see `🦀️config.rs`).

use crate::editor::puzzle3d::config::Puzzle3dRuntime;
use crate::editor::puzzle3d::puzzle3d_action;
use semio_framework_plugin::{world3d_projection_measures, WindowMeasure};

/// 🧭️ The projection select + its parameter sliders for the materialized window's camera.
pub async fn measure(runtime: &Puzzle3dRuntime) -> WindowMeasure {
    world3d_projection_measures("puzzle3d", &runtime.camera.projection, puzzle3d_action)
}

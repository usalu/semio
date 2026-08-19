//! ☀️ Edit-mode window option — the sun/environment group (enabled, azimuth, elevation, intensity),
//! delegated wholesale to the framework's shared `world3d_sun_measures` builder. Per window
//! instance: each keeps its own `WorldSunConfig` (see `🦀️config.rs`).

use crate::editor::puzzle3d::config::Puzzle3dRuntime;
use crate::editor::puzzle3d::puzzle3d_action;
use semio_framework_plugin::{world3d_sun_measures, WindowMeasure};

/// 🌞️ The sun toggle plus its azimuth/elevation/intensity sliders.
pub async fn measure(runtime: &Puzzle3dRuntime) -> WindowMeasure {
    world3d_sun_measures("puzzle3d", &runtime.sun, puzzle3d_action)
}

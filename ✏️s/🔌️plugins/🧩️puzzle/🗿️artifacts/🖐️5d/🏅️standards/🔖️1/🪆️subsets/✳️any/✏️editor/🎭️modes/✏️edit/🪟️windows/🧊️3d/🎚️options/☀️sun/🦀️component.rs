//! ☀️ 3D-window option — the sun/environment group (enabled, azimuth, elevation, intensity),
//! delegated wholesale to the framework's shared `world3d_sun_measures` builder. Genuinely
//! window-specific: only the World3d surface has an environment to light, so this stays under the 3D
//! window rather than at mode level.

use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::puzzle5d_action;
use semio_framework_plugin::{world3d_sun_measures, WindowMeasure};

/// 🌞️ The sun toggle plus its azimuth/elevation/intensity sliders.
pub fn measure(runtime: &Puzzle5dRuntime) -> WindowMeasure {
    world3d_sun_measures("puzzle5d", &runtime.sun, puzzle5d_action)
}

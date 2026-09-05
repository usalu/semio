//! ☀️ Edit-mode window option — the sun/environment group (enabled, azimuth, elevation, intensity),
//! delegated wholesale to the framework's shared `world3d_sun_measures` builder. Per window
//! instance: each keeps its own `WorldSunConfig` (see `🦀️config.rs`).

use crate::editor::puzzle3d::config::Puzzle3dRuntime;
use crate::editor::puzzle3d::puzzle3d_action;
use semio_framework_plugin::{world3d_sun_measures, WindowMeasure};

/// 🌞️ The sun toggle plus its azimuth/elevation/intensity sliders.
/// 🔗️ `world3d_sun_measures` is a framework helper still bound to `serde_json::Value` (framework
/// file, out of this crate's remit); `puzzle3d_action` takes the DSL-side `dsl::os_pack::json::
/// Value`, so this one closure bridges the unavoidable seam via the framework's own pre-existing
/// `DslValue: From<&serde_json::Value>` impl rather than widening `puzzle3d_action`'s signature.
pub fn measure(runtime: &Puzzle3dRuntime) -> WindowMeasure {
    world3d_sun_measures("puzzle3d", &runtime.sun, |action, args| puzzle3d_action(action, args.map(|value| dsl::os_pack::json::from_dsl_value(&dsl::DslValue::from(&value)))))
}

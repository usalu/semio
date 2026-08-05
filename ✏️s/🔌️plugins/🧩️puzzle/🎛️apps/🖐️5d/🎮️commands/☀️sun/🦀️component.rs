//! ☀️ Puzzle 5d play app commands — the sun/environment axes, delegated wholesale to the framework's
//! shared `apply_world3d_sun_action` so the toggle and the three sliders stay one arm.

use crate::apps::puzzle5d::Puzzle5dActionCtx;
use semio_framework_plugin::apply_world3d_sun_action;
use serde_json::Value;

/// 🌞️ `toggleSun`/`setSunAzimuth`/`setSunElevation`/`setSunIntensity` share one arm.
pub fn apply(ctx: &mut Puzzle5dActionCtx<'_>, action: &str, args: Option<&Value>) {
    apply_world3d_sun_action(&mut ctx.scene.runtime.sun, action, args);
}

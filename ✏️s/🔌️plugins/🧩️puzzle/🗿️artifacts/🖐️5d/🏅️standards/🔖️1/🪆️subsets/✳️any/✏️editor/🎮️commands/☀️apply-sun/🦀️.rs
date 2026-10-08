//! ☀️ `apply-sun` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use semio_framework_plugin::apply_world3d_sun_action;
use semio_framework_pack_json::Value;

/// 🌞️ `toggleSun`/`setSunAzimuth`/`setSunElevation`/`setSunIntensity` share one arm.
///
/// 🔗️ `apply_world3d_sun_action` now takes the first-party `semio_framework_pack_json::Value`, so `args`
/// passes straight through — the former `semio_framework_pack_json::Value` seam is gone.
pub fn apply(ctx: &mut Puzzle5dActionCtx<'_>, action: &str, args: Option<&Value>) {
    apply_world3d_sun_action(&mut ctx.scene.runtime.sun, action, args);
}

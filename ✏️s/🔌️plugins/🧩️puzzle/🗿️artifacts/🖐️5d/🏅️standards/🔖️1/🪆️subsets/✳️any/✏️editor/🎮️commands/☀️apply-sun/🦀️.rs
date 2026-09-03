//! ☀️ `apply-sun` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use semio_framework_plugin::apply_world3d_sun_action;
use dsl::os_pack::json::Value;

/// 🌞️ `toggleSun`/`setSunAzimuth`/`setSunElevation`/`setSunIntensity` share one arm.
///
/// 🔗️ `apply_world3d_sun_action` now takes the first-party `dsl::os_pack::json::Value`, so `args`
/// passes straight through — the former `serde_json::Value` seam is gone.
pub fn apply(ctx: &mut Puzzle5dActionCtx<'_>, action: &str, args: Option<&Value>) {
    apply_world3d_sun_action(&mut ctx.scene.runtime.sun, action, args);
}

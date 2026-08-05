//! ☀️ Puzzle 3d play app commands — the sun/environment toggle and its azimuth/elevation/intensity
//! sliders, all four delegated to the framework's shared `apply_world3d_sun_action` so puzzle3d's
//! environment behaves exactly like every other world-3d app's.

use crate::apps::puzzle3d::Puzzle3dActionCtx;
use semio_framework_plugin::apply_world3d_sun_action;
use serde_json::Value;

pub fn apply(ctx: &mut Puzzle3dActionCtx<'_>, action: &str, args: Option<&Value>) {
    apply_world3d_sun_action(&mut ctx.scene.runtime.sun, action, args);
}

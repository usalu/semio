//! ☀️ `apply-sun` command.

use crate::editor::puzzle3d::Puzzle3dActionCtx;
use semio_framework_plugin::apply_world3d_sun_action;
use serde_json::Value;

pub async fn apply(ctx: &mut Puzzle3dActionCtx<'_>, action: &str, args: Option<&Value>) {
    apply_world3d_sun_action(&mut ctx.scene.runtime.sun, action, args);
}

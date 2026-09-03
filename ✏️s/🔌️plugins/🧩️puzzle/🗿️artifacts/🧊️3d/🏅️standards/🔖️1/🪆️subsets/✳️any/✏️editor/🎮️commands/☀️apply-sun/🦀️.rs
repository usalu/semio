//! ☀️ `apply-sun` command.

use crate::editor::puzzle3d::Puzzle3dActionCtx;
use semio_framework_plugin::apply_world3d_sun_action;
use dsl::os_pack::json::Value;

/// ☀️ Applies a sun action through the framework helper, which now takes the same first-party
/// `dsl::os_pack::json::Value` this command's `args` already carries.
pub fn apply(ctx: &mut Puzzle3dActionCtx<'_>, action: &str, args: Option<&Value>) {
    apply_world3d_sun_action(&mut ctx.scene.runtime.sun, action, args);
}

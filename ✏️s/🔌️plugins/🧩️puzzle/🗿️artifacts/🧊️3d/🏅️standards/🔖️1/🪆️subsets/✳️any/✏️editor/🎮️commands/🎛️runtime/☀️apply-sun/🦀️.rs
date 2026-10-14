//! ☀️ `apply-sun` command.

use semio_s_artifact_puzzle_3d_editor_session::Puzzle3dActionCtx;
use semio_framework_plugin::apply_world3d_sun_action;
use semio_framework_pack_json::Value;

/// ☀️ Applies a sun action through the framework helper, which now takes the same first-party
/// `dsl::os_pack::json::Value` this command's `args` already carries.
pub fn apply(ctx: &mut Puzzle3dActionCtx<'_>, action: &str, args: Option<&Value>) {
    apply_world3d_sun_action(&mut ctx.scene.runtime.sun, action, args);
}

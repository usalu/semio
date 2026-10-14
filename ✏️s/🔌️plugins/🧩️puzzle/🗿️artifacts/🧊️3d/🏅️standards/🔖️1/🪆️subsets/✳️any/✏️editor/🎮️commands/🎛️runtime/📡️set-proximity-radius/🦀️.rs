//! ⚙️ `set-proximity-radius` command.

use {crate::editor::puzzle3d::snapshot::puzzle3d_absolute_or_delta, crate::editor::puzzle3d::session::Puzzle3dActionCtx};
use semio_framework_pack_json::Value;

pub fn set_proximity_radius(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = puzzle3d_absolute_or_delta(args, ctx.scene.runtime.proximity_radius) {
        ctx.scene.runtime.proximity_radius = value.max(0.0);
    }
}

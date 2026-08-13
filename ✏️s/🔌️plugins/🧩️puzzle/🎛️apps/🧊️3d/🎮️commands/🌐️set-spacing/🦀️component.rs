//! 🌐️ `set-spacing` command.

use crate::apps::puzzle3d::{puzzle3d_absolute_or_delta, Puzzle3dActionCtx};
use serde_json::Value;

pub fn set_spacing(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = puzzle3d_absolute_or_delta(args, ctx.scene.runtime.grid_spacing) {
        ctx.scene.runtime.grid_spacing = value.max(0.1);
    }
}

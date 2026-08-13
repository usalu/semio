//! 👆️ `world-vortex-hover` command.

use crate::apps::puzzle3d::modes::edit::windows::main::utilities;
use crate::apps::puzzle3d::{drive_precompute, Puzzle3dActionCtx};
use serde_json::Value;

pub fn world_vortex_hover(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.hovered_vortex_full_id = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()).map(str::to_string);
    if ctx.scene.active_utility == utilities::brush::UTILITY_ID && ctx.scene.runtime.hovered_vortex_full_id.is_some() {
        drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
    }
}

//! 👆️ Puzzle 3d play app commands — hover: which object, vortex or catalogue kind the pointer is
//! over. Pure chrome (the `PatchWorld3dChrome` effect carries it back to the host without a full
//! re-render); hovering a vortex while the brush utility is active additionally kicks the
//! background candidate precompute so the ghost preview is ready by the time the user clicks.

use crate::apps::puzzle3d::modes::edit::windows::main::utilities;
use crate::apps::puzzle3d::{drive_precompute, Puzzle3dActionCtx};
use serde_json::Value;

pub fn world_hover(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.hovered_object_id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(str::to_string);
}

pub fn set_hover(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.hovered_object_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).map(str::to_string);
}

pub fn world_vortex_hover(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.hovered_vortex_full_id = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()).map(str::to_string);
    if ctx.scene.active_utility == utilities::brush::UTILITY_ID && ctx.scene.runtime.hovered_vortex_full_id.is_some() {
        drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
    }
}

pub fn set_kind_hover(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.hovered_kind_id = args.and_then(|value| value.get("kindId")).and_then(|value| value.as_str()).map(str::to_string);
}

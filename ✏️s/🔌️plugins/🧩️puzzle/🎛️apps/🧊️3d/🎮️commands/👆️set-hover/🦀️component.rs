//! 👆️ `set-hover` command.

use crate::apps::puzzle3d::modes::edit::windows::main::utilities;
use crate::apps::puzzle3d::{drive_precompute, Puzzle3dActionCtx};
use serde_json::Value;

pub fn set_hover(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.hovered_object_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).map(str::to_string);
}

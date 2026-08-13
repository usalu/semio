//! 👆️ `set-hover` command.

use crate::apps::puzzle5d::config::puzzle5d_clear_non_grip_selection;
use crate::apps::puzzle5d::Puzzle5dActionCtx;
use semio_framework_plugin::SelectionSet;
use serde_json::Value;

pub fn set_hover(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.hovered_part_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).map(str::to_string);
}

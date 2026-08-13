//! 🗂️ `world-select` command.

use crate::apps::puzzle5d::config::{puzzle5d_clear_non_part_selection, puzzle5d_clear_selection, Puzzle5dSelection};
use crate::apps::puzzle5d::{classify_selection, Puzzle5dActionCtx};
use semio_framework_plugin::{merge_world_selection_ids, SelectionSet};
use serde_json::Value;

/// 🌍️ The world viewport's marquee result, merged per the host's `merge` mode.
pub fn world_select(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
    let ids: Vec<String> = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
    ctx.scene.runtime.selection.part_ids = merge_world_selection_ids(&ctx.scene.runtime.selection.part_ids, &ids, merge);
}

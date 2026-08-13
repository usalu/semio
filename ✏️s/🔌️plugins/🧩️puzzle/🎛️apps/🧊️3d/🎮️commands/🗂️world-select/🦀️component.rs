//! 🗂️ `world-select` command.

use crate::apps::puzzle3d::config::Puzzle3dSelection;
use semio_framework_plugin::{merge_world_selection_ids, SelectionSet};
use serde_json::Value;
use crate::apps::puzzle3d::Puzzle3dActionCtx;

pub fn world_select(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
    let ids: Vec<String> = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
    ctx.scene.runtime.selection.object_ids = merge_world_selection_ids(&ctx.scene.runtime.selection.object_ids, &ids, merge);
}

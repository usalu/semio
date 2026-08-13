//! 🗂️ `world-pick` command.

use crate::apps::puzzle3d::config::Puzzle3dSelection;
use semio_framework_plugin::{merge_world_selection_ids, SelectionSet};
use serde_json::Value;
use crate::apps::puzzle3d::Puzzle3dActionCtx;
use crate::apps::puzzle3d::puzzle3d_clear_non_object_selection;
use crate::apps::puzzle3d::puzzle3d_clear_selection;

pub fn world_pick(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
    if args.and_then(|value| value.get("id")).is_none_or(Value::is_null) {
        if merge == "replace" {
            puzzle3d_clear_selection(&mut ctx.scene.runtime.selection);
        }
    } else if ctx.scene.runtime.selectable_kinds.objects {
        let index = args.and_then(|value| value.get("id")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
        // 🔓️ Locked/hidden picks are equivalent to background: clear on replace instead of
        // no-opping while the mesh still absorbs the click ahead of `onPointerMissed`.
        match ctx.scene.fixture.objects.get(index).filter(|object| !object.locked && !object.hidden) {
            Some(object) => {
                let id = object.id.clone();
                if merge == "replace" {
                    puzzle3d_clear_non_object_selection(&mut ctx.scene.runtime.selection);
                }
                ctx.scene.runtime.selection.object_ids = merge_world_selection_ids(&ctx.scene.runtime.selection.object_ids, &[id], merge);
            }
            None if merge == "replace" => {
                puzzle3d_clear_selection(&mut ctx.scene.runtime.selection);
            }
            None => {}
        }
    }
}

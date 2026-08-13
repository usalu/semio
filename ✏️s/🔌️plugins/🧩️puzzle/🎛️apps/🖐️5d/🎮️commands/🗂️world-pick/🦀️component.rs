//! 🗂️ `world-pick` command.

use crate::apps::puzzle5d::config::{puzzle5d_clear_non_part_selection, puzzle5d_clear_selection, Puzzle5dSelection};
use crate::apps::puzzle5d::{classify_selection, Puzzle5dActionCtx};
use semio_framework_plugin::{merge_world_selection_ids, SelectionSet};
use serde_json::Value;

/// 🎯️ The world viewport's single-instance pick — `id` is the index into the emitted instance array.
pub fn world_pick(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
    if args.and_then(|value| value.get("id")).is_none_or(|value| value.is_null()) {
        if merge == "replace" {
            puzzle5d_clear_selection(&mut ctx.scene.runtime.selection);
        }
        return;
    }
    let index = args.and_then(|value| value.get("id")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
    match ctx.scene.document.parts.get(index).filter(|part| part.part_2d.locked != Some(true)) {
        Some(part) => {
            let id = part.id.clone();
            ctx.scene.runtime.selection.part_ids = match merge {
                "add" => {
                    let mut merged = ctx.scene.runtime.selection.part_ids.clone();
                    merged.push_unique(id);
                    merged
                }
                "toggle" => {
                    let mut merged = ctx.scene.runtime.selection.part_ids.clone();
                    if merged.contains(&id) {
                        merged.remove_id(&id);
                    } else {
                        merged.push_unique(id);
                    }
                    merged
                }
                _ => {
                    puzzle5d_clear_non_part_selection(&mut ctx.scene.runtime.selection);
                    SelectionSet::from_ids(vec![id])
                }
            };
        }
        None if merge == "replace" => {
            puzzle5d_clear_selection(&mut ctx.scene.runtime.selection);
        }
        None => {}
    }
}

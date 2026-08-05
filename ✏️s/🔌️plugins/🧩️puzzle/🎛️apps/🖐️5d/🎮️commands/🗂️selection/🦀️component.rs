//! 🗂️ Puzzle 5d play app commands — the selection surface: explicit id sets, the document-tree
//! bridge, select-all/clear, the same-kind expansion, the marquee method switch and the two
//! world-viewport picking paths.

use crate::apps::puzzle5d::config::{puzzle5d_clear_non_part_selection, puzzle5d_clear_selection, Puzzle5dSelection};
use crate::apps::puzzle5d::{classify_selection, Puzzle5dActionCtx};
use semio_framework_plugin::{merge_world_selection_ids, SelectionSet};
use serde_json::Value;

/// 🎯️ `setSelection`/`documentSelect`: a flat `ids` list is classified against the document, otherwise
/// the three typed bags are read directly.
pub fn set_selection(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(ids) = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok()) {
        ctx.scene.runtime.selection = classify_selection(&ctx.scene.document, &ids);
    } else {
        let read = |key: &str| args.and_then(|value| value.get(key)).and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok());
        ctx.scene.runtime.selection = Puzzle5dSelection {
            part_ids: SelectionSet::from_ids(read("partIds").unwrap_or_default()),
            grip_ids: SelectionSet::from_ids(read("gripIds").unwrap_or_default()),
            fastener_ids: SelectionSet::from_ids(read("fastenerIds").unwrap_or_default()),
        };
    }
}

pub fn clear_selection(ctx: &mut Puzzle5dActionCtx<'_>) {
    ctx.scene.runtime.selection = Puzzle5dSelection::default();
}

pub fn select_all(ctx: &mut Puzzle5dActionCtx<'_>) {
    ctx.scene.runtime.selection = Puzzle5dSelection { part_ids: ctx.scene.document.parts.iter().map(|part| part.id.clone()).collect(), grip_ids: SelectionSet::default(), fastener_ids: SelectionSet::default() };
}

/// 🧬️ Expands the selection to every part sharing the first selected part's kind. Aborts (emitting
/// nothing at all) when nothing is selected — the pre-migration `return Emit::default()`.
pub fn select_same_kind(ctx: &mut Puzzle5dActionCtx<'_>) {
    let Some(kind) = ctx.scene.runtime.selection.part_ids.first().and_then(|id| ctx.scene.document.parts.iter().find(|part| part.id == id)).map(|part| part.part_kind.clone()) else {
        ctx.abort = true;
        return;
    };
    ctx.scene.runtime.selection.part_ids = ctx.scene.document.parts.iter().filter(|part| part.part_kind == kind).map(|part| part.id.clone()).collect();
}

pub fn set_selection_method(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let method = args.and_then(|value| value.get("method")).and_then(|value| value.as_str()).unwrap_or("rectangle");
    ctx.scene.runtime.selection_method = method.into();
}

/// 🌍️ The world viewport's marquee result, merged per the host's `merge` mode.
pub fn world_select(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
    let ids: Vec<String> = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
    ctx.scene.runtime.selection.part_ids = merge_world_selection_ids(&ctx.scene.runtime.selection.part_ids, &ids, merge);
}

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

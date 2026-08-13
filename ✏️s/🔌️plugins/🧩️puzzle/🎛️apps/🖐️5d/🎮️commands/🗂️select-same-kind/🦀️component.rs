//! 🗂️ `select-same-kind` command.

use crate::apps::puzzle5d::config::{puzzle5d_clear_non_part_selection, puzzle5d_clear_selection, Puzzle5dSelection};
use crate::apps::puzzle5d::{classify_selection, Puzzle5dActionCtx};
use semio_framework_plugin::{merge_world_selection_ids, SelectionSet};
use serde_json::Value;

/// 🧬️ Expands the selection to every part sharing the first selected part's kind. Aborts (emitting
/// nothing at all) when nothing is selected — the pre-migration `return Emit::default()`.
pub fn select_same_kind(ctx: &mut Puzzle5dActionCtx<'_>) {
    let Some(kind) = ctx.scene.runtime.selection.part_ids.first().and_then(|id| ctx.scene.document.parts.iter().find(|part| part.id == id)).map(|part| part.part_kind.clone()) else {
        ctx.abort = true;
        return;
    };
    ctx.scene.runtime.selection.part_ids = ctx.scene.document.parts.iter().filter(|part| part.part_kind == kind).map(|part| part.id.clone()).collect();
}

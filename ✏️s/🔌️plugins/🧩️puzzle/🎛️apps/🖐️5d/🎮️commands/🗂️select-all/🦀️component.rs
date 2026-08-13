//! 🗂️ `select-all` command.

use crate::apps::puzzle5d::config::{puzzle5d_clear_non_part_selection, puzzle5d_clear_selection, Puzzle5dSelection};
use crate::apps::puzzle5d::{classify_selection, Puzzle5dActionCtx};
use semio_framework_plugin::{merge_world_selection_ids, SelectionSet};
use serde_json::Value;

pub fn select_all(ctx: &mut Puzzle5dActionCtx<'_>) {
    ctx.scene.runtime.selection = Puzzle5dSelection { part_ids: ctx.scene.document.parts.iter().map(|part| part.id.clone()).collect(), grip_ids: SelectionSet::default(), fastener_ids: SelectionSet::default() };
}

//! 🗂️ `clear-selection` command.

use crate::apps::puzzle5d::config::{puzzle5d_clear_non_part_selection, puzzle5d_clear_selection, Puzzle5dSelection};
use crate::apps::puzzle5d::{classify_selection, Puzzle5dActionCtx};
use semio_framework_plugin::{merge_world_selection_ids, SelectionSet};
use serde_json::Value;

pub fn clear_selection(ctx: &mut Puzzle5dActionCtx<'_>) {
    ctx.scene.runtime.selection = Puzzle5dSelection::default();
}

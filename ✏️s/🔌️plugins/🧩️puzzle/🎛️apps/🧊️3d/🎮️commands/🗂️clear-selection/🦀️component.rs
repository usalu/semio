//! 🗂️ `clear-selection` command.

use crate::apps::puzzle3d::config::Puzzle3dSelection;
use semio_framework_plugin::{merge_world_selection_ids, SelectionSet};
use serde_json::Value;
use crate::apps::puzzle3d::Puzzle3dActionCtx;

pub fn clear_selection(ctx: &mut Puzzle3dActionCtx<'_>) {
    ctx.scene.runtime.selection = Puzzle3dSelection::default();
}

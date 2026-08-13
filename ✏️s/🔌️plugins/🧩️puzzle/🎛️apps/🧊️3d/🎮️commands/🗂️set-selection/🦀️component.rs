//! 🗂️ `set-selection` command.

use crate::apps::puzzle3d::config::Puzzle3dSelection;
use semio_framework_plugin::{merge_world_selection_ids, SelectionSet};
use serde_json::Value;
use crate::apps::puzzle3d::Puzzle3dActionCtx;

pub fn set_selection(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(selection) = args.and_then(|value| value.get("selection")) {
        if let Ok(parsed) = serde_json::from_value(selection.clone()) {
            ctx.scene.runtime.selection = parsed;
        }
    }
}

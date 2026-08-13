//! 🗂️ `set-selection-method` command.

use crate::apps::puzzle3d::config::Puzzle3dSelection;
use semio_framework_plugin::{merge_world_selection_ids, SelectionSet};
use serde_json::Value;
use crate::apps::puzzle3d::Puzzle3dActionCtx;

pub fn set_selection_method(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let method = args.and_then(|value| value.get("method")).and_then(|value| value.as_str()).unwrap_or("rectangle");
    ctx.scene.runtime.selection_method = method.into();
}

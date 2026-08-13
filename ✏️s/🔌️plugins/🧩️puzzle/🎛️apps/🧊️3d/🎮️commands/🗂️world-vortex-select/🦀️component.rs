//! 🗂️ `world-vortex-select` command.

use crate::apps::puzzle3d::config::Puzzle3dSelection;
use semio_framework_plugin::{merge_world_selection_ids, SelectionSet};
use serde_json::Value;
use crate::apps::puzzle3d::Puzzle3dActionCtx;
use crate::apps::puzzle3d::drive_precompute;
use crate::apps::puzzle3d::puzzle3d_clear_non_vortex_selection;

pub fn world_vortex_select(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if !ctx.scene.runtime.selectable_kinds.vortices {
        return;
    }
    let Some(full_id) = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()) else {
        return;
    };
    let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or(&ctx.scene.runtime.selection_mode_default);
    let merge_mode = match merge {
        "additive" => "add",
        "subtractive" => "remove",
        "invertive" => "toggle",
        "default" => "replace",
        other => other,
    };
    if merge_mode == "replace" {
        puzzle3d_clear_non_vortex_selection(&mut ctx.scene.runtime.selection);
    }
    ctx.scene.runtime.selection.vortex_ids = merge_world_selection_ids(&ctx.scene.runtime.selection.vortex_ids, &[full_id.to_string()], merge_mode);
    drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
}

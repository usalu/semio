//! 🧊️ `set-selection-flag` command.

use crate::apps::puzzle3d::panels::inspection;
use crate::apps::puzzle3d::config::Puzzle3dSelection;
use semio_framework_plugin::SelectionSet;
use serde_json::Value;
use std::collections::HashSet;
use crate::apps::puzzle3d::Puzzle3dActionCtx;
use crate::apps::puzzle3d::apply_puzzle3d_selection_flag;

/// 🙈️ Explicit `{entity, ids}` (the document tree's row actions) patches exactly those; otherwise the
/// whole live object/vortex/target-volume selection is flagged at once (the context menu's path).
pub fn set_selection_flag(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("hidden");
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(true);
    let entity = args.and_then(|value| value.get("entity")).and_then(|value| value.as_str());
    let explicit_ids: Option<Vec<String>> = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok());
    match (entity, explicit_ids) {
        (Some(entity), Some(ids)) => apply_puzzle3d_selection_flag(&mut ctx.scene.fixture, entity, &ids, flag, value),
        _ => {
            let selection = ctx.scene.runtime.selection.clone();
            apply_puzzle3d_selection_flag(&mut ctx.scene.fixture, "object", selection.object_ids.as_slice(), flag, value);
            apply_puzzle3d_selection_flag(&mut ctx.scene.fixture, "vortex", selection.vortex_ids.as_slice(), flag, value);
            apply_puzzle3d_selection_flag(&mut ctx.scene.fixture, "targetVolume", selection.target_volume_ids.as_slice(), flag, value);
        }
    }
}

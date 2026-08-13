//! 🗂️ `select-all` command.

use crate::apps::puzzle3d::config::Puzzle3dSelection;
use semio_framework_plugin::{merge_world_selection_ids, SelectionSet};
use serde_json::Value;
use crate::apps::puzzle3d::Puzzle3dActionCtx;

pub fn select_all(ctx: &mut Puzzle3dActionCtx<'_>) {
    ctx.scene.runtime.selection.object_ids = if ctx.scene.runtime.selectable_kinds.objects {
        ctx.scene.fixture.objects.iter().filter(|object| !object.hidden && !object.locked).map(|object| object.id.clone()).collect::<SelectionSet>()
    } else {
        SelectionSet::default()
    };
    ctx.scene.runtime.selection.vortex_ids.clear();
    ctx.scene.runtime.selection.attraction_ids.clear();
    ctx.scene.runtime.selection.target_volume_ids.clear();
    ctx.scene.runtime.selection.reference_ids.clear();
}

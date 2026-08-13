//! 🗂️ `select-same-kind` command.

use crate::apps::puzzle3d::config::Puzzle3dSelection;
use semio_framework_plugin::{merge_world_selection_ids, SelectionSet};
use serde_json::Value;
use crate::apps::puzzle3d::Puzzle3dActionCtx;

/// 🎯️ Replaces the object selection with every object sharing the first selected object's kind.
/// Aborts the whole action (no config snapshot, no window save) when there is nothing to widen from,
/// exactly as the pre-migration early `return` did.
pub fn select_same_kind(ctx: &mut Puzzle3dActionCtx<'_>) {
    let Some(first_id) = ctx.scene.runtime.selection.object_ids.first().map(str::to_string) else {
        ctx.abort = true;
        return;
    };
    let Some(kind) = ctx.scene.fixture.objects.iter().find(|object| object.id == first_id).and_then(|object| object.object_kind.clone()).filter(|kind| !kind.is_empty()) else {
        ctx.abort = true;
        return;
    };
    ctx.scene.runtime.selection.object_ids = ctx.scene.fixture.objects.iter().filter(|object| object.object_kind.as_deref() == Some(kind.as_str())).map(|object| object.id.clone()).collect::<SelectionSet>();
}

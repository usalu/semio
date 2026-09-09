//! 🗂️ `select-same-kind` command.

use crate::editor::puzzle3d::Puzzle3dActionCtx;
use crate::editor::puzzle3d::PUZZLE3D_GRANULARITY_OBJECT;

/// 🎯️ Widens the selection to every object sharing the clicked object's kind. Selection is
/// framework-owned (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), so this emits one
/// `InteractionWrite` (`ctx.replace_selection`) instead of touching any app-owned selection state —
/// `VcsArtifactApp` applies it through the same `next_selection` machine the reserved
/// `interactionSelect` verb uses, right after this action's own mutations land. Aborts when there is
/// nothing to widen from (no selected object, or one with no kind), exactly as the pre-migration early
/// `return` did.
pub fn select_same_kind(ctx: &mut Puzzle3dActionCtx<'_>) {
    let Some(first_id) = ctx.selected_object_ids().first().cloned() else {
        ctx.abort = true;
        return;
    };
    let Some(kind) = ctx.scene.fixture.objects.iter().find(|object| object.id == first_id).and_then(|object| object.object_kind.clone()).filter(|kind| !kind.is_empty()) else {
        ctx.abort = true;
        return;
    };
    let ids: Vec<String> = ctx.scene.fixture.objects.iter().filter(|object| object.object_kind.as_deref() == Some(kind.as_str())).map(|object| object.id.clone()).collect();
    ctx.replace_selection(PUZZLE3D_GRANULARITY_OBJECT, ids);
}

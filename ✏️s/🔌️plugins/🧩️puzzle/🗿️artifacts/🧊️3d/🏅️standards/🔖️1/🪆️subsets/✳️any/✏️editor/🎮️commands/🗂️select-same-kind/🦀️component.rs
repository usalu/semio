//! 🗂️ `select-same-kind` command.

use crate::editor::puzzle3d::Puzzle3dActionCtx;

/// 🎯️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: selection is
/// framework-owned now and `handle` has no channel to write it back (the six reserved
/// `interactionSelect`-family verbs are the ONLY writer — see `dispatch_interaction_action`, private
/// to `semio-framework-plugin`), so this can no longer replace the selection with same-kind objects
/// itself. Still validates the "nothing to widen from" precondition (aborts exactly as the
/// pre-migration early `return` did) so a client pairing this dispatch with its own follow-up
/// `interactionSelect` still gets a correct abort signal; flagged to the coordinator as a case the W3
/// SDK wave did not provide a mechanism for, not fixed here (framework file, out of this crate's remit).
pub fn select_same_kind(ctx: &mut Puzzle3dActionCtx<'_>) {
    let Some(first_id) = ctx.selected_object_ids().first().cloned() else {
        ctx.abort = true;
        return;
    };
    let Some(kind) = ctx.scene.fixture.objects.iter().find(|object| object.id == first_id).and_then(|object| object.object_kind.clone()).filter(|kind| !kind.is_empty()) else {
        ctx.abort = true;
        return;
    };
    let _ = kind;
    ctx.abort = true;
}

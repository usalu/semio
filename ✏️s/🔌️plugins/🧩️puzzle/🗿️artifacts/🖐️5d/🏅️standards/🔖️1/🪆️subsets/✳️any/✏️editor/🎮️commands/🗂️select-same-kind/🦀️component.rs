//! 🗂️ `select-same-kind` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;

/// 🧬️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: selection is
/// framework-owned now and `handle` has no channel to write it back (see puzzle3d's
/// `select-same-kind` doc comment for the identical limitation), so this can no longer replace the
/// selection with same-kind parts itself. Still validates the "nothing to widen from" precondition
/// (aborts exactly as the pre-migration early `return` did).
pub async fn select_same_kind(ctx: &mut Puzzle5dActionCtx<'_>) {
    let Some(first_id) = ctx.selected_part_ids().first().cloned() else {
        ctx.abort = true;
        return;
    };
    let Some(kind) = ctx.scene.document.parts.iter().find(|part| part.id == first_id).map(|part| part.part_kind.clone()) else {
        ctx.abort = true;
        return;
    };
    let _ = kind;
    ctx.abort = true;
}

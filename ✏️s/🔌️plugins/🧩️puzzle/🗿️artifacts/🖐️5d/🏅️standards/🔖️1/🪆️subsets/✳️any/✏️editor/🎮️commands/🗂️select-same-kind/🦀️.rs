//! 🗂️ `select-same-kind` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use crate::editor::puzzle5d::PUZZLE5D_GRANULARITY_PART;

/// 🧬️ Widens the selection to every part sharing the clicked part's kind. Selection is framework-owned
/// (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), so this emits one `InteractionWrite`
/// through `ctx.replace_selection` — the same sanctioned reducer channel the reserved `interactionSelect`
/// verb uses — instead of touching app-owned selection state. Refuses with ONE localized notice when
/// there is nothing to widen from.
pub fn select_same_kind(ctx: &mut Puzzle5dActionCtx<'_>) {
    let selected = ctx.selected_part_ids();
    if ctx.refuse_without_selection(&selected) {
        return;
    }
    let Some(first_id) = selected.first().cloned() else {
        ctx.abort = true;
        return;
    };
    let Some(kind) = ctx.scene.document.parts.iter().find(|part| part.id == first_id).map(|part| part.part_kind.clone()).filter(|kind| !kind.is_empty()) else {
        ctx.notice(|labels| labels.nothing_selected.as_str());
        ctx.abort = true;
        return;
    };
    let ids: Vec<String> = ctx.scene.document.parts.iter().filter(|part| part.part_kind == kind).map(|part| part.id.clone()).collect();
    ctx.replace_selection(PUZZLE5D_GRANULARITY_PART, ids);
}

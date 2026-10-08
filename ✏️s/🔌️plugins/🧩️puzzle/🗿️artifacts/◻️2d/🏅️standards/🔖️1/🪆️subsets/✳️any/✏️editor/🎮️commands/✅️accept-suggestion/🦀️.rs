//! 💡️ `accept-suggestion` command.

use crate::editor::puzzle2d::{apply_host_events, puzzle2d_restore_brush_slot, puzzle2d_selection_write, Puzzle2dActionCtx};
use semio_framework_pack_json::Value;

/// 🧹️ Drops every trace of the one-shot picker. Called on ACCEPT and on a refused placement alike, so
/// a slot that could not place anything never leaves a sticky menu gating each pane's ordinary context menu.
fn dismiss(ctx: &mut Puzzle2dActionCtx<'_>) {
    ctx.scene.runtime.suggestion_menu = None;
    ctx.scene.runtime.brush_candidates.clear();
    ctx.scene.runtime.brush_candidate_source_handle_id.clear();
    ctx.scene.runtime.brush_candidate_index = 0;
}

/// ✅️ Commits the previewed (or explicitly indexed) candidate as ONE document edit and re-selects the
/// placed node. The board host is rebuilt per dispatch, so the slot is restored from the popup's own
/// handle first; `apply_host_events` then records the host's `brushPlace` as a `create-node` and a `connect-handles`
/// of the same edit. A document with no open handle places nothing and simply closes.
pub fn accept_suggestion(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let requested = args.and_then(|value| value.get("handleId")).and_then(|value| value.as_str()).map(str::to_string);
    if puzzle2d_restore_brush_slot(ctx, requested.as_deref()).is_none() {
        return dismiss(ctx);
    }
    let index = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()).map_or(ctx.scene.runtime.brush_candidate_index, |index| index as usize);
    ctx.host.borrow_mut().brush_set_candidate_index(index);
    ctx.host.borrow_mut().brush_commit_slot();
    apply_host_events(&mut ctx.host.borrow_mut(), ctx.recorder, &mut ctx.scene.runtime);
    dismiss(ctx);
    let placed = ctx.recorder.created_nodes().to_vec();
    if !placed.is_empty() {
        let write = puzzle2d_selection_write(ctx.recorder.value(), &placed);
        ctx.interaction_writes.push(write);
    }
}

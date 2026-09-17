//! 💡️ `accept-suggestion` command.

use crate::editor::puzzle2d::{apply_host_events, fixture_nodes, puzzle2d_restore_brush_slot, puzzle2d_selection_write, Puzzle2dActionCtx};
use serde_json::Value;

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
/// handle first; `apply_host_events` then replays the host's `brushPlace` into the fixture, node and
/// edge landing in the same delta. A document with no open handle places nothing and simply closes.
pub fn accept_suggestion(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    if puzzle2d_restore_brush_slot(ctx).is_none() {
        return dismiss(ctx);
    }
    let index = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()).map_or(ctx.scene.runtime.brush_candidate_index, |index| index as usize);
    ctx.host.borrow_mut().brush_set_candidate_index(index);
    let before: Vec<String> = fixture_nodes(&ctx.scene.fixture).iter().filter_map(|node| node.get("id").and_then(Value::as_str)).map(str::to_string).collect();
    ctx.host.borrow_mut().brush_commit_slot();
    apply_host_events(&mut ctx.host.borrow_mut(), ctx.scene);
    dismiss(ctx);
    let placed: Vec<String> = fixture_nodes(&ctx.scene.fixture).iter().filter_map(|node| node.get("id").and_then(Value::as_str)).filter(|id| !before.iter().any(|known| known == id)).map(str::to_string).collect();
    if !placed.is_empty() {
        let write = puzzle2d_selection_write(&ctx.scene.fixture, &placed);
        ctx.interaction_writes.push(write);
    }
}

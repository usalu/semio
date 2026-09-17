//! 💡️ `open-handle-suggestions` command.

use crate::editor::puzzle2d::config::Puzzle2dSuggestionMenu;
use crate::editor::puzzle2d::{puzzle2d_window_and_measures_scope, Puzzle2dActionCtx};
use serde_json::Value;

/// 💡️ One-shot placement popup over an open handle: enters the board host's brush slot — which
/// resolves the compatible candidates and paints the first one provisionally — WITHOUT arming the
/// brush utility, and records where the popup hangs so exactly one pane renders it. The 2d twin of
/// puzzle3d's `openVortexSuggestions`; the slot state it opens is the same one the armed brush uses,
/// so there is one candidate mechanism, not two.
pub fn open_handle_suggestions(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let Some(handle_id) = args.and_then(|value| value.get("handleId")).and_then(|value| value.as_str()).map(str::to_string) else {
        return;
    };
    ctx.host.borrow_mut().brush_open_slot(&handle_id);
    ctx.scene.runtime.brush_candidate_index = 0;
    let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let window_id = args.and_then(|value| value.get("windowId")).and_then(|value| value.as_str()).filter(|id| !id.is_empty()).or(ctx.window_id).unwrap_or_default().to_string();
    ctx.scene.runtime.suggestion_menu = Some(Puzzle2dSuggestionMenu { x, y, window_id, handle_id });
    *ctx.ui_scope = puzzle2d_window_and_measures_scope();
}

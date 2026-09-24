//! 🖌️ `open-vortex-suggestions` command.

use crate::editor::puzzle3d::config::Puzzle3dSuggestionMenu;
use crate::editor::puzzle3d::Puzzle3dActionCtx;
use dsl::os_pack::json::Value;

/// 💡️ Opens the suggestion menu over `fullId` WITHOUT switching the host-owned utility/tool into brush
/// mode: as its own floating popup, or — `submenu: true` — as the regular context menu's "suggest" submenu,
/// which the host opens the moment that context menu opens so the list is already filling when the pointer
/// reaches the row. 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: no longer also selects the
/// vortex — selection is framework-owned and unwritable from here (see `puzzle3d_brush_target_vortex`'s doc
/// comment).
///
/// 🔎️ Opening the menu points the read-only brush suggestions run at the vortex; the menu lists the free
/// candidates as the run finds them and the viewport paints every tested candidate with its verdict.
pub fn open_vortex_suggestions(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let Some(full_id) = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()).map(str::to_string) else {
        return;
    };
    ctx.scene.runtime.brush_candidate_index = 0;
    let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let window_id = args.and_then(|value| value.get("windowId")).and_then(|value| value.as_str()).filter(|id| !id.is_empty()).unwrap_or(ctx.window_id).to_string();
    let submenu = args.and_then(|value| value.get("submenu")).and_then(Value::as_bool).unwrap_or(false);
    ctx.scene.runtime.suggestion_menu = Some(Puzzle3dSuggestionMenu { x, y, window_id, vortex_full_id: full_id.clone(), submenu });
    ctx.brush_suggestions(|link| link.open_menu(&full_id));
}

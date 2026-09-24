//! 🔓️ `open-vortex-suggestions` command — the world host's suggestion verb, aimed at a grip (the host calls
//! every rim connector a vortex).

use crate::editor::puzzle5d::window::Puzzle5dSuggestionMenu;
use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;
use semio_s_artifact_puzzle_3d::editor::puzzle3d::modes::edit::windows::main::utilities::brush::run_effects;

/// 💡️ Opens the suggestion menu over the grip `fullId` WITHOUT arming the brush: as its own floating popup,
/// or — `submenu: true` — as the regular context menu's "suggest" submenu, which the host opens the moment
/// that context menu opens so the list is already filling when the pointer reaches the row. Opening points
/// the read-only brush suggestions run at the grip and emits the start (or wake) that gesture owes right here,
/// through the same `run_effects` ladder the refresh poll uses — so the search is running before any refresh
/// pass happens to drain the poll; the menu lists the free candidates as the run finds them.
pub fn open_vortex_suggestions(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let Some(full_id) = args.and_then(|value| value.get("fullId")).and_then(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string) else {
        return;
    };
    let number = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_f64).unwrap_or(0.0);
    let window_id = args.and_then(|value| value.get("windowId")).and_then(Value::as_str).filter(|id| !id.is_empty()).unwrap_or(ctx.window_id).to_string();
    let submenu = args.and_then(|value| value.get("submenu")).and_then(Value::as_bool).unwrap_or(false);
    ctx.scene.runtime.brush_candidate_index = 0;
    ctx.scene.runtime.suggestion_menu = Some(Puzzle5dSuggestionMenu { x: number("x"), y: number("y"), window_id, vortex_full_id: full_id.clone(), submenu });
    let effects = ctx
        .brush_suggestions(|link| {
            link.open_menu(&full_id);
            run_effects(link, ctx.tool_run)
        })
        .unwrap_or_default();
    ctx.effects.extend(effects);
}

//! 🤝️ Puzzle 2d play app commands — the engagement HUD's text command line: per-pane input echo,
//! the `select`/`brush`/`fill`/`clear`/`rectangle`/`lasso` verbs, abort, and the brush candidate
//! picker's select control.

use crate::apps::puzzle2d::modes::edit::tools::fill;
use crate::apps::puzzle2d::modes::edit::windows::overview;
use crate::apps::puzzle2d::modes::edit::windows::overview::utilities::select;
use crate::apps::puzzle2d::{puzzle2d_window_and_engagements_scope, Puzzle2dActionCtx, PUZZLE2D_PANES};
use semio_framework_plugin::kernel::HostEffect;
use serde_json::Value;

pub fn engagement_input(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let pane = args.and_then(|value| value.get("pane")).and_then(|value| value.as_str()).unwrap_or(overview::WINDOW_KIND_ID);
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
    if PUZZLE2D_PANES.contains(&pane) {
        ctx.scene.runtime.engagement_input_by_pane.insert(pane.to_string(), value.to_string());
        *ctx.ui_scope = puzzle2d_window_and_engagements_scope();
    }
}

pub fn engagement_submit(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let pane = args.and_then(|value| value.get("pane")).and_then(|value| value.as_str()).unwrap_or(overview::WINDOW_KIND_ID).to_string();
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).map(str::trim).unwrap_or("").to_lowercase();
    let applied = match value.as_str() {
        "select" | "brush" => {
            // 🧰️ Reconcile the engagement text-command utility switch through the host-owned active
            // utility: point the local engine now and let the framework persist the new active utility
            // for the pane via `HostEffect::SetActiveUtility`.
            ctx.host.borrow_mut().set_active_utility(value.as_str());
            ctx.effects.push(HostEffect::SetActiveUtility { window_id: pane.clone(), utility_id: value.clone() });
            true
        }
        "fill" => {
            // 🛠️ Fill is a mode-level tool, not a window utility — activate it through
            // `HostEffect::SetActiveTool`, leaving this window's active utility untouched.
            ctx.effects.push(HostEffect::SetActiveTool { tool_id: fill::TOOL_ID.into() });
            true
        }
        "clear" => {
            ctx.scene.runtime.selected_ids.clear();
            ctx.host.borrow_mut().set_selection_ids(&[]);
            true
        }
        "rectangle" => {
            ctx.scene.runtime.selection_method = "rectangle".into();
            ctx.host.borrow_mut().set_selection_options("rectangle", "replace", true, true, true);
            true
        }
        "lasso" => {
            ctx.scene.runtime.selection_method = "lasso".into();
            ctx.host.borrow_mut().set_selection_options("lasso", "replace", true, true, true);
            true
        }
        _ => false,
    };
    if applied && PUZZLE2D_PANES.contains(&pane.as_str()) {
        ctx.scene.runtime.engagement_input_by_pane.insert(pane, String::new());
    }
}

pub fn engagement_abort(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let pane = args.and_then(|value| value.get("pane")).and_then(|value| value.as_str()).unwrap_or(overview::WINDOW_KIND_ID);
    if PUZZLE2D_PANES.contains(&pane) {
        ctx.scene.runtime.engagement_input_by_pane.insert(pane.to_string(), String::new());
    }
    if ctx.active_utility != select::UTILITY_ID {
        ctx.host.borrow_mut().set_active_utility(select::UTILITY_ID);
        ctx.effects.push(HostEffect::SetActiveUtility { window_id: pane.to_string(), utility_id: select::UTILITY_ID.into() });
    }
}

pub fn engagement_control_select(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let candidate_id = args.and_then(|value| value.get("id").or_else(|| value.get("value"))).and_then(|value| value.as_str()).unwrap_or("");
    if let Some(index) = candidate_id.strip_prefix("puzzle2d.brush.candidate.").and_then(|rest| rest.parse::<usize>().ok()) {
        ctx.host.borrow_mut().brush_set_candidate_index(index);
        ctx.scene.runtime.brush_candidate_index = index;
    }
}

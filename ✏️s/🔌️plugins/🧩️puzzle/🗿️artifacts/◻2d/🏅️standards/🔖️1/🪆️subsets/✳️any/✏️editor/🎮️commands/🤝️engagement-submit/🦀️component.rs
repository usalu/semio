//! 🤝️ `engagement-submit` command.

use crate::editor::puzzle2d::modes::edit::tools::fill;
use crate::editor::puzzle2d::modes::edit::windows::overview;
use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::select;
use crate::editor::puzzle2d::{puzzle2d_window_and_engagements_scope, Puzzle2dActionCtx, PUZZLE2D_PANES};
use semio_framework_plugin::kernel::HostEffect;
use serde_json::Value;

pub fn engagement_submit(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let pane = args.and_then(|value| value.get("pane")).and_then(|value| value.as_str()).unwrap_or(overview::WINDOW_KIND_ID).to_string();
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).map_or("", str::trim).to_lowercase();
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
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: "clear"/"rectangle"/"lasso"
        // dropped — selection/method are framework-owned now (`clearSelection`/`interactionSelect`'s
        // `method` arg), unreachable from this app-level typed-command box.
        _ => false,
    };
    if applied && PUZZLE2D_PANES.contains(&pane.as_str()) {
        ctx.scene.runtime.engagement_input_by_pane.insert(pane, String::new());
    }
}

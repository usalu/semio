//! 🤝️ `engagement-submit` command.

use crate::editor::puzzle2d::modes::edit::tools::fill;
use crate::editor::puzzle2d::Puzzle2dActionCtx;
use semio_framework_plugin::kernel::Effect;
use serde_json::Value;

pub fn engagement_submit(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).map_or("", str::trim).to_lowercase();
    let applied = match value.as_str() {
        "select" | "brush" => {
            // 🧰️ Reconcile the engagement text-command utility switch through the host-owned active
            // utility: point the local engine now and let the framework persist the new active utility
            // for the pane via `Effect::SetActiveUtility`.
            ctx.host.borrow_mut().set_active_utility(value.as_str());
            ctx.effects.push(Effect::SetActiveUtility { window_id: ctx.window_id.unwrap_or(ctx.window_kind).to_string(), utility_id: value.clone() });
            true
        }
        "fill" => {
            // 🛠️ Fill is a mode-level tool, not a window utility — activate it through
            // `Effect::SetActiveTool`, leaving this window's active utility untouched.
            ctx.effects.push(Effect::SetActiveTool { tool_id: fill::TOOL_ID.into() });
            true
        }
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: "clear"/"rectangle"/"lasso"
        // dropped — selection/method are framework-owned now (`clearSelection`/`interactionSelect`'s
        // `method` arg), unreachable from this app-level typed-command box.
        _ => false,
    };
    if applied {
        ctx.scene.runtime.engagement_input_by_pane.insert(ctx.window_kind.to_string(), String::new());
    }
}

//! 🤝️ `engagement-abort` command.

use crate::editor::puzzle2d::modes::edit::windows::overview;
use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::select;
use crate::editor::puzzle2d::{Puzzle2dActionCtx, PUZZLE2D_PANES};
use semio_framework_plugin::kernel::Effect;
use serde_json::Value;

pub fn engagement_abort(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let pane = args.and_then(|value| value.get("pane")).and_then(|value| value.as_str()).unwrap_or(overview::WINDOW_KIND_ID);
    if PUZZLE2D_PANES.contains(&pane) {
        ctx.scene.runtime.engagement_input_by_pane.insert(pane.to_string(), String::new());
    }
    if ctx.active_utility != select::UTILITY_ID {
        ctx.host.borrow_mut().set_active_utility(select::UTILITY_ID);
        ctx.effects.push(Effect::SetActiveUtility { window_id: pane.to_string(), utility_id: select::UTILITY_ID.into() });
    }
}

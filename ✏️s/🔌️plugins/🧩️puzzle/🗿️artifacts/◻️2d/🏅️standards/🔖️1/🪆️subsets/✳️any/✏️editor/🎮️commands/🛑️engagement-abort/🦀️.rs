//! 🤝️ `engagement-abort` command.

use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::select;
use crate::editor::puzzle2d::Puzzle2dActionCtx;
use semio_framework_plugin::kernel::Effect;
use serde_json::Value;

pub fn engagement_abort(ctx: &mut Puzzle2dActionCtx<'_>, _args: Option<&Value>) {
    ctx.scene.runtime.engagement_input_by_pane.insert(ctx.window_kind.to_string(), String::new());
    if ctx.active_utility != select::UTILITY_ID {
        ctx.host.borrow_mut().set_active_utility(select::UTILITY_ID);
        ctx.effects.push(Effect::SetActiveUtility { window_id: ctx.window_id.unwrap_or(ctx.window_kind).to_string(), utility_id: select::UTILITY_ID.into() });
    }
}

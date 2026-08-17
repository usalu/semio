//! 🖌️ `set-fill-count` command.

use crate::editor::puzzle2d::modes::edit::tools::fill;
use crate::editor::puzzle2d::modes::edit::windows::overview;
use crate::editor::puzzle2d::{apply_brush_place_payload, Puzzle2dActionCtx};
use semio_framework_plugin::kernel::Effect;
use serde_json::Value;

fn apply_fill_placements(ctx: &mut Puzzle2dActionCtx<'_>, step_json: &str) {
    let Ok(progress) = serde_json::from_str::<Value>(step_json) else {
        return;
    };
    let Some(placements) = progress.get("placements").and_then(|value| value.as_array()) else {
        return;
    };
    for placement in placements {
        apply_brush_place_payload(&mut ctx.scene.fixture, placement);
    }
}

/// 🪣️ The Fill tool's one-shot entry point: activates the tool, runs the whole session in one step
/// and splices every placement into the fixture.
pub fn set_fill_count(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let count = args
        .and_then(|value| value.get("count").or_else(|| value.get("value")))
        .and_then(|value| value.as_f64())
        .map_or(0, |value| value.round().max(0.0) as u32)
        .min(fill::PUZZLE2D_FILL_COUNT_MAX);
    ctx.scene.runtime.fill_count = count;
    ctx.effects.push(Effect::SetActiveTool { tool_id: fill::TOOL_ID.into() });
    ctx.host.borrow_mut().set_active_utility(overview::utilities::brush::UTILITY_ID);
    ctx.host.borrow_mut().brush_fill_session_begin(count, 1);
    let step = ctx.host.borrow_mut().brush_fill_session_step(count.max(1));
    apply_fill_placements(ctx, &step);
}

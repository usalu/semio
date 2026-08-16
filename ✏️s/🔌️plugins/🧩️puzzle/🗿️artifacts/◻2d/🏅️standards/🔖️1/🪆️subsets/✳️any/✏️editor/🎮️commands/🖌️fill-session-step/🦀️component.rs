//! 🖌️ `fill-session-step` command.

use crate::editor::puzzle2d::modes::edit::options::brush::{PUZZLE2D_SUGGESTION_OFFSET_MAX, PUZZLE2D_SUGGESTION_OFFSET_MIN};
use crate::editor::puzzle2d::modes::edit::tools::fill;
use crate::editor::puzzle2d::modes::edit::windows::overview;
use crate::editor::puzzle2d::{apply_brush_place_payload, apply_host_events, puzzle2d_kind_ids, puzzle2d_window_and_engagements_scope, puzzle2d_window_and_measures_scope, puzzle2d_window_only_scope, Puzzle2dActionCtx};
use semio_framework_plugin::kernel::HostEffect;
use serde_json::{json, Value};
use std::collections::BTreeMap;

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

pub fn fill_session_step(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let budget = args.and_then(|value| value.get("chunkBudget")).and_then(|value| value.as_u64()).unwrap_or(8) as u32;
    let step = ctx.host.borrow_mut().brush_fill_session_step(budget);
    apply_fill_placements(ctx, &step);
}

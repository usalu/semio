//! 🖌️ `cycle-candidate` command.

use crate::apps::puzzle2d::modes::edit::options::brush::{PUZZLE2D_SUGGESTION_OFFSET_MAX, PUZZLE2D_SUGGESTION_OFFSET_MIN};
use crate::apps::puzzle2d::modes::edit::tools::fill;
use crate::apps::puzzle2d::modes::edit::windows::overview;
use crate::apps::puzzle2d::{apply_brush_place_payload, apply_host_events, puzzle2d_kind_ids, puzzle2d_window_and_engagements_scope, puzzle2d_window_and_measures_scope, puzzle2d_window_only_scope, Puzzle2dActionCtx};
use semio_framework_plugin::kernel::HostEffect;
use serde_json::{json, Value};
use std::collections::BTreeMap;

pub fn cycle_candidate(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let forward = args.and_then(|value| value.get("forward")).and_then(|value| value.as_bool()).unwrap_or(true);
    ctx.host.borrow_mut().brush_cycle_candidate(forward);
    ctx.scene.runtime.brush_candidate_index = ctx.scene.runtime.brush_candidate_index.saturating_add(1);
    *ctx.ui_scope = puzzle2d_window_and_engagements_scope();
}

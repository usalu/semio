//! 🖌️ `set-suggestion-offset` command.

use crate::editor::puzzle2d::modes::edit::options::brush::{PUZZLE2D_SUGGESTION_OFFSET_MAX, PUZZLE2D_SUGGESTION_OFFSET_MIN};
use crate::editor::puzzle2d::modes::edit::tools::fill;
use crate::editor::puzzle2d::modes::edit::windows::overview;
use crate::editor::puzzle2d::{apply_brush_place_payload, apply_host_events, puzzle2d_kind_ids, puzzle2d_window_and_engagements_scope, puzzle2d_window_and_measures_scope, puzzle2d_window_only_scope, Puzzle2dActionCtx};
use semio_framework_plugin::kernel::HostEffect;
use serde_json::{json, Value};
use std::collections::BTreeMap;

pub fn set_suggestion_offset(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let distance = args.and_then(|value| value.get("distance").or_else(|| value.get("value"))).and_then(|value| value.as_f64());
    if let Some(distance) = distance {
        let clamped = distance.clamp(PUZZLE2D_SUGGESTION_OFFSET_MIN, PUZZLE2D_SUGGESTION_OFFSET_MAX);
        ctx.scene.runtime.suggestion_offset = clamped;
        ctx.host.borrow_mut().set_suggestion_offset(clamped);
        *ctx.ui_scope = puzzle2d_window_and_measures_scope();
    }
}

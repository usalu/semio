//! 🖌️ `cancel-slot` command.

use crate::editor::puzzle2d::modes::edit::options::brush::{PUZZLE2D_SUGGESTION_OFFSET_MAX, PUZZLE2D_SUGGESTION_OFFSET_MIN};
use crate::editor::puzzle2d::modes::edit::tools::fill;
use crate::editor::puzzle2d::modes::edit::windows::overview;
use crate::editor::puzzle2d::{apply_brush_place_payload, apply_host_events, puzzle2d_kind_ids, puzzle2d_window_and_engagements_scope, puzzle2d_window_and_measures_scope, puzzle2d_window_only_scope, Puzzle2dActionCtx};
use semio_framework_plugin::kernel::HostEffect;
use serde_json::{json, Value};
use std::collections::BTreeMap;

pub fn cancel_slot(ctx: &mut Puzzle2dActionCtx<'_>) {
    ctx.host.borrow_mut().brush_cancel_slot();
}

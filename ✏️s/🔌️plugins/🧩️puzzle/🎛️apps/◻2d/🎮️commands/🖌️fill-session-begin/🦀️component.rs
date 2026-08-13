//! 🖌️ `fill-session-begin` command.

use crate::apps::puzzle2d::modes::edit::options::brush::{PUZZLE2D_SUGGESTION_OFFSET_MAX, PUZZLE2D_SUGGESTION_OFFSET_MIN};
use crate::apps::puzzle2d::modes::edit::tools::fill;
use crate::apps::puzzle2d::modes::edit::windows::overview;
use crate::apps::puzzle2d::{apply_brush_place_payload, apply_host_events, puzzle2d_kind_ids, puzzle2d_window_and_engagements_scope, puzzle2d_window_and_measures_scope, puzzle2d_window_only_scope, Puzzle2dActionCtx};
use semio_framework_plugin::kernel::HostEffect;
use serde_json::{json, Value};
use std::collections::BTreeMap;

pub fn fill_session_begin(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let max_count = args.and_then(|value| value.get("maxCount")).and_then(|value| value.as_u64()).unwrap_or(0) as u32;
    let seed = args.and_then(|value| value.get("seed")).and_then(|value| value.as_u64()).unwrap_or(1) as u32;
    ctx.host.borrow_mut().brush_fill_session_begin(max_count, u64::from(seed));
}

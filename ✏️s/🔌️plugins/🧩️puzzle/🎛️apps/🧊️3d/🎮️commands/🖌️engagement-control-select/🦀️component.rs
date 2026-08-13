//! 🖌️ `engagement-control-select` command.

use crate::apps::puzzle3d::config::Puzzle3dSuggestionMenu;
use crate::artifacts::puzzle3d::schema::{BrushPlacePayload, Puzzle3dEngineCommand, Puzzle3dEngineOutcome};
use semio_framework_plugin::SelectionSet;
use serde_json::Value;
use crate::apps::puzzle3d::Puzzle3dActionCtx;

/// 🎚️ The brush placement picker's select — its option values are `puzzle3d.brush.candidate.<index>`.
pub fn engagement_control_select(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let candidate_id = args.and_then(|value| value.get("id").or_else(|| value.get("value"))).and_then(|value| value.as_str()).unwrap_or("");
    if let Some(index) = candidate_id.strip_prefix("puzzle3d.brush.candidate.").and_then(|rest| rest.parse::<usize>().ok()) {
        ctx.scene.runtime.brush_candidate_index = index;
    }
}

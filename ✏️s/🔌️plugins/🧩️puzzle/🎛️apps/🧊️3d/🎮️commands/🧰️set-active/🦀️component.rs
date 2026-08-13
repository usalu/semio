//! 🧰️ `set-active` command.

use crate::apps::puzzle3d::modes::edit::windows::main::utilities;
use crate::apps::puzzle3d::{drive_precompute, puzzle3d_scene_active_utility, Puzzle3dActionCtx};
use semio_framework_plugin::SET_ACTIVE_UTILITY_ACTION_ID;
use serde_json::Value;

pub fn set_active(ctx: &mut Puzzle3dActionCtx<'_>, action: &str, args: Option<&Value>) {
    if action == SET_ACTIVE_UTILITY_ACTION_ID {
        if let Some(utility_id) = args.and_then(|value| value.get("utilityId")).and_then(|value| value.as_str()) {
            ctx.scene.runtime.active_utility_by_window_id.insert(ctx.window_id.to_string(), utility_id.to_string());
            ctx.scene.active_utility = utility_id.to_string();
        }
    } else {
        let tool_id = args.and_then(|value| value.get("toolId")).and_then(|value| value.as_str()).filter(|id| !id.is_empty());
        ctx.scene.runtime.active_tool_id = tool_id.map(str::to_string);
        ctx.scene.active_utility = puzzle3d_scene_active_utility(&ctx.scene.runtime, Some(ctx.window_id));
    }
    ctx.app.clear_transform_session();
    ctx.scene.runtime.hovered_object_id = None;
    ctx.scene.runtime.hovered_vortex_full_id = None;
    ctx.scene.runtime.suggestion_menu = None;
    ctx.scene.runtime.engagement_input = String::new();
    ctx.scene.runtime.brush_candidate_index = 0;
    if ctx.scene.active_utility == utilities::brush::UTILITY_ID || ctx.scene.active_utility == "fill" {
        drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
    }
}

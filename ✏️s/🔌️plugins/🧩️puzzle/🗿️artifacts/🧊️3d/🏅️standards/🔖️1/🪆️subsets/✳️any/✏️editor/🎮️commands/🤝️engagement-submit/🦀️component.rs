//! 🤝️ `engagement-submit` command.

use crate::editor::puzzle3d::modes::edit::windows::main::utilities;
use crate::editor::puzzle3d::{apply_puzzle3d_fill_count, apply_puzzle3d_focus_selection, drive_precompute, Puzzle3dActionCtx, PUZZLE3D_FILL_COUNT_MAX};
use semio_framework_plugin::strip_engagement_prefix;
use serde_json::Value;

pub fn engagement_submit(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let raw = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("").trim().to_string();
    if let Some(rest) = strip_engagement_prefix(&raw, "fill") {
        ctx.scene.active_utility = "fill".into();
        drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
        let count = rest.parse::<u32>().ok().unwrap_or(ctx.scene.runtime.fill_count).min(PUZZLE3D_FILL_COUNT_MAX);
        apply_puzzle3d_fill_count(&mut ctx.app.precompute.borrow_mut(), ctx.scene, count);
    } else {
        match raw.to_lowercase().as_str() {
            "brush" => {
                ctx.scene.active_utility = utilities::brush::UTILITY_ID.into();
                drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
            }
            // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: "clear"/"rectangle"/"lasso"
            // dropped — selection/method are framework-owned now (`clearSelection`/`interactionSelect`'s
            // `method` arg), unreachable from this app-level typed-command box.
            "zoom" => {
                let object_ids = ctx.selected_object_ids();
                apply_puzzle3d_focus_selection(ctx.scene, &object_ids);
            }
            _ => {}
        }
    }
    ctx.scene.runtime.engagement_input = String::new();
}

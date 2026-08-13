//! 🤝️ `engagement-submit` command.

use crate::apps::puzzle3d::modes::edit::windows::main::utilities;
use crate::apps::puzzle3d::{apply_puzzle3d_fill_count, apply_puzzle3d_focus_selection, drive_precompute, puzzle3d_clear_selection, Puzzle3dActionCtx, PUZZLE3D_DEFAULT_UTILITY, PUZZLE3D_FILL_COUNT_MAX};
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
            "zoom" => apply_puzzle3d_focus_selection(ctx.scene),
            "clear" => puzzle3d_clear_selection(&mut ctx.scene.runtime.selection),
            "rectangle" => ctx.scene.runtime.selection_method = "rectangle".into(),
            "lasso" => ctx.scene.runtime.selection_method = "lasso".into(),
            _ => {}
        }
    }
    ctx.scene.runtime.engagement_input = String::new();
}

//! 🧰️ Puzzle 5d play app commands — the framework-injected utility switch.

use crate::apps::puzzle5d::{Puzzle5dActionCtx, PUZZLE5D_PLAY_WINDOWS};
use serde_json::Value;

/// 🧰️ B1: this Command IS the utility switch now (was host-applied ambient
/// `view_state.active_utility_id`/`active_utility_by_window_id` — the host no longer owns that state,
/// `Puzzle5dConfig` does), so this arm must itself write the new value before clearing in-progress
/// engagement scratch and refreshing the placement engine.
pub fn set_active(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(utility_id) = args.and_then(|value| value.get("utilityId")).and_then(|value| value.as_str()) {
        ctx.scene.runtime.active_utility_by_window_id.insert(ctx.window_id.to_string(), utility_id.to_string());
        ctx.scene.active_utility = utility_id.to_string();
    }
    for window in PUZZLE5D_PLAY_WINDOWS {
        ctx.scene.runtime.engagement_input_by_window.insert(window.to_string(), String::new());
    }
    ctx.scene.runtime.brush_candidate_index = 0;
    if ctx.scene.active_utility == "brush" || ctx.scene.active_utility == "fill" {
        ctx.app.drive_precompute(ctx.scene);
    }
}

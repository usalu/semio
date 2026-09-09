//! 📨️ `engagement-submit` command.

use crate::editor::puzzle5d::modes::edit::windows::world3d;
use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;

/// ⌨️ One submitted token: `select`/`brush`/`fill` switch the utility (the 3D window's `select` lands
/// on the `move` gumball instead), `clear` drops the selection, `rectangle`/`lasso` set the marquee.
pub fn engagement_submit(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).map_or("", str::trim).to_lowercase();
    match value.as_str() {
        "select" if ctx.window_kind == world3d::WINDOW_KIND_ID => ctx.scene.active_utility = "move".into(),
        "select" | "brush" | "fill" => {
            ctx.scene.active_utility = if value == "select" { "select".into() } else { value };
            if ctx.scene.active_utility != "select" {
                ctx.app.drive_precompute(ctx.scene);
            }
        }
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: "clear"/"rectangle"/"lasso"
        // dropped — selection/method are framework-owned now (`clearSelection`/`interactionSelect`'s
        // `method` arg), unreachable from this app-level typed-command box.
        _ => {}
    }
    ctx.scene.runtime.engagement_input_by_window.insert(ctx.window_id.to_string(), String::new());
}

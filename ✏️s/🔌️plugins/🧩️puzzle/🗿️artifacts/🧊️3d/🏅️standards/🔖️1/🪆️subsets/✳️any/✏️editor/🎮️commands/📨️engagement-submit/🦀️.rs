//! 🤝️ `engagement-submit` command.

use crate::editor::puzzle3d::commands::set_fill_count;
use crate::editor::puzzle3d::modes::edit::windows::main::utilities;
use crate::editor::puzzle3d::{apply_puzzle3d_focus_selection, drive_precompute, Puzzle3dActionCtx, PUZZLE3D_FILL_COUNT_MAX, PUZZLE3D_SELECTION_METHOD_LASSO, PUZZLE3D_SELECTION_METHOD_PICK, PUZZLE3D_SELECTION_METHOD_RECTANGLE};
use dsl::os_pack::json::Value;
use semio_framework_plugin::strip_engagement_prefix;

/// 🗣️ Every sub-verb the engagement input accepts, in the order the window's placeholder advertises
/// them (`🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs` `engagement`). The placeholder is derived from this
/// list, so an advertised verb that no arm below implements cannot exist.
pub const PUZZLE3D_ENGAGEMENT_VERBS: &[&str] = &["brush", "fill <n>", "zoom", "clear", "pick", "rectangle", "lasso"];

pub fn engagement_submit(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let raw = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("").trim().to_string();
    if let Some(rest) = strip_engagement_prefix(&raw, "fill") {
        ctx.scene.active_utility = "fill".into();
        drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
        let count = rest.parse::<u32>().ok().unwrap_or(ctx.scene.runtime.fill_count).min(PUZZLE3D_FILL_COUNT_MAX);
        ctx.effects.push(set_fill_count::request(count));
    } else {
        match raw.to_lowercase().as_str() {
            "brush" => {
                ctx.scene.active_utility = utilities::brush::UTILITY_ID.into();
                drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
            }
            "zoom" => {
                let object_ids = ctx.selected_object_ids();
                apply_puzzle3d_focus_selection(ctx.scene, &object_ids);
            }
            // 🧹️ Empties the framework-owned `vortex` domain through the one sanctioned reducer channel
            // (`Emit.interaction_writes`) — never by poking selection state this app does not own.
            "clear" => ctx.clear_selection(),
            // 🖱️ Marquee method for the next viewport drag, read back by `world_selection_json` into
            // `World3dHost.selection.method`. `pick` is the way back to the default sweep.
            "pick" => ctx.scene.runtime.selection_method = PUZZLE3D_SELECTION_METHOD_PICK.into(),
            "rectangle" => ctx.scene.runtime.selection_method = PUZZLE3D_SELECTION_METHOD_RECTANGLE.into(),
            "lasso" => ctx.scene.runtime.selection_method = PUZZLE3D_SELECTION_METHOD_LASSO.into(),
            _ => {}
        }
    }
    ctx.scene.runtime.engagement_input = String::new();
}

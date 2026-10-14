//! 🤝️ `engagement-submit` command.

use crate::set_fill_count;
use semio_s_artifact_puzzle_3d_editor_modes::windows::main::utilities;
use {semio_s_artifact_puzzle_3d_editor_snapshot::apply_puzzle3d_focus_selection, semio_s_artifact_puzzle_3d_editor_session::drive_precompute, semio_s_artifact_puzzle_3d_editor_session::Puzzle3dActionCtx, semio_s_artifact_puzzle_3d_editor_config::PUZZLE3D_SELECTION_METHOD_LASSO, semio_s_artifact_puzzle_3d_editor_config::PUZZLE3D_SELECTION_METHOD_PICK, semio_s_artifact_puzzle_3d_editor_config::PUZZLE3D_SELECTION_METHOD_RECTANGLE};
use semio_framework_pack_json::Value;
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::strip_engagement_prefix;
use semio_framework_tool_run::TOOL_RUN_START_ACTION_ID;

/// 🗣️ `fill <n>` arms the Fill tool, sets the shared count and starts a fill run through the framework's
/// `toolRunStart`; the other verbs switch utilities or selection methods in place.
pub fn engagement_submit(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let raw = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("").trim().to_string();
    if let Some(rest) = strip_engagement_prefix(&raw, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID) {
        ctx.scene.active_utility = semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID.into();
        let count = rest.parse::<u32>().ok().unwrap_or(ctx.scene.runtime.fill_count);
        ctx.effects.push(set_fill_count::request(count));
        let start = semio_framework::dsl_value!({ TOOL_RUN_ARG_TOOL_ID: semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID });
        ctx.effects.push(Effect::DispatchAction { req: semio_framework_plugin::RequestId(semio_framework_job::allocate_operation_id().0), action: TOOL_RUN_START_ACTION_ID.into(), args: Some(start), delay_ms: 0 });
    } else {
        match raw.to_lowercase().as_str() {
            "brush" => {
                ctx.scene.active_utility = utilities::brush::UTILITY_ID.into();
                drive_precompute(&mut ctx.session.precompute.borrow_mut(), ctx.scene);
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

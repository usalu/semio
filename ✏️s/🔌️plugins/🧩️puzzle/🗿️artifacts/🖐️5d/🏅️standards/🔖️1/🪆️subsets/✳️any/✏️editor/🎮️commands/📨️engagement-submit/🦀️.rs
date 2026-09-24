//! 📨️ `engagement-submit` command.

use crate::editor::puzzle5d::commands::{focus_selection, rotate_selection, scale_selection, set_fill_count, translate_selection};
use crate::editor::puzzle5d::modes::edit::tools::fill as fill_tool;
use crate::editor::puzzle5d::modes::edit::windows::world3d;
use crate::editor::puzzle5d::{Puzzle5dActionCtx, PUZZLE5D_DEFAULT_UTILITY};
use dsl::json;
use dsl::os_pack::json::Value;
use semio_framework_plugin::kernel::Effect;
use semio_framework_tool_run::{TOOL_RUN_ARG_TOOL_ID, TOOL_RUN_START_ACTION_ID};

/// 🗣️ Every sub-verb the engagement input accepts, in the order the window's placeholder advertises them
/// (`🎭️modes/✏️edit/🦀️.rs`'s `puzzle5d_engagement`). The placeholder is DERIVED from this list, so an
/// advertised verb that no arm below implements cannot exist.
pub const PUZZLE5D_ENGAGEMENT_VERBS: &[&str] = &["select", "brush", "fill <n>", "clear", "zoom", "move dx dy [dz]", "rotate deg", "scale f"];

/// 🔁️ The spatial rotation a typed `rotate <deg>` means: about the world up axis, the one turn a plan-view
/// operator asks for by naming a single angle.
const PUZZLE5D_ENGAGEMENT_ROTATE_AXIS: [f64; 3] = [0.0, 0.0, 1.0];

/// ⌨️ One submitted line. `fill [<n>]` arms the Fill TOOL, sets the shared count and starts a run through the
/// framework's `toolRunStart` — one submit both retargets and starts. `select`/`brush` switch the window utility
/// (the 3D window's `select` lands on the `move` gumball), `clear` empties the selection through the framework
/// `vortex` domain, `zoom` frames it, and `move`/`rotate`/`scale` transform the selected parts' SPATIAL pose —
/// the same pose the gumball verbs move, so a typed transform and a dragged one are the same edit.
pub fn engagement_submit(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let raw = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).map_or("", str::trim).to_lowercase();
    let mut words = raw.split_whitespace();
    let verb = words.next().unwrap_or("");
    let numbers: Vec<f64> = words.filter_map(|word| word.parse::<f64>().ok()).filter(|number| number.is_finite()).collect();
    match verb {
        "fill" => {
            ctx.scene.active_utility = fill_tool::TOOL_ID.into();
            let count = numbers.first().filter(|count| **count >= 0.0 && count.round() <= f64::from(u32::MAX)).map_or(ctx.scene.runtime.fill_count, |count| count.round() as u32);
            ctx.effects.push(set_fill_count::request(count));
            ctx.effects.push(Effect::DispatchAction {
                req: semio_framework_plugin::RequestId(semio_framework_job::allocate_operation_id().0),
                action: TOOL_RUN_START_ACTION_ID.into(),
                args: semio_framework::optional_json_to_dsl(Some(serde_json::json!({ TOOL_RUN_ARG_TOOL_ID: fill_tool::TOOL_ID }))),
                delay_ms: 0,
            });
        }
        "select" => ctx.scene.active_utility = if ctx.window_kind == world3d::WINDOW_KIND_ID { world3d::utilities::transform::UTILITY_ID.into() } else { PUZZLE5D_DEFAULT_UTILITY.into() },
        "brush" => ctx.scene.active_utility = "brush".into(),
        // 🧹️ Empties the framework-owned `vortex` domain through the one sanctioned reducer channel, never by
        // poking selection state this app does not own.
        "clear" => ctx.clear_selection(),
        "zoom" => focus_selection::focus_selection(ctx),
        "move" if numbers.len() >= 2 => translate_selection::translate_selection(ctx, Some(&json!({ "dx": numbers[0], "dy": numbers[1], "dz": numbers.get(2).copied().unwrap_or(0.0) }))),
        "rotate" if !numbers.is_empty() => rotate_selection::rotate_selection(
            ctx,
            Some(&json!({ "ax": PUZZLE5D_ENGAGEMENT_ROTATE_AXIS[0], "ay": PUZZLE5D_ENGAGEMENT_ROTATE_AXIS[1], "az": PUZZLE5D_ENGAGEMENT_ROTATE_AXIS[2], "angle": numbers[0].to_radians() })),
        ),
        "scale" if numbers.first().is_some_and(|factor| *factor > 0.0) => scale_selection::scale_selection(ctx, Some(&json!({ "sx": numbers[0], "sy": numbers[0], "sz": numbers[0] }))),
        _ => {}
    }
    ctx.scene.runtime.engagement_input_by_window.insert(ctx.window_id.to_string(), String::new());
}

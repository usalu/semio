//! 🚧️ `set-brush-placement-contact-tolerance` command.
use semio_s_artifact_puzzle_5d_mut_any_8::editor::puzzle5d::entities::puzzle5d_absolute_or_delta;

use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::interaction::Puzzle5dActionCtx;


use semio_framework_pack_json::Value;

/// 🖌️ The collision budget every brush/fill placement is tested against — an absolute `value` or a
/// `delta` nudge, clamped into the unit band the brush option's slider declares.
pub fn set_brush_placement_contact_tolerance(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = puzzle5d_absolute_or_delta(args, ctx.scene.runtime.contact_tolerance) {
        ctx.scene.runtime.contact_tolerance = value.clamp(0.0, 1.0);
    }
}

//! 🚧️ `set-brush-placement-contact-tolerance` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;

pub fn set_brush_placement_contact_tolerance(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64()) {
        ctx.scene.runtime.contact_tolerance = value.clamp(0.0, 1.0);
    }
}

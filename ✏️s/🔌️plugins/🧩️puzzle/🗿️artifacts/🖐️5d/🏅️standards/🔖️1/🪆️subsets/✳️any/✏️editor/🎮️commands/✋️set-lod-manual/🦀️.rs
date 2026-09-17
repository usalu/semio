//! 🔭️ `set-lod-manual` command — the manual detail override, clamped into the slider's own band.

use crate::editor::puzzle5d::{Puzzle5dActionCtx, PUZZLE5D_LOD_SLIDER_MAX, PUZZLE5D_LOD_SLIDER_MIN};
use dsl::os_pack::json::Value;

pub fn set_lod_manual(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = args.and_then(|value| value.get("value")).and_then(Value::as_f64) {
        ctx.scene.runtime.lod_manual = value.clamp(PUZZLE5D_LOD_SLIDER_MIN, PUZZLE5D_LOD_SLIDER_MAX);
    }
}

//! 🔭️ `set-lod-automatic` command — whether the world pane picks its detail level from the zoom.

use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::interaction::Puzzle5dActionCtx;
use semio_framework_pack_json::Value;

pub fn set_lod_automatic(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.lod_automatic = args.and_then(|value| value.get("pressed")).and_then(Value::as_bool).unwrap_or(!ctx.scene.runtime.lod_automatic);
}

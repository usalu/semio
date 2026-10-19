//! 🌐️ `set-grid-visible` command — toggles whether the target pane draws its grid at all.

use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::interaction::Puzzle5dActionCtx;
use semio_framework_pack_json::Value;

pub fn set_grid_visible(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.grid_visible = args.and_then(|value| value.get("pressed")).and_then(Value::as_bool).unwrap_or(!ctx.scene.runtime.grid_visible);
}

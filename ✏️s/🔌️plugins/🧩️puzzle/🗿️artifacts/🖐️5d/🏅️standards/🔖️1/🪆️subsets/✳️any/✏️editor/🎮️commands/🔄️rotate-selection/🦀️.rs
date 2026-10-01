//! 🔄️ `rotate-selection` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;

/// 🔄️ One gumball turn `{ax, ay, az, angle}` (or a typed `rotate deg` submit) as ONE transform-tool
/// transaction: the `rotate-selection3d` leaf, each target turned about its own origin.
pub fn rotate_selection(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    ctx.commit_gumball("rotateSelection", args);
}

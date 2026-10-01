//! 📏️ `scale-selection` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;

/// 📏️ One gumball scaling `{sx, sy, sz}` (or a typed `scale f` submit) as ONE transform-tool transaction: the
/// `scale-selection3d` leaf, each target scaled about its own origin.
pub fn scale_selection(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    ctx.commit_gumball("scaleSelection", args);
}

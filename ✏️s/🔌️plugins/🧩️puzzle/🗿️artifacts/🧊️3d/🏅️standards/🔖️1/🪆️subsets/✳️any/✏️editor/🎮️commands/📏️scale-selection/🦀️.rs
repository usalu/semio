//! 📏️ `scale-selection` command.

use crate::editor::puzzle3d::Puzzle3dActionCtx;
use dsl::os_pack::json::Value;

/// 📏️ One gumball scale `{ids?, sx, sy, sz}` as ONE transform-tool transaction yielding `scale-selection`.
pub fn scale_selection(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    ctx.commit_gumball("scaleSelection", args);
}

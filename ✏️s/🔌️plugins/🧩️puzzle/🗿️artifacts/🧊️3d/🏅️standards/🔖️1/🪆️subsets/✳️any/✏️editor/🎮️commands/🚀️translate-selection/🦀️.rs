//! 🚀️ `translate-selection` command.

use crate::editor::puzzle3d::Puzzle3dActionCtx;
use dsl::os_pack::json::Value;

/// 🚀️ One gumball translate `{ids?, dx, dy, dz}` as ONE transform-tool transaction yielding `drag-selection`.
pub fn translate_selection(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    ctx.commit_gumball("translateSelection", args);
}

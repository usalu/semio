//! 🚀️ `translate-selection` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use semio_framework_pack_json::Value;

/// 🚀️ One gumball move `{dx, dy, dz}` (or a typed `move dx dy [dz]` submit) as ONE transform-tool transaction:
/// the `drag-selection3d` leaf, which carries each part's board pin along.
pub fn translate_selection(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    ctx.commit_gumball("translateSelection", args);
}

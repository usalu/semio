//! 🕸️ `add-node` command.

use crate::editor::puzzle2d::Puzzle2dActionCtx;
use semio_framework_pack_json::Value;

/// 🌱️ Records one `create-node` of the requested kind at the requested position.
pub fn add_node(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str());
    ctx.recorder.add_node(kind, args);
}

//! 🖌️ `set-brush-node-size` command.

use crate::editor::puzzle2d::{puzzle2d_window_only_scope, Puzzle2dActionCtx};
use serde_json::Value;

pub async fn set_brush_node_size(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    if let Some(size) = args.and_then(|value| value.get("size")).and_then(|value| value.as_f64()) {
        ctx.host.borrow_mut().set_brush_node_size(size);
        *ctx.ui_scope = puzzle2d_window_only_scope();
    }
}

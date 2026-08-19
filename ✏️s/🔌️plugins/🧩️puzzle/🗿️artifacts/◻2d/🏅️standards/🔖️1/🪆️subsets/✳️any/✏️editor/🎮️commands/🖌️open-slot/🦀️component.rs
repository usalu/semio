//! 🖌️ `open-slot` command.

use crate::editor::puzzle2d::Puzzle2dActionCtx;
use serde_json::Value;

pub async fn open_slot(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    if let Some(handle_id) = args.and_then(|value| value.get("handleId")).and_then(|value| value.as_str()) {
        ctx.host.borrow_mut().brush_open_slot(handle_id);
    }
}

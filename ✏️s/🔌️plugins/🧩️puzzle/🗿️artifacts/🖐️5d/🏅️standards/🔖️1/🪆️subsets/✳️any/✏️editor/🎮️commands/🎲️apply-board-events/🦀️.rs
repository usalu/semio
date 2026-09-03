//! 🎲️ `apply-board-events` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;

pub fn apply_board_events(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(events_json) = args.and_then(|value| value.get("eventsJson")).and_then(|value| value.as_str()) {
        ctx.app.apply_board_events_from_json(events_json, ctx.scene);
    }
}

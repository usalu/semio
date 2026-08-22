//! 🖌️ `fill-session-step` command.

use crate::editor::puzzle2d::Puzzle2dActionCtx;
use serde_json::Value;

pub fn fill_session_step(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let generation = args.and_then(|value| value.get("generation")).and_then(Value::as_u64);
    crate::editor::puzzle2d::commands::set_fill_count::step_fill_job(ctx, generation);
}

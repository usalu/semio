//! 🖌️ `fill-session-begin` command.

use crate::editor::puzzle2d::config::Puzzle2dFillRuntime;
use serde_json::Value;

/// 🏁️ Opens a fill session at an explicit count and seed. Returns the `(count, seed)` the retained
/// session work must search for; the caller publishes the runtime transition. The count has no
/// ceiling — a board that cannot hold that many is an outcome the session reports, not a refusal
/// before it ever looks.
pub fn fill_session_begin(args: Option<&Value>, runtime: &mut Puzzle2dFillRuntime) -> Result<Option<(u32, u64)>, &'static str> {
    let Some(max_count) = args.and_then(|args| args.get("maxCount")).and_then(Value::as_u64) else {
        return Err("puzzle2d-fill-start-count");
    };
    let Ok(max_count) = u32::try_from(max_count) else {
        return Err("puzzle2d-fill-start-count");
    };
    let Some(seed) = args.and_then(|args| args.get("seed")).and_then(Value::as_u64) else {
        return Err("puzzle2d-fill-start-seed");
    };
    runtime.fill_count = max_count;
    Ok(Some((max_count, seed)))
}

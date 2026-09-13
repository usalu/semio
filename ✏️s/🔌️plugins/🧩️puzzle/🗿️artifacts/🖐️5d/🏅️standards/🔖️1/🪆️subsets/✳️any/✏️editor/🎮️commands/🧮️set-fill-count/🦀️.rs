//! 🧮️ `set-fill-count` command.

use crate::editor::puzzle5d::{merge_engine_fixture, Puzzle5dActionCtx};
use dsl::os_pack::json::Value;

/// 🪣️ Applies the requested placement count verbatim — there is no ceiling, so a document that
/// cannot hold that many reports a stall reason through the fill progress measure instead of being
/// silently clamped. A count of zero only records the runtime value, leaving the document untouched.
pub fn set_fill_count(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    ctx.app.drive_precompute(ctx.scene);
    let count = args.and_then(|value| value.get("count").or_else(|| value.get("value"))).and_then(|value| value.as_f64()).map_or(0, |value| value.round().clamp(0.0, f64::from(u32::MAX)) as u32);
    ctx.scene.runtime.fill_count = count;
    ctx.app.precompute.borrow_mut().set_fill_requested_count(count);
    if count == 0 {
        return;
    }
    ctx.scene.active_utility = "fill".into();
    if let Ok(fixture_json) = ctx.app.precompute.borrow_mut().apply_fill_count_rust(count) {
        if let Some(next) = merge_engine_fixture(ctx.scene, &fixture_json) {
            *ctx.scene = next;
        }
    }
}

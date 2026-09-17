//! 🧮️ `set-fill-count` command — the fill tool's requested count, a shared `Puzzle2dConfig` preference. A
//! running fill tool run observes the config change and resumes from its checkpoint with the new count (a raise
//! continues, a lower retracts the provisional tail); the command itself never touches the document.

use crate::editor::puzzle2d::{puzzle2d_window_and_measures_scope, Puzzle2dActionCtx};
use semio_framework_plugin::kernel::Effect;
use serde_json::{json, Value};

/// 📨️ Routes a programmatic count request (the engagement bar's repeat-last) through the declared
/// public command rather than writing the config behind the tool-run driver's back.
pub fn request(count: u32) -> Effect {
    Effect::DispatchAction {
        req: semio_framework_plugin::RequestId(semio_framework_job::allocate_operation_id().0),
        action: "setFillCount".into(),
        args: semio_framework_plugin::optional_json_to_dsl(Some(json!({ "value": count }))),
        delay_ms: 0,
    }
}

/// 🔢️ Reads `count` or a numeric measure's `value`; a missing, non-finite, negative or out-of-`u32` count changes nothing.
pub fn set_fill_count(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let count = args.and_then(|args| args.get("count").or_else(|| args.get("value"))).and_then(Value::as_f64);
    if let Some(count) = count.filter(|count| count.is_finite() && *count >= 0.0 && count.round() <= f64::from(u32::MAX)) {
        ctx.scene.runtime.fill_count = count.round() as u32;
        *ctx.ui_scope = puzzle2d_window_and_measures_scope();
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

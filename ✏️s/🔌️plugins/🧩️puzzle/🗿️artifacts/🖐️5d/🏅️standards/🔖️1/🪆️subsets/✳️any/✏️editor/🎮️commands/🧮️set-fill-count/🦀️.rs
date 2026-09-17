//! 🧮️ `set-fill-count` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;
use semio_framework::kernel::Effect;

/// 📨️ Routes a text-entry or repeat-last request through the retained public command, so the count lands on the
/// one lane a live run's declared settings watch instead of being written behind the driver's back.
pub fn request(count: u32) -> Effect {
    Effect::DispatchAction {
        req: semio_framework_plugin::RequestId(semio_framework_job::allocate_operation_id().0),
        action: "setFillCount".into(),
        args: semio_framework::optional_json_to_dsl(Some(serde_json::json!({ "value": count }))),
        delay_ms: 0,
    }
}

/// 🪣️ Records the requested placement count verbatim in the document instance's configuration — there is no
/// ceiling and a malformed count is a no-op. The document is untouched; a live fill run reads the new count
/// through its declared settings and reconfigures.
pub fn set_fill_count(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(count) = args.and_then(|value| value.get("count").or_else(|| value.get("value"))).and_then(Value::as_f64).filter(|count| count.is_finite()) {
        ctx.scene.runtime.fill_count = count.round().clamp(0.0, f64::from(u32::MAX)) as u32;
    }
}

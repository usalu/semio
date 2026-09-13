//! 🪣️ `set-fill-count` retained-operation helpers: the count is shared configuration only. A fill run
//! reads it when its job is built, and the framework tool run driver reconfigures a live run when it
//! changes (`📋️tool-run-contract.md` §3.3, §3.7.5) — this command never touches the document.

use dsl::os_pack::json::Value;
use semio_framework::kernel::Effect;
use serde_json::json;

/// 📨️ Routes text-entry requests through the retained public command.
pub fn request(count: u32) -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(semio_framework_job::allocate_operation_id().0), action: "setFillCount".into(), args: semio_framework::optional_json_to_dsl(Some(json!({ "value": count }))), delay_ms: 0 }
}

/// 🔢️ The requested count is any `u32` — the planner plans toward exactly what was asked and reports
/// a capacity it cannot reach as a visible stall, never as a silent clamp
/// (`26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS` master plan §1 decision 1).
pub(crate) fn parse_count(args: Option<&Value>) -> u32 {
    args.and_then(|value| value.get("count").or_else(|| value.get("value"))).and_then(Value::as_f64).map_or(0, |value| value.round().clamp(0.0, f64::from(u32::MAX)) as u32)
}

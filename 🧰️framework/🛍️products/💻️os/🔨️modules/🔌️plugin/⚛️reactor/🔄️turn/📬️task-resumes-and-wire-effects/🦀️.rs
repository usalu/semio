//! 📬️ Task resumes and wire effects of the reactor turn: the drained task resumes and the decoded wire effects, replay shell commands, file opens, app events and admitted effects. See [`super`].

use super::*;

/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): drains `TASK_RESUMES` (bounded
/// to `max_rounds` entries — the SAME defensive-cap shape `run_until_idle` uses, so an endlessly
/// respawning follow-up chain cannot stall a turn forever) and routes each resolved task's outcome
/// back into the SAME instance's `dyn PluginApp` via `plugin_runtime::plugin_resume_task`: a
/// `Command` resume through the existing typed-command dispatch, an `Emit` resume through a
/// decode + `dispatch_emit`, and a `Fault` resume straight to the shell as a message. Either
/// dispatch path's frames are fed through the SAME `route_app_frame` every other frame this turn
/// goes through — one implementation, not two (the eventual `world actor-async` runner calls this
/// SAME function, which is why it is `pub`, not `pub(crate)`).
///
/// Returns whether entries remain queued (the round cap was hit) or a `Command` resume was handed to its
/// instance, which delivers it across the following turns — folded into `poll`'s `turn-status::more-work`
/// so neither a saturated resume queue nor a queued delivery is ever silently dropped.
///
/// 🚫️async: E5 executor bridge — `plugin_resume_task` stays genuinely `async fn`; see
/// `poll`'s `plugin_exchange` call for the same safety argument.
///
/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): resumed `AsyncTask` follow-
/// ups (and any replayed `task_restarts` from a `restore` before this turn) — AFTER
/// `run_until_idle` so a task that resolved just now is redispatched the SAME turn, not the
/// next one. A resume can itself spawn more tasks (`dispatch_emit` runs for real), so the
/// executor may have fresh ready work by the time this returns — folded into `more_work` below
/// rather than requiring a second `run_until_idle` pass this turn (the next `poll` picks it up).
pub fn drain_task_resumes<PA: semio_framework_plugin_reactor_app::PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, effects: &mut Vec<Effect>, max_rounds: u32, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> bool {
    let mut delivering = false;
    for _ in 0..max_rounds {
        let Some(resume) = TASK_RESUMES.with(|resumes| resumes.borrow_mut().pop()) else {
            return delivering;
        };
        if native_close_key(runtime, resume.instance).is_err() {
            TASK_RESUMES.with(|resumes| resumes.borrow_mut().push_admitted(resume));
            continue;
        }
        let input = match resume.outcome {
            TaskResumeOutcome::Fault(fault) => {
                effects.push(shell_fault_effect(resume.instance, &fault));
                continue;
            }
            TaskResumeOutcome::Command(bytes) => {
                delivering = true;
                semio_framework_plugin_runtime_ops::ops_typed_operations::TaskResumeInput::Command(bytes)
            }
            TaskResumeOutcome::Emit { artifact_ops, config_ops, draft_ops } => semio_framework_plugin_runtime_ops::ops_typed_operations::TaskResumeInput::Emit { artifact_ops, config_ops, draft_ops },
        };
        let output = ::semio_framework_async::poll::resolve_ready(semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_resume_task(runtime, resume.instance, &resume.meta, input, identity, cx));
        for frame_bytes in output.frames {
            route_app_frame(resume.instance, &frame_bytes, effects);
        }
        for one in &output.effects {
            if let Ok(effect) = decode_wire_effect(one) {
                push_admitted_effect(effects, resume.instance, effect);
            }
        }
        for one in &output.events {
            if let Ok(event) = decode_wire_app_event(one) {
                effects.push(Effect::PublishEvent { topic: event.kind, payload: store::pack_rt::encode_wire_value(&event.payload) });
            }
        }
    }
    delivering || !TASK_RESUMES.with(|resumes| resumes.borrow().is_empty())
}

// 🚫️async: E5 executor bridge — `store::pack_rt::decode_wire_value` is genuinely `async fn`
// (out of `path_scope`, `🏪️store/**`); `resolve_ready` is safe here for the same reason as
// `kernel_effect_to_wit`'s own `pack` helper above — `world actor` imports no `host-async`.
#[doc(hidden)]
pub fn decode_wire_effect(bytes: &[u8]) -> Result<Effect, ()> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|_| ())?;
    if let Ok(effect) = <Effect as semio_framework_value::FromValue>::from_value(value.clone()) {
        return Ok(effect);
    }
    if let Ok(effect) = serde_json::from_value::<Effect>(value.clone().into()) {
        return Ok(effect);
    }
    if let Some(effect) = decode_wire_request_file_open(&value) {
        return Ok(effect);
    }
    decode_wire_replay_shell_command(&value).ok_or(())
}

fn decode_wire_replay_shell_command(value: &semio_framework_value::DslValue) -> Option<Effect> {
    let replay = value.get("replayShellCommand").or_else(|| value.get("ReplayShellCommand"))?;
    let action_id = replay.get("actionId").or_else(|| replay.get("action_id")).and_then(semio_framework_value::DslValue::as_str)?;
    Some(Effect::ReplayShellCommand { action_id: action_id.to_string(), args: replay.get("args").cloned() })
}

/// 📤️ W-G3's `ReplayShellCommand` fallback is the same table: `from_dsl_value` misses some
/// camelCase effect objects, and a silent `Err(())` drops the host picker (`effects:0`).
fn decode_wire_request_file_open(value: &semio_framework_value::DslValue) -> Option<Effect> {
    let file = value.get("requestFileOpen").or_else(|| value.get("RequestFileOpen"))?;
    let req = file.get("req").and_then(|req| req.as_u64().or_else(|| req.get("id").and_then(semio_framework_value::DslValue::as_u64)))?;
    let accept = file.get("accept").and_then(semio_framework_value::DslValue::as_str)?.to_string();
    let import_action = file.get("importAction").or_else(|| file.get("import_action")).and_then(semio_framework_value::DslValue::as_str)?.to_string();
    let read_as = file.get("readAs").or_else(|| file.get("read_as")).and_then(semio_framework_value::DslValue::as_str).map(str::to_string);
    let multiple = file.get("multiple").and_then(semio_framework_value::DslValue::as_bool).unwrap_or(false);
    Some(Effect::RequestFileOpen { req: semio_framework::kernel::RequestId(req), accept, read_as, import_action, multiple, args: file.get("args").cloned() })
}

pub(super) fn push_admitted_effect(effects: &mut Vec<Effect>, instance: u32, effect: Effect) {
    if let Effect::SetTimer { id, .. } = &effect {
        if ARMED_TIMERS.with(|timers| timers.borrow_mut().insert(instance, *id)).is_err() {
            let fault = semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.timer-capacity"), "fixed timer authority is saturated or collided");
            effects.push(shell_fault_effect(instance, &fault));
            return;
        }
    }
    if let Effect::SpawnJob { job, .. } = &effect {
        if JOB_RENDER_BINDINGS.with(|bindings| bindings.borrow_mut().bind(instance, *job)).is_err() {
            let fault = semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.job-render-binding-capacity"), "fixed job-to-surface render authority is saturated or collided");
            effects.push(shell_fault_effect(instance, &fault));
            return;
        }
    }
    effects.push(effect);
}

pub(super) fn decode_wire_app_event(bytes: &[u8]) -> Result<semio_framework::kernel::AppEvent, ()> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|_| ())?;
    semio_framework_value::FromValue::from_value(value).map_err(|_| ())
}

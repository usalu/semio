//! 🔦️ More-work sources and tracing of the reactor turn: the settled and busy more-work sources, the late UI retirement, the turn event note and the guest memory and phase retention traces. See [`super`].

use super::*;

/// 🔦️ The named sources a turn folds into `TurnStatus::MoreWork`, recorded once per turn.
///
/// An idle actor must answer `Idle`: every `MoreWork` costs the host another turn round trip, and at
/// the browser's cadence that is the difference between a settled app and a permanently hot reactor.
/// A law that asserts "idle" needs to say WHICH source stayed armed, so the reading is structural
/// rather than a boolean. Costs one `Cell` store per turn. See `📓️idle-turns-2026-09-10.md`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TurnMoreWorkSources {
    /// ⏱️ The reactor executor still had ready work when its 8 ms slice expired.
    pub executor_deadline: bool,
    /// 🧵️ The cooperative process-worker pool advanced a worker this turn.
    pub process_pool: bool,
    /// 🚪️ An instance close ladder has un-run cleanup steps.
    pub close_cleanup: bool,
    /// 🧬️ A typed operation continuation has not reached its terminal state.
    pub typed_operation: bool,
    /// 🔒️ OBSERVATION, never work: another owner held an instance's operation authority when the
    /// scan ran, so the scan could not answer. Deliberately outside [`Self::any`] — see the fold in
    /// `poll_kernel_turn`.
    pub typed_operation_contended: bool,
    /// 🩹️ A surface reconcile has publishable or unpublished patch work.
    pub reconcile: bool,
    /// 🔁️ Resumed async tasks remained undrained at the resume budget.
    pub resumes: bool,
    /// 🧵️ The reactor executor holds tasks that are parked, not ready.
    pub executor_pending: bool,
    /// 📥️ A command ingress slot still owns an in-assembly command.
    pub command_ingress: bool,
    /// 📨️ An original extension request still owes invocation, publication or paid retirement.
    pub extension_invocation: bool,
    /// 🚪️ The guest lifecycle registry owes a receipt or a transition.
    pub lifecycle: bool,
    /// ♻️ UI owners released during this turn still owe bounded retirement work.
    pub ui_retirement: bool,
    /// 🛬️ A cold-pair or checkpoint-restored document still loads through the stepped archive load.
    pub document_load: bool,
}

impl TurnMoreWorkSources {
    /// 😴️ The reading of a turn that answered `Idle`.
    pub const SETTLED: Self = Self {
        executor_deadline: false,
        process_pool: false,
        close_cleanup: false,
        typed_operation: false,
        typed_operation_contended: false,
        reconcile: false,
        resumes: false,
        executor_pending: false,
        command_ingress: false,
        extension_invocation: false,
        lifecycle: false,
        ui_retirement: false,
        document_load: false,
    };

    /// 🔦️ Whether any source is armed — equal to the turn's `MoreWork` verdict.
    pub fn any(self) -> bool {
        !self.names().is_empty()
    }

    /// 🏷️ The armed source names, for a law's failure message.
    pub fn names(self) -> Vec<&'static str> {
        [
            (self.executor_deadline, "executor_deadline"),
            (self.process_pool, "process_pool"),
            (self.close_cleanup, "close_cleanup"),
            (self.typed_operation, "typed_operation"),
            (self.reconcile, "reconcile"),
            (self.resumes, "resumes"),
            (self.executor_pending, "executor_pending"),
            (self.command_ingress, "command_ingress"),
            (self.extension_invocation, "extension_invocation"),
            (self.lifecycle, "lifecycle"),
            (self.ui_retirement, "ui_retirement"),
            (self.document_load, "document_load"),
        ]
        .into_iter()
        .filter_map(|(armed, name)| armed.then_some(name))
        .collect()
    }
}

/// 🧹️ Retires late-built children and documents before their released arguments and keeps queued ownership runnable.
#[doc(hidden)]
pub fn close_late_ui_retirement() -> Result<bool, semio_framework::Fault> {
    let built_complete = ui_contract::close_built_node_page_one();
    let documents = ui_contract::close_ui_document_page_with_grant(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).map_err(reactor_close_fault)?;
    let values = ui_contract::close_ui_value_page_with_grant(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).map_err(reactor_close_fault)?;
    Ok(!built_complete || !documents.complete || !values.complete)
}

/// 🩺️ Records what this turn was handed so the per-turn memory line can put growth beside input
/// size — the one comparison that separates "the events retain" from "the call itself leaks". Only
/// the component boundary can price the events, and only wasm has a linear memory to price.
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub fn note_turn_events(count: usize, payload_bytes: usize) {
    TURN_EVENTS.set((count, payload_bytes));
}

/// 🔦️ The [`TurnMoreWorkSources`] of the most recent turn on this actor's thread.
pub fn last_turn_more_work_sources() -> TurnMoreWorkSources {
    LAST_MORE_WORK_SOURCES.get()
}

/// 🧮️ Attributes a turn's retained bytes to the PHASE that kept them. A settled turn must retain
/// nothing; the wasm harness measured 4 437 B per turn against an instance-less guest answering
/// `Idle`, so the owner is a framework phase, not app work. Only weighs anything in a binary that
/// installed `semio_framework_trace::HeapWitness`, and only under `SEMIO_RUNTIME_DIAGNOSTICS` — which
/// a guest arms for itself with `set_runtime_diagnostics` when it ships the witness at all.
/// See `📓️idle-turns-2026-09-10.md`.
pub(super) fn trace_turn_phase_retention(phase: &str) {
    if !semio_framework_trace::runtime_diagnostics_enabled() {
        return;
    }
    let retained = semio_framework_trace::retained_heap_bytes();
    let previous = TURN_PHASE_RETENTION.replace(retained);
    trace_guest_line(&format!("[TRACE] turn phase {phase}: retained {retained} B, delta {}", retained - previous));
}

/// 🗣️ One diagnostic line, out where it can actually be read. A `wasm32-wasip2` guest's `stderr` is
/// whatever `WasiCtx` the host built — nothing, in the Wasmtime harness and in the browser — while
/// the actor world's own `log` import always lands somewhere the host prints. Off wasm the two are
/// the same stream, so this stays one call site.
pub(super) fn trace_guest_line(line: &str) {
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    ::semio_framework_async::poll::resolve_ready(semio_framework_plugin_abi::wasip2::log("debug", line));
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    eprintln!("{line}");
}

/// 🧮️ Weighs the guest's own linear memory once per turn — one `memory.size` on wasm, nothing at all
/// off it. Reaching the wasm `maximum` is a bare `rust_oom` → `unreachable` with no Rust panic and no
/// diagnosis (boot #9b trapped at `189328 bytes failed` inside the exported `poll`'s own task box),
/// so the guest says how full it is BEFORE it traps: once, on first crossing the schema's install-peak
/// ceiling, unconditionally — and every 16 MiB of further growth under
/// [`semio_framework_trace::runtime_diagnostics_enabled`]
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub(super) fn trace_guest_memory_pressure() {
    let Some(bytes) = semio_framework_trace::guest_linear_memory_bytes() else { return };
    let witnessed = GUEST_MEMORY_WITNESS.get();
    let turn = GUEST_MEMORY_TURN.get().saturating_add(1);
    GUEST_MEMORY_TURN.set(turn);
    let delta = bytes as i64 - witnessed as i64;
    if bytes > witnessed {
        GUEST_MEMORY_WITNESS.set(bytes);
        if bytes >= semio_framework_trace::guest_linear_memory_install_peak_ceiling_bytes() && !GUEST_MEMORY_REPORTED.replace(true) {
            eprintln!(
                "guest linear memory at {bytes} B — {}% of the {} B budget, past the {}% install-peak ceiling",
                semio_framework_trace::guest_linear_memory_percent(bytes),
                semio_framework_trace::GUEST_LINEAR_MEMORY_MAXIMUM_BYTES,
                semio_framework_trace::GUEST_LINEAR_MEMORY_INSTALL_PEAK_PERCENT
            );
        }
    }
    if semio_framework_trace::runtime_diagnostics_enabled() {
        let (events, events_bytes) = TURN_EVENTS.get();
        trace_guest_line(&format!(
            "[TRACE] guest linear memory turn={turn} bytes={bytes} delta={delta} percent={} events={events} eventsBytes={events_bytes} elapsed_us={}",
            semio_framework_trace::guest_linear_memory_percent(bytes),
            guest_turn_executing_us().map(|us| us.to_string()).unwrap_or_else(|| "unmeasured".into())
        ));
    }
}

pub(super) fn native_close_key<PA: semio_framework_plugin_reactor_app::PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, instance: u32) -> Result<instance_lifetime::NativeCloseKey, semio_framework::Fault> {
    runtime.live_instance_key(instance).map_err(|error|reactor_close_fault(if error.message=="lifecycle authority busy"{"lifecycle authority busy"}else{"instance has no acknowledged live lifetime"}))
}

/// 🔦️ The work the executors reported on one turn.
#[derive(Clone, Copy)]
pub(super) struct TurnExecutorWork {
    deadline: bool,
    task: bool,
    pool: bool,
}

/// 🧮️ Every source the turn read before deciding whether another turn is needed.
pub(super) struct TurnWorkInputs {
    pub(super) events_pending: bool,
    pub(super) executors: TurnExecutorWork,
    pub(super) close_cleanup_work: bool,
    pub(super) typed_operation_runnable: bool,
    pub(super) typed_operation_contended: bool,
    pub(super) reconcile_work: bool,
    pub(super) resumes_remain: bool,
    pub(super) document_load_work: bool,
}

/// 🏃️ Runs the reactor and task executors to their deadline, pumps the process worker pool and admits the registry's effects.
///
/// 🚫️async: E5 executor bridge (× 2) — `LocalExecutor::{run_until_idle,has_ready}` stay
/// genuinely `async fn` (its own doc: "run_until_idle handles Pending without ever yielding
/// its own future" — matches `⚛️reactor/💼️jobs`'s identical use of this exact bridge).
///
/// 🧵️ The COLD executor's own slice of the same hold: every `AsyncTask` `spawn_task` admitted
/// lives on `TASK_EXECUTOR`, and until the task lane became a product lane nothing but a native
/// fixture ever polled it — a spawned task would have parked forever inside a real guest. Its
/// answer is "some task is still alive", ready or parked, so it is folded into `more_work` the
/// same way `REACTOR_EXECUTOR`'s is.
///
/// 🔁️ The process-pool pump stays gated on the REACTOR executor alone. `TASK_EXECUTOR`'s answer
/// is "some task is still alive", ready OR PARKED, so folding it in here stopped pumping the
/// pool for as long as any task was parked on a host round trip — which starved every
/// interactive job step behind it (measured 2026-09-22: a command page set never reached its
/// terminal ingress status in 512 turns).
pub(super) fn drive_turn_executors(effects: &mut Vec<Effect>) -> TurnExecutorWork {
    let executor_deadline_work = REACTOR_EXECUTOR.with(|executor| executor.run_until_deadline(64, 256 * 1_024, std::time::Instant::now() + std::time::Duration::from_millis(REACTOR_TURN_EXECUTOR_HOLD_MS)));
    let task_executor_work = TASK_EXECUTOR.with(|executor| executor.run_until_deadline(64, std::time::Instant::now() + std::time::Duration::from_millis(REACTOR_TURN_EXECUTOR_HOLD_MS)));
    let process_pool_work = !executor_deadline_work && pump_process_worker_pool();
    for effect in REGISTRY.with(|registry| registry.drain()) {
        push_admitted_effect(&mut *effects, 0, effect);
    }
    TurnExecutorWork { deadline: executor_deadline_work, task: task_executor_work, pool: process_pool_work }
}

/// 🔦️ Decides whether the turn answers `MoreWork`, records which sources armed it and traces the more-work streak.
///
/// 🔒️ `typed_operation_scan.contended` is deliberately NOT folded in. A busy instance lock is not
/// an answer of "there is runnable work", it is "another owner is mid-step and I could not look" —
/// and the guest is single-threaded, so on wasm it can only ever be a reader inside this very turn.
/// Answering `MoreWork` for it turned every concurrent read into a host turn round trip that
/// produced nothing: 12 of 512 idle generation3d turns, measured 2026-09-10. Every owner that can
/// hold that lock is itself either a reactor executor task (`executor_pending`), a task resume
/// (`resumes`), an ingress command (`command_ingress`) or a lifecycle step (`lifecycle`), each of
/// which already arms this turn on its own account. See `📓️idle-turns-2026-09-10.md`.
pub(super) fn settle_turn_work<PA: semio_framework_plugin_reactor_app::PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, inputs: TurnWorkInputs, effects: &[Effect]) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
    let TurnWorkInputs { events_pending, executors, close_cleanup_work, typed_operation_runnable, typed_operation_contended, reconcile_work, resumes_remain, document_load_work } = inputs;
    let (executor_deadline_work, task_executor_work, process_pool_work) = (executors.deadline, executors.task, executors.pool);
    let more_work = executor_deadline_work || task_executor_work || process_pool_work;
    let executor_pending = REACTOR_EXECUTOR.with(|executor| executor.has_pending()) || TASK_EXECUTOR.with(|executor| executor.has_pending());
    let command_ingress_pending = COMMAND_INGRESS.with(|ingress| ingress.borrow().iter().any(Option::is_some));
    let lifecycle_work = runtime.guest_lifetimes.borrow().has_work();
    let ui_retirement_work = close_late_ui_retirement()?;
    let extension_invocation_pending=RETAINED_EXTENSION_INVOCATIONS.with(|owners|owners.borrow().iter().any(Option::is_some))||original_receiving::response_pending()||original_receiving::intent_pending();
    let more_work = events_pending || more_work || extension_invocation_pending || close_cleanup_work || typed_operation_runnable || reconcile_work || resumes_remain || executor_pending || command_ingress_pending || lifecycle_work || ui_retirement_work || document_load_work;
    LAST_MORE_WORK_SOURCES.set(TurnMoreWorkSources {
        executor_deadline: executor_deadline_work,
        process_pool: process_pool_work,
        close_cleanup: close_cleanup_work,
        typed_operation: typed_operation_runnable,
        typed_operation_contended,
        reconcile: reconcile_work,
        resumes: resumes_remain,
        executor_pending,
        command_ingress: command_ingress_pending,
        extension_invocation: extension_invocation_pending,
        lifecycle: lifecycle_work,
        ui_retirement: ui_retirement_work,
        document_load: document_load_work,
    });
    trace_turn_phase_retention("render");
    trace_guest_memory_pressure();
    MORE_WORK_TRACE.with(|trace| {
        let mut trace = trace.borrow_mut();
        let (streak, seen) = if more_work { (trace.0 + 1, trace.1 + 1) } else { (0, trace.1) };
        if !semio_framework_trace::runtime_diagnostics_enabled() {
            *trace = (streak, seen);
            return;
        }
        if more_work {
            trace_guest_line(&format!(
                "[TRACE] reactor more-work streak={streak} seen={seen} sources={:?} contended={typed_operation_contended} effects={} patches=[{}] pending=[{}]",
                LAST_MORE_WORK_SOURCES.get().names(),
                effects.len(),
                PATCHES.with(patches::PatchTracker::debug_state),
                with_pending_patches(|pending| pending.borrow().debug_state())
            ));
        } else if trace.0 > 0 {
            trace_guest_line(&format!("[TRACE] reactor more-work streak ended after {} turns (seen={seen})", trace.0));
        }
        *trace = (streak, seen);
    });
    Ok(more_work)
}

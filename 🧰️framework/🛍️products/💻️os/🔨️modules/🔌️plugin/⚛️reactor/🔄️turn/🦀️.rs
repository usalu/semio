use super::*;

enum CommandIngressOwner {
    ReservedPresence { cursor: semio_framework::kernel::CommandPageCursor, admission: crate::app::PresenceRosterAdmission, page: semio_framework::kernel::FixedCommandPage },
    Presence { cursor: semio_framework::kernel::CommandPageCursor, publication_generation: u64 },
    PendingPresencePage { cursor: semio_framework::kernel::CommandPageCursor, publication_generation: u64, page: semio_framework::kernel::FixedCommandPage },
    GenericAssembly { cursor: semio_framework::kernel::CommandPageCursor, pages: semio_framework::kernel::CommandPageSet },
    ClosingAssembly { cursor: semio_framework::kernel::CommandPageCursor, pages: semio_framework::kernel::CommandPageSet },
    Generic { cursor: semio_framework::kernel::CommandPageCursor, command: crate::plugin_runtime::PluginCommandIngress },
}

struct RetainedCommandIngress {
    key: instance_lifetime::NativeCloseKey,
    state: CommandIngressOwner,
}

fn retire_command_ingress(state: CommandIngressOwner) -> Option<CommandIngressOwner> {
    match state {
        CommandIngressOwner::ReservedPresence { admission, .. } => {
            admission.cancel.cancel_now();
            None
        }
        CommandIngressOwner::Presence { .. } | CommandIngressOwner::PendingPresencePage { .. } => None,
        CommandIngressOwner::GenericAssembly { cursor, mut pages } | CommandIngressOwner::ClosingAssembly { cursor, mut pages } => {
            if pages.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES).0 {
                None
            } else {
                Some(CommandIngressOwner::ClosingAssembly { cursor, pages })
            }
        }
        CommandIngressOwner::Generic { cursor, command } => command.retire_step().map(|command| CommandIngressOwner::Generic { cursor, command }),
    }
}

pub(super) fn close_command_ingress_step(key: instance_lifetime::NativeCloseKey) -> Result<bool, semio_framework::Fault> {
    COMMAND_INGRESS.with(|ingress| {
        let mut ingress = ingress.try_borrow_mut().map_err(|_| reactor_close_fault("command ingress close authority busy"))?;
        let Some(index) = ingress.iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.key == key)) else { return Ok(true) };
        let entry = ingress[index].take().expect("selected exact ingress owner");
        ingress[index] = retire_command_ingress(entry.state).map(|state| RetainedCommandIngress { key, state });
        Ok(!ingress.iter().any(|entry| entry.as_ref().is_some_and(|entry| entry.key == key)))
    })
}

crate::component_persistent_local! {
    static COMMAND_INGRESS: RefCell<[Option<RetainedCommandIngress>; 2]> = RefCell::new([None, None]);
}

/// 🧮️ Occupied slots of the guest's retained command-ingress authority, out of its fixed two. A
/// settled reactor holds none: an owner survives a turn only while its own command is still being
/// assembled, dispatched or closed. This is what an ingress-saturation law reads between commands —
/// a stream that leaves an owner behind pins the authority and backpressures every later command.
pub fn retained_command_ingress_occupancy() -> usize {
    COMMAND_INGRESS.with(|ingress| ingress.borrow().iter().filter(|entry| entry.is_some()).count())
}

thread_local! {
    /// 🐞️ `[DEBUG]` more-work streak trace: (current streak, total more-work turns) — temporary, ticket 26/09/02/PUZZLE-3D-END-TO-END.
    static MORE_WORK_TRACE: RefCell<(u64, u64)> = const { RefCell::new((0, 0)) };
    /// 🧮️ Largest guest linear-memory reading this actor has witnessed, in bytes.
    static GUEST_MEMORY_WITNESS: Cell<usize> = const { Cell::new(0) };
    /// 🧮️ Whether the install-peak ceiling has already been reported for this actor.
    static GUEST_MEMORY_REPORTED: Cell<bool> = const { Cell::new(false) };
    /// 🎯️ Turns sampled by [`trace_guest_memory_pressure`] on this actor.
    static GUEST_MEMORY_TURN: Cell<u64> = const { Cell::new(0) };
    /// 🩺️ `(event count, bulk payload bytes)` the current turn was handed, for the memory trace.
    static TURN_EVENTS: Cell<(usize, usize)> = const { Cell::new((0, 0)) };
    /// 🧮️ Retained-heap reading at the previous [`trace_turn_phase_retention`] probe.
    static TURN_PHASE_RETENTION: Cell<isize> = const { Cell::new(0) };
    /// 🔦️ Which sources answered `MoreWork` on the most recent turn.
    static LAST_MORE_WORK_SOURCES: Cell<TurnMoreWorkSources> = const { Cell::new(TurnMoreWorkSources::SETTLED) };
}

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
    /// 🚪️ The guest lifecycle registry owes a receipt or a transition.
    pub lifecycle: bool,
}

impl TurnMoreWorkSources {
    /// 😴️ The reading of a turn that answered `Idle`.
    pub const SETTLED: Self = Self { executor_deadline: false, process_pool: false, close_cleanup: false, typed_operation: false, typed_operation_contended: false, reconcile: false, resumes: false, executor_pending: false, command_ingress: false, lifecycle: false };

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
            (self.lifecycle, "lifecycle"),
        ]
        .into_iter()
        .filter_map(|(armed, name)| armed.then_some(name))
        .collect()
    }
}

/// 🩺️ Records what this turn was handed so the per-turn memory line can put growth beside input
/// size — the one comparison that separates "the events retain" from "the call itself leaks". Only
/// the component boundary can price the events, and only wasm has a linear memory to price.
#[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]
pub(super) fn note_turn_events(count: usize, payload_bytes: usize) {
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
fn trace_turn_phase_retention(phase: &str) {
    if !semio_framework_trace::runtime_diagnostics_enabled() {
        return;
    }
    let retained = semio_framework_trace::retained_heap_bytes();
    let previous = TURN_PHASE_RETENTION.replace(retained);
    trace_guest_line(&format!("[DEBUG] turn phase {phase}: retained {retained} B, delta {}", retained - previous));
}

/// 🗣️ One diagnostic line, out where it can actually be read. A `wasm32-wasip2` guest's `stderr` is
/// whatever `WasiCtx` the host built — nothing, in the Wasmtime harness and in the browser — while
/// the actor world's own `log` import always lands somewhere the host prints. Off wasm the two are
/// the same stream, so this stays one call site.
fn trace_guest_line(line: &str) {
    #[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]
    crate::app::resolve_ready(crate::component::wasip2::log("debug", line));
    #[cfg(not(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2")))]
    eprintln!("{line}");
}

/// 🧮️ Weighs the guest's own linear memory once per turn — one `memory.size` on wasm, nothing at all
/// off it. Reaching the wasm `maximum` is a bare `rust_oom` → `unreachable` with no Rust panic and no
/// diagnosis (boot #9b trapped at `189328 bytes failed` inside the exported `poll`'s own task box),
/// so the guest says how full it is BEFORE it traps: once, on first crossing the schema's install-peak
/// ceiling, unconditionally — and every 16 MiB of further growth under
/// [`semio_framework_trace::runtime_diagnostics_enabled`]
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
fn trace_guest_memory_pressure() {
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
            "[DEBUG] guest linear memory turn={turn} bytes={bytes} delta={delta} percent={} events={events} eventsBytes={events_bytes} elapsed_us={}",
            semio_framework_trace::guest_linear_memory_percent(bytes),
            guest_turn_executing_us().map(|us| us.to_string()).unwrap_or_else(|| "unmeasured".into())
        ));
    }
}

fn native_close_key<PA: crate::app::PluginApp>(runtime: &crate::plugin_runtime::PluginRuntime<PA>, instance: u32) -> Result<instance_lifetime::NativeCloseKey, semio_framework::Fault> {
    let lifetimes = runtime.guest_lifetimes.try_borrow().map_err(|_| reactor_close_fault("lifecycle authority busy"))?;
    lifetimes.get(instance).filter(|slot| slot.cell.is_live()).and_then(|slot| slot.cell.owner()).map(|owner| owner.key()).ok_or_else(|| reactor_close_fault("instance has no acknowledged live lifetime"))
}

const DIRTY_RENDER_CAPACITY: usize = 64;
const DIRTY_INTENT_INSTANCE_CAPACITY: usize = 64;
const DIRTY_INTENT_CAPACITY: usize = 64;

struct DirtyIntentBatch {
    instance: u32,
    intents: ui_contract::UiFixedList<ui_contract::UiIntent, DIRTY_INTENT_CAPACITY>,
}

struct DirtyPollOwners {
    surfaces: ui_contract::UiFixedList<(u32, ui_contract::SurfaceId), DIRTY_RENDER_CAPACITY>,
    intents: ui_contract::UiFixedList<DirtyIntentBatch, DIRTY_INTENT_INSTANCE_CAPACITY>,
}

impl DirtyPollOwners {
    fn new() -> Self {
        Self { surfaces: ui_contract::UiFixedList::default(), intents: ui_contract::UiFixedList::default() }
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the original fixed-capacity input so its caller can retry or retire that exact owner without an extra allocation.")]
    fn try_surface(&mut self, instance: u32, surface: ui_contract::SurfaceId) -> Result<(), ui_contract::SurfaceId> {
        if self.surfaces.iter().any(|queued| queued.0 == instance && queued.1 == surface) {
            return Ok(());
        }
        self.surfaces.try_push((instance, surface)).map_err(|(_, surface)| surface)
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the original fixed-capacity input so its caller can retry or retire that exact owner without an extra allocation.")]
    fn try_intent(&mut self, instance: u32, intent: ui_contract::UiIntent) -> Result<(), ui_contract::UiIntent> {
        if let Some(batch) = self.intents.iter_mut().find(|batch| batch.instance == instance) {
            return batch.intents.try_push(intent);
        }
        if self.intents.len() == DIRTY_INTENT_INSTANCE_CAPACITY {
            return Err(intent);
        }
        let mut intents = ui_contract::UiFixedList::default();
        intents.try_push(intent)?;
        let _ = self.intents.try_push(DirtyIntentBatch { instance, intents });
        Ok(())
    }
}

//#region ⏱️TurnExecution
/// ⏱️ One turn's EXECUTING microseconds — wall time spent inside the guest's own `Future::poll`
/// calls, with every `Pending` gap excluded. The strict lifecycle authority
/// ([`semio_framework_trace::GUEST_LIFECYCLE_TURN_CEILING_US`]) must bound the guest's own work; a
/// plain `now_us() - started_us` also bills the guest for time it is not running at all — a JSPI
/// suspension parked on a host import, a hidden browser tab whose timer/microtask continuation is
/// throttled, or an OS deschedule of a loaded box. Measured 2026-09-10: generation3d's whole
/// `InstanceOpen` turn executes in 13.9 ms natively, yet its browser first step was killed at the
/// 5 s ceiling, because the wall clock kept running across the descheduled continuations.
#[derive(Clone, Copy)]
struct TurnExecution {
    accumulated_us: u64,
    poll_started_us: Option<u64>,
    clock_lost: bool,
}

const TURN_EXECUTION_IDLE: TurnExecution = TurnExecution { accumulated_us: 0, poll_started_us: None, clock_lost: false };

thread_local! {
    static TURN_EXECUTION: Cell<TurnExecution> = const { Cell::new(TURN_EXECUTION_IDLE) };
}

/// ⏱️ Executing microseconds of the turn in flight, including the poll currently on the stack.
/// `None` means the host clock was missing or ran backward, which the lifecycle authority retains
/// its receipt for exactly like an overrun.
pub(crate) fn guest_turn_executing_us() -> Option<u64> {
    let execution = TURN_EXECUTION.with(Cell::get);
    if execution.clock_lost {
        return None;
    }
    let Some(poll_started_us) = execution.poll_started_us else { return Some(execution.accumulated_us) };
    Some(execution.accumulated_us.saturating_add(semio_framework_job::default_now_us()?.checked_sub(poll_started_us)?))
}

/// ⏱️ Runs one turn under [`TURN_EXECUTION`] accounting; the gaps between polls never accrue.
///
/// 📏️ Takes the turn future ALREADY PINNED, by pointer. Taking it `impl Future` by value cost an
/// exact duplicate of the largest generator in the guest: a parameter owns a coroutine slot from
/// `Unresumed` onward, `pin!` then moved it into a second local that lives across the `.await`, and
/// the moved-from parameter slot is never reused — `with_turn_execution` measured 243 184 B around
/// a 121 584 B `poll_kernel_turn`. `Pin<&mut …>` is 8 bytes and the caller owns the one slot. See
/// `📓️poll-task-leak-2026-09-10.md` §3.1.
async fn with_turn_execution<T>(mut future: std::pin::Pin<&mut (impl std::future::Future<Output = T> + ?Sized)>) -> T {
    TURN_EXECUTION.with(|execution| execution.set(TURN_EXECUTION_IDLE));
    std::future::poll_fn(move |context| {
        TURN_EXECUTION.with(|execution| {
            let mut current = execution.get();
            current.poll_started_us = semio_framework_job::default_now_us();
            current.clock_lost = current.clock_lost || current.poll_started_us.is_none();
            execution.set(current);
        });
        let polled = future.as_mut().poll(context);
        TURN_EXECUTION.with(|execution| {
            let mut current = execution.get();
            match current.poll_started_us.take().zip(semio_framework_job::default_now_us()).map(|(started_us, now_us)| now_us.checked_sub(started_us)) {
                Some(Some(spent_us)) => current.accumulated_us = current.accumulated_us.saturating_add(spent_us),
                _ => current.clock_lost = true,
            }
            execution.set(current);
        });
        polled
    })
    .await
}
/// ⏱️ Charges the turn in flight with executing microseconds it never really spent, so a law can
/// reach the strict ceiling deterministically instead of sleeping through it.
#[cfg(test)]
pub(crate) fn charge_turn_execution_us(spent_us: u64) {
    TURN_EXECUTION.with(|execution| {
        let mut current = execution.get();
        current.accumulated_us = current.accumulated_us.saturating_add(spent_us);
        execution.set(current);
    });
}

#[cfg(test)]
#[path = "🧪️tests/⏱️execution/🦀️.rs"]
mod turn_execution_tests;

#[cfg(test)]
#[path = "🧪️tests/📥️inbound-request/🦀️.rs"]
mod inbound_request_tests;
//#endregion ⏱️TurnExecution

/// 🧠️ Repository-owned actor ABI entrypoint. The component-model wrapper above and the native
/// interpreter both call this exact kernel reducer, so WIT lifting is no longer the production
/// host's semantic authority.
pub async fn poll_kernel<PA: crate::app::PluginApp + 'static>(
    runtime: &crate::plugin_runtime::PluginRuntime<PA>,
    events: Vec<Event>,
    command_page: Option<(semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage)>,
    cold_pair_page: Option<semio_framework::kernel::ColdDocumentPairPage>,
    budget: semio_framework::kernel::Budget,
) -> Result<semio_framework::kernel::TurnResult, semio_framework::Fault> {
    poll_kernel_output(runtime, events, command_page, cold_pair_page, budget, |_| Ok(()), |result, ()| result).await
}

/// 🧠️ One guest turn under [`with_turn_execution`] accounting — the only door into [`poll_kernel_turn`].
#[expect(clippy::result_large_err, reason = "Patch reservation and publication callbacks return the original fixed surface or patch owner on refusal; these transfers must not allocate an error wrapper.")]
pub(super) async fn poll_kernel_output<PA: crate::app::PluginApp, T, Prepared>(
    runtime: &crate::plugin_runtime::PluginRuntime<PA>,
    events: Vec<Event>,
    command_page: Option<(semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage)>,
    cold_pair_page: Option<semio_framework::kernel::ColdDocumentPairPage>,
    budget: semio_framework::kernel::Budget,
    prepare: impl FnOnce(&semio_framework::kernel::TurnResult) -> Result<Prepared, semio_framework::Fault>,
    publish: impl FnOnce(semio_framework::kernel::TurnResult, Prepared) -> T,
) -> Result<T, semio_framework::Fault> {
    with_turn_execution(std::pin::pin!(poll_kernel_turn(runtime, events, command_page, cold_pair_page, budget, prepare, publish))).await
}

/// 📏️ `plugin_exchange`'s future is 76 848 B — awaiting it INLINE made it the single largest local
/// of the turn generator, so every `poll` boxed a task future two thirds of which was one callee
/// that most turns never reach. Boxed here, the turn generator holds an 8-byte pointer and the
/// exchange state is allocated only on the turns that actually run a command. See
/// `📓️poll-task-leak-2026-09-10.md` §3.2.
fn plugin_exchange_boxed<'a, PA: crate::app::PluginApp>(
    runtime: &'a crate::plugin_runtime::PluginRuntime<PA>,
    instance_id: u32,
    command: Option<(u64, crate::plugin_runtime::PluginCommandIngress)>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<crate::plugin_runtime::PluginExchangeOutput, semio_framework::Fault>> + 'a>> {
    Box::pin(crate::plugin_runtime::plugin_exchange(runtime, instance_id, command))
}

#[expect(clippy::result_large_err, reason = "Patch reservation and publication callbacks return the original fixed surface or patch owner on refusal; these transfers must not allocate an error wrapper.")]
async fn poll_kernel_turn<PA: crate::app::PluginApp, T, Prepared>(
    runtime: &crate::plugin_runtime::PluginRuntime<PA>,
    events: Vec<Event>,
    command_page: Option<(semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage)>,
    cold_pair_page: Option<semio_framework::kernel::ColdDocumentPairPage>,
    budget: semio_framework::kernel::Budget,
    prepare: impl FnOnce(&semio_framework::kernel::TurnResult) -> Result<Prepared, semio_framework::Fault>,
    publish: impl FnOnce(semio_framework::kernel::TurnResult, Prepared) -> T,
) -> Result<T, semio_framework::Fault> {
    trace_turn_phase_retention("enter");
    let retryable_lifecycle = command_page.is_none() && cold_pair_page.is_none() && events.iter().all(|event| matches!(event, Event::InstanceOpen { .. } | Event::InstanceClose(_) | Event::InstanceLifecycleAck(_)));
    let mut dirty = DirtyPollOwners::new();
    let mut focus = None;
    for event in &events {
        let instance = match event {
            Event::InstanceOpen { request, .. } => Some(request.instance_id),
            Event::InstanceClose(request) => Some(request.lifetime.instance_id),
            Event::InstanceLifecycleAck(ack) => Some(match ack.receipt {
                ActorInstanceLifecycleReceipt::Captured { lifetime, .. } | ActorInstanceLifecycleReceipt::Accepted { lifetime, .. } | ActorInstanceLifecycleReceipt::Retired { lifetime, .. } => lifetime.instance_id,
            }),
            _ => None,
        };
        if let Some(instance) = instance {
            if focus.replace(instance).is_some() {
                return Err(reactor_close_fault("one lifecycle command is admitted per turn"));
            }
        }
    }
    for event in &events {
        match event {
            Event::InstanceClose(request) => {
                let mut lifetimes = runtime.guest_lifetimes.try_borrow_mut().map_err(|_| reactor_close_fault("lifecycle authority busy"))?;
                let slot = lifetimes.get_mut(request.lifetime.instance_id).ok_or_else(|| reactor_close_fault("close lifetime absent"))?;
                slot.cell.validate_close(*request).map_err(reactor_close_fault)?;
                slot.cell.owner_mut().ok_or_else(|| reactor_close_fault("close native owner absent"))?.request_close(*request, runtime)?;
            }
            Event::InstanceLifecycleAck(ack) => {
                let mut lifetimes = runtime.guest_lifetimes.try_borrow_mut().map_err(|_| reactor_close_fault("lifecycle authority busy"))?;
                lifetimes.get_mut(focus.expect("ACK focus")).ok_or_else(|| reactor_close_fault("ACK lifetime absent"))?.cell.stage_ack(*ack).map_err(reactor_close_fault)?;
            }
            _ => {}
        }
    }
    if focus.is_none() {
        focus = runtime.guest_lifetimes.borrow_mut().next_work();
    }
    if let Some(instance) = focus {
        let mut lifetimes = runtime.guest_lifetimes.borrow_mut();
        if let Some(slot) = lifetimes.get_mut(instance) {
            if let Some(admission) = slot.cell.owner_mut().map(|owner| owner.advance_admission(runtime)).transpose()?.flatten() {
                slot.cell.record_close_admission(admission.0, admission.1).map_err(reactor_close_fault)?;
            }
        }
    }
    let mut close_instances: Vec<u32> = events.iter().filter_map(|event| if let Event::InstanceClose(request) = event { Some(request.lifetime.instance_id) } else { None }).collect();
    let mut document_backbone_effects = Vec::new();
    // ↩️ One `Effect::Respond` per inbound `Event::Request` this turn served — kept apart from the
    // document-backbone effects so the answer's ordering against them is explicit rather than
    // incidental (answers trail, so a request that also wrote the document publishes the write first).
    let mut inbound_request_effects: Vec<Effect> = Vec::new();
    trace_turn_phase_retention("lifecycle");
    let retirement_deadline = std::time::Instant::now() + std::time::Duration::from_millis(u64::from(budget.deadline_ms));
    retire_while_progress_fallible(retirement_deadline, REACTOR_CLOSE_UNITS_PER_TURN, step_reactor_close)?;
    retire_while_progress_bounded(retirement_deadline, REACTOR_CLOSE_UNITS_PER_TURN, || COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().advance_close_one()));
    PATCHES.with(|patches| {
        for unit in 0..PATCH_CLOSE_UNITS_PER_TURN {
            if unit > 0 && unit % PATCH_CLOSE_DEADLINE_STRIDE == 0 && std::time::Instant::now() >= retirement_deadline {
                break;
            }
            if patches.close_step(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT) {
                break;
            }
        }
    });
    for unit in 0..PATCH_CLOSE_UNITS_PER_TURN {
        if semio_framework_ui_runtime::close_surface_reconcile_handback_one().map_err(reactor_close_fault)? {
            break;
        }
        if unit % PATCH_CLOSE_DEADLINE_STRIDE == 0 && std::time::Instant::now() >= retirement_deadline {
            break;
        }
    }
    retire_until_complete(retirement_deadline, || ui_contract::close_ui_document_page_with_grant(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).expect("queued document retirement remains valid").complete);
    retire_until_complete(retirement_deadline, ui_contract::close_ui_patch_owner_one);
    retire_until_complete(retirement_deadline, || ui_contract::close_ui_value_page_with_grant(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).expect("exact UI value retirement queue remains valid").complete);
    retire_until_complete(retirement_deadline, ui_contract::close_built_node_page_one);
    retire_while_progress(retirement_deadline, || semio_framework::kernel::close_ui_turn_patch_owner_with_grant(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT));
    retire_while_progress(retirement_deadline, || {
        matches!(
            semio_framework::kernel::close_ui_turn_patch_transport_with_grant(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT),
            Ok(semio_framework::kernel::UiTurnPatchTransportProgress::Pending { .. })
        )
    });
    retire_while_progress(retirement_deadline, || crate::app::close_table_rows_view_with_grant(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT));
    with_pending_patches(|pending| pending.borrow_mut().advance_rejection(|surface, generation| PATCHES.with(|patches| patches.mark_rejected(surface, generation))));
    for unit in 0..PATCH_CLOSE_UNITS_PER_TURN {
        if unit > 0 && unit % PATCH_CLOSE_DEADLINE_STRIDE == 0 && std::time::Instant::now() >= retirement_deadline {
            break;
        }
        if with_pending_patches(|pending| pending.borrow_mut().close_step(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT)).map_err(reactor_close_fault)? {
            break;
        }
    }
    let close_cleanup_work = crate::plugin_runtime::plugin_step_close_cleanup(runtime)?;
    let _ = crate::plugin_runtime::plugin_step_live_cleanup(runtime)?;
    if let Some(surface) = PATCHES.with(patches::PatchTracker::take_deferred_ready) {
        if let Some(instance) = parse_surface_instance(surface.as_ref()) {
            dirty.try_surface(instance, surface).map_err(|_| reactor_close_fault("fixed dirty surface authority saturated"))?;
        }
    }
    for event in events {
        match event {
            Event::InstanceOpen { request, app_id, actor, quotas, .. } => {
                let instance = request.instance_id;
                let fresh = runtime.guest_lifetimes.borrow_mut().admit(request).map_err(reactor_close_fault)?;
                if fresh {
                    if let Err(error) = INSTANCE_METADATA.with(|metadata| metadata.borrow_mut().insert(instance, app_id.0.clone(), quotas)) {
                        runtime.guest_lifetimes.borrow_mut().remove_uncreated(instance).map_err(reactor_close_fault)?;
                        return Err(error);
                    }
                    if let Err(error) = crate::plugin_runtime::plugin_open_actor_instance(runtime, request, &app_id.0, &actor).await {
                        INSTANCE_METADATA.with(|metadata| drop(metadata.borrow_mut().remove(instance)));
                        runtime.guest_lifetimes.borrow_mut().remove_uncreated(instance).map_err(reactor_close_fault)?;
                        return Err(error);
                    }
                }
            }
            Event::InstanceClose(_) | Event::InstanceLifecycleAck(_) => {}
            Event::CommandIngressPage { .. } => {
                return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.command-page-event-bypass"), "command page must use poll_kernel's dedicated owner argument"));
            }
            Event::ColdDocumentPairPage(_) => {
                return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.cold-pair-page-event-bypass"), "cold document pair page must use poll_kernel's dedicated owner argument"));
            }
            // 🎯️ M1 (ticket 26/08/17 `design-unified.md`): decodes the pack-encoded
            // `ui_contract::UiIntent`, drops it if it targets a tree the user can no longer see (the
            // revision guard, at the reconciler that owns the revision — `PATCHES.revision`,
            // `ui_runtime::is_stale_intent` imported rather than reimplemented), and otherwise
            // batches it per instance for the dispatch pass below (mirrors `app_commands`'
            // batch-then-dispatch shape). Real dispatch replaces the prior packet's "decode-and-
            // mark-dirty" interim — see `📓️terra-sdk-wire-report.md`'s M1 section for the full route.
            Event::UiIntent { instance, intent } => {
                let numeric_instance = instance.0.parse::<u32>().map_err(|_| reactor_close_fault("invalid intent instance"))?;
                native_close_key(runtime, numeric_instance)?;
                if let Ok(intent_value) = store::pack_rt::decode_wire_value(&intent) {
                    if let Ok(intent) = serde_json::from_value::<ui_contract::UiIntent>(intent_value.into()) {
                        if parse_surface_instance(intent.surface.as_ref()) != Some(numeric_instance) {
                            return Err(reactor_close_fault("intent surface names another lifetime"));
                        }
                        let current_revision = PATCHES.with(|patches| patches.revision(&intent.surface.0));
                        if !is_stale_intent(intent.revision, current_revision, DEFAULT_REVISION_TOLERANCE) {
                            dirty
                                .try_intent(numeric_instance, intent)
                                .map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.dirty-intent-capacity"), "fixed dirty intent authority is saturated"))?;
                        }
                    }
                }
            }
            Event::SurfaceVisible { surface, body_key, view_state } => {
                if let Some(instance) = parse_surface_instance(&surface) {
                    native_close_key(runtime, instance)?;
                    crate::plugin_runtime::plugin_mount_surface(runtime, instance, surface.clone(), body_key, &view_state).await?;
                    let surface =
                        ui_contract::SurfaceId::try_from(surface).map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.surface-capacity"), "surface id exceeds fixed text capacity"))?;
                    dirty.try_surface(instance, surface).map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.dirty-surface-capacity"), "fixed dirty surface authority is saturated"))?;
                }
            }
            Event::SurfaceHidden { surface } => {
                if let Some(instance) = parse_surface_instance(&surface) {
                    if native_close_key(runtime, instance).is_ok() {
                        crate::plugin_runtime::plugin_hide_surface(runtime, instance, &surface).await?;
                    }
                }
            }
            Event::SurfaceResized { .. } => {}
            Event::PatchAck { receipt, surface, revision } => {
                if !live_patch_receipt(runtime, receipt) {
                    continue;
                }
                with_pending_patches(|pending| pending.borrow_mut().apply_issued_ack(receipt, &surface, revision, semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES, |ack| PATCHES.with(|patches| patches.mark_published_ack(ack))))
                    .map_err(|reason| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.patch-ack-authority"), reason))?;
            }
            Event::PatchRejected { receipt, surface, revision, .. } => {
                if !live_patch_receipt(runtime, receipt) {
                    continue;
                }
                with_pending_patches(|pending| pending.borrow_mut().apply_issued_rejection(receipt, &surface, revision, |generation| PATCHES.with(|patches| patches.mark_rejected(&surface, generation))));
            }
            // 🔁️ Two delivery shapes share one id space. A `RequestRegistry::request` id parks a
            // future and is woken by `resolve`; a `request_continuation` id parks NOTHING and is
            // answered by redispatching its `response_action` into the owning app instance with the
            // outcome merged onto the original request object. Before this branch existed the second
            // shape did not: a plugin that wanted an extension result had to hand-mint a `RequestId`,
            // which owns no registry slot, so `resolve` silently dropped every extension outcome.
            Event::Completed { req, result } => {
                let outcome = crate::host::outcome_to_result(result);
                match take_extension_response(req, outcome) {
                    Ok((instance, action, args)) if native_close_key(runtime, instance).is_ok() => {
                        let output = crate::plugin_runtime::plugin_dispatch_response_action(runtime, instance, &action, &args).await;
                        for frame_bytes in output.frames {
                            route_app_frame(instance, &frame_bytes, &mut document_backbone_effects);
                        }
                        for one in &output.effects {
                            if let Ok(effect) = decode_wire_effect(one) {
                                push_admitted_effect(&mut document_backbone_effects, instance, effect);
                            }
                        }
                        for one in &output.events {
                            if let Ok(event) = decode_wire_app_event(one) {
                                document_backbone_effects.push(Effect::PublishEvent { topic: event.kind, payload: store::pack_rt::encode_wire_value(&event.payload) });
                            }
                        }
                    }
                    Ok(_) => eprintln!("[DEBUG] continuation resolve dropped req={} — owning instance has no acknowledged live lifetime", req.0),
                    Err(unclaimed) => REGISTRY.with(|registry| registry.resolve(req, unclaimed)),
                }
            }
            // 📄️ A host answer larger than one `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` page arrives
            // as PROLOGUE pages here and its terminal page rides the `Event::Completed` above, so
            // `Event::Completed` stays THE one completion door for a `req` and this arm never needs
            // a second dispatch site in the turn generator. A continuation claims its own pages;
            // everything else falls through to the parked-future accumulator.
            Event::HttpChunk { req, bytes, done } => {
                if append_extension_response_page(req, &bytes) {
                    continue;
                }
                // 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): used to discard every
                // non-final chunk outright (`if done { resolve(req, Ok(bytes)) }` — every earlier
                // `bytes` was simply dropped on the floor, silent data loss for any multi-chunk
                // response). `append_chunk` accumulates instead; `cap` is the owning instance's
                // `QuotaSchema.message_bytes` (default 64 MiB when unset/unknown — matches
                // `instance_task_quota`'s own `unwrap_or` fallback idiom above).
                REGISTRY.with(|registry| {
                    let cap = registry.instance_of(req).and_then(|instance| INSTANCE_METADATA.with(|metadata| metadata.borrow().get(instance).and_then(|entry| entry.quota.message_bytes))).unwrap_or(64 * 1024 * 1024) as usize;
                    registry.append_chunk(req, &bytes, done, cap);
                });
            }
            Event::JobProgress { job, .. } => {
                if let Some(binding) = JOB_RENDER_BINDINGS.with(|bindings| bindings.borrow().accepted(job)) {
                    let surface = ui_contract::UiText::try_format(format_args!("{}:window", binding.instance))
                        .map(ui_contract::SurfaceId)
                        .ok_or_else(|| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.surface-capacity"), "surface id exceeds fixed text capacity"))?;
                    dirty
                        .try_surface(binding.instance, surface)
                        .map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.dirty-surface-capacity"), "fixed dirty surface authority is saturated"))?;
                }
            }
            Event::JobCompleted { job, result } => {
                // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (J1, design-abi.md §4): a job spawned
                // through `host::jobs::spawn` (`🌐host/🦀️.rs`) allocates its `job` id from
                // THE SAME `RequestRegistry` counter as every other awaitable `host::*` call — the
                // `Effect::SpawnJob{job, ..}` this actor emitted carried `job == req.0` — so
                // `Event::JobCompleted{job, result}` resolves the identical parked `RequestFuture`
                // an `Event::Completed{req, result}` would, closing the "no `req`-per-job
                // correlation table yet" gap `📓️terra-M5-report.md` §4 named (no separate table
                // needed: the request id already IS the job id).
                let instance = JOB_RENDER_BINDINGS.with(|bindings| bindings.borrow().accepted(job).map(|binding| binding.instance));
                if let Some(binding) = JOB_RENDER_BINDINGS.with(|bindings| bindings.borrow_mut().complete(job)) {
                    let surface = ui_contract::UiText::try_format(format_args!("{}:window", binding.instance))
                        .map(ui_contract::SurfaceId)
                        .ok_or_else(|| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.surface-capacity"), "surface id exceeds fixed text capacity"))?;
                    dirty
                        .try_surface(binding.instance, surface)
                        .map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.dirty-surface-capacity"), "fixed dirty surface authority is saturated"))?;
                }
                let outcome = crate::host::outcome_to_result(result);
                if let Some(instance) = instance {
                    if native_close_key(runtime, instance).is_ok() {
                        let reserved_output = match &outcome {
                            Ok(bytes) => Ok(bytes.clone()),
                            Err(fault) => Err(fault.clone()),
                        };
                        let output = semio_framework::io::resolve_ready(crate::plugin_runtime::plugin_complete_reserved_spawned_job(runtime, instance, job, reserved_output));
                        for frame_bytes in output.frames {
                            route_app_frame(instance, &frame_bytes, &mut document_backbone_effects);
                        }
                        for one in &output.effects {
                            if let Ok(effect) = decode_wire_effect(one) {
                                push_admitted_effect(&mut document_backbone_effects, instance, effect);
                            }
                        }
                        for one in &output.events {
                            if let Ok(event) = decode_wire_app_event(one) {
                                document_backbone_effects.push(Effect::PublishEvent { topic: event.kind, payload: store::pack_rt::encode_wire_value(&event.payload) });
                            }
                        }
                    }
                }
                REGISTRY.with(|registry| registry.resolve(semio_framework::kernel::RequestId(job), outcome));
            }
            Event::Message { source: MessageEndpoint::Shell { instance }, payload } => {
                if let Some(token) = crate::app::TypedOperationResultPage::renderer_ack_token(&payload) {
                    if instance.0.parse::<u32>().ok() == Some(token.receiver) {
                        let _ = crate::plugin_runtime::plugin_acknowledge_typed_operation_result(runtime, token).await?;
                    }
                } else if let Some(effects) = crate::plugin_runtime::plugin_handle_document_backbone_binding(runtime, &instance.0, &payload).await? {
                    document_backbone_effects.extend(effects);
                }
            }
            Event::Message { source: MessageEndpoint::Backbone { uri }, payload } => {
                let output = crate::plugin_runtime::plugin_receive_document_backbone(runtime, &uri, &payload).await?;
                let surface = ui_contract::UiText::try_format(format_args!("{}:window", output.instance_id))
                    .map(ui_contract::SurfaceId)
                    .ok_or_else(|| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.surface-capacity"), "surface id exceeds fixed text capacity"))?;
                dirty.try_surface(output.instance_id, surface).map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.dirty-surface-capacity"), "fixed dirty surface authority is saturated"))?;
                for frame in output.frames {
                    route_app_frame(output.instance_id, &frame, &mut document_backbone_effects);
                }
                document_backbone_effects.extend(output.effects);
            }
            Event::Message { .. } => {}
            Event::Timer { id } => {
                ARMED_TIMERS.with(|timers| {
                    timers.borrow_mut().remove(id);
                });
                #[cfg(test)]
                TEST_FUTURE_EXECUTOR.with(|executor| executor.wake(id));
            }
            Event::Wake => {}
            // 📥️ The inbound half of the `request`/`respond` seam (`📜️.wit`'s `request-event`,
            // "every 'someone else calls INTO this actor' seam is now one inbound `request`,
            // answered with the `respond` effect within a bounded number of turns"). This actor's
            // installed `ExtensionBundle` is the ONE capability table that answers it, so the
            // request is served inline and answered on the SAME turn — an extension handler is a
            // pure `Fn(&[u8]) -> Result<Vec<u8>, Fault>` with nothing to await. An actor with no
            // bundle answers the bundle's own typed refusal (`extension.inactive`/
            // `extension.missing`), never silence: dropping the event stranded every caller's
            // parked request forever (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
            Event::Request { req, capability, payload, .. } => {
                let result = match crate::plugin_runtime::extension_invoke(&capability, &payload).await {
                    Ok(answer) => semio_framework::kernel::RequestOutcome::Ok(answer),
                    Err(fault) => semio_framework::kernel::RequestOutcome::Err(store::pack_rt::encode_wire_value(&dsl::to_dsl_value(&fault).map_err(|error| semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.request-fault-encode"), error.to_string()))?)),
                };
                inbound_request_effects.push(Effect::Respond { req, result });
            }
            Event::Activate { .. } | Event::SuspendRequest | Event::CapabilityChanged { .. } | Event::QuotaChanged { .. } => {}
        }
    }

    let mut effects: Vec<Effect> = document_backbone_effects;
    effects.extend(inbound_request_effects);
    let mut cold_pair_ingress = semio_framework::kernel::ColdPairIngressStatus::Idle;
    if let Some(page) = cold_pair_page {
        let lifetime = page.header.lifetime;
        let transfer_generation = page.header.transfer_generation;
        let terminal_cursor = page.header.cursor(page.header.page_count.saturating_sub(1));
        let live = native_close_key(runtime, lifetime.instance_id).ok().filter(|key| key.lifetime() == lifetime).map(|key| key.lifetime());
        cold_pair_ingress = COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().accept_page(&page, live));
        if matches!(cold_pair_ingress, semio_framework::kernel::ColdPairIngressStatus::Loading(_)) {
            let live = native_close_key(runtime, lifetime.instance_id).ok().filter(|key| key.lifetime() == lifetime).map(|key| key.lifetime());
            let load = COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().begin_load(lifetime, transfer_generation, live));
            if let Some(load) = load {
                let result = crate::plugin_runtime::plugin_load_document_pack(runtime, lifetime.instance_id, load.files()).await.map_err(|fault| dsl::encode_fault_bytes(&fault));
                let live = native_close_key(runtime, lifetime.instance_id).ok().filter(|key| key.lifetime() == lifetime).map(|key| key.lifetime());
                cold_pair_ingress = COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().finish_load(load, result, live));
            } else {
                cold_pair_ingress = semio_framework::kernel::ColdPairIngressStatus::Fault { cursor: terminal_cursor, fault: b"cold-pair.load-admission".to_vec() };
            }
        }
    }
    let mut command_ingress = semio_framework::kernel::CommandIngressStatus::Idle;
    let (mut retained, retained_slot, mut retained_key) = COMMAND_INGRESS.with(|ingress| {
        let mut ingress = ingress.borrow_mut();
        if ingress[0].is_some() {
            let entry = ingress[0].take().expect("retained command");
            (Some(entry.state), 0, Some(entry.key))
        } else {
            match ingress[1].take() {
                Some(entry) => (Some(entry.state), 1, Some(entry.key)),
                None => (None, 1, None),
            }
        }
    });
    if let Some(key) = retained_key {
        if native_close_key(runtime, key.instance()).ok() != Some(key) {
            close_instances.push(key.instance());
        }
    }
    retained = match retained.take() {
        Some(CommandIngressOwner::ReservedPresence { cursor, admission, .. }) if close_instances.contains(&cursor.instance) => {
            admission.cancel.cancel_now();
            command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-cancelled-by-close".to_vec() };
            None
        }
        Some(CommandIngressOwner::PendingPresencePage { cursor, .. }) if close_instances.contains(&cursor.instance) => {
            command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-cancelled-by-close".to_vec() };
            None
        }
        Some(CommandIngressOwner::Generic { cursor, command }) if close_instances.contains(&cursor.instance) => {
            command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
            Some(CommandIngressOwner::Generic {
                cursor,
                command: command.cancel(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.command-cancelled-by-close"), "command ingress was cancelled by instance close")),
            })
        }
        Some(CommandIngressOwner::GenericAssembly { cursor, pages }) if close_instances.contains(&cursor.instance) => {
            command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
            Some(CommandIngressOwner::ClosingAssembly { cursor, pages })
        }
        owner => owner,
    };
    retained = match retained.take() {
        Some(CommandIngressOwner::ClosingAssembly { cursor, mut pages }) => {
            let (complete, _) = pages.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
            if complete {
                command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-cancelled-by-close".to_vec() };
                None
            } else {
                command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
                Some(CommandIngressOwner::ClosingAssembly { cursor, pages })
            }
        }
        owner => owner,
    };
    retained = match retained.take() {
        Some(CommandIngressOwner::ReservedPresence { cursor, admission, page }) => {
            let now_ms = crate::host::now_ms().await;
            match crate::plugin_runtime::plugin_admit_reserved_presence(runtime, cursor.instance, admission, if cursor.metadata & 0x100 != 0 { Some((cursor.metadata & 0xff) as u8) } else { None }, cursor.item_count, page, now_ms).await {
                Ok(publication_generation) => {
                    command_ingress = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor.clone());
                    Some(CommandIngressOwner::Presence { cursor, publication_generation })
                }
                Err((_fault, admission, page)) => {
                    command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
                    Some(CommandIngressOwner::ReservedPresence { cursor, admission, page })
                }
            }
        }
        owner => owner,
    };
    retained = match retained.take() {
        Some(CommandIngressOwner::PendingPresencePage { cursor, publication_generation, page }) => match crate::plugin_runtime::plugin_push_reserved_presence_page(runtime, cursor.instance, publication_generation, cursor.page_index, page).await {
            Ok(()) => {
                command_ingress = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor.clone());
                Some(CommandIngressOwner::Presence { cursor, publication_generation })
            }
            Err((_fault, page)) => {
                command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
                Some(CommandIngressOwner::PendingPresencePage { cursor, publication_generation, page })
            }
        },
        owner => owner,
    };
    if matches!(command_ingress, semio_framework::kernel::CommandIngressStatus::Idle) {
        if let Some(CommandIngressOwner::Presence { cursor, .. }) = retained.as_ref() {
            let cursor = cursor.clone();
            if close_instances.contains(&cursor.instance) {
                retained = None;
                command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-cancelled-by-close".to_vec() };
            } else {
                match plugin_exchange_boxed(runtime, cursor.instance, None).await {
                    Ok(output) => {
                        let instance = cursor.instance;
                        if output.presence_terminal == Some(cursor.seq) {
                            retained = None;
                            command_ingress = match advance_command_cursor(cursor) {
                                Ok(terminal) => match output.presence_terminal_fault.as_ref() {
                                    Some(fault) => semio_framework::kernel::CommandIngressStatus::Fault { cursor: terminal, fault: fault.clone() },
                                    None => semio_framework::kernel::CommandIngressStatus::CommandComplete(terminal),
                                },
                                Err(cursor) => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-index-exhausted".to_vec() },
                            };
                        } else if cursor.page_index.checked_add(1) == Some(cursor.page_count) {
                            command_ingress = match advance_command_cursor(cursor.clone()) {
                                Ok(pending) => semio_framework::kernel::CommandIngressStatus::CommandPending(pending),
                                Err(cursor) => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-index-exhausted".to_vec() },
                            };
                        }
                        route_exchange_output(instance, output, &mut effects);
                    }
                    Err(fault) => {
                        command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: dsl::encode_fault_bytes(&fault) };
                        retained = None;
                    }
                }
            }
        }
    }
    if let Some(owner) = retained.take() {
        match owner {
            CommandIngressOwner::Generic { cursor, command } => match plugin_exchange_boxed(runtime, cursor.instance, Some((cursor.seq, command))).await {
                Ok(mut output) => {
                    match advance_command_cursor(cursor.clone()) {
                        Ok(terminal) => {
                            if let Some((_, command)) = output.retry_command.take() {
                                retained = Some(CommandIngressOwner::Generic { cursor: cursor.clone(), command });
                                command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(terminal);
                            } else if let Some(fault) = output.command_terminal_fault.as_ref() {
                                command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor: terminal, fault: fault.clone() };
                            } else {
                                command_ingress = semio_framework::kernel::CommandIngressStatus::CommandComplete(terminal);
                            }
                        }
                        Err(cursor) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-index-exhausted".to_vec() },
                    }
                    route_exchange_output(cursor.instance, output, &mut effects);
                }
                Err(fault) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: dsl::encode_fault_bytes(&fault) },
            },
            owner => retained = Some(owner),
        }
    }
    if let Some((cursor, page)) = command_page {
        if retained.is_none() {
            retained_key = native_close_key(runtime, cursor.instance).ok();
        }
        if !runtime.guest_lifetimes.borrow().get(cursor.instance).is_some_and(|slot| slot.cell.is_live())
            || close_instances.contains(&cursor.instance)
            || page.len() > semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES
            || cursor.page_count == 0
            || cursor.page_count as usize > semio_framework::kernel::COMMAND_MAXIMUM_PAGES
            || cursor.command_count == 0
            || cursor.command_count as usize > semio_framework::kernel::COMMAND_BATCH_MAXIMUM_ITEMS
            || cursor.command_index >= cursor.command_count
            || cursor.page_index >= cursor.page_count
            || cursor.item_count as usize > semio_framework::kernel::COMMAND_BATCH_MAXIMUM_ITEMS
            || (cursor.kind == 28 && cursor.page_count != cursor.item_count.max(1))
            || (cursor.kind == 28 && ((cursor.item_count == 0) != page.is_empty()))
            || (cursor.kind != 28
                && (page.is_empty()
                    || cursor.item_count != 0
                    || cursor.metadata != 0
                    || (cursor.page_index == 0 && cursor.kind != page.as_slice()[0])
                    || (cursor.page_index.checked_add(1).is_some_and(|next| next < cursor.page_count) && page.len() != semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES)))
        {
            command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-invalid".to_vec() };
        } else if retained.as_ref().is_some_and(|owner| match owner {
            CommandIngressOwner::ReservedPresence { cursor: active, .. }
            | CommandIngressOwner::Presence { cursor: active, .. }
            | CommandIngressOwner::PendingPresencePage { cursor: active, .. }
            | CommandIngressOwner::GenericAssembly { cursor: active, .. }
            | CommandIngressOwner::ClosingAssembly { cursor: active, .. }
            | CommandIngressOwner::Generic { cursor: active, .. } => !same_command_cursor(active, &cursor),
        }) {
            command_ingress = semio_framework::kernel::CommandIngressStatus::Backpressure(cursor);
        } else if matches!(retained, Some(CommandIngressOwner::ReservedPresence { .. } | CommandIngressOwner::PendingPresencePage { .. })) {
            command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor);
        } else if cursor.kind == 28 {
            let own_color = if cursor.metadata & !0x1ff != 0 {
                command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor: cursor.clone(), fault: b"plugin.command-presence-metadata".to_vec() };
                None
            } else if cursor.metadata & 0x100 != 0 {
                Some((cursor.metadata & 0xff) as u8)
            } else {
                None
            };
            if !matches!(command_ingress, semio_framework::kernel::CommandIngressStatus::Fault { .. }) {
                if cursor.page_index == 0 && retained.is_none() {
                    match crate::plugin_runtime::plugin_reserve_presence_ingress(runtime, cursor.instance, cursor.seq).await {
                        Ok(admission) => {
                            let now_ms = crate::host::now_ms().await;
                            match crate::plugin_runtime::plugin_admit_reserved_presence(runtime, cursor.instance, admission, own_color, cursor.item_count, page, now_ms).await {
                                Ok(publication_generation) => {
                                    retained = Some(CommandIngressOwner::Presence { cursor: cursor.clone(), publication_generation });
                                    command_ingress = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor);
                                }
                                Err((_fault, admission, page)) => {
                                    retained = Some(CommandIngressOwner::ReservedPresence { cursor: cursor.clone(), admission, page });
                                    command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor);
                                }
                            }
                        }
                        Err(_) => command_ingress = semio_framework::kernel::CommandIngressStatus::Backpressure(cursor),
                    }
                } else if let Some(publication_generation) = retained.as_ref().and_then(|owner| match owner {
                    CommandIngressOwner::Presence { publication_generation, .. } => Some(*publication_generation),
                    _ => None,
                }) {
                    match crate::plugin_runtime::plugin_push_reserved_presence_page(runtime, cursor.instance, publication_generation, cursor.page_index, page).await {
                        Ok(()) => {
                            if let Some(CommandIngressOwner::Presence { cursor: active, .. }) = retained.as_mut() {
                                active.page_index = cursor.page_index;
                            }
                            command_ingress = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor);
                        }
                        Err((_fault, page)) => {
                            retained = Some(CommandIngressOwner::PendingPresencePage { cursor: cursor.clone(), publication_generation, page });
                            command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor);
                        }
                    }
                }
            }
        } else if cursor.page_index == 0 && retained.is_none() {
            match semio_framework::kernel::CommandPageSet::try_new(cursor.page_count as usize) {
                Ok(mut pages) => match pages.try_push(page) {
                    Err((fault, _page)) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: dsl::encode_fault_bytes(&fault) },
                    Ok(()) if cursor.page_count == 1 => match semio_framework::kernel::PagedCommand::try_from_pages(pages) {
                        Ok(command) => match plugin_exchange_boxed(runtime, cursor.instance, Some((cursor.seq, crate::plugin_runtime::PluginCommandIngress::Encoded(command)))).await {
                            Ok(mut output) => {
                                if let Some((_, command)) = output.retry_command.take() {
                                    retained = Some(CommandIngressOwner::Generic { cursor: cursor.clone(), command });
                                    command_ingress = match advance_command_cursor(cursor.clone()) {
                                        Ok(pending) => semio_framework::kernel::CommandIngressStatus::CommandPending(pending),
                                        Err(cursor) => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-index-exhausted".to_vec() },
                                    };
                                } else {
                                    command_ingress = terminal_command_ingress(cursor.clone(), output.command_terminal_fault.take());
                                }
                                route_exchange_output(cursor.instance, output, &mut effects);
                            }
                            Err(fault) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: dsl::encode_fault_bytes(&fault) },
                        },
                        Err((fault, _pages)) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: dsl::encode_fault_bytes(&fault) },
                    },
                    Ok(()) => {
                        retained = Some(CommandIngressOwner::GenericAssembly { cursor: cursor.clone(), pages });
                        command_ingress = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor);
                    }
                },
                Err(fault) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: dsl::encode_fault_bytes(&fault) },
            }
        } else if let Some(CommandIngressOwner::GenericAssembly { cursor: active, mut pages }) = retained.take() {
            if cursor.page_index as usize != pages.len() {
                command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-order".to_vec() };
            } else {
                match pages.try_push(page) {
                    Err((fault, _page)) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: dsl::encode_fault_bytes(&fault) },
                    Ok(()) if cursor.page_index.checked_add(1) == Some(cursor.page_count) => match semio_framework::kernel::PagedCommand::try_from_pages(pages) {
                        Ok(command) => match plugin_exchange_boxed(runtime, cursor.instance, Some((cursor.seq, crate::plugin_runtime::PluginCommandIngress::Encoded(command)))).await {
                            Ok(mut output) => {
                                if let Some((_, command)) = output.retry_command.take() {
                                    retained = Some(CommandIngressOwner::Generic { cursor: cursor.clone(), command });
                                    command_ingress = match advance_command_cursor(cursor.clone()) {
                                        Ok(pending) => semio_framework::kernel::CommandIngressStatus::CommandPending(pending),
                                        Err(cursor) => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-index-exhausted".to_vec() },
                                    };
                                } else {
                                    command_ingress = terminal_command_ingress(cursor.clone(), output.command_terminal_fault.take());
                                }
                                route_exchange_output(cursor.instance, output, &mut effects);
                            }
                            Err(fault) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: dsl::encode_fault_bytes(&fault) },
                        },
                        Err((fault, _pages)) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: dsl::encode_fault_bytes(&fault) },
                    },
                    Ok(()) => {
                        retained = Some(CommandIngressOwner::GenericAssembly { cursor: active, pages });
                        if let Some(CommandIngressOwner::GenericAssembly { cursor: active, .. }) = retained.as_mut() {
                            active.page_index = cursor.page_index;
                        }
                        command_ingress = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor);
                    }
                }
            }
        }
    }
    if let Some(retained) = retained {
        COMMAND_INGRESS.with(|ingress| {
            ingress.borrow_mut()[retained_slot] = Some(RetainedCommandIngress { key: retained_key.expect("admitted command retains its exact lifetime"), state: retained });
        });
    }
    // 🎯️ M1: surviving intents dispatch through the SAME `route_app_frame`/effects/events plumbing
    // as `app_commands` above, immediately after it (so a mutation an app command made this turn is
    // already visible to the intent's own dispatch) — via the NEW `plugin_dispatch_intents`, which
    // routes through the app's EXISTING typed command path (`PluginApp::handle_intent_frame` →
    // `ArtifactApp::command_from_intent` → `dispatch_typed_command_inner`), never a parallel path.
    // Each handled batch's surfaces feed the retained render set so the reply patch — the next `UiPatch`
    // revision bump — is produced in this SAME turn (design decision: no new reply channel).
    let intent_batches = std::mem::take(&mut dirty.intents);
    for DirtyIntentBatch { instance, intents } in intent_batches {
        native_close_key(runtime, instance)?;
        // 🚫️async: E5 executor bridge — `plugin_dispatch_intents` stays genuinely `async fn`; safe to
        // resolve synchronously here for the same reason as `plugin_exchange` above.
        match crate::plugin_runtime::plugin_dispatch_intents(runtime, instance, &intents).await {
            Ok(output) => {
                for frame_bytes in output.frames {
                    route_app_frame(instance, &frame_bytes, &mut effects);
                }
                for one in &output.effects {
                    if let Ok(effect) = decode_wire_effect(one) {
                        push_admitted_effect(&mut effects, instance, effect);
                    }
                }
                for one in &output.events {
                    if let Ok(event) = decode_wire_app_event(one) {
                        effects.push(Effect::PublishEvent { topic: event.kind, payload: store::pack_rt::encode_wire_value(&event.payload) });
                    }
                }
            }
            Err(fault) => effects.push(shell_fault_effect(instance, &fault)),
        }
        // 🌳️ Deduped per instance — several intents on the same surface this turn must not queue a
        // redundant re-render (the second `diff()` would return `None` anyway, but there is no reason
        // to pay for it).
        let mut surfaces: semio_framework_ui_contract::UiFixedList<semio_framework_ui_contract::UiText> = semio_framework_ui_contract::UiFixedList::default();
        for intent in &intents {
            if surfaces.iter().any(|surface| surface == &intent.surface.0) {
                continue;
            }
            if surfaces.try_push(intent.surface.0.clone()).is_err() {
                effects.push(shell_fault_effect(instance, &semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.surface-capacity"), "dirty render surface capacity exceeded")));
                break;
            }
        }
        for surface in surfaces {
            dirty
                .try_surface(instance, ui_contract::SurfaceId(surface))
                .map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.dirty-surface-capacity"), "fixed dirty surface authority is saturated"))?;
        }
    }

    trace_turn_phase_retention("ingress");
    let (continuation, typed_operation_scan) = crate::plugin_runtime::plugin_continue_typed_operations(runtime, crate::plugin_runtime::TypedOperationGrant::turn(budget)).await?;
    let typed_operation_contended = typed_operation_scan.contended;
    if let Some((instance, output)) = continuation {
        route_exchange_output(instance, output, &mut effects);
    }

    // 👥️ M2 (ticket 26/08/17 `design-unified.md`): `now_ms` is read ONCE for both `record_peer`'s
    // expiry stamping below and `PRESENCE.expire` at the end of this turn — a single wall-clock
    // reading per poll, not one per presence update.
    trace_turn_phase_retention("continuation");
    let now_ms = u64::try_from(crate::host::now_ms().await).unwrap_or(0);

    trace_turn_phase_retention("presence-clock");
    for (instance, surface) in dirty.surfaces {
        if native_close_key(runtime, instance).is_err() {
            continue;
        }
        let surface_key = surface.as_ref().to_owned();
        let mounted = match native_close_key(runtime, instance) {
            Ok(key) => PATCHES.with(|patches| patches.reserve_mounted(surface, key)),
            Err(_) => Err(surface),
        };
        match mounted {
            Ok(grant) => match crate::plugin_runtime::plugin_render_surface(runtime, instance, &surface_key).await {
                Ok((tree, presence)) => {
                    let _ = grant.commit_source(tree.root);
                    for update in presence {
                        PRESENCE.with(|hub| {
                            let mut hub = hub.borrow_mut();
                            hub.record_own(update.surface.clone(), update.node_key.clone(), update.own, update.ttl_ms);
                            for peer in update.peers {
                                hub.record_peer(update.surface.clone(), update.node_key.clone(), peer, update.ttl_ms, now_ms);
                            }
                        });
                    }
                }
                Err(fault) => {
                    grant.cancel();
                    effects.push(shell_fault_effect(instance, &fault));
                }
            },
            Err(surface) => PATCHES.with(|patches| {
                let _ = patches.defer(surface);
            }),
        }
    }
    let reconcile_work = PATCHES
        .with(|patches| -> Result<bool, &'static str> {
            let opportunities = reconcile_step_opportunities(budget.fuel);
            let deadline = std::time::Instant::now() + std::time::Duration::from_millis(u64::from(budget.deadline_ms));
            let mut more = patches.has_publishable_work();
            for opportunity in 0..opportunities {
                if !more {
                    break;
                }
                if opportunity > 0 && opportunity % 64 == 0 && std::time::Instant::now() >= deadline {
                    break;
                }
                patches.drive_one();
                more = patches.has_publishable_work();
                let can_publish = with_pending_patches(|pending| pending.borrow().has_capacity());
                if can_publish {
                    if let Some((key, generation)) = patches.ready_patch_key()? {
                        let mut target = None;
                        if patches.take_ready_patch_into(key, generation, &mut target, semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES)? {
                            if let Some(patch) = target {
                                // 🩹️ `take_ready_patch_into` already committed this output to its closing
                                // lifecycle; there is no `return_ready_patch` any more, so losing this
                                // capacity race simply drops the extracted page instead of re-queueing it.
                                match with_pending_patches(|pending| pending.borrow_mut().push_reconcile(patch)) {
                                    Ok(()) => {}
                                    Err(_dropped) => {}
                                }
                            }
                        }
                    }
                }
            }
            Ok(more || patches.has_publishable_work() || with_pending_patches(|pending| pending.borrow().has_unpublished()))
        })
        .map_err(|reason| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.patch-reconcile-authority"), reason))?;
    if let Some((instance, message)) = PATCHES.with(patches::PatchTracker::take_render_fault) {
        effects.push(shell_fault_effect(instance, &semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.surface-render"), message)));
    }

    // 🚫️async: E5 executor bridge (× 2) — `LocalExecutor::{run_until_idle,has_ready}` stay
    // genuinely `async fn` (its own doc: "run_until_idle handles Pending without ever yielding
    // its own future" — matches `⚛️reactor/💼️jobs`'s identical use of this exact bridge).
    let executor_deadline_work = REACTOR_EXECUTOR.with(|executor| executor.run_until_deadline(64, 256 * 1_024, std::time::Instant::now() + std::time::Duration::from_millis(8)));
    let process_pool_work = !executor_deadline_work && pump_process_worker_pool();
    let more_work = executor_deadline_work || process_pool_work;
    for effect in REGISTRY.with(|registry| registry.drain()) {
        push_admitted_effect(&mut effects, 0, effect);
    }

    // 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): resumed `AsyncTask` follow-
    // ups (and any replayed `task_restarts` from a `restore` before this turn) — AFTER
    // `run_until_idle` so a task that resolved just now is redispatched the SAME turn, not the
    // next one. A resume can itself spawn more tasks (`dispatch_emit` runs for real), so the
    // executor may have fresh ready work by the time this returns — folded into `more_work` below
    // rather than requiring a second `run_until_idle` pass this turn (the next `poll` picks it up).
    let resumes_remain = drain_task_resumes(runtime, &mut effects, 64);
    effects.extend(crate::plugin_runtime::plugin_drain_document_backbones(runtime)?);
    let executor_pending = REACTOR_EXECUTOR.with(|executor| executor.has_pending());
    let command_ingress_pending = COMMAND_INGRESS.with(|ingress| ingress.borrow().iter().any(Option::is_some));
    let lifecycle_work = runtime.guest_lifetimes.borrow().has_work();
    // 🔒️ `typed_operation_scan.contended` is deliberately NOT folded in. A busy instance lock is not
    // an answer of "there is runnable work", it is "another owner is mid-step and I could not look" —
    // and the guest is single-threaded, so on wasm it can only ever be a reader inside this very turn.
    // Answering `MoreWork` for it turned every concurrent read into a host turn round trip that
    // produced nothing: 12 of 512 idle generation3d turns, measured 2026-09-10. Every owner that can
    // hold that lock is itself either a reactor executor task (`executor_pending`), a task resume
    // (`resumes`), an ingress command (`command_ingress`) or a lifecycle step (`lifecycle`), each of
    // which already arms this turn on its own account. See `📓️idle-turns-2026-09-10.md`.
    let more_work = more_work || close_cleanup_work || typed_operation_scan.runnable || reconcile_work || resumes_remain || executor_pending || command_ingress_pending || lifecycle_work;
    LAST_MORE_WORK_SOURCES.set(TurnMoreWorkSources {
        executor_deadline: executor_deadline_work,
        process_pool: process_pool_work,
        close_cleanup: close_cleanup_work,
        typed_operation: typed_operation_scan.runnable,
        typed_operation_contended,
        reconcile: reconcile_work,
        resumes: resumes_remain,
        executor_pending,
        command_ingress: command_ingress_pending,
        lifecycle: lifecycle_work,
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
                "[DEBUG] reactor more-work streak={streak} seen={seen} sources={:?} contended={typed_operation_contended} effects={} patches=[{}] pending=[{}]",
                LAST_MORE_WORK_SOURCES.get().names(),
                effects.len(),
                PATCHES.with(patches::PatchTracker::debug_state),
                with_pending_patches(|pending| pending.borrow().debug_state())
            ));
        } else if trace.0 > 0 {
            trace_guest_line(&format!("[DEBUG] reactor more-work streak ended after {} turns (seen={seen})", trace.0));
        }
        *trace = (streak, seen);
    });

    let lifecycle_receipt = focus.map(|instance| runtime.guest_lifetimes.borrow_mut().prepare_turn(instance)).transpose()?.flatten();
    let mut ui_patches = semio_framework::kernel::UiTurnPatches::default();
    let mut ui_patch_receipt = None;
    let taken = with_pending_patches(|pending| pending.borrow_mut().take_one(semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES))
        .map_err(|reason| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.pending-patch-authority"), reason))?;
    if let Some(patch) = taken {
        let instance = parse_surface_instance(&patch.surface.0);
        match ui_patches.try_push_ui_patch(patch) {
            Ok(()) => ui_patch_receipt = instance.and_then(|instance| runtime.guest_lifetimes.borrow_mut().next_patch_receipt(instance)),
            Err(patch) => {
                with_pending_patches(|pending| pending.borrow_mut().hand_back_turn(patch)).expect("exact unpublished patch returns to its reserved slot");
            }
        }
    }
    // 👥️ M2: once per poll — expire ages-out peer marks, then flush drains every key touched since
    // the last flush into one coalesced `PresenceUpdate` each (free burst coalescing: a hover storm
    // between polls still costs exactly one update per `(surface, node_key)`).
    let presence = PRESENCE.with(|hub| {
        let mut hub = hub.borrow_mut();
        hub.expire(now_ms);
        hub.flush()
    });
    let status = if more_work { TurnStatus::MoreWork } else { TurnStatus::Idle };

    let mut result = semio_framework::kernel::TurnResult { ui_patches, effects, presence, next_wake: ARMED_TIMERS.with(|timers| timers.borrow().first()), status, fuel_used: 0, command_ingress, cold_pair_ingress, lifecycle_receipt, ui_patch_receipt };
    with_pending_patches(|pending| {
        let mut pending = pending.borrow_mut();
        let prepared = (|| {
            result.validate_ui_patch_receipt().map_err(reactor_close_fault)?;
            if let Some(receipt) = result.ui_patch_receipt {
                pending.stage_emission(receipt, result.ui_patches.iter().next().expect("paired patch owner")).map_err(reactor_close_fault)?;
            }
            let prepared = prepare(&result)?;
            if let Some(instance) = focus {
                runtime.guest_lifetimes.borrow_mut().finish_turn(instance, guest_turn_executing_us()).map_err(|reason| {
                    if reason == instance_lifetime::GUEST_LIFECYCLE_TURN_DEADLINE {
                        semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.reactor-turn-deadline"), reason).with_retryable(retryable_lifecycle)
                    } else {
                        reactor_close_fault(reason)
                    }
                })?;
            }
            Ok(prepared)
        })();
        match prepared {
            Err(fault) => {
                let returned = result.ui_patches.try_transfer_one(|patch| pending.hand_back_turn(patch));
                assert!(!matches!(returned, semio_framework::kernel::UiTurnPatchTransfer::Refused), "failed output retains its exact pending patch slot");
                Err(fault)
            }
            Ok(prepared) => {
                if result.ui_patch_receipt.is_some() {
                    pending.commit_emission();
                }
                let published = publish(result, prepared);
                trace_turn_phase_retention("publish");
                Ok(published)
            }
        }
    })
}

/// 🏃️ Runs queued process-pool job steps inside this turn on wasm, where the pool has no threads and
/// a step submitted by a mounted worker session (retained commands, framework reserved routes) only
/// executes when the pool is pumped. Before this the only pump was the cooperative-maintenance cadence,
/// so every submitted step waited ~170 reactor turns (measured 2026-09-09). Bounded by
/// [`PROCESS_POOL_PUMPS_PER_TURN`] and [`PROCESS_POOL_WALL_MS`]; returns whether pool work remains.
fn pump_process_worker_pool() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        PROCESS_POOL.with(|pool| {
            if !pool.has_pending_work() {
                return false;
            }
            let deadline = std::time::Instant::now() + std::time::Duration::from_millis(PROCESS_POOL_WALL_MS);
            let mut pumps = 0;
            while pumps < PROCESS_POOL_PUMPS_PER_TURN && pool.has_pending_work() && std::time::Instant::now() < deadline {
                let Some(now_ms) = semio_framework_job::default_now_ms() else { break };
                pool.pump(now_ms);
                pumps += 1;
            }
            pool.has_pending_work()
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    false
}

#[cfg(target_arch = "wasm32")]
crate::component_persistent_local! {
    /// 🧵️ The one process worker pool, resolved once instead of once per turn.
    ///
    /// `process_worker_pool` re-reads `available_parallelism`, rebuilds a `WorkerPoolConfig` and
    /// re-asserts it against the established one on every call; `pump_process_worker_pool` runs on
    /// EVERY reactor turn, so that was a per-turn cost on the guest's hottest path for a handle that
    /// can never change.
    static PROCESS_POOL: semio_framework_async::WorkerPool = {
        let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
        semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, cores))
    };
}

/// 🏃️ Upper bound on process-pool pumps per reactor turn on wasm.
/// 🧹️ Drives one per-turn retirement ladder entry for a bounded RUN of units instead of the single
/// unit it used to get, stopping as soon as the entry reports itself complete, its run is spent, or
/// the turn's own wall-clock budget is gone.
///
/// A whole-document swap fills every one of these arenas with DOCUMENT-scaled garbage (the replaced
/// window bodies, their patches, their built-node pages), and each `…_one()` retires one item. At one
/// unit per turn that is one host↔guest round trip per retired item, because the same turn answers
/// `MoreWork` while any of it is outstanding — measured 2026-09-10 (ticket 26/09/02 W-S2) as 8 799
/// worker messages and 24.3 s for the 180-object Nakagin switch, of which only 5.4 s was main-thread
/// work. Retirement is bounded work per TURN, not per item.
// 🚫️async: E1 pure bounded retirement driver called from the turn's own close ladder — see R9.
fn retire_until_complete(deadline: std::time::Instant, mut unit: impl FnMut() -> bool) {
    for index in 0..PATCH_CLOSE_UNITS_PER_TURN {
        if unit() {
            return;
        }
        if index % PATCH_CLOSE_DEADLINE_STRIDE == 0 && std::time::Instant::now() >= deadline {
            return;
        }
    }
}

/// 🧹️ [`retire_until_complete`] for a ladder entry that reports PROGRESS rather than completion.
// 🚫️async: E1 pure bounded retirement driver called from the turn's own close ladder — see R9.
fn retire_while_progress(deadline: std::time::Instant, mut unit: impl FnMut() -> bool) {
    for index in 0..PATCH_CLOSE_UNITS_PER_TURN {
        if !unit() {
            return;
        }
        if index % PATCH_CLOSE_DEADLINE_STRIDE == 0 && std::time::Instant::now() >= deadline {
            return;
        }
    }
}

/// 🧹️ [`retire_while_progress`] with its own unit ceiling — for a ladder whose fixed slot geometry is
/// wider than the patch ladder's 256 units.
// 🚫️async: E1 pure bounded retirement driver called from the turn's own close ladder — see R9.
fn retire_while_progress_bounded(deadline: std::time::Instant, units: usize, mut unit: impl FnMut() -> bool) {
    for index in 0..units {
        if !unit() {
            return;
        }
        if index % PATCH_CLOSE_DEADLINE_STRIDE == 0 && std::time::Instant::now() >= deadline {
            return;
        }
    }
}

/// 🧹️ [`retire_while_progress_bounded`] for a ladder unit that may fault — the fault propagates out
/// of the turn instead of being swallowed by the driver.
// 🚫️async: E1 pure bounded retirement driver called from the turn's own close ladder — see R9.
fn retire_while_progress_fallible(deadline: std::time::Instant, units: usize, mut unit: impl FnMut() -> Result<bool, semio_framework::Fault>) -> Result<(), semio_framework::Fault> {
    for index in 0..units {
        if !unit()? {
            return Ok(());
        }
        if index % PATCH_CLOSE_DEADLINE_STRIDE == 0 && std::time::Instant::now() >= deadline {
            return Ok(());
        }
    }
    Ok(())
}

/// 🚪️ Instance-close ladder units per reactor turn. The reactor close walks FIXED slot geometry —
/// [`super::REACTOR_TASK_SLOTS`] task slots, the request registry's own slot array, the armed-timer
/// array — one slot per unit, and until 2026-09-12 it took exactly ONE of those units per reactor
/// turn. Measured (`🧫️fixtures/🚪️close-ladder`, ticket 26/09/09): a generation3d editor reached
/// `Retired` after **2 052** close turns whether the session had loaded zero documents or eight with
/// thirty-two preview renders — the cost is the geometry, not the retained data. In the browser each
/// of those turns is one worker round trip, so the ⌘⌥V role switch spent 62–87 s inside
/// `retireInstanceLifecycle` and then failed `plugin-ui.lifecycle-close-budget-exhausted`. Driving the
/// whole geometry in ONE turn, cut short by the turn's own wall-clock budget every
/// [`PATCH_CLOSE_DEADLINE_STRIDE`] units, makes a close cost a bounded handful of round trips — the
/// same correction [`PATCH_CLOSE_UNITS_PER_TURN`] applied to the patch ladder.
const REACTOR_CLOSE_UNITS_PER_TURN: usize = 4_096;
/// 🧹️ Retained terminal retirement units per reactor turn, bounded so one turn stays inside its
/// interactive ceiling — [`PATCH_CLOSE_DEADLINE_STRIDE`] cuts the run short on the turn's own
/// wall-clock budget, exactly like the reconcile loop above it.
///
/// This is NOT background cleanup that a slow drip can be indifferent to: an acknowledged publication
/// keeps `PendingPatchAuthority::has_unpublished` — and therefore the whole turn — in `MoreWork` until
/// it is retired, and the host answers every `MoreWork` with another turn ROUND TRIP. One 180-object
/// world-3d publication takes 1 092 retirement units; at the previous 8 units per turn that is 137
/// round trips per published surface, and it is why the Nakagin example switch took 24.3 s and 8 799
/// worker messages with only 5.4 s of main-thread work in it (measured 2026-09-10, ticket 26/09/02
/// W-S2). At 256 it is 5. The turn stays interactive because [`PATCH_CLOSE_DEADLINE_STRIDE`] re-reads
/// the clock every 8 units against the turn's own budget — the unit count is the ceiling, the
/// deadline is the bound.
pub(crate) const PATCH_CLOSE_UNITS_PER_TURN: usize = 256;
/// ⏱️ How often the retirement run re-reads the clock — the same 64-opportunity stride the reconcile
/// loop uses, scaled to this shorter run.
const PATCH_CLOSE_DEADLINE_STRIDE: usize = 8;
/// 🧹️ Items one retirement unit may retire. Priced per PAGE like every other stage of the turn
/// (`SURFACE_RECONCILE_PAGE_BYTES` for bytes, this for items) instead of per item, so a
/// document-scaled patch retires in a bounded handful of turns instead of one turn per item.
pub(crate) const PATCH_RETIREMENT_ITEMS_PER_UNIT: usize = 1_024;
/// 🧹️ Bytes one retirement unit may retire — the publication's own page budget.
pub(crate) const PATCH_RETIREMENT_BYTES_PER_UNIT: usize = semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES;
#[cfg(target_arch = "wasm32")]
const PROCESS_POOL_PUMPS_PER_TURN: usize = 64;
/// ⏱️ Wall-clock bound on process-pool pumping per reactor turn on wasm.
#[cfg(target_arch = "wasm32")]
const PROCESS_POOL_WALL_MS: u64 = 2;

fn live_patch_receipt<PA: crate::app::PluginApp>(runtime: &crate::plugin_runtime::PluginRuntime<PA>, receipt: ActorUiPatchReceipt) -> bool {
    runtime.guest_lifetimes.borrow().get(receipt.lifetime.instance_id).is_some_and(|slot| slot.cell.is_live() && slot.cell.lifetime() == receipt.lifetime)
}

fn route_exchange_output(instance: u32, output: crate::plugin_runtime::PluginExchangeOutput, effects: &mut Vec<Effect>) {
    if let Some(page) = output.typed_operation_result.as_ref() {
        effects.push(Effect::SendMessage { target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) }, payload: page.renderer_exchange_bytes() });
    }
    for frame_bytes in output.frames {
        route_app_frame(instance, &frame_bytes, effects);
    }
    for one in &output.effects {
        if let Ok(effect) = decode_wire_effect(one) {
            push_admitted_effect(effects, instance, effect);
        }
    }
    for one in &output.events {
        if let Ok(event) = decode_wire_app_event(one) {
            effects.push(Effect::PublishEvent { topic: event.kind, payload: store::pack_rt::encode_wire_value(&event.payload) });
        }
    }
}

fn same_command_cursor(left: &semio_framework::kernel::CommandPageCursor, right: &semio_framework::kernel::CommandPageCursor) -> bool {
    left.owner == right.owner
        && left.generation == right.generation
        && left.command_index == right.command_index
        && left.command_count == right.command_count
        && left.instance == right.instance
        && left.seq == right.seq
        && left.kind == right.kind
        && left.page_count == right.page_count
        && left.item_count == right.item_count
        && left.metadata == right.metadata
}

use super::pending::{parse_surface_instance, with_state as with_pending_patches};

/// 🔀️ `AppFrame::UiPatch` → a real `kernel::UiPatch` passthrough into `PENDING_PATCHES` (the wire
/// frame is already `UiPatch`-shaped field-for-field — channel v12/A4 — so this is a decode, not a
/// render); `AppFrame::Effects`/`Events` no longer exist as frames (`poll` decodes
/// `plugin_exchange`'s `PluginExchangeOutput.effects`/`.events` directly instead — see there);
/// `AppFrame::UiSnapshotEnd` has no consumer yet in this wave (patches apply incrementally, no
/// snapshot-boundary bookkeeping); everything else → `Effect::SendMessage` to the shell, matching
/// design-abi.md §2's table verbatim.
#[expect(clippy::result_large_err, reason = "A rejected external patch is returned intact by its fixed pending-queue callback.")]
fn route_app_frame(instance: u32, frame_bytes: &[u8], effects: &mut Vec<Effect>) {
    // 🚫️async: E5 executor bridge (× 3) — `protocol::{decode,encode}_app_frame` (`📡️spr/**`, out
    // of `path_scope`) and `store::pack_rt::decode_wire_value` stay genuinely `async fn`; safe to
    // resolve synchronously for the same reason as this file's other WIT-boundary bridges.
    let Ok(frame) = semio_framework::io::resolve_ready(protocol::decode_app_frame(frame_bytes)) else {
        return;
    };
    match frame {
        // 🎯️ `sdk-flip` (26/08/20): `protocol::AppFrame::UiPatch` still carries the PRE-flip shape
        // (`kind: String`, `ops` pack-encoding the old `kernel::PatchOp`) — its crate, `📡️spr/**`, is
        // FORBIDDEN to this packet, so the frame struct itself is untouched. `kind` is bound but
        // dropped (the new `UiPatch` has no such field); `ops` decodes into `UiPatchOp` on the
        // OPTIMISTIC assumption the sender re-encodes with the new op set too — genuinely stale
        // until whichever packet updates `📡️spr/🧵️channel` re-frames this variant to match (flagged
        // in `📓️terra-wit-flip-report.md`'s consumer inventory; not fixed here, out of `OWNS`).
        protocol::AppFrame::UiPatch { surface, kind: _, revision, base_revision, ops, .. } => {
            let Ok(ops_value) = store::pack_rt::decode_wire_value(&ops) else { return };
            let Ok(ops) = serde_json::from_value::<ui_contract::UiPatchOps>(ops_value.into()) else { return };
            let Ok(surface) = ui_contract::SurfaceId::try_from(surface) else { return };
            let patch = UiPatch { surface, base_revision: ui_contract::UiRevision(base_revision), revision: ui_contract::UiRevision(revision), ops };
            if let Err(patch) = with_pending_patches(|pending| pending.borrow_mut().push_external(patch)) {
                effects.push(Effect::SendMessage {
                    target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) },
                    payload: format!("patch-capacity-refused:{}:{}", patch.surface.0, patch.revision.0).into_bytes(),
                });
            }
        }
        protocol::AppFrame::UiSnapshotEnd { .. } => {}
        other => {
            let payload = semio_framework::io::resolve_ready(protocol::encode_app_frame(&other));
            effects.push(Effect::SendMessage { target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) }, payload });
        }
    }
}

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
/// Returns whether entries remain queued (the round cap was hit) — folded into `poll`'s
/// `turn-status::more-work` so a saturated resume queue is never silently dropped.
pub fn drain_task_resumes<PA: crate::app::PluginApp>(runtime: &crate::plugin_runtime::PluginRuntime<PA>, effects: &mut Vec<Effect>, max_rounds: u32) -> bool {
    for _ in 0..max_rounds {
        let Some(resume) = TASK_RESUMES.with(|resumes| resumes.borrow_mut().pop()) else {
            return false;
        };
        if native_close_key(runtime, resume.instance).is_err() {
            TASK_RESUMES.with(|resumes| resumes.borrow_mut().push_admitted(resume));
            continue;
        }
        let input = match resume.outcome {
            #[cfg(test)]
            TaskResumeOutcome::Fault(fault) => {
                effects.push(shell_fault_effect(resume.instance, &fault));
                continue;
            }
            TaskResumeOutcome::Command(bytes) => crate::plugin_runtime::TaskResumeInput::Command(bytes),
            #[cfg(test)]
            TaskResumeOutcome::Emit { artifact_ops, config_ops, draft_ops } => crate::plugin_runtime::TaskResumeInput::Emit { artifact_ops, config_ops, draft_ops },
        };
        // 🚫️async: E5 executor bridge — `plugin_resume_task` stays genuinely `async fn`; see
        // `poll`'s `plugin_exchange` call for the same safety argument.
        let output = semio_framework::io::resolve_ready(crate::plugin_runtime::plugin_resume_task(runtime, resume.instance, &resume.meta, input));
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
    !TASK_RESUMES.with(|resumes| resumes.borrow().is_empty())
}

// 🚫️async: E5 executor bridge — `store::pack_rt::decode_wire_value` is genuinely `async fn`
// (out of `path_scope`, `🏪️store/**`); `resolve_ready` is safe here for the same reason as
// `kernel_effect_to_wit`'s own `pack` helper above — `world actor` imports no `host-async`.
fn decode_wire_effect(bytes: &[u8]) -> Result<Effect, ()> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|_| ())?;
    if let Ok(effect) = dsl::from_dsl_value::<Effect>(value.clone()) {
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

fn decode_wire_replay_shell_command(value: &dsl::DslValue) -> Option<Effect> {
    let replay = value.get("replayShellCommand").or_else(|| value.get("ReplayShellCommand"))?;
    let action_id = replay.get("actionId").or_else(|| replay.get("action_id")).and_then(dsl::DslValue::as_str)?;
    Some(Effect::ReplayShellCommand { action_id: action_id.to_string(), args: replay.get("args").cloned() })
}

/// 📤️ W-G3's `ReplayShellCommand` fallback is the same table: `from_dsl_value` misses some
/// camelCase effect objects, and a silent `Err(())` drops the host picker (`effects:0`).
fn decode_wire_request_file_open(value: &dsl::DslValue) -> Option<Effect> {
    let file = value.get("requestFileOpen").or_else(|| value.get("RequestFileOpen"))?;
    let req = file
        .get("req")
        .and_then(|req| req.as_u64().or_else(|| req.get("id").and_then(dsl::DslValue::as_u64)))?;
    let accept = file.get("accept").and_then(dsl::DslValue::as_str)?.to_string();
    let import_action = file.get("importAction").or_else(|| file.get("import_action")).and_then(dsl::DslValue::as_str)?.to_string();
    let read_as = file.get("readAs").or_else(|| file.get("read_as")).and_then(dsl::DslValue::as_str).map(str::to_string);
    let multiple = file.get("multiple").and_then(dsl::DslValue::as_bool).unwrap_or(false);
    Some(Effect::RequestFileOpen { req: semio_framework::kernel::RequestId(req), accept, read_as, import_action, multiple })
}

fn push_admitted_effect(effects: &mut Vec<Effect>, instance: u32, effect: Effect) {
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

fn decode_wire_app_event(bytes: &[u8]) -> Result<semio_framework::kernel::AppEvent, ()> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|_| ())?;
    dsl::from_dsl_value(value).map_err(|_| ())
}


#[cfg(test)]
mod wire_effect_laws {
    use super::*;
    use protocol::ToValue;
    use semio_framework::kernel::{Effect, RequestId};

    #[test]
    fn request_file_open_survives_wire_effect_round_trip() {
        let effect = Effect::RequestFileOpen {
            req: RequestId(121),
            accept: "application/json,.json".into(),
            read_as: Some("text".into()),
            import_action: "importFixture".into(),
            multiple: false,
        };
        let bytes = store::pack_rt::encode_wire_value(&effect.to_value());
        let decoded = decode_wire_effect(&bytes).expect("RequestFileOpen must survive the browser wire table");
        match decoded {
            Effect::RequestFileOpen { accept, read_as, import_action, multiple, .. } => {
                assert!(accept.contains("json"), "{accept}");
                assert_eq!(read_as.as_deref(), Some("text"));
                assert_eq!(import_action, "importFixture");
                assert!(!multiple);
            }
            other => panic!("wire dropped RequestFileOpen: {other:?}"),
        }
    }

    fn effect_wire_kind(effect: &Effect) -> &'static str {
        match effect {
            Effect::OpenWindow { .. } => "openWindow",
            Effect::CloseWindow { .. } => "closeWindow",
            Effect::Notify { .. } => "notify",
            Effect::ClipboardWrite { .. } => "clipboardWrite",
            Effect::RequestSync => "requestSync",
            Effect::Navigate { .. } => "navigate",
            Effect::LoadDocument { .. } => "loadDocument",
            Effect::OpenExternalUrl { .. } => "openExternalUrl",
            Effect::SetPanel { .. } => "setPanel",
            Effect::DownloadMediaExport { .. } => "downloadMediaExport",
            Effect::IconRenderExport { .. } => "iconRenderExport",
            Effect::RequestFileOpen { .. } => "requestFileOpen",
            Effect::RequestMediaFrames { .. } => "requestMediaFrames",
            Effect::SpawnPluginInstance { .. } => "spawnPluginInstance",
            Effect::OpenPluginInstance { .. } => "openPluginInstance",
            Effect::SetActiveUtility { .. } => "setActiveUtility",
            Effect::SetActiveTool { .. } => "setActiveTool",
            Effect::OpenDialog { .. } => "openDialog",
            Effect::DispatchAction { .. } => "dispatchAction",
            Effect::ReplayShellCommand { .. } => "replayShellCommand",
            Effect::InvokeExtension { .. } => "invokeExtension",
            Effect::SendMessage { .. } => "sendMessage",
            Effect::PublishEvent { .. } => "publishEvent",
            Effect::BlobWrite { .. } => "blobWrite",
            Effect::BlobLoad { .. } => "blobLoad",
            Effect::HttpRequest { .. } => "httpRequest",
            Effect::DocumentRead { .. } => "documentRead",
            Effect::DocumentWrite { .. } => "documentWrite",
            Effect::LinkResolve { .. } => "linkResolve",
            Effect::RegistryQuery { .. } => "registryQuery",
            Effect::IoCompose { .. } => "ioCompose",
            Effect::CacheDerive { .. } => "cacheDerive",
            Effect::CacheRead { .. } => "cacheRead",
            Effect::SetTimer { .. } => "setTimer",
            Effect::SpawnJob { .. } => "spawnJob",
            Effect::CancelJob { .. } => "cancelJob",
            Effect::Respond { .. } => "respond",
            Effect::StorageRead { .. } => "storageRead",
            Effect::StorageWrite { .. } => "storageWrite",
            Effect::StorageDelete { .. } => "storageDelete",
            Effect::RequestCapability { .. } => "requestCapability",
            Effect::ReleaseCapability { .. } => "releaseCapability",
            Effect::Subscribe { .. } => "subscribe",
            Effect::Unsubscribe { .. } => "unsubscribe",
            Effect::RequestInferenceProposal { .. } => "requestInferenceProposal",
        }
    }

    fn all_effect_wire_fixtures() -> Vec<Effect> {
        use semio_framework::kernel::{ArtifactHandle, CapabilityId, CapabilityRequest, ClipboardFragment, IconRenderExportItem, InferenceProposalKind, JobPlacement, MessageEndpoint, PluginInstanceId, RequestOutcome, WindowHandle, WindowKindId};
        use semio_framework::{MediaClass, MediaForm, MediaType};
        let req = RequestId(7);
        let media = MediaType { class: MediaClass::Data, form: MediaForm::Value };
        vec![
            Effect::OpenWindow { req, kind: WindowKindId("main".into()), params: dsl::DslValue::Null },
            Effect::CloseWindow { window: WindowHandle(1) },
            Effect::Notify { message: "n".into() },
            Effect::ClipboardWrite { fragment: ClipboardFragment { schema: "s".into(), media_type: media.clone(), dsl_text: "{}".into(), pack_bytes: None, source_app: "a".into(), label: "l".into() } },
            Effect::RequestSync,
            Effect::Navigate { uri: "semio://x".into() },
            Effect::LoadDocument { pack: vec![1], spr: vec![2] },
            Effect::OpenExternalUrl { url: "https://example.test".into() },
            Effect::SetPanel { panel_json: "{}".into() },
            Effect::DownloadMediaExport { filename: "a.bin".into(), mime_type: "application/octet-stream".into(), data: "AA==".into(), encoding: None },
            Effect::IconRenderExport { items: vec![IconRenderExportItem { filename: "i.png".into(), request: dsl::DslValue::Null }] },
            Effect::RequestFileOpen { req, accept: "*".into(), read_as: None, import_action: "import".into(), multiple: false },
            Effect::RequestMediaFrames { req, accept: "video/*".into(), frame_action: "frame".into(), done_action: "done".into(), fallback_action: "fallback".into(), sample_stride: 0, max_frames: 0, max_long_edge_px: 0, fps_hint: 0.0, payload: None, args: None },
            Effect::SpawnPluginInstance { req, plugin_id: "p".into(), app_id: "a".into(), os_instance_id: None, label: None, document_json: None },
            Effect::OpenPluginInstance { plugin_id: "p".into(), app_id: "a".into(), os_instance_id: None },
            Effect::SetActiveUtility { window_id: "w".into(), utility_id: "u".into() },
            Effect::SetActiveTool { tool_id: "t".into() },
            Effect::OpenDialog { req, dialog_id: "d".into(), args: None },
            Effect::DispatchAction { req, action: "act".into(), args: None, delay_ms: 0 },
            Effect::ReplayShellCommand { action_id: "panelTab".into(), args: None },
            Effect::invoke_extension(req, "ext".into(), "cap".into(), "{}".into()),
            Effect::SendMessage { target: MessageEndpoint::Topic { name: "t".into() }, payload: vec![1] },
            Effect::PublishEvent { topic: "t".into(), payload: vec![1] },
            Effect::BlobWrite { req, media_type: media, bytes: vec![1] },
            Effect::BlobLoad { req, hash: "h".into() },
            Effect::HttpRequest { req, method: "GET".into(), url: "https://example.test".into(), headers: Vec::new(), body: None, stream: false },
            Effect::DocumentRead { req, doc: ArtifactHandle(1), lane: "main".into() },
            Effect::DocumentWrite { req, doc: ArtifactHandle(1), lane: "main".into(), ops: vec![1] },
            Effect::LinkResolve { req, link: "l".into() },
            Effect::RegistryQuery { req, kind: "k".into(), filter: None },
            Effect::IoCompose { req, key: "k".into(), sources: vec!["s".into()] },
            Effect::CacheDerive { req, engine_id: "e".into(), input: vec![1] },
            Effect::CacheRead { req, engine_id: "e".into(), key: "k".into() },
            Effect::SetTimer { id: 1, after_ms: 1, repeat: false },
            Effect::SpawnJob { job: 1, kind: "framework.reserved.tool".into(), input: vec![1], placement: JobPlacement::Isolated },
            Effect::CancelJob { job: 1 },
            Effect::Respond { req, result: RequestOutcome::Ok(vec![1]) },
            Effect::StorageRead { req, key: "k".into() },
            Effect::StorageWrite { req, key: "k".into(), bytes: vec![1] },
            Effect::StorageDelete { req, key: "k".into() },
            Effect::RequestCapability { req, capability: CapabilityRequest { id: CapabilityId("c".into()), scope: "s".into(), reason: "r".into(), optional: false } },
            Effect::ReleaseCapability { id: CapabilityId("c".into()) },
            Effect::Subscribe { topic: "t".into() },
            Effect::Unsubscribe { topic: "t".into() },
            Effect::RequestInferenceProposal { kind: InferenceProposalKind::GisMapBoundsRegion },
        ]
    }

    /// 🧪 W-G3 §8.21 — every `Effect` kind survives leftover `pack_rt` encode / `decode_wire_effect`.
    /// A new variant that is not in `effect_wire_kind` fails compile; a fixture gap fails this count.
    #[test]
    fn every_effect_kind_survives_wire_effect_round_trip() {
        let fixtures = all_effect_wire_fixtures();
        let mut seen = std::collections::BTreeSet::new();
        for effect in &fixtures {
            let kind = effect_wire_kind(effect);
            assert!(seen.insert(kind), "duplicate wire-table fixture {kind}");
            let bytes = store::pack_rt::encode_wire_value(&effect.to_value());
            let decoded = decode_wire_effect(&bytes).unwrap_or_else(|_| panic!("wire table dropped {kind}"));
            assert_eq!(effect_wire_kind(&decoded), kind, "{kind} decoded as a different arm");
        }
        assert_eq!(seen.len(), fixtures.len(), "wire-table completeness fixtures must be unique");
        assert_eq!(fixtures.len(), 45, "every Effect kind must have a leftover wire-table fixture");
    }
}

//! ⚛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2, design-abi.md §4): the turn loop —
//! `reactor::poll`'s real implementation. Ties together `🧵️executor` (async task scheduling),
//! `📮️requests` (the host-effect request/completion registry), `🩹️patches` (revisioned UI diffing),
//! `💼️jobs` (the absorbed `semio.io-run`/`semio.io-sniff` cold job kinds), `📸️checkpoint`, and
//! `🌐host` (the async `host::*` API surface plugin/extension code awaits).
//!
//! Converts between the WIT-generated `semio::framework::{effects,events,ui,reactor}::*` types
//! (crossing the component boundary) and the Rust SSOT `semio_framework::kernel::{Effect, Event,
//! UiPatch, PatchOp, TurnResult, TurnStatus, Budget}` (packet A3, landed in
//! `🎠️kernel/🦀️.rs` while this packet was in flight). `app-command` events route through
//! the EXISTING `plugin_runtime::plugin_exchange` dispatcher unchanged (design-abi.md §4) — this
//! module never reimplements command dispatch, only translates its `AppFrame` output into
//! `Effect`/`UiPatch`.

#[path = "📸️checkpoint/🦀️.rs"]
pub mod checkpoint;
#[path = "📥️cold-pair/🦀️.rs"]
pub(crate) mod cold_pair;
#[path = "🧵️executor/🦀️.rs"]
pub mod executor;
#[path = "🚪️lifetime/🦀️.rs"]
pub(crate) mod instance_lifetime;
#[path = "💼️jobs/🦀️.rs"]
pub mod jobs;
#[path = "🩹️patches/🦀️.rs"]
pub mod patches;
#[path = "📨️pending/🦀️.rs"]
mod pending;
#[path = "📮️requests/🦀️.rs"]
pub mod requests;

#[path = "🪟️surfaces/🦀️.rs"]
pub(crate) mod surface_context;

// 🧬️ Only `wit_bridge` below (component-guest/-extension-guest wasm32-wasip2) consumes these —
// a plain native build never reaches the WIT-boundary translation code, so unlike `RefCell` these
// two must be gated identically to `wit_bridge` itself or they warn as unused on native.
use semio_framework::kernel::{ActorInstanceLifecycleReceipt, ActorUiPatchReceipt, Effect, Event, MessageEndpoint, TurnStatus, UiPatch};
// 🧬️ Same gating rationale as the `kernel` import above: only the WIT-boundary code below names the
// semantic-UI contract types (`UiIntent`, `UiRevision`, `Activity`), so an ungated alias warns as
// unused on native. ALSO enabled under `cfg(test)` (M2, ticket 26/08/17 `design-unified.md`): the
// native `test_support` module below (behind its own `#[cfg(test)]`) exercises `PATCHES`/`PRESENCE`
// directly with real `ui_contract` values — `wit_bridge` still cannot run under `cargo test`
// (wasm32-wasip2-only), but its own type vocabulary can be reused for a native fixture.
use semio_framework_ui_contract as ui_contract;
// 🧬️ Same gating rationale as `ui_contract` above: `is_stale_intent`/`DEFAULT_REVISION_TOLERANCE`
// are only named inside `wit_bridge::poll`'s intent-batching loop (M1, ticket 26/08/17
// `design-unified.md`) — gated identically to that module so an ungated alias never warns as unused
// on native.
#[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]
use semio_framework::kernel::{RequestOutcome, UiPatchOp};
/// 👥️ M2 (ticket 26/08/17 `design-unified.md`): standalone — needs no `EntityStore`/`UiRuntime`, just
/// `record_own`/`record_peer`/`expire`/`flush` — so unlike the two imports directly above, this one
/// is ungated: the `PRESENCE` thread_local right below references it on EVERY build, matching how
/// `patches::PatchTracker` (an equally wit_bridge-only consumer) is reached through the ungated
/// `pub mod patches;` at this file's top.
use semio_framework_ui_runtime::PresenceHub;
#[cfg(test)]
use semio_framework_ui_runtime::SurfaceReconcileReadyPatch;
use semio_framework_ui_runtime::{is_stale_intent, DEFAULT_REVISION_TOLERANCE};
use std::cell::{Cell, RefCell};
// 🧵️ Turn-local command/intent grouping and the pre-admitted task-resume ring use these
// collections; all identity and close authority is held by fixed direct registries below.
use std::collections::VecDeque;

const RECONCILE_STEP_OPPORTUNITY_LIMIT: u64 = 1_024;

//#region 📬️ShellFaultFrame
fn shell_fault_effect(instance: u32, fault: &semio_framework::Fault) -> Effect {
    let fault = store::pack_rt::encode_wire_value(&dsl::to_dsl_value(fault).expect("shell diagnostic must serialize"));
    let frame = protocol::AppFrame::Error { in_reply_to: None, fault, report: Vec::new() };
    Effect::SendMessage { target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) }, payload: semio_framework::io::resolve_ready(protocol::encode_app_frame(&frame)) }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️shell-fault-frame/🦀️.rs"]
mod shell_fault_frame_tests;
//#endregion 📬️ShellFaultFrame

fn reconcile_step_opportunities(fuel: u64) -> usize {
    usize::try_from(fuel.min(RECONCILE_STEP_OPPORTUNITY_LIMIT)).unwrap_or(RECONCILE_STEP_OPPORTUNITY_LIMIT as usize).max(1)
}

crate::component_persistent_local! {
    /// 🩹️ One `PatchTracker` shared by every instance this actor hosts (surfaces are already
    /// namespaced by their own `surface` string, which today embeds the instance — see
    /// `render_surface`'s key).
    // 🌉️ `thread_local!` initializer expressions run in a plain (non-const, non-async) context —
    // bridged via `resolve_ready` since every `::new()` here is a pure `Self::default()`.
    static PATCHES: patches::PatchTracker = patches::PatchTracker::new();
    /// 👥️ Collects presence by concrete mounted surface and node, bound atomically with each render.
    static PRESENCE: RefCell<PresenceHub> = RefCell::new(PresenceHub::new());
    /// 📮️ One `RequestRegistry` per actor (today: shared process-wide, matching the "one actor per
    /// app instance is the default" granularity design-abi.md §4 names — a multi-instance pooled
    /// actor is opt-in first-party-only future work, out of this wave).
    static REGISTRY: requests::RequestRegistry = requests::RequestRegistry::new();
    static REACTOR_EXECUTOR: executor::ReactorExecutor = executor::ReactorExecutor::new();
    #[cfg(test)]
    static TEST_FUTURE_EXECUTOR: executor::ColdFutureExecutor = executor::ColdFutureExecutor::new();
    /// 🪪️ Every instance this actor currently has open — `(id, app_id)`, in `InstanceOpen` order.
    /// Used by `📸️checkpoint`.
    static INSTANCE_METADATA: RefCell<InstanceMetadataRegistry> = RefCell::new(InstanceMetadataRegistry::new());
    /// ⏱️ Live timer ids this actor has armed via `Effect::SetTimer`, carried into the checkpoint
    /// pack (design-abi.md §4).
    static ARMED_TIMERS: RefCell<FixedTimerRegistry> = RefCell::new(FixedTimerRegistry::new());
    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): every `AsyncTask` this actor
    /// currently has spawned on `EXECUTOR`, keyed by its `executor::TaskId` slot — the bookkeeping
    /// `spawn_task`'s quota gate counts against, `cancel_instance_tasks` (`Event::InstanceClose`)
    /// walks, and `checkpoint_now` reads `restart` out of.
    static TASK_RECORDS: RefCell<TaskRecordRegistry> = RefCell::new(TaskRecordRegistry::new());
    /// 📬️ Resolved-but-not-yet-redispatched `AsyncTask` follow-ups, type-erased to bytes at
    /// resolution time (see `spawn_task`'s doc) — drained by `drain_task_resumes`, called from
    /// `poll` right after `run_until_idle`. Also the target of a checkpoint `restore`'s replayed
    /// `task_restarts` (`restore_now`), so a restart is just an ordinary `Command` resume queued
    /// one call earlier than usual.
    static TASK_RESUMES: RefCell<FixedResumeQueue> = RefCell::new(FixedResumeQueue::new());
    static JOB_RENDER_BINDINGS: RefCell<JobRenderBindingRegistry> = RefCell::new(JobRenderBindingRegistry::new());
    /// 🎛️ Per-instance `QuotaSchema`, real values decoded off `Event::InstanceOpen.quotas`
    /// (previously always defaulted — see `wit_event_to_kernel`'s `InstanceOpen` arm) and dropped
    /// again on `Event::InstanceClose`. `spawn_task`'s quota gate is the first real reader.
    static REACTOR_CLOSES: RefCell<ReactorCloseRegistry> = RefCell::new(ReactorCloseRegistry::new());
    static REACTOR_CLOSE_CURSOR: Cell<usize> = Cell::new(0);
    static COLD_PAIR_INGRESS: RefCell<cold_pair::ColdDocumentPairIngressRegistry<PLUGIN_REACTOR_INSTANCE_SLOTS>> = RefCell::new(cold_pair::ColdDocumentPairIngressRegistry::new());
}

/// 🧵️ A task retains its instance and optional checkpoint restart command.
struct TaskRecord {
    instance: u32,
    #[cfg(test)]
    key: Option<String>,
    restart: Option<Vec<u8>>,
}

const REACTOR_TASK_SLOTS: usize = 1_024;
const REACTOR_FIXED_WORDS: usize = REACTOR_TASK_SLOTS / u64::BITS as usize;
#[cfg(test)]
const REACTOR_TASK_KEY_BYTES: usize = 256;
#[cfg(test)]
const REACTOR_TASK_LABEL_BYTES: usize = 256;
#[cfg(test)]
const REACTOR_TASK_RESTART_BYTES: usize = 64 * 1_024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct JobRenderBinding {
    job: u64,
    instance: u32,
    generation: u64,
}

struct JobRenderBindingRegistry {
    by_job: [Option<JobRenderBinding>; REACTOR_TASK_SLOTS],
    current_by_instance: [Option<JobRenderBinding>; REACTOR_TASK_SLOTS],
    next_generation: u64,
}

impl JobRenderBindingRegistry {
    const fn new() -> Self {
        Self { by_job: [None; REACTOR_TASK_SLOTS], current_by_instance: [None; REACTOR_TASK_SLOTS], next_generation: 0 }
    }

    fn bind(&mut self, instance: u32, job: u64) -> Result<JobRenderBinding, ()> {
        let instance_slot = instance as usize % REACTOR_TASK_SLOTS;
        if self.current_by_instance[instance_slot].is_some_and(|binding| binding.instance != instance) {
            return Err(());
        }
        let job_slot = job as usize % REACTOR_TASK_SLOTS;
        if self.by_job[job_slot].is_some() {
            return Err(());
        }
        let generation = self.next_generation.checked_add(1).ok_or(())?;
        self.next_generation = generation;
        let binding = JobRenderBinding { job, instance, generation };
        if let Some(previous) = self.current_by_instance[instance_slot] {
            let previous_slot = previous.job as usize % REACTOR_TASK_SLOTS;
            if self.by_job[previous_slot] == Some(previous) {
                self.by_job[previous_slot] = None;
            }
        }
        self.by_job[job_slot] = Some(binding);
        self.current_by_instance[instance_slot] = Some(binding);
        Ok(binding)
    }

    fn accepted(&self, job: u64) -> Option<JobRenderBinding> {
        let binding = self.by_job[job as usize % REACTOR_TASK_SLOTS].filter(|binding| binding.job == job)?;
        self.current_by_instance[binding.instance as usize % REACTOR_TASK_SLOTS].filter(|current| *current == binding)
    }

    fn complete(&mut self, job: u64) -> Option<JobRenderBinding> {
        let job_slot = job as usize % REACTOR_TASK_SLOTS;
        let binding = self.by_job[job_slot].filter(|binding| binding.job == job)?;
        self.by_job[job_slot] = None;
        let instance_slot = binding.instance as usize % REACTOR_TASK_SLOTS;
        (self.current_by_instance[instance_slot] == Some(binding)).then(|| {
            self.current_by_instance[instance_slot] = None;
            binding
        })
    }

    fn close_instance(&mut self, instance: u32) {
        let instance_slot = instance as usize % REACTOR_TASK_SLOTS;
        let Some(binding) = self.current_by_instance[instance_slot].filter(|binding| binding.instance == instance) else { return };
        self.current_by_instance[instance_slot] = None;
        let job_slot = binding.job as usize % REACTOR_TASK_SLOTS;
        if self.by_job[job_slot] == Some(binding) {
            self.by_job[job_slot] = None;
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️job-render-binding/🦀️.rs"]
mod job_render_binding_tests;

struct ReactorFixedSlots<T> {
    values: Box<[std::mem::MaybeUninit<T>]>,
    occupied: [u64; REACTOR_FIXED_WORDS],
    allocation_admitted: bool,
}

impl<T> ReactorFixedSlots<T> {
    fn new() -> Self {
        let mut values = Vec::new();
        let allocation_admitted = values.try_reserve_exact(REACTOR_TASK_SLOTS).is_ok();
        if allocation_admitted {
            values.resize_with(REACTOR_TASK_SLOTS, std::mem::MaybeUninit::uninit);
        }
        Self { values: values.into_boxed_slice(), occupied: [0; REACTOR_FIXED_WORDS], allocation_admitted }
    }

    fn occupied(&self, index: usize) -> bool {
        self.occupied[index / u64::BITS as usize] & (1u64 << (index % u64::BITS as usize)) != 0
    }

    fn set_occupied(&mut self, index: usize, occupied: bool) {
        let word = &mut self.occupied[index / u64::BITS as usize];
        let mask = 1u64 << (index % u64::BITS as usize);
        if occupied {
            *word |= mask;
        } else {
            *word &= !mask;
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        if !self.occupied(index) {
            return None;
        }
        self.values.get(index).map(|value| {
            // SAFETY: occupancy is set only after `write` and cleared before `assume_init_read`.
            unsafe { value.assume_init_ref() }
        })
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if !self.occupied(index) {
            return None;
        }
        self.values.get_mut(index).map(|value| {
            // SAFETY: occupancy is set only after `write` and cleared before `assume_init_read`.
            unsafe { value.assume_init_mut() }
        })
    }

    fn insert(&mut self, index: usize, value: T) -> Result<(), T> {
        if !self.allocation_admitted || self.occupied(index) {
            return Err(value);
        }
        self.values[index].write(value);
        self.set_occupied(index, true);
        Ok(())
    }

    fn insert_admitted(&mut self, index: usize, value: T) {
        debug_assert!(self.allocation_admitted && !self.occupied(index));
        self.values[index].write(value);
        self.set_occupied(index, true);
    }

    fn take(&mut self, index: usize) -> Option<T> {
        if !self.occupied(index) {
            return None;
        }
        self.set_occupied(index, false);
        // SAFETY: occupancy was checked and is now cleared before the exact initialized read.
        Some(unsafe { self.values[index].assume_init_read() })
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        (0..REACTOR_TASK_SLOTS).filter_map(|index| self.get(index))
    }
}

impl<T> Drop for ReactorFixedSlots<T> {
    fn drop(&mut self) {}
}

struct TaskRecordRegistry {
    slots: ReactorFixedSlots<(executor::TaskId, TaskRecord)>,
}

impl TaskRecordRegistry {
    fn new() -> Self {
        Self { slots: ReactorFixedSlots::new() }
    }

    #[cfg(test)]
    fn index(id: executor::TaskId) -> usize {
        id as usize % REACTOR_TASK_SLOTS
    }

    #[cfg(test)]
    fn can_insert(&self, id: executor::TaskId) -> bool {
        self.slots.allocation_admitted && self.slots.get(Self::index(id)).is_none()
    }

    #[cfg(test)]
    fn insert_admitted(&mut self, id: executor::TaskId, record: TaskRecord) {
        debug_assert!(self.can_insert(id));
        self.slots.insert_admitted(Self::index(id), (id, record));
    }

    #[cfg(test)]
    fn remove(&mut self, id: executor::TaskId) -> Option<TaskRecord> {
        let index = Self::index(id);
        if self.slots.get(index).is_none_or(|(candidate, _)| *candidate != id) {
            None
        } else {
            self.slots.take(index).map(|(_, record)| record)
        }
    }

    #[cfg(test)]
    fn find_key(&self, instance: u32, key: &str) -> Option<executor::TaskId> {
        self.slots.iter().find_map(|(id, record)| (record.instance == instance && record.key.as_deref() == Some(key)).then_some(*id))
    }

    #[cfg(test)]
    fn count_instance(&self, instance: u32) -> usize {
        self.slots.iter().filter(|(_, record)| record.instance == instance).count()
    }

    fn iter(&self) -> impl Iterator<Item = (executor::TaskId, &TaskRecord)> {
        self.slots.iter().map(|(id, record)| (*id, record))
    }

    #[cfg(test)]
    fn entry_at(&self, index: usize) -> Option<(executor::TaskId, &TaskRecord)> {
        self.slots.get(index).map(|(id, record)| (*id, record))
    }
}

struct ReactorCloseState {
    key: instance_lifetime::NativeCloseKey,
    instance: u32,
    active: bool,
    complete: bool,
    task_cursor: usize,
    timer_cursor: usize,
    request_cursor: requests::RequestCloseCursor,
    resume_remaining: usize,
    command_ingress_complete: bool,
    requests_complete: bool,
    resumes_complete: bool,
    /// ✅️ `cancel_instance_tasks_step`'s own completion witness. The cursor alone cannot stand in
    /// for it: the executor answers `Complete` without advancing its cursor whenever the sweep has
    /// nothing left to visit, so a cursor-bound loop condition never terminates (26/09/09).
    tasks_complete: bool,
    timers_complete: bool,
    metadata_complete: bool,
}

struct ReactorCloseRegistry {
    slots: ReactorFixedSlots<ReactorCloseState>,
}

impl ReactorCloseRegistry {
    fn new() -> Self {
        Self { slots: ReactorFixedSlots::new() }
    }

    fn index(instance: u32) -> usize {
        instance as usize % PLUGIN_REACTOR_INSTANCE_SLOTS
    }

    fn insert(&mut self, state: ReactorCloseState) -> Result<(), ReactorCloseState> {
        let index = Self::index(state.instance);
        self.slots.insert(index, state)
    }

    fn take_at(&mut self, index: usize) -> Option<ReactorCloseState> {
        self.slots.take(index)
    }
}

const PLUGIN_REACTOR_INSTANCE_SLOTS: usize = 1_024;
const PLUGIN_REACTOR_APP_ID_BYTES: usize = 256;

const REACTOR_TIMER_SLOTS: usize = 1_024;

struct TimerEntry {
    id: u64,
    instance: u32,
    previous: Option<usize>,
    next: Option<usize>,
}

struct FixedTimerRegistry {
    slots: ReactorFixedSlots<TimerEntry>,
    head: Option<usize>,
    tail: Option<usize>,
    live: usize,
    allocation_admitted: bool,
}

impl FixedTimerRegistry {
    fn new() -> Self {
        let slots = ReactorFixedSlots::new();
        let allocation_admitted = slots.allocation_admitted;
        Self { slots, head: None, tail: None, live: 0, allocation_admitted }
    }

    fn index(id: u64) -> usize {
        id as usize % REACTOR_TIMER_SLOTS
    }

    fn insert(&mut self, instance: u32, id: u64) -> Result<(), u64> {
        let index = Self::index(id);
        if !self.allocation_admitted || self.slots.get(index).is_some() {
            return Err(id);
        }
        let Some(live) = self.live.checked_add(1).filter(|live| *live <= REACTOR_TIMER_SLOTS) else { return Err(id) };
        let previous = self.tail;
        self.slots.insert(index, TimerEntry { id, instance, previous, next: None }).map_err(|entry| entry.id)?;
        if let Some(previous) = previous {
            let Some(tail) = self.slots.get_mut(previous) else {
                let _ = self.slots.take(index);
                return Err(id);
            };
            tail.next = Some(index);
        } else {
            self.head = Some(index);
        }
        self.tail = Some(index);
        self.live = live;
        Ok(())
    }

    fn remove(&mut self, id: u64) -> bool {
        let index = Self::index(id);
        if !self.slots.get(index).is_some_and(|entry| entry.id == id) {
            return false;
        }
        let Some(live) = self.live.checked_sub(1) else { return false };
        let Some(entry_ref) = self.slots.get(index) else { return false };
        if entry_ref.previous.is_some_and(|previous| self.slots.get(previous).is_none()) || entry_ref.next.is_some_and(|next| self.slots.get(next).is_none()) {
            return false;
        }
        let Some(entry) = self.slots.take(index) else { return false };
        if let Some(previous) = entry.previous {
            let Some(predecessor) = self.slots.get_mut(previous) else { return false };
            predecessor.next = entry.next;
        } else {
            self.head = entry.next;
        }
        if let Some(next) = entry.next {
            let Some(successor) = self.slots.get_mut(next) else { return false };
            successor.previous = entry.previous;
        } else {
            self.tail = entry.previous;
        }
        self.live = live;
        true
    }

    fn first(&self) -> Option<u64> {
        self.head.and_then(|index| self.slots.get(index)).map(|entry| entry.id)
    }

    #[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]
    fn contains(&self, id: u64) -> bool {
        self.slots.get(Self::index(id)).is_some_and(|entry| entry.id == id)
    }

    fn rows(&self) -> Vec<u64> {
        let mut rows = Vec::with_capacity(self.live);
        let mut cursor = self.head;
        while let Some(index) = cursor {
            let Some(entry) = self.slots.get(index) else { break };
            rows.push(entry.id);
            cursor = entry.next;
        }
        rows
    }

    fn is_empty(&self) -> bool {
        self.live == 0
    }

    fn cancel_instance_step(&mut self, instance: u32, cursor: &mut usize) -> bool {
        if *cursor >= REACTOR_TIMER_SLOTS {
            return true;
        }
        let timer = self.slots.get(*cursor).and_then(|entry| (entry.instance == instance).then_some(entry.id));
        if let Some(timer) = timer {
            self.remove(timer);
        }
        *cursor += 1;
        *cursor >= REACTOR_TIMER_SLOTS
    }
}

struct InstanceMetadata {
    instance: u32,
    app_id: String,
    quota: semio_framework::kernel::QuotaSchema,
}

struct InstanceMetadataRegistry {
    slots: ReactorFixedSlots<InstanceMetadata>,
}

impl InstanceMetadataRegistry {
    fn new() -> Self {
        Self { slots: ReactorFixedSlots::new() }
    }

    fn index(instance: u32) -> usize {
        instance as usize % PLUGIN_REACTOR_INSTANCE_SLOTS
    }

    fn insert(&mut self, instance: u32, app_id: String, quota: semio_framework::kernel::QuotaSchema) -> Result<(), semio_framework::Fault> {
        if app_id.len() > PLUGIN_REACTOR_APP_ID_BYTES {
            return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.instance-app-id-too-large"), "instance app id exceeds its admitted fixed byte bound"));
        }
        let index = Self::index(instance);
        if !self.slots.allocation_admitted || self.slots.get(index).is_some() {
            return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.instance-metadata-capacity"), "fixed instance metadata authority is saturated or collided"));
        }
        self.slots
            .insert(index, InstanceMetadata { instance, app_id, quota })
            .map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.instance-metadata-capacity"), "fixed instance metadata authority changed during insert"))
    }

    fn get(&self, instance: u32) -> Option<&InstanceMetadata> {
        self.slots.get(Self::index(instance)).filter(|entry| entry.instance == instance)
    }

    fn remove(&mut self, instance: u32) -> Option<InstanceMetadata> {
        let index = Self::index(instance);
        if self.slots.get(index).is_some_and(|entry| entry.instance == instance) {
            self.slots.take(index)
        } else {
            None
        }
    }

    fn checkpoint_rows(&self) -> Vec<(u32, String)> {
        self.slots.iter().map(|entry| (entry.instance, entry.app_id.clone())).collect()
    }
}

/// 🧵️ A resolved `AsyncTask`, erased to bytes — see `spawn_task`'s doc for why no `Mutation`/
/// `ConfigMutation`/`DraftMutation` generic ever reaches this type. Mirrors `TaskResolution`
/// one-for-one except `Emit` is pre-encoded (the SAME `protocol::encode_ops_vec`/`OpBinary::
/// encode_op` idiom `dispatch_emit`'s own `last_emit_wire` uses) and a task future that resolved
/// `Err(fault)` gets its own variant (never silently dropped).
enum TaskResumeOutcome {
    Command(Vec<u8>),
    #[cfg(test)]
    Emit {
        artifact_ops: Vec<u8>,
        config_ops: Vec<u8>,
        draft_ops: Vec<u8>,
    },
    #[cfg(test)]
    Fault(semio_framework::Fault),
}

/// 🧵️ One entry in `TASK_RESUMES` — `meta` is the task's CLONED originating `ActionMeta` (spawn
/// time for a real task resolution; best-effort `instance_actor(instance)` at restore time for a
/// `task_restarts` replay, since a checkpoint restart has no "spawn" of its own to snapshot from —
/// see `restore_now`), preserved so the follow-up dispatch stays attributed to the actor that
/// asked for it even if a different one is active by the time it resolves.
struct PendingResume {
    instance: u32,
    meta: crate::app::ActionMeta,
    outcome: TaskResumeOutcome,
}

const REACTOR_RESUME_SLOTS: usize = 1_024;
const REACTOR_RESUME_BYTES: usize = 3 * 64 * 1_024;
const REACTOR_RESUME_ACTOR_BYTES: usize = 256;

impl PendingResume {
    fn admitted_bytes(&self) -> Option<usize> {
        let mut bytes = self.meta.actor.len();
        if bytes > REACTOR_RESUME_ACTOR_BYTES {
            return None;
        }
        match &self.outcome {
            TaskResumeOutcome::Command(command) => bytes = bytes.checked_add(command.len())?,
            #[cfg(test)]
            TaskResumeOutcome::Emit { artifact_ops, config_ops, draft_ops } => {
                bytes = bytes.checked_add(artifact_ops.len())?.checked_add(config_ops.len())?.checked_add(draft_ops.len())?;
            }
            #[cfg(test)]
            TaskResumeOutcome::Fault(fault) => {
                if fault.causes.len() > 16 {
                    return None;
                }
                bytes = bytes.checked_add(fault.code.0.len())?.checked_add(fault.message.len())?;
                for value in [&fault.scope.plugin_id, &fault.scope.app_id, &fault.scope.instance_id, &fault.scope.module, &fault.scope.body_key].into_iter().flatten() {
                    bytes = bytes.checked_add(value.len())?;
                }
                for cause in &fault.causes {
                    bytes = bytes.checked_add(cause.message.len())?;
                    if let Some(code) = &cause.code {
                        bytes = bytes.checked_add(code.0.len())?;
                    }
                }
            }
        }
        (bytes <= REACTOR_RESUME_BYTES).then_some(bytes)
    }
}

struct FixedResumeQueue {
    entries: std::mem::ManuallyDrop<VecDeque<PendingResume>>,
    allocation_admitted: bool,
}

impl FixedResumeQueue {
    fn new() -> Self {
        let mut entries = VecDeque::new();
        let allocation_admitted = entries.try_reserve_exact(REACTOR_RESUME_SLOTS).is_ok();
        Self { entries: std::mem::ManuallyDrop::new(entries), allocation_admitted }
    }

    #[expect(clippy::result_large_err, reason = "The fixed resume queue returns the admitted view snapshot, task metadata, and payload owner intact without allocating on rejection.")]
    fn push(&mut self, value: PendingResume) -> Result<(), PendingResume> {
        if !self.allocation_admitted || self.entries.len() >= REACTOR_RESUME_SLOTS || value.admitted_bytes().is_none() {
            return Err(value);
        }
        self.entries.push_back(value);
        Ok(())
    }

    fn push_admitted(&mut self, value: PendingResume) {
        debug_assert!(self.allocation_admitted && self.entries.len() < REACTOR_RESUME_SLOTS && value.admitted_bytes().is_some());
        self.entries.push_back(value);
    }

    fn pop(&mut self) -> Option<PendingResume> {
        self.entries.pop_front()
    }

    fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn begin_cancel_instance(&self) -> usize {
        self.entries.len()
    }

    fn cancel_instance_step(&mut self, instance: u32, remaining: &mut usize) -> bool {
        if *remaining == 0 || self.entries.is_empty() {
            *remaining = 0;
            return true;
        }
        let Some(entry) = self.entries.pop_front() else {
            *remaining = 0;
            return true;
        };
        *remaining -= 1;
        if entry.instance != instance {
            self.push_admitted(entry);
        }
        *remaining == 0
    }
}

impl Drop for FixedResumeQueue {
    fn drop(&mut self) {}
}

/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): the outstanding-task quota for
/// `instance` — `QuotaSchema.outstanding_requests`, defaulting to 16 when the instance never
/// declared one (or hasn't opened yet, which should not happen in practice: `spawn_task` is only
/// ever reachable from `dispatch_emit`, itself only reachable after `Event::InstanceOpen`).
#[cfg(test)]
async fn instance_task_quota(instance: u32) -> u64 {
    INSTANCE_METADATA.with(|metadata| metadata.borrow().get(instance).and_then(|entry| entry.quota.outstanding_requests)).unwrap_or(16)
}

/// 🌐️ Every `host::Host` handle vended to plugin/extension code shares this actor's one
/// `RequestRegistry`, scoped to `instance` (`RequestRegistry::for_instance` — design-abi.md §4's
/// per-request instance tagging, so `Event::InstanceClose` can cancel exactly this instance's
/// pending host round-trips and no other's) — see `host::Host::new`.
pub async fn host_for_instance(instance: u32) -> crate::host::Host {
    // 🌉️ `LocalKey::with`'s closure is sync — bridged via `resolve_ready` (`for_instance` is a
    // pure clone-and-scope, no real suspension); `Host::new` itself is awaited normally outside.
    let registry = REGISTRY.with(|registry| registry.for_instance(instance));
    crate::host::Host::new(registry).await
}

//#region 🔖️ExtensionContinuation
/// 🔁️ Mints one `Effect::InvokeExtension` for `instance` through this actor's `RequestRegistry` and
/// records `response_action` against the minted id — the ONE place in the repo where that effect is
/// created on the guest side outside `host::Host::invoke_extension`'s awaiting variant. Returns the
/// minted id (diagnostic; the effect itself rides the registry's own outbound queue, drained into
/// `turn-result.effects` by `poll` later in the SAME turn).
pub(crate) fn queue_extension_invocation(instance: u32, invocation: &crate::app::ExtensionInvocation) -> Result<semio_framework::kernel::RequestId, semio_framework::Fault> {
    let registry = REGISTRY.with(|registry| registry.for_instance(instance));
    let extension_id = invocation.extension_id.clone();
    let capability = invocation.capability.clone();
    let request_json = invocation.request_json.clone();
    registry.request_continuation(invocation.response_action.clone(), invocation.request_json.clone(), move |req| Effect::invoke_extension(req, extension_id, capability, request_json))
}

/// 📤️ Drains this actor's queued outbound effects — exactly what `poll` folds into
/// `turn-result.effects` once per turn. A fixture that drives a `PluginApp` without a live
/// `PluginRuntime` has no `poll`, so this is its only way to observe a minted
/// `Effect::InvokeExtension` (`🔌️plugin/🦀️.rs`'s `testkit::settle_extension_invocations`).
pub(crate) fn drain_queued_effects(instance: u32) -> Vec<Effect> {
    REGISTRY.with(|registry| registry.for_instance(instance).drain())
}

/// 🔁️ The arguments `response_action` is dispatched with: the ORIGINAL request object's own fields
/// (whatever correlation the app put there — a `nodeHash`, a handle, an operator id) merged with
/// this invocation's outcome. Domain-neutral by construction: the SDK never invents a key the app
/// did not already send, it only adds `ok` plus either `outputJson` or `faultCode`/`faultMessage`.
pub fn extension_response_args(request_json: &str, outcome: &Result<Vec<u8>, semio_framework::Fault>) -> dsl::DslValue {
    let mut fields: Vec<(String, dsl::DslValue)> = match dsl::json::from_json_str::<dsl::DslValue>(request_json) {
        Ok(dsl::DslValue::Object(object)) => object,
        _ => Vec::new(),
    };
    fields.retain(|(key, value)| {
        if key == "ok" || key == "outputJson" || key == "faultCode" || key == "faultMessage" {
            return false;
        }
        // 📏️ CORRELATION, not the body: the SDK echoes the app's own request fields back so the
        // response action can find its node/window/handle again, and a request BODY (a serialized
        // operator input, a geometry blob) is not correlation — echoing it would carry the payload
        // into the guest a second time, on top of the outcome
        // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). The bound is the one every structurally
        // addressed argument already crosses: a string the shell could not have SENT as a command
        // argument is not a string this may hand back as one.
        let over_bound = matches!(value, dsl::DslValue::String(text) if text.chars().map(semio_framework::public_invocation_char_cost).sum::<usize>() > semio_framework::PUBLIC_INVOCATION_STRING_BYTES);
        if over_bound && semio_framework_trace::runtime_diagnostics_enabled() {
            eprintln!("[DEBUG] extension response dropped the oversized request field {key:?} from the echoed correlation");
        }
        !over_bound
    });
    match outcome {
        // 📦️ The ABI carries the answer as a `pack` (`🔌️plugin/🧬️schema/📜️.wit`'s `type pack =
        // list<u8>`: "no JSON string … anywhere on this ABI's data path"), so the SDK DECODES it
        // into the response action's declared `outputJson` text. Lossy-stringifying the container
        // bytes instead — which is what this did — handed every browser-served answer to the app as
        // mojibake, because the shell packs (`encodePackValue(JSON.parse(outputJson))`,
        // `🏛️ShellHost/🟦️.tsx`) while only a native fixture ever sent raw JSON
        // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        Ok(bytes) => match store::pack_rt::decode_wire_value(bytes) {
            Ok(value) => {
                fields.push(("ok".to_string(), dsl::DslValue::Bool(true)));
                fields.push(("outputJson".to_string(), dsl::DslValue::String(dsl::json::to_json_string(&value))));
            }
            Err(error) => {
                fields.push(("ok".to_string(), dsl::DslValue::Bool(false)));
                fields.push(("faultCode".to_string(), dsl::DslValue::String("extension.answer-not-a-pack".to_string())));
                fields.push(("faultMessage".to_string(), dsl::DslValue::String(format!("{} answer bytes are not a pack wire value: {error}", bytes.len()))));
            }
        },
        Err(fault) => {
            fields.push(("ok".to_string(), dsl::DslValue::Bool(false)));
            fields.push(("faultCode".to_string(), dsl::DslValue::String(fault.code.0.clone())));
            fields.push(("faultMessage".to_string(), dsl::DslValue::String(fault.message.clone())));
        }
    }
    dsl::DslValue::object(fields)
}

/// 🔁️ `Event::Completed`'s continuation branch, factored out of `poll`'s event loop so it is unit
/// testable without a live `PluginRuntime`: takes the id's continuation (if any) and builds the
/// exact `(instance, action, args)` triple the follow-up dispatch uses. `None` means the id is an
/// ordinary parked-future request and must go to `RequestRegistry::resolve` instead.
pub(crate) fn take_extension_response(
    req: semio_framework::kernel::RequestId,
    terminal: Result<Vec<u8>, semio_framework::Fault>,
) -> Result<(u32, String, dsl::DslValue), Result<Vec<u8>, semio_framework::Fault>> {
    let Some(continuation) = REGISTRY.with(|registry| registry.take_continuation(req)) else { return Err(terminal) };
    let (instance, response_action, request_json, outcome) = continuation.into_response(terminal);
    Ok((instance, response_action, extension_response_args(&request_json, &outcome)))
}

/// 📄️ Accumulates one PROLOGUE page of `req`'s answer — the door a host answer larger than one
/// `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` page arrives through, whose terminal page rides the
/// `Event::Completed` that answers the request. `false` means `req` owns no continuation and the
/// caller must fall through to the parked-future chunk path.
pub(crate) fn append_extension_response_page(req: semio_framework::kernel::RequestId, bytes: &[u8]) -> bool {
    REGISTRY.with(|registry| registry.append_continuation_chunk(req, bytes)) == requests::ContinuationChunkStep::Accumulated
}
//#endregion 🔖️ExtensionContinuation

/// 🌐️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): the instance-agnostic sibling of
/// `host_for_instance` — a `Host` scoped to instance 0, the SAME "no instance tag declared" default
/// `RequestRegistry::for_instance`'s own doc names. `⚛️reactor/💼️jobs/🦀️.rs::spawn_job`
/// calls this (as `crate::reactor::host()`, zero args): a job is actor-global, not tied to one open
/// instance the way an `AsyncTask` is, so it has no `instance: u32` to scope by in the first place.
pub async fn host() -> crate::host::Host {
    REGISTRY.with(|registry| crate::host::Host::new(registry.clone())).await
}

/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): spawns `task` onto this actor's
/// shared `LocalExecutor`, quota-gated then key-deduped (in that order — a same-key respawn at
/// exactly the quota limit legitimately fails; the caller may retry once the cancelled slot is
/// actually freed on a later turn). Called from `🔌️plugin/🦀️.rs`'s `dispatch_emit`, right
/// after a gesture's mutation lanes land — `M`/`C`/`D` are that call's concrete `A::Mutation`/
/// `A::ConfigMutation`/`A::DraftMutation`, monomorphized per app. The moment the task's future
/// resolves, its `TaskResolution` is erased to bytes (`TaskResumeOutcome`, the SAME
/// `last_emit_wire` wire idiom `dispatch_emit` itself uses for its OWN mutation lanes) and queued
/// on `TASK_RESUMES` — no `M`/`C`/`D` generic ever crosses into the executor or the resume queue,
/// which is what lets ALL of this actor's apps (each with its own concrete `A`) share ONE
/// `LocalExecutor`/`TASK_RESUMES` pair.
#[cfg(test)]
pub(crate) async fn spawn_task<M, C, D>(instance: u32, meta: &crate::app::ActionMeta, task: crate::app::AsyncTask<M, C, D>) -> Result<(), semio_framework::Fault>
where
    M: ::protocol::OpBinary + 'static,
    C: ::protocol::OpBinary + 'static,
    D: ::protocol::OpBinary + 'static,
{
    let quota = instance_task_quota(instance).await;
    let live = TASK_RECORDS.with(|records| records.borrow().count_instance(instance) as u64);
    if live >= quota {
        return Err(semio_framework::Fault::new(
            semio_framework::FaultOrigin::Plugin,
            semio_framework::FaultCode::new("plugin.task.quota-exceeded"),
            format!("instance {instance} already has {live} outstanding task(s) (quota {quota}); `{}` was not spawned", task.label),
        ));
    }

    let (label, key, restart, run) = task.into_parts().await;
    if label.len() > REACTOR_TASK_LABEL_BYTES || restart.as_ref().is_some_and(|bytes| bytes.len() > REACTOR_TASK_RESTART_BYTES) {
        return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("plugin.task-authority-too-large"), "task label or restart authority exceeds its fixed admitted byte bound"));
    }
    if key.as_ref().is_some_and(|key| key.len() > REACTOR_TASK_KEY_BYTES) {
        return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("plugin.task.key-too-large"), format!("task key exceeds {REACTOR_TASK_KEY_BYTES} bytes")));
    }

    // 🔑️ Latest-wins dedupe: a task spawned with the same `(instance, key)` as one still live
    // cancels the live one FIRST — its future (and anything it owns, including a parked
    // `RequestFuture`) is dropped without ever completing, so no resume is ever queued for it.
    if let Some(key) = &key {
        if TASK_RECORDS.with(|records| records.borrow().find_key(instance, key)).is_some() {
            return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("plugin.task.supersession-pending"), "keyed task supersession awaits bounded disposal of its previous owner"));
        }
    }

    let reservation = TEST_FUTURE_EXECUTOR.with(|executor| executor.reserve()).map_err(|message| semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.task.executor-capacity"), message))?;
    let task_id = reservation.id();
    if !TASK_RECORDS.with(|records| records.borrow().can_insert(task_id)) {
        return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.task.record-capacity"), "fixed task record authority rejected an executor-reserved direct slot"));
    }
    TASK_RECORDS.with(|records| records.borrow_mut().insert_admitted(task_id, TaskRecord { instance, key, restart }));

    let ctx = crate::app::TaskCtx { host: host_for_instance(instance).await, meta: meta.clone() };
    let future = run(ctx);
    let resume_instance = instance;
    let resume_meta = meta.clone();
    reservation.install(Box::pin(async move {
        let outcome = match future.await {
            Ok(crate::app::TaskResolution::Command(bytes)) => Some(TaskResumeOutcome::Command(bytes)),
            Ok(crate::app::TaskResolution::Emit(emit)) => {
                Some(TaskResumeOutcome::Emit { artifact_ops: encode_mutation_lane(&emit.artifact_mutations).await, config_ops: encode_mutation_lane(&emit.config_mutations).await, draft_ops: encode_mutation_lane(&emit.draft_mutations).await })
            }
            Ok(crate::app::TaskResolution::Done) => None,
            Err(fault) => Some(TaskResumeOutcome::Fault(fault)),
        };
        TASK_RECORDS.with(|records| drop(records.borrow_mut().remove(task_id)));
        if let Some(outcome) = outcome {
            assert!(TASK_RESUMES.with(|resumes| resumes.borrow_mut().push(PendingResume { instance: resume_instance, meta: resume_meta, outcome })).is_ok(), "fixed task-resume authority is saturated");
        }
    }));

    Ok(())
}

/// 🔀️ The exact wire shape `dispatch_emit`'s own `last_emit_wire` uses for one mutation lane —
/// factored out so `spawn_task`'s `TaskResolution::Emit` erasure and `dispatch_emit` stay
/// byte-identical without one calling the other across the crate's plugin/reactor split.
#[cfg(test)]
async fn encode_mutation_lane<T: ::protocol::OpBinary>(ops: &[T]) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(ops.len());
    for op in ops.iter() {
        encoded.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
    }
    protocol::encode_ops_vec(&encoded)
}

/// 🚫️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): `Event::InstanceClose`
/// cancellation — drops every task `instance` owns from `EXECUTOR` (dropping its future, and
/// everything IT owns including any parked `RequestFuture`) and clears the key-dedupe index.
/// Deliberately does NOT touch `REGISTRY`/`RequestRegistry::cancel_instance` — that is a SEPARATE
/// step the caller (`poll`'s `Event::InstanceClose` handling) runs right after this one, since a
/// task's own `RequestFuture` is already gone the moment its owning future is dropped here; the
/// registry sweep is defense-in-depth for a pending request whose task somehow isn't tracked here
/// (there should be none, by construction — every `RequestFuture` is created inside `TaskCtx.host`,
/// itself only ever handed to a task by `spawn_task`).
// 🚫️async: E1 pure in-memory sweep over `TASK_RECORDS`/`EXECUTOR`/`TASK_KEYS` (all sync now,
// R9) consumed by `poll`'s sync `world actor` boundary — zero suspension.
pub(crate) fn cancel_instance_tasks_step(instance: u32, cursor: &mut usize) -> bool {
    #[cfg(not(test))]
    {
        let budget = executor::ReactorTaskBudget { operation: 0, generation: 0, cancellation_generation: 0, maximum_units: 1, maximum_bytes: 4_096, deadline: std::time::Instant::now() + std::time::Duration::from_millis(8) };
        matches!(REACTOR_EXECUTOR.with(|executor| executor.close_instance_step(instance, cursor, budget)), executor::ReactorTaskStep::Complete)
    }
    #[cfg(test)]
    {
        if *cursor >= REACTOR_TASK_SLOTS {
            return true;
        }
        let entry = TASK_RECORDS.with(|records| records.borrow().entry_at(*cursor).and_then(|(id, record)| (record.instance == instance).then_some(id)));
        if let Some(id) = entry {
            let poll = TEST_FUTURE_EXECUTOR.with(|executor| executor.poll_one(id));
            if poll == executor::TaskPoll::Pending {
                return false;
            }
            TASK_RECORDS.with(|records| drop(records.borrow_mut().remove(id)));
        }
        *cursor += 1;
        *cursor >= REACTOR_TASK_SLOTS
    }
}

fn preflight_reactor_close(key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
    REACTOR_CLOSES.with(|closes| {
        let closes = closes.try_borrow().map_err(|_| reactor_close_fault("reactor close preflight busy"))?;
        if !closes.slots.allocation_admitted {
            return Err(reactor_close_fault("reactor close backing unavailable"));
        }
        if closes.slots.get(ReactorCloseRegistry::index(key.instance())).is_some_and(|retained| retained.key != key) {
            return Err(reactor_close_fault("reactor close slot belongs to another allocation"));
        }
        Ok(())
    })
}

fn reserve_reactor_close(key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
    let instance = key.instance();
    let request_cursor = REGISTRY.with(|registry| registry.begin_cancel_instance(instance));
    let resume_remaining = TASK_RESUMES.with(|resumes| resumes.borrow().begin_cancel_instance());
    let state = ReactorCloseState {
        key,
        instance,
        active: false,
        complete: false,
        task_cursor: 0,
        timer_cursor: 0,
        request_cursor,
        resume_remaining,
        command_ingress_complete: false,
        requests_complete: false,
        resumes_complete: false,
        tasks_complete: false,
        timers_complete: false,
        metadata_complete: false,
    };
    REACTOR_CLOSES.with(|closes| {
        let mut closes = closes.try_borrow_mut().map_err(|_| reactor_close_fault("reactor close authority is busy"))?;
        if let Some(retained) = closes.slots.get(ReactorCloseRegistry::index(instance)) {
            return if retained.key == key { Ok(()) } else { Err(reactor_close_fault("reactor close slot belongs to another captured allocation")) };
        }
        closes.insert(state).map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.reactor-close-capacity"), "fixed reactor close authority is saturated or collided"))
    })
}

fn reactor_close_fault(message: &'static str) -> semio_framework::Fault {
    semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.reactor-close-authority"), message)
}

fn activate_reactor_close(key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
    REACTOR_CLOSES.with(|closes| {
        let mut closes = closes.try_borrow_mut().map_err(|_| reactor_close_fault("reactor close authority is busy"))?;
        let state = closes.slots.get_mut(ReactorCloseRegistry::index(key.instance())).filter(|state| state.key == key).ok_or_else(|| reactor_close_fault("exact reactor close reservation missing"))?;
        state.active = true;
        Ok(())
    })
}

fn reactor_close_complete(key: instance_lifetime::NativeCloseKey) -> Result<bool, semio_framework::Fault> {
    REACTOR_CLOSES.with(|closes| {
        let closes = closes.try_borrow().map_err(|_| reactor_close_fault("reactor close receipt is busy"))?;
        closes.slots.get(ReactorCloseRegistry::index(key.instance())).filter(|state| state.key == key).map(|state| state.complete).ok_or_else(|| reactor_close_fault("exact reactor close receipt missing"))
    })
}

fn release_reactor_close(key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
    REACTOR_CLOSES.with(|closes| {
        let mut closes = closes.try_borrow_mut().map_err(|_| reactor_close_fault("reactor close receipt is busy"))?;
        let index = ReactorCloseRegistry::index(key.instance());
        if !closes.slots.get(index).is_some_and(|state| state.key == key && state.complete) {
            return Err(reactor_close_fault("reactor close receipt is not terminal"));
        }
        let _ = closes.take_at(index);
        Ok(())
    })
}

fn preflight_cold_pair_close(key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
    COLD_PAIR_INGRESS.with(|ingress| ingress.try_borrow().map_err(|_| reactor_close_fault("cold pair close preflight busy"))?.preflight_close_instance(key).map_err(reactor_close_fault))
}

fn reserve_cold_pair_close(key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
    COLD_PAIR_INGRESS.with(|ingress| ingress.try_borrow_mut().map_err(|_| reactor_close_fault("cold pair close reservation busy"))?.reserve_close_instance(key).map_err(reactor_close_fault))
}

fn activate_cold_pair_close(key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
    COLD_PAIR_INGRESS.with(|ingress| ingress.try_borrow_mut().map_err(|_| reactor_close_fault("cold pair close activation busy"))?.activate_close_instance(key).map_err(reactor_close_fault))
}

fn cold_pair_close_complete(key: instance_lifetime::NativeCloseKey) -> Result<bool, &'static str> {
    COLD_PAIR_INGRESS.with(|ingress| ingress.try_borrow().map_err(|_| "cold pair close receipt busy")?.close_instance_complete(key))
}

fn release_cold_pair_close(key: instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
    COLD_PAIR_INGRESS.with(|ingress| ingress.try_borrow_mut().map_err(|_| "cold pair close release busy")?.release_close_instance(key))
}

fn step_reactor_close() -> Result<bool, semio_framework::Fault> {
    REACTOR_CLOSES.with(|closes| {
        let Ok(mut closes) = closes.try_borrow_mut() else { return Ok(false) };
        let start = REACTOR_CLOSE_CURSOR.with(Cell::get);
        let Some(index) = (0..PLUGIN_REACTOR_INSTANCE_SLOTS).map(|offset| (start + offset) % PLUGIN_REACTOR_INSTANCE_SLOTS).find(|index| closes.slots.get(*index).is_some_and(|state| state.active && !state.complete)) else { return Ok(false) };
        REACTOR_CLOSE_CURSOR.with(|cursor| cursor.set((index + 1) % PLUGIN_REACTOR_INSTANCE_SLOTS));
        let state = closes.slots.get_mut(index).expect("selected exact active close");
        let complete = if !state.command_ingress_complete {
            state.command_ingress_complete = turn::close_command_ingress_step(state.key)?;
            false
        } else if !state.requests_complete {
            state.requests_complete = REGISTRY.with(|registry| registry.cancel_instance_step(&mut state.request_cursor) == requests::RequestCloseStep::Complete);
            false
        } else if !state.resumes_complete {
            state.resumes_complete = TASK_RESUMES.with(|resumes| resumes.borrow_mut().cancel_instance_step(state.instance, &mut state.resume_remaining));
            false
        } else if !state.tasks_complete {
            state.tasks_complete = cancel_instance_tasks_step(state.instance, &mut state.task_cursor);
            false
        } else if !state.timers_complete {
            state.timers_complete = ARMED_TIMERS.with(|timers| timers.borrow_mut().cancel_instance_step(state.instance, &mut state.timer_cursor));
            false
        } else if !state.metadata_complete {
            INSTANCE_METADATA.with(|metadata| {
                drop(metadata.borrow_mut().remove(state.instance));
            });
            state.metadata_complete = true;
            false
        } else {
            true
        };
        state.complete = complete;
        Ok(true)
    })
}

#[cfg(test)]
pub(crate) fn cancel_instance_tasks(instance: u32) {
    let mut cursor = 0;
    for _ in 0..REACTOR_TASK_SLOTS {
        let before = cursor;
        if cancel_instance_tasks_step(instance, &mut cursor) || cursor == before {
            break;
        }
    }
}

/// 📸️ `checkpoint::checkpoint` body — unconditional (no WIT type in its signature, only
/// `Vec<u8>`/kernel types), unlike `poll`/the `wit_*`/`kernel_*_to_wit` bridge below.
pub async fn checkpoint_now<PA: crate::app::PluginApp>(runtime: &crate::plugin_runtime::PluginRuntime<PA>) -> Result<Vec<u8>, semio_framework::Fault> {
    let instances = INSTANCE_METADATA.with(|metadata| metadata.borrow().checkpoint_rows());
    let timers = ARMED_TIMERS.with(|timers| timers.borrow().rows());
    let pending = REGISTRY.with(|registry| registry.pending_ids().into_iter().map(|id| id.0).collect());
    // 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): the task itself is never
    // serialized (`TASK_RECORDS`/`EXECUTOR` are process memory, not pack state) — only the
    // `restart` command bytes of every LIVE task that declared one via `.restartable(..)` survive
    // into the pack, one `TaskRestart{instance, command}` per such task.
    let task_restarts: Vec<checkpoint::TaskRestart> =
        TASK_RECORDS.with(|records| records.borrow().iter().filter_map(|(_, record)| record.restart.as_ref().map(|command| checkpoint::TaskRestart { instance: record.instance, command: command.clone() })).collect());
    checkpoint::checkpoint(runtime, &instances, timers, pending, task_restarts).await
}

/// 📸️ `checkpoint::restore` body — re-arms the timer list from the restored pack;
/// `pending_requests` are intentionally NOT re-parked (design-abi.md §4: async tasks are marked
/// re-run-on-restore, not resumed as though the host round-trip were still in flight).
/// `task_restarts` ARE re-dispatched, though not synchronously here: each one is queued onto
/// `TASK_RESUMES` as an ordinary `Command` resume (the SAME resume path a live task's own
/// `TaskResolution::Command` takes), drained by the first `poll` after restore — restoring is a
/// pure state-load, it must not itself re-enter app dispatch.
#[expect(clippy::result_large_err, reason = "The fixed resume queue returns the admitted view snapshot, task metadata, and payload owner intact without allocating on rejection.")]
pub async fn restore_now<PA: crate::app::PluginApp>(runtime: &crate::plugin_runtime::PluginRuntime<PA>, state: &[u8]) -> Result<(), semio_framework::Fault> {
    let pack = checkpoint::restore(runtime, state).await?;
    let instances = pack.instances().await;
    let armed_timers = pack.timers().await.to_vec();
    INSTANCE_METADATA.with(|metadata| {
        let mut metadata = metadata.borrow_mut();
        for (instance, app_id) in instances {
            metadata.insert(instance, app_id, semio_framework::kernel::QuotaSchema::default())?;
        }
        Ok::<(), semio_framework::Fault>(())
    })?;
    ARMED_TIMERS.with(|timers| {
        if !timers.borrow().is_empty() {
            return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.timer-restore-live"), "restore cannot replace live fixed timer authority"));
        }
        if !armed_timers.is_empty() {
            return Err(semio_framework::Fault::new(
                semio_framework::FaultOrigin::Framework,
                semio_framework::FaultCode::new("plugin.timer-restore-owner-missing"),
                "checkpoint timer rows lack the exact numeric instance owner required by the fixed close authority",
            ));
        }
        Ok::<(), semio_framework::Fault>(())
    })?;
    for restart in pack.task_restarts().await {
        let meta = crate::app::ActionMeta { actor: crate::plugin_runtime::instance_actor(runtime, restart.instance).await, instance_id: restart.instance, view_state: None };
        let pending = PendingResume { instance: restart.instance, meta, outcome: TaskResumeOutcome::Command(restart.command.clone()) };
        TASK_RESUMES
            .with(|resumes| resumes.borrow_mut().push(pending))
            .map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.task-resume-capacity"), "fixed task-resume authority is saturated during restore"))?;
    }
    Ok(())
}

/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): re-exported so the (future)
/// `world actor-async` runner can call the SAME `drain_task_resumes` `poll` uses — one
/// implementation of "how a resumed `AsyncTask` re-enters the reducer", not two.
#[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]
pub use turn::drain_task_resumes;

//#region 🔁️CommandIngressTerminal
fn advance_command_cursor(mut cursor: semio_framework::kernel::CommandPageCursor) -> Result<semio_framework::kernel::CommandPageCursor, semio_framework::kernel::CommandPageCursor> {
    let Some(page_index) = cursor.page_index.checked_add(1) else { return Err(cursor) };
    cursor.page_index = page_index;
    Ok(cursor)
}

fn terminal_command_ingress(cursor: semio_framework::kernel::CommandPageCursor, fault: Option<Vec<u8>>) -> semio_framework::kernel::CommandIngressStatus {
    match advance_command_cursor(cursor) {
        Ok(terminal) => match fault {
            Some(fault) => semio_framework::kernel::CommandIngressStatus::Fault { cursor: terminal, fault },
            None => semio_framework::kernel::CommandIngressStatus::CommandComplete(terminal),
        },
        Err(cursor) => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-index-exhausted".to_vec() },
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️command-ingress-terminal/🦀️.rs"]
mod command_ingress_terminal_tests;
//#endregion 🔁️CommandIngressTerminal

/// 🧬️ Everything below crosses the wasm component boundary — gated identically to `component`
/// (`🦀️.rs` at crate root) since it names `crate::component::wasip2::exports::...` types that
/// simply do not exist outside a `component-guest`/`component-extension-guest` wasm32-wasip2
/// build (mirrors the OLD `host_port`'s per-function `#[cfg(...)]` pattern, just hoisted to one
/// module instead of repeated per function).
#[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]
pub use wit_bridge::{poll, stage_cold_pair_page, stage_command_page};

#[path = "🔄️turn/🦀️.rs"]
mod turn;
pub use turn::poll_kernel;
pub use turn::retained_command_ingress_occupancy;
pub use turn::{last_turn_more_work_sources, TurnMoreWorkSources};

#[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]
mod wit_bridge {
    use super::*;
    /// 🧭️ `reactor`/`jobs`/`checkpoint`/`describe` are the only interfaces `world actor` directly
    /// `export`s, so wit-bindgen only aliases THEIR top-level types under `exports::…`. `effects`/
    /// `events`/`ui`/`types` are merely `use`d by `reactor.wit` (design-abi.md §1/§4) — their own
    /// payload records live at the plain (non-`exports::`) path alongside the `pure` import, one
    /// level down from where the nesting stops being re-aliased. Verified empirically: a
    /// deliberately wrong `wit::OpenWindowEffect` import made `cargo check --target wasm32-wasip2
    /// --features component-guest` emit `help: consider importing … effects::OpenWindowEffect`
    /// (and the `events`/`ui` siblings the same way) — not guessed.
    use crate::component::wasip2::semio::framework::effects as wit_effects;
    use crate::component::wasip2::semio::framework::events as wit_events;
    use crate::component::wasip2::semio::framework::instance_lifetime as wit_lifetime;
    use crate::component::wasip2::semio::framework::types as wit_types;
    use crate::component::wasip2::semio::framework::ui as wit_ui;

    crate::component_persistent_local! {
        /// 📥️ The one command-ingress page staged for the next turn, and the one cold document-pair
        /// page beside it. Staged by their own exports and TAKEN by `poll`, never left behind.
        static STAGED_PAGES: RefCell<StagedTurnPages> = RefCell::new(StagedTurnPages { command: None, cold_pair: None });
    }

    /// 📦️ Both pages are BOXED. A staged `FixedCommandPage` is 4 KiB by value, and every `.with()`
    /// take on the deepest frame of the turn would memcpy it across the guest's 64 KiB shadow stack —
    /// measured as `memory fault at wasm address 0xffff3798`, a stack-pointer underflow, on turn 0.
    #[derive(Default)]
    struct StagedTurnPages {
        command: Option<(semio_framework::kernel::CommandPageCursor, Box<semio_framework::kernel::FixedCommandPage>)>,
        cold_pair: Option<Box<semio_framework::kernel::ColdDocumentPairPage>>,
    }

    fn staging_fault(reason: &'static str) -> semio_framework::Fault {
        semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.turn-page-staging"), reason)
    }

    /// 📥️ `reactor::stage-command-page` body — see the WIT for why the page is staged instead of
    /// riding `poll`'s parameter list.
    pub async fn stage_command_page(
        cursor: crate::component::wasip2::exports::semio::framework::reactor::CommandPageCursor,
        bytes: Vec<u8>,
    ) -> Result<(), semio_framework::Fault> {
        let staged = wit_command_page_to_kernel(cursor, bytes)?;
        let staged = (staged.0, Box::new(staged.1));
        STAGED_PAGES.with(|pages| {
            let mut pages = pages.try_borrow_mut().map_err(|_| staging_fault("turn page staging authority busy"))?;
            if pages.command.is_some() {
                return Err(staging_fault("a command page is already staged for the next turn"));
            }
            pages.command = Some(staged);
            Ok(())
        })
    }

    /// 🧊️ `reactor::stage-cold-pair-page` body — the cold-pair twin of [`stage_command_page`].
    pub async fn stage_cold_pair_page(page: crate::component::wasip2::exports::semio::framework::reactor::ColdDocumentPairPage) -> Result<(), semio_framework::Fault> {
        let staged = Box::new(wit_cold_pair_page_to_kernel(page)?);
        STAGED_PAGES.with(|pages| {
            let mut pages = pages.try_borrow_mut().map_err(|_| staging_fault("turn page staging authority busy"))?;
            if pages.cold_pair.is_some() {
                return Err(staging_fault("a cold document-pair page is already staged for the next turn"));
            }
            pages.cold_pair = Some(staged);
            Ok(())
        })
    }

    /// ▶️ The real `reactor::poll` body — see module doc for the shape. `events`/`budget` are the
    /// WIT-generated types from `exports::semio::framework::reactor`; the return is that same
    /// module's `TurnResult`. The turn's pages come from [`STAGED_PAGES`], which this call empties.
    pub async fn poll<PA: crate::app::PluginApp + 'static>(
        runtime: &crate::plugin_runtime::PluginRuntime<PA>,
        events: Vec<crate::component::wasip2::exports::semio::framework::reactor::Event>,
        budget: crate::component::wasip2::exports::semio::framework::reactor::Budget,
    ) -> Result<crate::component::wasip2::exports::semio::framework::reactor::TurnResult, semio_framework::Fault> {
        turn::note_turn_events(events.len(), wit_events_payload_bytes(&events));
        let mut kernel_events = Vec::with_capacity(events.len());
        for event in events {
            kernel_events.push(wit_event_to_kernel(event));
        }
        let kernel_budget = semio_framework::kernel::Budget { fuel: budget.fuel, deadline_ms: budget.deadline_ms, max_effects: budget.max_effects, max_patch_bytes: budget.max_patch_bytes, max_frames: budget.max_frames };
        let staged = STAGED_PAGES.with(|pages| pages.try_borrow_mut().map(|mut pages| std::mem::take(&mut *pages)).map_err(|_| staging_fault("turn page staging authority busy")))?;
        turn::poll_kernel_output(runtime, kernel_events, staged.command.map(|(cursor, page)| (cursor, *page)), staged.cold_pair.map(|page| *page), kernel_budget, |result| kernel_turn_result_to_wit(result, budget), |_, prepared| prepared).await
    }

    /// 🩺️ Bulk bytes a turn's events carry across the component boundary — every `pack`/`list<u8>`
    /// field, which is all a turn can retain that scales with its input. Fixed-shape variants
    /// (`timer`, `wake`, `surface-resized`, …) contribute nothing by construction, so a turn reading
    /// `eventsBytes=0` genuinely carried no payload. Read by the per-turn guest memory trace.
    fn wit_events_payload_bytes(events: &[crate::component::wasip2::exports::semio::framework::reactor::Event]) -> usize {
        use crate::component::wasip2::exports::semio::framework::reactor as wit;
        let completion_bytes = |outcome: &wit_events::CompletionResult| match outcome {
            wit_events::CompletionResult::Ok(pack) | wit_events::CompletionResult::Fault(pack) => pack.len(),
        };
        events
            .iter()
            .map(|event| match event {
                wit::Event::InstanceOpen(open) => open.config.len() + open.quotas.len() + open.assets.iter().map(|(name, pack)| name.len() + pack.len()).sum::<usize>(),
                wit::Event::QuotaChanged(changed) => changed.quotas.len(),
                wit::Event::UiIntent(intent) => intent.intent.len(),
                wit::Event::SurfaceVisible(visible) => visible.body_key.len() + visible.view_state.len(),
                wit::Event::PatchRejected(rejected) => rejected.reason.len(),
                wit::Event::Completed(completed) => completion_bytes(&completed.outcome),
                wit::Event::HttpChunk(chunk) => chunk.params.bytes.len(),
                wit::Event::JobProgress(progress) => progress.progress.len(),
                wit::Event::JobCompleted(completed) => completion_bytes(&completed.outcome),
                wit::Event::Message(message) => message.payload.len(),
                wit::Event::Request(request) => request.params.capability.len() + request.params.payload.len(),
                wit::Event::InstanceClose(_) | wit::Event::InstanceLifecycleAck(_) | wit::Event::Activate(_) | wit::Event::SuspendRequest(_) | wit::Event::CapabilityChanged(_) => 0,
                wit::Event::SurfaceHidden(_) | wit::Event::SurfaceResized(_) | wit::Event::PatchAck(_) | wit::Event::Timer(_) | wit::Event::Wake => 0,
            })
            .sum()
    }

    /// 📄️ Lifts one staged command page into the kernel's fixed 4 KiB owner. The bytes arrive as a
    /// `list<u8>` the guest owns and drops; the fixed page they are copied into is the authority the
    /// ingress assembler reads.
    fn wit_command_page_to_kernel(
        cursor: crate::component::wasip2::exports::semio::framework::reactor::CommandPageCursor,
        bytes: Vec<u8>,
    ) -> Result<(semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage), semio_framework::Fault> {
        if bytes.len() > semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES {
            return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.command-page-lift-cap"), "fixed command page declares more than 4096 bytes"));
        }
        let length = bytes.len() as u32;
        let mut fixed = [0; semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES];
        fixed[..bytes.len()].copy_from_slice(&bytes);
        let bytes = semio_framework::kernel::FixedCommandPage::try_from_array(fixed, length)?;
        Ok((
            semio_framework::kernel::CommandPageCursor {
                owner: cursor.owner,
                generation: cursor.generation,
                command_index: cursor.command_index,
                command_count: cursor.command_count,
                instance: cursor.instance,
                seq: cursor.seq,
                kind: cursor.kind,
                page_index: cursor.page_index,
                page_count: cursor.page_count,
                item_count: cursor.item_count,
                metadata: cursor.metadata,
            },
            bytes,
        ))
    }

    fn kernel_command_cursor_to_wit(cursor: &semio_framework::kernel::CommandPageCursor) -> crate::component::wasip2::exports::semio::framework::reactor::CommandPageCursor {
        use crate::component::wasip2::exports::semio::framework::reactor as wit;
        wit::CommandPageCursor {
            owner: cursor.owner,
            generation: cursor.generation,
            command_index: cursor.command_index,
            command_count: cursor.command_count,
            instance: cursor.instance,
            seq: cursor.seq,
            kind: cursor.kind,
            page_index: cursor.page_index,
            page_count: cursor.page_count,
            item_count: cursor.item_count,
            metadata: cursor.metadata,
        }
    }

    /// 💤️ Canonical cursor payload for the scalar idle command-ingress record.
    fn idle_command_cursor_to_wit() -> crate::component::wasip2::exports::semio::framework::reactor::CommandPageCursor {
        use crate::component::wasip2::exports::semio::framework::reactor as wit;
        wit::CommandPageCursor { owner: 0, generation: 0, command_index: 0, command_count: 0, instance: 0, seq: 0, kind: 0, page_index: 0, page_count: 0, item_count: 0, metadata: 0 }
    }

    /// 🔢️ Kernel command ingress → scalar WIT record, avoiding nested variant discriminants in async results.
    fn kernel_command_ingress_to_wit(status: semio_framework::kernel::CommandIngressStatus) -> crate::component::wasip2::exports::semio::framework::reactor::CommandIngressStatus {
        use crate::component::wasip2::exports::semio::framework::reactor as wit;
        let (kind, cursor, fault) = match status {
            semio_framework::kernel::CommandIngressStatus::Idle => (0, idle_command_cursor_to_wit(), Vec::new()),
            semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor) => (1, kernel_command_cursor_to_wit(&cursor), Vec::new()),
            semio_framework::kernel::CommandIngressStatus::Backpressure(cursor) => (2, kernel_command_cursor_to_wit(&cursor), Vec::new()),
            semio_framework::kernel::CommandIngressStatus::CommandPending(cursor) => (3, kernel_command_cursor_to_wit(&cursor), Vec::new()),
            semio_framework::kernel::CommandIngressStatus::CommandComplete(cursor) => (4, kernel_command_cursor_to_wit(&cursor), Vec::new()),
            semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault } => (5, kernel_command_cursor_to_wit(&cursor), fault),
        };
        wit::CommandIngressStatus { kind, cursor, fault }
    }

    fn wit_cold_sha256(bytes: Vec<u8>, field: &'static str) -> Result<[u8; 32], semio_framework::Fault> {
        bytes.try_into().map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.cold-pair-hash-width"), field))
    }

    fn wit_cold_pair_page_to_kernel(page: crate::component::wasip2::exports::semio::framework::reactor::ColdDocumentPairPage) -> Result<semio_framework::kernel::ColdDocumentPairPage, semio_framework::Fault> {
        let header = page.header;
        let frontier = header.baseline_frontier;
        Ok(semio_framework::kernel::ColdDocumentPairPage {
            header: semio_framework::kernel::ColdDocumentPairHeader {
                lifetime: wit_lifetime_to_kernel(header.lifetime),
                transfer_generation: header.transfer_generation,
                descriptor_sha256: wit_cold_sha256(header.descriptor_sha256, "cold pair descriptor hash must be 32 bytes")?,
                baseline_frontier: semio_framework::kernel::ColdDocumentPairFrontier {
                    document_id: frontier.document_id,
                    head_edit_ordinal: frontier.head_edit_ordinal,
                    head_edit_id: frontier.head_edit_id,
                    last_commit_seq: frontier.last_commit_seq,
                    chain_sha256: wit_cold_sha256(frontier.chain_sha256, "cold pair frontier hash must be 32 bytes")?,
                },
                pack_sha256: wit_cold_sha256(header.pack_sha256, "cold pair Pack hash must be 32 bytes")?,
                spr_sha256: wit_cold_sha256(header.spr_sha256, "cold pair SPR hash must be 32 bytes")?,
                aggregate_sha256: wit_cold_sha256(header.aggregate_sha256, "cold pair aggregate hash must be 32 bytes")?,
                pack_length: header.pack_length,
                spr_length: header.spr_length,
                page_count: header.page_count,
            },
            page_index: page.page_index,
            bytes: page.bytes,
        })
    }

    fn kernel_cold_frontier_to_wit(frontier: semio_framework::kernel::ColdDocumentPairFrontier) -> crate::component::wasip2::exports::semio::framework::reactor::ColdDocumentPairFrontier {
        use crate::component::wasip2::exports::semio::framework::reactor as wit;
        wit::ColdDocumentPairFrontier { document_id: frontier.document_id, head_edit_ordinal: frontier.head_edit_ordinal, head_edit_id: frontier.head_edit_id, last_commit_seq: frontier.last_commit_seq, chain_sha256: frontier.chain_sha256.to_vec() }
    }

    fn kernel_cold_cursor_to_wit(cursor: semio_framework::kernel::ColdDocumentPairCursor) -> crate::component::wasip2::exports::semio::framework::reactor::ColdDocumentPairCursor {
        use crate::component::wasip2::exports::semio::framework::reactor as wit;
        wit::ColdDocumentPairCursor { lifetime: kernel_lifetime_to_wit(cursor.lifetime), transfer_generation: cursor.transfer_generation, page_index: cursor.page_index, page_count: cursor.page_count }
    }

    fn kernel_cold_pair_ingress_to_wit(status: semio_framework::kernel::ColdPairIngressStatus) -> crate::component::wasip2::exports::semio::framework::reactor::ColdPairIngressStatus {
        use crate::component::wasip2::exports::semio::framework::reactor as wit;
        match status {
            semio_framework::kernel::ColdPairIngressStatus::Idle => wit::ColdPairIngressStatus::Idle,
            semio_framework::kernel::ColdPairIngressStatus::PageAccepted(cursor) => wit::ColdPairIngressStatus::PageAccepted(kernel_cold_cursor_to_wit(cursor)),
            semio_framework::kernel::ColdPairIngressStatus::Backpressure(cursor) => wit::ColdPairIngressStatus::Backpressure(kernel_cold_cursor_to_wit(cursor)),
            semio_framework::kernel::ColdPairIngressStatus::Loading(cursor) => wit::ColdPairIngressStatus::Loading(kernel_cold_cursor_to_wit(cursor)),
            semio_framework::kernel::ColdPairIngressStatus::Applied(receipt) => wit::ColdPairIngressStatus::Applied(wit::ColdDocumentPairApplied {
                lifetime: kernel_lifetime_to_wit(receipt.lifetime),
                transfer_generation: receipt.transfer_generation,
                baseline_frontier: kernel_cold_frontier_to_wit(receipt.baseline_frontier),
                aggregate_sha256: receipt.aggregate_sha256.to_vec(),
            }),
            semio_framework::kernel::ColdPairIngressStatus::Fault { cursor, fault } => wit::ColdPairIngressStatus::Fault(wit::ColdDocumentPairFault { cursor: kernel_cold_cursor_to_wit(cursor), fault }),
        }
    }

    fn decode_wire_quotas(bytes: &[u8]) -> semio_framework::kernel::QuotaSchema {
        store::pack_rt::decode_wire_value(bytes).ok().and_then(|value| dsl::from_dsl_value(value).ok()).unwrap_or_default()
    }

    /// 🔀️ WIT `event` → kernel `Event`. Thin field-for-field translation — the WIT side already
    /// mirrors the kernel shape (see `📓️design-abi.md` §2 / `events.wit`'s own doc comments).
    fn wit_lifetime_to_kernel(value: wit_lifetime::Lifetime) -> semio_framework::kernel::ActorInstanceLifetime {
        semio_framework::kernel::ActorInstanceLifetime { activation_generation: value.activation_generation, instance_id: value.instance_id, guest_lifetime: value.guest_lifetime }
    }

    fn kernel_lifetime_to_wit(value: semio_framework::kernel::ActorInstanceLifetime) -> wit_lifetime::Lifetime {
        wit_lifetime::Lifetime { activation_generation: value.activation_generation, instance_id: value.instance_id, guest_lifetime: value.guest_lifetime }
    }

    fn wit_patch_receipt_to_kernel(value: wit_lifetime::UiPatchReceipt) -> ActorUiPatchReceipt {
        ActorUiPatchReceipt { lifetime: wit_lifetime_to_kernel(value.lifetime), patch_sequence: value.patch_sequence }
    }

    fn kernel_patch_receipt_to_wit(value: ActorUiPatchReceipt) -> wit_lifetime::UiPatchReceipt {
        wit_lifetime::UiPatchReceipt { lifetime: kernel_lifetime_to_wit(value.lifetime), patch_sequence: value.patch_sequence }
    }

    fn wit_lifecycle_receipt_to_kernel(value: wit_lifetime::Receipt) -> ActorInstanceLifecycleReceipt {
        use semio_framework::kernel::ActorInstanceLifecycleReceipt as R;
        match value {
            wit_lifetime::Receipt::Captured(value) => R::Captured { lifetime: wit_lifetime_to_kernel(value.lifetime), request_sequence: value.request_sequence },
            wit_lifetime::Receipt::Accepted(value) => R::Accepted { lifetime: wit_lifetime_to_kernel(value.lifetime), request_sequence: value.request_sequence, close_generation: value.close_generation },
            wit_lifetime::Receipt::Retired(value) => R::Retired { lifetime: wit_lifetime_to_kernel(value.lifetime), request_sequence: value.request_sequence, close_generation: value.close_generation },
        }
    }

    fn kernel_lifecycle_receipt_to_wit(value: ActorInstanceLifecycleReceipt) -> wit_lifetime::Receipt {
        use semio_framework::kernel::ActorInstanceLifecycleReceipt as R;
        match value {
            R::Captured { lifetime, request_sequence } => wit_lifetime::Receipt::Captured(wit_lifetime::CapturedReceipt { lifetime: kernel_lifetime_to_wit(lifetime), request_sequence }),
            R::Accepted { lifetime, request_sequence, close_generation } => wit_lifetime::Receipt::Accepted(wit_lifetime::CloseReceipt { lifetime: kernel_lifetime_to_wit(lifetime), request_sequence, close_generation }),
            R::Retired { lifetime, request_sequence, close_generation } => wit_lifetime::Receipt::Retired(wit_lifetime::CloseReceipt { lifetime: kernel_lifetime_to_wit(lifetime), request_sequence, close_generation }),
        }
    }

    fn wit_event_to_kernel(event: crate::component::wasip2::exports::semio::framework::reactor::Event) -> Event {
        use crate::component::wasip2::exports::semio::framework::reactor::Event as W;
        match event {
            W::InstanceOpen(payload) => Event::InstanceOpen {
                request: semio_framework::kernel::ActorInstanceOpenRequest { activation_generation: payload.activation_generation, instance_id: payload.instance, request_sequence: payload.request_sequence },
                app_id: semio_framework::kernel::AppInstanceId(payload.app_id),
                actor: payload.actor,
                config: payload.config,
                assets: payload.assets,
                capabilities: Vec::new(),
                // 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): previously always
                // `default()` — `spawn_task`'s quota gate is the first real reader of this field, so a
                // decode failure (malformed/empty pack) also falls back to `default()` rather than
                // failing `InstanceOpen` outright; a missing quota is "no limit declared", not a fault.
                quotas: decode_wire_quotas(&payload.quotas),
            },
            W::InstanceClose(payload) => Event::InstanceClose(semio_framework::kernel::ActorInstanceCloseRequest { lifetime: wit_lifetime_to_kernel(payload.lifetime), request_sequence: payload.request_sequence }),
            W::InstanceLifecycleAck(receipt) => Event::InstanceLifecycleAck(semio_framework::kernel::ActorInstanceLifecycleAck { receipt: wit_lifecycle_receipt_to_kernel(receipt) }),
            W::Activate(payload) => Event::Activate { reason: wit_activation_to_kernel(payload.reason) },
            W::SuspendRequest(_) => Event::SuspendRequest,
            W::CapabilityChanged(_) => Event::SuspendRequest,
            W::QuotaChanged(_) => Event::SuspendRequest,
            // 🎬️ `wit-flip` (26/08/20): UI intents no longer masquerade as `app-command` — see kernel
            // `Event::UiIntent`'s own doc.
            W::UiIntent(payload) => Event::UiIntent { instance: semio_framework::kernel::PluginInstanceId(payload.instance.to_string()), intent: payload.intent },
            W::SurfaceVisible(payload) => Event::SurfaceVisible { surface: format!("{}:{}", payload.surface.instance, payload.surface.surface), body_key: payload.body_key, view_state: payload.view_state },
            W::SurfaceHidden(payload) => Event::SurfaceHidden { surface: format!("{}:{}", payload.surface.instance, payload.surface.surface) },
            W::SurfaceResized(payload) => Event::SurfaceResized { surface: format!("{}:{}", payload.surface.instance, payload.surface.surface), width: payload.width, height: payload.height },
            W::PatchAck(payload) => Event::PatchAck { receipt: wit_patch_receipt_to_kernel(payload.receipt), surface: format!("{}:{}", payload.surface.instance, payload.surface.surface), revision: payload.revision },
            W::PatchRejected(payload) => Event::PatchRejected { receipt: wit_patch_receipt_to_kernel(payload.receipt), surface: format!("{}:{}", payload.surface.instance, payload.surface.surface), revision: payload.revision, reason: payload.reason },
            W::Completed(payload) => Event::Completed { req: semio_framework::kernel::RequestId(payload.req), result: wit_completion_to_kernel(payload.outcome) },
            W::HttpChunk(payload) => Event::HttpChunk { req: semio_framework::kernel::RequestId(payload.req), bytes: payload.params.bytes, done: payload.params.done },
            W::JobProgress(payload) => Event::JobProgress { job: payload.job, progress: Some(payload.progress) },
            W::JobCompleted(payload) => Event::JobCompleted { job: payload.job, result: wit_completion_to_kernel(payload.outcome) },
            W::Message(payload) => Event::Message { source: wit_endpoint_to_kernel(payload.source), payload: payload.payload },
            W::Timer(payload) => Event::Timer { id: payload.id },
            W::Wake => Event::Wake,
            W::Request(payload) => Event::Request { req: semio_framework::kernel::RequestId(payload.req), from: wit_endpoint_to_kernel(payload.params.origin), capability: payload.params.capability, payload: payload.params.payload },
        }
    }

    fn wit_activation_to_kernel(reason: wit_events::ActivationEvent) -> semio_framework::kernel::ActivationEvent {
        use wit_events::ActivationEvent as W;
        match reason {
            W::OnCommand(id) => semio_framework::kernel::ActivationEvent::OnCommand { id },
            W::OnViewVisible(id) => semio_framework::kernel::ActivationEvent::OnViewVisible { id },
            W::OnFileType(ext) => semio_framework::kernel::ActivationEvent::OnFileType { ext },
            W::OnArtifactKind(kind) => semio_framework::kernel::ActivationEvent::OnArtifactKind { kind },
            W::OnExtensionRequest(point) => semio_framework::kernel::ActivationEvent::OnExtensionRequest { point },
            W::OnStartupFinished => semio_framework::kernel::ActivationEvent::OnStartupFinished,
        }
    }

    fn wit_completion_to_kernel(result: wit_events::CompletionResult) -> RequestOutcome {
        use wit_events::CompletionResult as W;
        match result {
            W::Ok(bytes) => RequestOutcome::Ok(bytes),
            W::Fault(bytes) => RequestOutcome::Err(bytes),
        }
    }

    fn wit_endpoint_to_kernel(endpoint: wit_types::MessageEndpoint) -> MessageEndpoint {
        use wit_types::MessageEndpoint as W;
        match endpoint {
            W::Shell(instance) => MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) },
            W::Backbone(uri) => MessageEndpoint::Backbone { uri },
            W::PluginInstance(instance) => MessageEndpoint::PluginInstance { id: semio_framework::kernel::PluginInstanceId(instance.to_string()) },
            W::Extension(id) => MessageEndpoint::Extension { id },
            W::Topic(name) => MessageEndpoint::Topic { name },
        }
    }

    /// 🔀️ kernel `TurnResult` → WIT `turn-result`. `budget` is currently unused beyond documenting
    /// the seam — `max-effects`/`max-patch-bytes` capping is real, mechanical follow-up work (design-
    /// abi.md §4's "capped by `max-effects`, overflow carries over") not yet wired into this wave.
    ///
    /// 👥️ M2 (ticket 26/08/17 `design-unified.md`): `presence` is now real — `kernel::TurnResult`
    /// gained the matching field this doc used to say it lacked (see `poll`'s own body for the
    /// `PresenceHub` that fills it). Marshaled through the SAME `presence-update.peer: pack` WIT field
    /// the schema already declares — that field is opaque `pack` bytes at the WIT boundary already, so
    /// no STRUCTURAL wit change is needed to repoint what it carries: `wit-flip`'s doc comment there
    /// still names the OLD replication `PresencePeer` payload; this packet's report carries the exact
    /// (doc-comment-only, plus a field rename for clarity) WIT diff as a registrar lease-request.
    fn kernel_turn_result_to_wit(
        result: &semio_framework::kernel::TurnResult,
        _budget: crate::component::wasip2::exports::semio::framework::reactor::Budget,
    ) -> Result<crate::component::wasip2::exports::semio::framework::reactor::TurnResult, semio_framework::Fault> {
        use crate::component::wasip2::exports::semio::framework::reactor as wit;
        Ok(wit::TurnResult {
            ui_patches: result.ui_patches.iter().map(kernel_ui_patch_to_wit).collect(),
            effects: result.effects.iter().cloned().map(kernel_effect_to_wit).collect::<Result<Vec<_>, _>>()?,
            presence: result.presence.iter().map(kernel_presence_update_to_wit).collect(),
            next_wake: result.next_wake,
            status: match &result.status {
                TurnStatus::Idle => wit::TurnStatus::Idle,
                TurnStatus::MoreWork => wit::TurnStatus::MoreWork,
                TurnStatus::CheckpointReady { checkpoint } => wit::TurnStatus::CheckpointReady(wit::JobCheckpoint { state: checkpoint.state.clone(), applied_progress: checkpoint.applied_progress }),
                TurnStatus::Faulted(bytes) => wit::TurnStatus::Faulted(bytes.clone()),
            },
            fuel_used: result.fuel_used,
            command_ingress: kernel_command_ingress_to_wit(result.command_ingress.clone()),
            cold_pair_ingress: kernel_cold_pair_ingress_to_wit(result.cold_pair_ingress.clone()),
            lifecycle_receipt: result.lifecycle_receipt.map(kernel_lifecycle_receipt_to_wit),
            ui_patch_receipt: result.ui_patch_receipt.map(kernel_patch_receipt_to_wit),
        })
    }

    /// 👥️ M2: pack-encodes a whole `ui_contract::PresenceUpdate` into the WIT `presence-update.update`
    /// field — same `pack_patch_field` helper every `patch-op` variant already uses, since a
    /// render-plane presence update is exactly as opaque to the WIT boundary as a patch op's payload.
    fn kernel_presence_update_to_wit(update: &ui_contract::PresenceUpdate) -> crate::component::wasip2::exports::semio::framework::reactor::PresenceUpdate {
        use crate::component::wasip2::exports::semio::framework::reactor as wit;
        wit::PresenceUpdate { update: pack_patch_field(&update) }
    }

    fn kernel_ui_patch_to_wit(patch: &UiPatch) -> crate::component::wasip2::exports::semio::framework::reactor::UiPatch {
        use crate::component::wasip2::exports::semio::framework::reactor as wit;
        let (instance, surface) = patch.surface.0.split_once(':').unwrap_or(("0", patch.surface.0.as_str()));
        wit::UiPatch {
            surface: wit_ui::SurfaceRef { instance: instance.parse().unwrap_or(0), surface: surface.to_owned() },
            revision: patch.revision.0,
            base_revision: patch.base_revision.0,
            ops: patch.ops.iter().map(kernel_patch_op_to_wit).collect(),
        }
    }

    /// 🩹️ Packs any `Serialize` payload the same way `kernel_effect_to_wit`'s own `pack` helper does —
    /// shared here rather than duplicated because every `patch-op` variant but `Remove`/`SetRoot` carries
    /// exactly one `pack`-encoded field.
    // 🚫️async: E5 executor bridge — `store::pack_rt::encode_wire_value` is genuinely `async fn` (out of
    // this packet's `path_scope`, `🏪️store/**`); safe to resolve synchronously here for the same "world
    // actor has no host-async import" reason as this file's other WIT-boundary bridges.
    fn pack_patch_field<T: serde::Serialize>(value: &T) -> Vec<u8> {
        let value = serde_json::to_value(value).map_or(dsl::DslValue::Null, |json| dsl::DslValue::from(&json));
        store::pack_rt::encode_wire_value(&value)
    }

    /// 🩹️ Wire payload for `patch-set-activity`'s `activity: pack` field. `component.wit`'s
    /// `patch-set-activity` (`wit-flip`, 26/08/20) carries only `activity: pack`, with no sibling
    /// `disabled: bool`, while the contract crate's own `UiPatchOp::SetActivity` carries `disabled` as a
    /// peer field (`🧬️contract/🦀️document.rs:148`) — a real WIT/contract mismatch, flagged rather than
    /// silently resolved by `📓️terra-wit-flip-report.md`'s decisions section, and resolved HERE (the
    /// first packet to actually encode this op) by folding `disabled` into the packed payload instead of
    /// dropping it. A future packet editing `component.wit` may add a `disabled: bool` field there
    /// instead, at which point this wrapper — and its symmetric decode on the host side — goes away.
    /// 🌉️ `serde::Serialize` only — no `ToValue` derive: `pack_patch_field` bridges through
    /// `serde_json` now (`ui_contract::Activity` has no `ToValue` of its own), so this payload
    /// never needs one either.
    #[derive(serde::Serialize)]
    struct ActivityPatchPayload<'a> {
        activity: &'a ui_contract::Activity,
        disabled: bool,
    }

    fn kernel_patch_op_to_wit(op: &UiPatchOp) -> wit_ui::PatchOp {
        match op {
            UiPatchOp::Upsert(record) => wit_ui::PatchOp::Upsert(wit_ui::PatchUpsert { node: pack_patch_field(&record) }),
            UiPatchOp::SetComponent { id, component } => wit_ui::PatchOp::SetComponent(wit_ui::PatchSetComponent { node: id.0, component: pack_patch_field(&component) }),
            UiPatchOp::SetLayout { id, layout } => wit_ui::PatchOp::SetLayout(wit_ui::PatchSetLayout { node: id.0, layout: pack_patch_field(&layout) }),
            UiPatchOp::SetActivity { id, activity, disabled } => wit_ui::PatchOp::SetActivity(wit_ui::PatchSetActivity { node: id.0, activity: pack_patch_field(&ActivityPatchPayload { activity, disabled: *disabled }) }),
            UiPatchOp::SetChildren { id, children } => wit_ui::PatchOp::SetChildren(wit_ui::PatchSetChildren { node: id.0, children: children.iter().map(|child| child.0).collect() }),
            UiPatchOp::SetStyle { id, style } => wit_ui::PatchOp::SetStyle(wit_ui::PatchSetStyle { node: id.0, style: pack_patch_field(&style) }),
            UiPatchOp::SetAccessibility { id, accessibility } => wit_ui::PatchOp::SetAccessibility(wit_ui::PatchSetAccessibility { node: id.0, accessibility: pack_patch_field(&accessibility) }),
            UiPatchOp::SetBindings { id, bindings } => wit_ui::PatchOp::SetBindings(wit_ui::PatchSetBindings { node: id.0, bindings: pack_patch_field(&bindings) }),
            UiPatchOp::SetMenu { id, menu } => wit_ui::PatchOp::SetMenu(wit_ui::PatchSetMenu { node: id.0, menu: pack_patch_field(&menu) }),
            UiPatchOp::Remove { id } => wit_ui::PatchOp::Remove(id.0),
            UiPatchOp::SetRoot { id } => wit_ui::PatchOp::SetRoot(id.0),
        }
    }

    /// 🔀️ kernel `Effect` → WIT `effect`. Field-for-field per `📓️design-abi.md` §2's table; complex
    /// Rust-only field types (`WindowKindId`, `DslValue`, `MediaType`, `ClipboardFragment`, ...) are
    /// wire-encoded through the SAME `store::pack_rt::encode_wire_value`/`dsl::to_dsl_value` idiom
    /// every existing host boundary in this crate already uses.
    fn kernel_effect_to_wit(effect: Effect) -> Result<crate::component::wasip2::exports::semio::framework::reactor::Effect, semio_framework::Fault> {
        use crate::component::wasip2::exports::semio::framework::reactor as wit;
        // 🚫️async: E5 executor bridge — `store::pack_rt::encode_wire_value` is genuinely `async fn`
        // (out of this packet's `path_scope`, `🏪️store/**`), but every caller in this match below is
        // itself sync (R9: `kernel_effect_to_wit`'s only consumer is the WIT-fixed sync `world actor`
        // boundary, no suspension point of its own) — `resolve_ready` is safe here because `world
        // actor` imports no `host-async`, so this store call never has anything real to suspend on.
        fn pack<T: serde::Serialize>(value: &T) -> Vec<u8> {
            let value = serde_json::to_value(value).map_or(dsl::DslValue::Null, |json| dsl::DslValue::from(&json));
            store::pack_rt::encode_wire_value(&value)
        }
        Ok(match effect {
            Effect::OpenWindow { req, kind, params } => wit::Effect::OpenWindow(wit_effects::OpenWindowEffect { req: req.0, params: wit_effects::OpenWindowParams { kind: kind.0, params: pack(&params) } }),
            Effect::CloseWindow { window } => wit::Effect::CloseWindow(wit_effects::CloseWindowEffect { window: window.0 as u64 }),
            Effect::Notify { message } => wit::Effect::Notify(wit_effects::NotifyEffect { message }),
            Effect::ClipboardWrite { fragment } => wit::Effect::ClipboardWrite(wit_effects::ClipboardWriteEffect { fragment: pack(&fragment) }),
            Effect::RequestSync => wit::Effect::RequestSync,
            Effect::Navigate { uri } => wit::Effect::Navigate(wit_effects::NavigateEffect { uri }),
            Effect::LoadDocument { pack: doc_pack, spr } => wit::Effect::LoadDocument(wit_effects::LoadDocumentEffect { doc_pack, spr }),
            Effect::OpenExternalUrl { url } => wit::Effect::OpenExternalUrl(wit_effects::OpenExternalUrlEffect { url }),
            Effect::SetPanel { panel_json } => wit::Effect::SetPanel(wit_effects::SetPanelEffect { panel_json }),
            Effect::DownloadMediaExport { filename, mime_type, data, encoding } => wit::Effect::DownloadMediaExport(wit_effects::DownloadMediaExportEffect { filename, mime_type, data, encoding }),
            Effect::IconRenderExport { items } => wit::Effect::IconRenderExport(wit_effects::IconRenderExportEffect { items: pack(&items) }),
            Effect::RequestFileOpen { req, accept, read_as, import_action, multiple } => {
                wit::Effect::RequestFileOpen(wit_effects::RequestFileOpenEffect { req: req.0, params: wit_effects::RequestFileOpenParams { accept, read_as, multiple, import_action } })
            }
            Effect::RequestMediaFrames { req, accept, frame_action, done_action, fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload, args } => wit::Effect::RequestMediaFrames(wit_effects::RequestMediaFramesEffect {
                req: req.0,
                params: wit_effects::RequestMediaFramesParams { accept, frame_action, done_action, fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload, args: args.map(|value| pack(&value)) },
            }),
            Effect::SpawnPluginInstance { req, plugin_id, app_id, os_instance_id, label, document_json } => {
                wit::Effect::SpawnPluginInstance(wit_effects::SpawnPluginInstanceEffect { req: req.0, params: wit_effects::SpawnPluginInstanceParams { plugin_id, app_id, os_instance_id, label, document_json } })
            }
            Effect::OpenPluginInstance { plugin_id, app_id, os_instance_id } => wit::Effect::OpenPluginInstance(wit_effects::OpenPluginInstanceEffect { plugin_id, app_id, os_instance_id }),
            Effect::SetActiveUtility { window_id, utility_id } => wit::Effect::SetActiveUtility(wit_effects::SetActiveUtilityEffect { window_id, utility_id }),
            Effect::SetActiveTool { tool_id } => wit::Effect::SetActiveTool(wit_effects::SetActiveToolEffect { tool_id }),
            Effect::OpenDialog { req, dialog_id, args } => wit::Effect::OpenDialog(wit_effects::OpenDialogEffect { req: req.0, params: wit_effects::OpenDialogParams { dialog_id, args: args.map(|value| pack(&value)) } }),
            Effect::DispatchAction { req, action, args, delay_ms } => wit::Effect::DispatchAction(wit_effects::DispatchActionEffect { req: req.0, params: wit_effects::DispatchActionParams { action, args: args.map(|value| pack(&value)), delay_ms } }),
            Effect::ReplayShellCommand { action_id, args } => wit::Effect::ReplayShellCommand(wit_effects::ReplayShellCommandEffect { action_id, args: args.map(|value| pack(&value)) }),
            Effect::InvokeExtension { req, extension_id, capability, request_json, .. } => {
                wit::Effect::InvokeExtension(wit_effects::InvokeExtensionEffect { req: req.0, params: wit_effects::InvokeExtensionParams { extension_id, capability, payload: request_json.into_bytes() } })
            }
            Effect::SendMessage { target, payload } => wit::Effect::SendMessage(wit_effects::SendMessageEffect { target: kernel_endpoint_to_wit(target), payload }),
            Effect::PublishEvent { topic, payload } => wit::Effect::PublishEvent(wit_effects::PublishEventEffect { topic, payload }),
            Effect::BlobWrite { req, media_type, bytes } => wit::Effect::BlobWrite(wit_effects::BlobWriteEffect { req: req.0, params: wit_effects::BlobWriteParams { media_type: pack(&media_type), bytes } }),
            Effect::BlobLoad { req, hash } => wit::Effect::BlobLoad(wit_effects::BlobLoadEffect { req: req.0, params: wit_effects::BlobLoadParams { hash } }),
            Effect::HttpRequest { req, method, url, headers, body, stream } => wit::Effect::HttpRequest(wit_effects::HttpRequestEffect { req: req.0, params: wit_effects::HttpParams { method, url, headers, body, streaming: stream } }),
            Effect::DocumentRead { req, doc, lane } => wit::Effect::DocumentRead(wit_effects::DocumentReadEffect { req: req.0, params: wit_effects::DocumentReadParams { doc: doc.0 as u64, lane } }),
            Effect::DocumentWrite { req, doc, lane, ops } => wit::Effect::DocumentWrite(wit_effects::DocumentWriteEffect { req: req.0, params: wit_effects::DocumentWriteParams { doc: doc.0 as u64, lane, ops } }),
            Effect::LinkResolve { req, link } => wit::Effect::LinkResolve(wit_effects::LinkResolveEffect { req: req.0, link: link.into_bytes() }),
            Effect::RegistryQuery { req, kind, filter } => wit::Effect::RegistryQuery(wit_effects::RegistryQueryEffect { req: req.0, params: wit_effects::RegistryQueryParams { kind, filter: filter.map(|value| pack(&value)).unwrap_or_default() } }),
            Effect::IoCompose { req, key, sources } => wit::Effect::IoCompose(wit_effects::IoComposeEffect { req: req.0, params: wit_effects::IoComposeParams { key: key.into_bytes(), sources: pack(&sources) } }),
            Effect::CacheDerive { req, engine_id, input } => wit::Effect::CacheDerive(wit_effects::CacheDeriveEffect { req: req.0, params: wit_effects::CacheDeriveParams { engine_id, input } }),
            Effect::CacheRead { req, engine_id, key } => wit::Effect::CacheRead(wit_effects::CacheReadEffect { req: req.0, params: wit_effects::CacheReadParams { engine_id, key: key.into_bytes() } }),
            Effect::SetTimer { id, after_ms, repeat } => {
                if !ARMED_TIMERS.with(|timers| timers.borrow().contains(id)) {
                    return Err(semio_framework::Fault::new(
                        semio_framework::FaultOrigin::Framework,
                        semio_framework::FaultCode::new("plugin.timer-owner-missing"),
                        "timer effect reached the WIT boundary without exact pre-admitted instance ownership",
                    ));
                }
                wit::Effect::SetTimer(wit_effects::SetTimerEffect { id, after_ms: after_ms as u32, repeat })
            }
            Effect::SpawnJob { job, kind, input, placement } => wit::Effect::SpawnJob(wit_effects::SpawnJobEffect { job, kind, input, placement: kernel_placement_to_wit(placement) }),
            Effect::CancelJob { job } => wit::Effect::CancelJob(wit_effects::CancelJobEffect { job }),
            Effect::Respond { req, result } => wit::Effect::Respond(wit_effects::RespondEffect { req: req.0, outcome: kernel_outcome_to_wit_respond(result) }),
            Effect::StorageRead { req, key } => wit::Effect::StorageRead(wit_effects::StorageReadEffect { req: req.0, params: wit_effects::StorageReadParams { key } }),
            Effect::StorageWrite { req, key, bytes } => wit::Effect::StorageWrite(wit_effects::StorageWriteEffect { req: req.0, params: wit_effects::StorageWriteParams { key, value: bytes } }),
            Effect::StorageDelete { req, key } => wit::Effect::StorageDelete(wit_effects::StorageDeleteEffect { req: req.0, params: wit_effects::StorageDeleteParams { key } }),
            Effect::RequestCapability { req, capability } => wit::Effect::RequestCapability(wit_effects::RequestCapabilityEffect {
                req: req.0,
                params: wit_effects::RequestCapabilityParams { id: capability.id.0, scope: capability.scope, reason: capability.reason, optional: capability.optional },
            }),
            Effect::ReleaseCapability { id } => wit::Effect::ReleaseCapability(wit_effects::ReleaseCapabilityEffect { id: id.0 }),
            Effect::Subscribe { topic } => wit::Effect::Subscribe(wit_effects::SubscribeEffect { topic }),
            Effect::Unsubscribe { topic } => wit::Effect::Unsubscribe(wit_effects::SubscribeEffect { topic }),
            Effect::RequestInferenceProposal { kind } => wit::Effect::RequestInferenceProposal(wit_effects::RequestInferenceProposalEffect { kind: kernel_inference_proposal_kind_to_wit(kind) }),
        })
    }

    fn kernel_endpoint_to_wit(endpoint: MessageEndpoint) -> wit_types::MessageEndpoint {
        match endpoint {
            MessageEndpoint::Shell { instance } => wit_types::MessageEndpoint::Shell(instance.0.parse().unwrap_or(0)),
            MessageEndpoint::Backbone { uri } => wit_types::MessageEndpoint::Backbone(uri),
            MessageEndpoint::PluginInstance { id } => wit_types::MessageEndpoint::PluginInstance(id.0.parse().unwrap_or(0)),
            MessageEndpoint::Extension { id } => wit_types::MessageEndpoint::Extension(id),
            MessageEndpoint::Topic { name } => wit_types::MessageEndpoint::Topic(name),
        }
    }

    /// 💡️ kernel `InferenceProposalKind` → WIT — one closed intent, no transport detail.
    fn kernel_inference_proposal_kind_to_wit(kind: semio_framework::kernel::InferenceProposalKind) -> wit_effects::InferenceProposalKind {
        match kind {
            semio_framework::kernel::InferenceProposalKind::GisMapBoundsRegion => wit_effects::InferenceProposalKind::GisMapBoundsRegion,
        }
    }

    fn kernel_placement_to_wit(placement: semio_framework::kernel::JobPlacement) -> wit_effects::JobPlacement {
        match placement {
            semio_framework::kernel::JobPlacement::Inline => wit_effects::JobPlacement::Inline,
            semio_framework::kernel::JobPlacement::Isolated => wit_effects::JobPlacement::Isolated,
            semio_framework::kernel::JobPlacement::Exclusive => wit_effects::JobPlacement::Exclusive,
        }
    }

    fn kernel_outcome_to_wit_respond(result: RequestOutcome) -> wit_effects::RespondResult {
        match result {
            RequestOutcome::Ok(bytes) => wit_effects::RespondResult::Ok(bytes),
            RequestOutcome::Err(bytes) => wit_effects::RespondResult::Fault(bytes),
        }
    }
}

/// 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: test-only hooks into this actor's UNGATED
/// per-actor state (`EXECUTOR`/`REGISTRY`/`TASK_RESUMES`) — `poll`'s real event routing
/// (`Event::Completed` → `REGISTRY::resolve`, `run_until_idle`, `drain_task_resumes`) lives inside
/// `wit_bridge`, which is wasm32-wasip2-only and cannot be exercised by a native `cargo test`. This
/// module gives native tests (this crate's own, and `plugin_runtime`'s `test_push_instance`-backed
/// integration tests) the SAME three primitives `poll` itself drives, so the mechanism — spawn,
/// park, get resolved by an injected completion, resume — is exercised end-to-end without needing
/// a wasm32-wasip2 build. `pub(crate)`, `#[cfg(test)]`-gated: never part of the real API surface.
#[cfg(test)]
#[path = "🧪️tests/🔬️test-support/🦀️.rs"]
pub(crate) mod test_support;

//#region 🧪️M1M2ReactorTests
/// 🎯️👥️ M1/M2 (ticket 26/08/17 `design-unified.md`) acceptance, driven through `test_support`'s
/// direct hooks into `PATCHES`/`PRESENCE` — the same two thread-locals `poll`'s real intent-batching
/// and dirty-render loops touch, exercised without needing a wasm32-wasip2 build (`poll` itself,
/// gated to `wit_bridge`, cannot run under a native `cargo test` — see `test_support`'s own doc).
#[cfg(test)]
#[path = "🧪️tests/🔬️m1-m2-reactor/🦀️.rs"]
mod m1_m2_reactor_tests;
//#endregion 🧪️M1M2ReactorTests

#[cfg(test)]
#[path = "🧪️tests/🔬️reconcile-budget/🦀️.rs"]
mod reconcile_budget_tests;

//#region 🧪️ExtensionContinuationTests
#[cfg(test)]
#[path = "🧪️tests/🔬️extension-continuation/🦀️.rs"]
mod extension_continuation_tests;
//#endregion 🧪️ExtensionContinuationTests

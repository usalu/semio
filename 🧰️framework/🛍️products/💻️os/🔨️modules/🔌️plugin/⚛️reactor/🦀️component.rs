//! ⚛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2, design-abi.md §4): the turn loop —
//! `reactor::poll`'s real implementation. Ties together `🧵️executor` (async task scheduling),
//! `📮️requests` (the host-effect request/completion registry), `🩹️patches` (revisioned UI diffing),
//! `💼️jobs` (the absorbed `semio.io-run`/`semio.io-sniff` cold job kinds), `📸️checkpoint`, and
//! `🌐host` (the async `host::*` API surface plugin/extension code awaits).
//!
//! Converts between the WIT-generated `semio::framework::{effects,events,ui,reactor}::*` types
//! (crossing the component boundary) and the Rust SSOT `semio_framework::kernel::{Effect, Event,
//! UiPatch, PatchOp, TurnResult, TurnStatus, Budget}` (packet A3, landed in
//! `🎠️kernel/🦀️component.rs` while this packet was in flight). `app-command` events route through
//! the EXISTING `plugin_runtime::plugin_exchange` dispatcher unchanged (design-abi.md §4) — this
//! module never reimplements command dispatch, only translates its `AppFrame` output into
//! `Effect`/`UiPatch`.

#[path = "📸️checkpoint/🦀️component.rs"]
pub mod checkpoint;
#[path = "🧵️executor/🦀️component.rs"]
pub mod executor;
#[path = "💼️jobs/🦀️component.rs"]
pub mod jobs;
#[path = "🩹️patches/🦀️component.rs"]
pub mod patches;
#[path = "📮️requests/🦀️component.rs"]
pub mod requests;

// 🧬️ Only `wit_bridge` below (component-guest/-extension-guest wasm32-wasip2) consumes these —
// a plain native build never reaches the WIT-boundary translation code, so unlike `RefCell` these
// two must be gated identically to `wit_bridge` itself or they warn as unused on native.
#[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]
use semio_framework::kernel::{Effect, Event, MessageEndpoint, RequestOutcome, TurnStatus, UiPatch, UiPatchOp};
// 🧬️ Same gating rationale as the `kernel` import above: only the WIT-boundary code below names the
// semantic-UI contract types (`UiIntent`, `UiRevision`, `Activity`), so an ungated alias warns as
// unused on native. ALSO enabled under `cfg(test)` (M2, ticket 26/08/17 `design-unified.md`): the
// native `test_support` module below (behind its own `#[cfg(test)]`) exercises `PATCHES`/`PRESENCE`
// directly with real `ui_contract` values — `wit_bridge` still cannot run under `cargo test`
// (wasm32-wasip2-only), but its own type vocabulary can be reused for a native fixture.
#[cfg(any(test, all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2")))]
use semio_framework_ui_contract as ui_contract;
// 🧬️ Same gating rationale as `ui_contract` above: `is_stale_intent`/`DEFAULT_REVISION_TOLERANCE`
// are only named inside `wit_bridge::poll`'s intent-batching loop (M1, ticket 26/08/17
// `design-unified.md`) — gated identically to that module so an ungated alias never warns as unused
// on native.
/// 👥️ M2 (ticket 26/08/17 `design-unified.md`): standalone — needs no `EntityStore`/`UiRuntime`, just
/// `record_own`/`record_peer`/`expire`/`flush` — so unlike the two imports directly above, this one
/// is ungated: the `PRESENCE` thread_local right below references it on EVERY build, matching how
/// `patches::PatchTracker` (an equally wit_bridge-only consumer) is reached through the ungated
/// `pub mod patches;` at this file's top.
use semio_framework_ui_runtime::PresenceHub;
#[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]
use semio_framework_ui_runtime::{is_stale_intent, SurfaceReconcilePublishedPatch, SurfaceReconcileReadyPatch, DEFAULT_REVISION_TOLERANCE};
use std::cell::{Cell, RefCell};
// 🧵️ Turn-local command/intent grouping and the pre-admitted task-resume ring use these
// collections; all identity and close authority is held by fixed direct registries below.
use std::collections::{HashMap, VecDeque};

crate::component_persistent_local! {
    /// 🩹️ One `PatchTracker` shared by every instance this actor hosts (surfaces are already
    /// namespaced by their own `surface` string, which today embeds the instance — see
    /// `render_surface`'s key).
    // 🌉️ `thread_local!` initializer expressions run in a plain (non-const, non-async) context —
    // bridged via `resolve_ready` since every `::new()` here is a pure `Self::default()`.
    static PATCHES: patches::PatchTracker = patches::PatchTracker::new();
    /// 👥️ M2 (ticket 26/08/17 `design-unified.md`): one `PresenceHub` shared by every instance this
    /// actor hosts, beside `PATCHES` — same "surfaces are already namespaced by instance" reasoning
    /// (a `PresenceHub` entry is keyed `(surface, node_key)`, and `surface` already embeds the
    /// instance via `plugin_take_presence`'s `"<instance>:<body-key>"` stamping). Fed from each dirty
    /// render's `plugin_take_presence(instance)` drain, expired and flushed once per `poll` — see
    /// `poll`'s own body for both halves.
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
}

/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): one `AsyncTask` this actor's
/// `LocalExecutor` currently owns, keyed by its `executor::TaskId` slot. `label` is diagnostic-only
/// today (no telemetry/checkpoint surface reads it back yet — an honest, named gap, not a silent
/// unused field: kept because a task with no human-readable name is a debugging dead end the
/// moment more than one is ever live on an instance).
struct TaskRecord {
    instance: u32,
    key: Option<String>,
    #[allow(dead_code)]
    label: String,
    restart: Option<Vec<u8>>,
}

const REACTOR_TASK_SLOTS: usize = 1_024;
const REACTOR_FIXED_WORDS: usize = REACTOR_TASK_SLOTS / u64::BITS as usize;
const REACTOR_TASK_KEY_BYTES: usize = 256;
const REACTOR_TASK_LABEL_BYTES: usize = 256;
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
mod job_render_binding_tests {
    use super::*;

    #[test]
    fn progress_accepts_only_the_current_instance_generation() {
        let mut registry = JobRenderBindingRegistry::new();
        let first = registry.bind(7, 41).expect("first binding");
        assert_eq!(registry.accepted(41), Some(first));
        let second = registry.bind(7, 42).expect("superseding binding");
        assert_ne!(first.generation, second.generation);
        assert_eq!(registry.accepted(41), None);
        assert_eq!(registry.accepted(42), Some(second));
        assert_eq!(registry.complete(41), None);
        assert_eq!(registry.complete(42), Some(second));
    }

    #[test]
    fn direct_slots_reject_collisions_and_close_exactly_one_instance() {
        let mut registry = JobRenderBindingRegistry::new();
        let binding = registry.bind(3, 9).expect("binding");
        assert!(registry.bind(3 + REACTOR_TASK_SLOTS as u32, 10).is_err());
        assert!(registry.bind(4, 9 + REACTOR_TASK_SLOTS as u64).is_err());
        registry.close_instance(3);
        assert_eq!(registry.accepted(binding.job), None);
    }
}

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

    fn index(id: executor::TaskId) -> usize {
        id as usize % REACTOR_TASK_SLOTS
    }

    fn can_insert(&self, id: executor::TaskId) -> bool {
        self.slots.allocation_admitted && self.slots.get(Self::index(id)).is_none()
    }

    fn insert(&mut self, id: executor::TaskId, record: TaskRecord) -> Result<(), TaskRecord> {
        if !self.can_insert(id) {
            return Err(record);
        }
        self.slots.insert(Self::index(id), (id, record)).map_err(|(_, record)| record)
    }

    fn insert_admitted(&mut self, id: executor::TaskId, record: TaskRecord) {
        debug_assert!(self.can_insert(id));
        self.slots.insert_admitted(Self::index(id), (id, record));
    }

    fn remove(&mut self, id: executor::TaskId) -> Option<TaskRecord> {
        let index = Self::index(id);
        if self.slots.get(index).is_none_or(|(candidate, _)| *candidate != id) {
            None
        } else {
            self.slots.take(index).map(|(_, record)| record)
        }
    }

    fn find_key(&self, instance: u32, key: &str) -> Option<executor::TaskId> {
        self.slots.iter().find_map(|(id, record)| (record.instance == instance && record.key.as_deref() == Some(key)).then_some(*id))
    }

    fn count_instance(&self, instance: u32) -> usize {
        self.slots.iter().filter(|(_, record)| record.instance == instance).count()
    }

    fn iter(&self) -> impl Iterator<Item = (executor::TaskId, &TaskRecord)> {
        self.slots.iter().map(|(id, record)| (*id, record))
    }

    fn entry_at(&self, index: usize) -> Option<(executor::TaskId, &TaskRecord)> {
        self.slots.get(index).map(|(id, record)| (*id, record))
    }
}

struct ReactorCloseState {
    instance: u32,
    task_cursor: usize,
    timer_cursor: usize,
    request_cursor: requests::RequestCloseCursor,
    resume_remaining: usize,
    requests_complete: bool,
    resumes_complete: bool,
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

    fn put_at(&mut self, index: usize, state: ReactorCloseState) -> Result<(), ReactorCloseState> {
        self.slots.insert(index, state)
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
    Emit { artifact_ops: Vec<u8>, config_ops: Vec<u8>, draft_ops: Vec<u8> },
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
            TaskResumeOutcome::Emit { artifact_ops, config_ops, draft_ops } => {
                bytes = bytes.checked_add(artifact_ops.len())?.checked_add(config_ops.len())?.checked_add(draft_ops.len())?;
            }
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

/// 🌐️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): the instance-agnostic sibling of
/// `host_for_instance` — a `Host` scoped to instance 0, the SAME "no instance tag declared" default
/// `RequestRegistry::for_instance`'s own doc names. `⚛️reactor/💼️jobs/🦀️component.rs::spawn_job`
/// calls this (as `crate::reactor::host()`, zero args): a job is actor-global, not tied to one open
/// instance the way an `AsyncTask` is, so it has no `instance: u32` to scope by in the first place.
pub async fn host() -> crate::host::Host {
    REGISTRY.with(|registry| crate::host::Host::new(registry.clone())).await
}

/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): spawns `task` onto this actor's
/// shared `LocalExecutor`, quota-gated then key-deduped (in that order — a same-key respawn at
/// exactly the quota limit legitimately fails; the caller may retry once the cancelled slot is
/// actually freed on a later turn). Called from `🔌️plugin/🦀️component.rs`'s `dispatch_emit`, right
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
    TASK_RECORDS.with(|records| records.borrow_mut().insert_admitted(task_id, TaskRecord { instance, key, label, restart }));

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
        return matches!(REACTOR_EXECUTOR.with(|executor| executor.close_instance_step(instance, cursor, budget)), executor::ReactorTaskStep::Complete);
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

fn begin_reactor_close(instance: u32) -> Result<(), semio_framework::Fault> {
    let request_cursor = REGISTRY.with(|registry| registry.begin_cancel_instance(instance));
    let resume_remaining = TASK_RESUMES.with(|resumes| resumes.borrow().begin_cancel_instance());
    let state = ReactorCloseState { instance, task_cursor: 0, timer_cursor: 0, request_cursor, resume_remaining, requests_complete: false, resumes_complete: false, timers_complete: false, metadata_complete: false };
    REACTOR_CLOSES.with(|closes| {
        closes
            .try_borrow_mut()
            .map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.reactor-close-busy"), "reactor close authority is busy"))?
            .insert(state)
            .map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.reactor-close-capacity"), "fixed reactor close authority is saturated or collided"))
    })
}

fn abort_reactor_close(instance: u32) {
    REACTOR_CLOSES.with(|closes| {
        let Ok(mut closes) = closes.try_borrow_mut() else { return };
        let index = ReactorCloseRegistry::index(instance);
        if closes.slots.get(index).is_some_and(|state| state.instance == instance) {
            drop(closes.take_at(index));
        }
    });
}

fn step_reactor_close() -> Result<bool, semio_framework::Fault> {
    let index = REACTOR_CLOSE_CURSOR.with(|cursor| {
        let index = cursor.get();
        cursor.set((index + 1) % PLUGIN_REACTOR_INSTANCE_SLOTS);
        index
    });
    let Some(mut state) = REACTOR_CLOSES.with(|closes| closes.try_borrow_mut().ok().and_then(|mut closes| closes.take_at(index))) else { return Ok(false) };
    let complete = if !state.requests_complete {
        state.requests_complete = REGISTRY.with(|registry| registry.cancel_instance_step(&mut state.request_cursor) == requests::RequestCloseStep::Complete);
        false
    } else if !state.resumes_complete {
        state.resumes_complete = TASK_RESUMES.with(|resumes| resumes.borrow_mut().cancel_instance_step(state.instance, &mut state.resume_remaining));
        false
    } else if state.task_cursor < REACTOR_TASK_SLOTS {
        cancel_instance_tasks_step(state.instance, &mut state.task_cursor);
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
    if !complete {
        REACTOR_CLOSES.with(|closes| {
            closes
                .try_borrow_mut()
                .map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.reactor-close-busy"), "reactor close authority is busy while restoring an exact cleanup cursor"))?
                .put_at(index, state)
                .map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.reactor-close-lost"), "reactor close cursor lost its exact fixed slot"))
        })?;
    }
    Ok(true)
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
        let meta = crate::app::ActionMeta { actor: crate::plugin_runtime::instance_actor(runtime, restart.instance).await, instance_id: restart.instance };
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
pub use wit_bridge::drain_task_resumes;
/// 🧬️ Everything below crosses the wasm component boundary — gated identically to `component`
/// (`🦀️component.rs` at crate root) since it names `crate::component::component::exports::...` types that
/// simply do not exist outside a `component-guest`/`component-extension-guest` wasm32-wasip2
/// build (mirrors the OLD `host_port`'s per-function `#[cfg(...)]` pattern, just hoisted to one
/// module instead of repeated per function).
#[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]
pub use wit_bridge::{poll, poll_kernel};

#[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]
mod wit_bridge {
    use super::*;

    enum CommandIngressOwner {
        ReservedPresence { cursor: semio_framework::kernel::CommandPageCursor, admission: crate::app::PresenceRosterAdmission, page: semio_framework::kernel::FixedCommandPage },
        Presence { cursor: semio_framework::kernel::CommandPageCursor, publication_generation: u64 },
        PendingPresencePage { cursor: semio_framework::kernel::CommandPageCursor, publication_generation: u64, page: semio_framework::kernel::FixedCommandPage },
        GenericAssembly { cursor: semio_framework::kernel::CommandPageCursor, pages: semio_framework::kernel::CommandPageSet },
        ClosingAssembly { cursor: semio_framework::kernel::CommandPageCursor, pages: semio_framework::kernel::CommandPageSet },
        Generic { cursor: semio_framework::kernel::CommandPageCursor, command: crate::plugin_runtime::PluginCommandIngress },
        Terminal { cursor: semio_framework::kernel::CommandPageCursor },
    }

    crate::component_persistent_local! {
        static COMMAND_INGRESS: RefCell<[Option<CommandIngressOwner>; 2]> = RefCell::new([None, None]);
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

        fn try_surface(&mut self, instance: u32, surface: ui_contract::SurfaceId) -> Result<(), ui_contract::SurfaceId> {
            if self.surfaces.iter().any(|queued| queued.0 == instance && queued.1 == surface) {
                return Ok(());
            }
            self.surfaces.try_push((instance, surface)).map_err(|(_, surface)| surface)
        }

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

    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: the raw WIT event, aliased for `poll`'s
    /// `Event::InstanceClose` handling below — the KERNEL `Event::InstanceClose` (SSOT, `🎠️kernel`,
    /// not this packet's file) carries no instance id, so the raw payload must be read BEFORE
    /// `wit_event_to_kernel` erases it.
    use crate::component::component::exports::semio::framework::reactor::Event as WitReactorEvent;
    /// 🧭️ `reactor`/`jobs`/`checkpoint`/`describe` are the only interfaces `world actor` directly
    /// `export`s, so wit-bindgen only aliases THEIR top-level types under `exports::…`. `effects`/
    /// `events`/`ui`/`types` are merely `use`d by `reactor.wit` (design-abi.md §1/§4) — their own
    /// payload records live at the plain (non-`exports::`) path alongside the `pure` import, one
    /// level down from where the nesting stops being re-aliased. Verified empirically: a
    /// deliberately wrong `wit::OpenWindowEffect` import made `cargo check --target wasm32-wasip2
    /// --features component-guest` emit `help: consider importing … effects::OpenWindowEffect`
    /// (and the `events`/`ui` siblings the same way) — not guessed.
    use crate::component::component::semio::framework::effects as wit_effects;
    use crate::component::component::semio::framework::events as wit_events;
    use crate::component::component::semio::framework::types as wit_types;
    use crate::component::component::semio::framework::ui as wit_ui;

    /// ▶️ The real `reactor::poll` body — see module doc for the shape. `events`/`budget` are the
    /// WIT-generated types from `exports::semio::framework::reactor`; the return is that same
    /// module's `TurnResult`.
    pub fn poll<PA: crate::app::PluginApp + 'static>(
        runtime: &crate::plugin_runtime::PluginRuntime<PA>,
        events: Vec<crate::component::component::exports::semio::framework::reactor::Event>,
        command_page: Option<crate::component::component::exports::semio::framework::reactor::CommandIngressPage>,
        budget: crate::component::component::exports::semio::framework::reactor::Budget,
    ) -> Result<crate::component::component::exports::semio::framework::reactor::TurnResult, semio_framework::Fault> {
        let mut close_instances = Vec::new();
        let mut kernel_events = Vec::with_capacity(events.len());
        for event in events {
            if let WitReactorEvent::InstanceClose(ref payload) = event {
                close_instances.push(payload.instance);
            }
            kernel_events.push(wit_event_to_kernel(event));
        }
        let kernel_budget = semio_framework::kernel::Budget { fuel: budget.fuel, deadline_ms: budget.deadline_ms, max_effects: budget.max_effects, max_patch_bytes: budget.max_patch_bytes, max_frames: budget.max_frames };
        let command_page = command_page.map(wit_command_page_to_kernel).transpose()?;
        let result = poll_kernel(runtime, kernel_events, command_page, kernel_budget, &close_instances)?;
        kernel_turn_result_to_wit(result, budget)
    }

    /// 🧠️ Repository-owned actor ABI entrypoint. The component-model wrapper above and the native
    /// interpreter both call this exact kernel reducer, so WIT lifting is no longer the production
    /// host's semantic authority.
    pub fn poll_kernel<PA: crate::app::PluginApp + 'static>(
        runtime: &crate::plugin_runtime::PluginRuntime<PA>,
        events: Vec<Event>,
        command_page: Option<(semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage)>,
        _budget: semio_framework::kernel::Budget,
        close_instances: &[u32],
    ) -> Result<semio_framework::kernel::TurnResult, semio_framework::Fault> {
        let mut dirty = DirtyPollOwners::new();
        // 🎯️ M1 (ticket 26/08/17 `design-unified.md`): intents that survived the revision guard below,
        // batched per instance exactly like `app_commands` — dispatched in its own pass, after
        // `app_commands`, so a mutation from an app command this same turn is already visible.

        for numeric_instance in close_instances {
            JOB_RENDER_BINDINGS.with(|bindings| bindings.borrow_mut().close_instance(*numeric_instance));
            begin_reactor_close(*numeric_instance)?;
            PATCHES.with(|patches| patches.begin_close_instance(*numeric_instance));
            PENDING_PATCHES.with(|pending| pending.borrow_mut().begin_close_instance(*numeric_instance));
            if let Err(error) = semio_framework::io::resolve_ready(crate::plugin_runtime::plugin_destroy_app(runtime, *numeric_instance)) {
                abort_reactor_close(*numeric_instance);
                return Err(error);
            }
        }
        let _ = step_reactor_close()?;
        PATCHES.with(|patches| {
            patches.close_step();
        });
        let _ = semio_framework_ui_runtime::close_surface_reconcile_handback_one();
        let _ = ui_contract::close_ui_document_page_one();
        let _ = ui_contract::close_ui_patch_owner_one();
        let _ = ui_contract::close_ui_value_page_one();
        let _ = semio_framework::kernel::close_ui_turn_patch_owner_one();
        let _ = semio_framework::kernel::close_ui_turn_patch_transport_one();
        let _ = crate::app::close_table_rows_view_one();
        PENDING_PATCHES.with(|pending| {
            pending.borrow_mut().close_step();
        });
        let _ = crate::plugin_runtime::plugin_step_close_cleanup(runtime)?;
        let _ = crate::plugin_runtime::plugin_step_live_cleanup(runtime)?;
        if let Some(surface) = PATCHES.with(patches::PatchTracker::take_deferred_ready) {
            if let Some(instance) = parse_surface_instance(surface.as_ref()) {
                dirty.try_surface(instance, surface).map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.dirty-surface-capacity"), "fixed dirty surface authority is saturated"))?;
            }
        }

        for event in events {
            match event {
                Event::InstanceOpen { instance, app_id, actor, quotas, .. } => {
                    let numeric_instance = instance.0.parse::<u32>().unwrap_or(0);
                    INSTANCE_METADATA.with(|metadata| metadata.borrow_mut().insert(numeric_instance, app_id.0.clone(), quotas))?;
                    // 🚫️async: E5 executor bridge (× 2) — `plugin_create_app_with_id`/`set_instance_actor`
                    // stay genuinely `async fn` (broad `plugin_runtime` consumers elsewhere await them for
                    // real); `resolve_ready` is safe here for the same "world actor has no host-async
                    // import" reason as this file's other WIT-boundary bridges. R13: previously a BARE
                    // dropped future (`let _ = ...`/un-awaited statement) — now genuinely resolved.
                    if let Err(error) = semio_framework::io::resolve_ready(crate::plugin_runtime::plugin_create_app_with_id(runtime, numeric_instance, &app_id.0)) {
                        INSTANCE_METADATA.with(|metadata| drop(metadata.borrow_mut().remove(numeric_instance)));
                        return Err(error);
                    }
                    // 🪪️ Channel v12 (A4) retired the `AppCommand::Hello` handshake that used to record
                    // this — lifecycle now arrives here as `Event::InstanceOpen` (design-abi.md §4).
                    if let Err(error) = semio_framework::io::resolve_ready(crate::plugin_runtime::set_instance_actor(runtime, numeric_instance, actor)) {
                        INSTANCE_METADATA.with(|metadata| drop(metadata.borrow_mut().remove(numeric_instance)));
                        return Err(error);
                    }
                }
                Event::InstanceClose => {}
                Event::CommandIngressPage { .. } => {
                    return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.command-page-event-bypass"), "command page must use poll_kernel's dedicated owner argument"));
                }
                // 🎯️ M1 (ticket 26/08/17 `design-unified.md`): decodes the pack-encoded
                // `ui_contract::UiIntent`, drops it if it targets a tree the user can no longer see (the
                // revision guard, at the reconciler that owns the revision — `PATCHES.revision`,
                // `ui_runtime::is_stale_intent` imported rather than reimplemented), and otherwise
                // batches it per instance for the dispatch pass below (mirrors `app_commands`'
                // batch-then-dispatch shape). Real dispatch replaces the prior packet's "decode-and-
                // mark-dirty" interim — see `📓️terra-sdk-wire-report.md`'s M1 section for the full route.
                Event::UiIntent { instance, intent } => {
                    let numeric_instance = instance.0.parse::<u32>().unwrap_or(0);
                    if let Ok(intent_value) = store::pack_rt::decode_wire_value(&intent) {
                        if let Ok(intent) = dsl::from_dsl_value::<ui_contract::UiIntent>(intent_value) {
                            let current_revision = PATCHES.with(|patches| patches.revision(&intent.surface.0));
                            if !is_stale_intent(intent.revision, current_revision, DEFAULT_REVISION_TOLERANCE) {
                                dirty
                                    .try_intent(numeric_instance, intent)
                                    .map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.dirty-intent-capacity"), "fixed dirty intent authority is saturated"))?;
                            }
                        }
                    }
                }
                Event::SurfaceVisible { surface } => {
                    if let Some(instance) = parse_surface_instance(&surface) {
                        let surface =
                            ui_contract::SurfaceId::try_from(surface).map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.surface-capacity"), "surface id exceeds fixed text capacity"))?;
                        dirty.try_surface(instance, surface).map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.dirty-surface-capacity"), "fixed dirty surface authority is saturated"))?;
                    }
                }
                Event::SurfaceHidden { .. } | Event::SurfaceResized { .. } => {}
                Event::PatchAck { surface, revision } => {
                    PENDING_PATCHES.with(|pending| {
                        pending.borrow_mut().apply_published_ack(&surface, revision, |ack| PATCHES.with(|patches| patches.mark_published_ack(ack)));
                    });
                }
                Event::PatchRejected { surface, .. } => {
                    PATCHES.with(|patches| patches.mark_rejected(&surface));
                }
                Event::Completed { req, result } => {
                    REGISTRY.with(|registry| registry.resolve(req, crate::host::outcome_to_result(result)));
                }
                Event::HttpChunk { req, bytes, done } => {
                    // 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): used to discard every
                    // non-final chunk outright (`if done { resolve(req, Ok(bytes)) }` — every earlier
                    // `bytes` was simply dropped on the floor, silent data loss for any multi-chunk
                    // response). `append_chunk` accumulates instead; `cap` is the owning instance's
                    // `QuotaSchema.message_bytes` (default 64 MiB when unset/unknown — matches
                    // `instance_task_quota`'s own `unwrap_or` fallback idiom above).
                    REGISTRY.with(|registry| {
                        let cap = registry.instance_of(req).and_then(|instance| INSTANCE_METADATA.with(|metadata| metadata.borrow().get(instance).and_then(|entry| entry.quota.message_bytes))).unwrap_or(64 * 1024 * 1024) as usize;
                        registry.append_chunk(req, bytes, done, cap);
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
                    // through `host::jobs::spawn` (`🌐host/🦀️component.rs`) allocates its `job` id from
                    // THE SAME `RequestRegistry` counter as every other awaitable `host::*` call — the
                    // `Effect::SpawnJob{job, ..}` this actor emitted carried `job == req.0` — so
                    // `Event::JobCompleted{job, result}` resolves the identical parked `RequestFuture`
                    // an `Event::Completed{req, result}` would, closing the "no `req`-per-job
                    // correlation table yet" gap `📓️terra-M5-report.md` §4 named (no separate table
                    // needed: the request id already IS the job id).
                    if let Some(binding) = JOB_RENDER_BINDINGS.with(|bindings| bindings.borrow_mut().complete(job)) {
                        let surface = ui_contract::UiText::try_format(format_args!("{}:window", binding.instance))
                            .map(ui_contract::SurfaceId)
                            .ok_or_else(|| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.surface-capacity"), "surface id exceeds fixed text capacity"))?;
                        dirty
                            .try_surface(binding.instance, surface)
                            .map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.dirty-surface-capacity"), "fixed dirty surface authority is saturated"))?;
                    }
                    REGISTRY.with(|registry| registry.resolve(semio_framework::kernel::RequestId(job), crate::host::outcome_to_result(result)));
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
                Event::Request { .. } => {}
                Event::Activate { .. } | Event::SuspendRequest | Event::CapabilityChanged { .. } | Event::QuotaChanged { .. } => {}
            }
        }

        let mut effects: Vec<Effect> = Vec::new();
        let mut command_ingress = semio_framework::kernel::CommandIngressStatus::Idle;
        let (mut retained, retained_slot) = COMMAND_INGRESS.with(|ingress| {
            let mut ingress = ingress.borrow_mut();
            if ingress[0].is_some() {
                (ingress[0].take(), 0)
            } else {
                (ingress[1].take(), 1)
            }
        });
        retained = match retained.take() {
            Some(CommandIngressOwner::ReservedPresence { cursor, .. }) | Some(CommandIngressOwner::PendingPresencePage { cursor, .. }) if close_instances.contains(&cursor.instance) => {
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
            Some(CommandIngressOwner::Terminal { cursor }) if close_instances.contains(&cursor.instance) => {
                command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-cancelled-by-close".to_vec() };
                None
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
                let now_ms = semio_framework::io::resolve_ready(crate::host::now_ms());
                match semio_framework::io::resolve_ready(crate::plugin_runtime::plugin_admit_reserved_presence(
                    runtime,
                    cursor.instance,
                    admission,
                    cursor.seq,
                    if cursor.metadata & 0x100 != 0 { Some((cursor.metadata & 0xff) as u8) } else { None },
                    cursor.item_count,
                    page,
                    now_ms,
                )) {
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
            Some(CommandIngressOwner::PendingPresencePage { cursor, publication_generation, page }) => {
                match semio_framework::io::resolve_ready(crate::plugin_runtime::plugin_push_reserved_presence_page(runtime, cursor.instance, publication_generation, cursor.page_index, page)) {
                    Ok(()) => {
                        command_ingress = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor.clone());
                        Some(CommandIngressOwner::Presence { cursor, publication_generation })
                    }
                    Err((_fault, page)) => {
                        command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
                        Some(CommandIngressOwner::PendingPresencePage { cursor, publication_generation, page })
                    }
                }
            }
            owner => owner,
        };
        if matches!(command_ingress, semio_framework::kernel::CommandIngressStatus::Idle) {
            if let Some(CommandIngressOwner::Presence { cursor, .. }) = retained.as_ref() {
                let cursor = cursor.clone();
                if close_instances.contains(&cursor.instance) {
                    retained = None;
                    command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-cancelled-by-close".to_vec() };
                } else {
                    match semio_framework::io::resolve_ready(crate::plugin_runtime::plugin_exchange(runtime, cursor.instance, None)) {
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
        if let Some(CommandIngressOwner::Terminal { cursor }) = retained.take() {
            command_ingress = match advance_command_cursor(cursor) {
                Ok(terminal) => semio_framework::kernel::CommandIngressStatus::CommandComplete(terminal),
                Err(cursor) => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-index-exhausted".to_vec() },
            };
        }
        if let Some(owner) = retained.take() {
            match owner {
                CommandIngressOwner::Generic { cursor, command } => match semio_framework::io::resolve_ready(crate::plugin_runtime::plugin_exchange(runtime, cursor.instance, Some((cursor.seq, command)))) {
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
            if close_instances.contains(&cursor.instance)
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
                | CommandIngressOwner::Generic { cursor: active, .. }
                | CommandIngressOwner::Terminal { cursor: active } => !same_command_cursor(active, &cursor),
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
                        match semio_framework::io::resolve_ready(crate::plugin_runtime::plugin_reserve_presence_ingress(runtime, cursor.instance, cursor.seq)) {
                            Ok(admission) => {
                                let now_ms = semio_framework::io::resolve_ready(crate::host::now_ms());
                                match semio_framework::io::resolve_ready(crate::plugin_runtime::plugin_admit_reserved_presence(runtime, cursor.instance, admission, cursor.seq, own_color, cursor.item_count, page, now_ms)) {
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
                        match semio_framework::io::resolve_ready(crate::plugin_runtime::plugin_push_reserved_presence_page(runtime, cursor.instance, publication_generation, cursor.page_index, page)) {
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
                match semio_framework::kernel::CommandPageSet::try_new() {
                    Ok(mut pages) => match pages.try_push(page) {
                        Err((fault, _page)) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: dsl::encode_fault_bytes(&fault) },
                        Ok(()) if cursor.page_count == 1 => match semio_framework::kernel::PagedCommand::try_from_pages(pages) {
                            Ok(command) => match semio_framework::io::resolve_ready(crate::plugin_runtime::plugin_exchange(runtime, cursor.instance, Some((cursor.seq, crate::plugin_runtime::PluginCommandIngress::Encoded(command))))) {
                                Ok(mut output) => {
                                    if let Some((_, command)) = output.retry_command.take() {
                                        retained = Some(CommandIngressOwner::Generic { cursor: cursor.clone(), command });
                                    } else {
                                        retained = Some(CommandIngressOwner::Terminal { cursor: cursor.clone() });
                                    }
                                    command_ingress = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor.clone());
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
                            Ok(command) => match semio_framework::io::resolve_ready(crate::plugin_runtime::plugin_exchange(runtime, cursor.instance, Some((cursor.seq, crate::plugin_runtime::PluginCommandIngress::Encoded(command))))) {
                                Ok(mut output) => {
                                    if let Some((_, command)) = output.retry_command.take() {
                                        retained = Some(CommandIngressOwner::Generic { cursor: cursor.clone(), command });
                                    } else {
                                        retained = Some(CommandIngressOwner::Terminal { cursor: cursor.clone() });
                                    }
                                    command_ingress = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor.clone());
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
                ingress.borrow_mut()[retained_slot] = Some(retained);
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
            // 🚫️async: E5 executor bridge — `plugin_dispatch_intents` stays genuinely `async fn`; safe to
            // resolve synchronously here for the same reason as `plugin_exchange` above.
            match semio_framework::io::resolve_ready(crate::plugin_runtime::plugin_dispatch_intents(runtime, instance, &intents)) {
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
                Err(fault) => effects.push(Effect::SendMessage { target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) }, payload: dsl::encode_fault_bytes(&fault) }),
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
                    effects.push(Effect::SendMessage {
                        target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) },
                        payload: dsl::encode_fault_bytes(&semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.surface-capacity"), "dirty render surface capacity exceeded")),
                    });
                    break;
                }
            }
            for surface in surfaces {
                dirty
                    .try_surface(instance, ui_contract::SurfaceId(surface))
                    .map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.dirty-surface-capacity"), "fixed dirty surface authority is saturated"))?;
            }
        }

        // 👥️ M2 (ticket 26/08/17 `design-unified.md`): `now_ms` is read ONCE for both `record_peer`'s
        // expiry stamping below and `PRESENCE.expire` at the end of this turn — a single wall-clock
        // reading per poll, not one per presence update.
        let now_ms = u64::try_from(semio_framework::io::resolve_ready(crate::host::now_ms())).unwrap_or(0);

        for (instance, surface) in dirty.surfaces {
            PATCHES.with(|patches| match patches.reserve_mounted(surface) {
                Ok(grant) => match semio_framework::io::resolve_ready(crate::plugin_runtime::plugin_render(runtime, instance, "window", "{}")) {
                    Ok(tree) => {
                        let _ = grant.commit_source(tree.root);
                    }
                    Err(_) => grant.cancel(),
                },
                Err(surface) => {
                    let _ = patches.defer(surface);
                }
            });
            // 👥️ M2: drains this instance's render-plane presence outbox (`VcsArtifactApp::
            // pending_presence`, filled by `stamp_and_cache_interaction_ui` during the render just above)
            // into `PRESENCE` right after its render — the SAME turn that presented the tree also records
            // its presence. NEVER touches `PENDING_PATCHES`/the document store — the whole point of this
            // separate channel (see `PresenceHub`'s own doc: a mouse-move must never bump a revision).
            match semio_framework::io::resolve_ready(crate::plugin_runtime::plugin_take_presence(runtime, instance)) {
                Ok(updates) => {
                    for update in updates {
                        PRESENCE.with(|hub| {
                            let mut hub = hub.borrow_mut();
                            hub.record_own(update.surface.clone(), update.node_key.clone(), update.own, update.ttl_ms);
                            for peer in update.peers {
                                hub.record_peer(update.surface.clone(), update.node_key.clone(), peer, update.ttl_ms, now_ms);
                            }
                        });
                    }
                }
                Err(fault) => effects.push(Effect::SendMessage { target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) }, payload: dsl::encode_fault_bytes(&fault) }),
            }
        }
        let reconcile_work = PATCHES.with(|patches| {
            let more = patches.drive_one();
            let can_publish = PENDING_PATCHES.with(|pending| pending.borrow().has_capacity());
            if can_publish {
                if let Some(patch) = patches.take_ready_patch() {
                    if let Err(patch) = PENDING_PATCHES.with(|pending| pending.borrow_mut().push_reconcile(patch)) {
                        let _ = patches.return_ready_patch(patch);
                    }
                }
            }
            more || patches.has_work() || PENDING_PATCHES.with(|pending| pending.borrow().has_unpublished())
        });

        // 🚫️async: E5 executor bridge (× 2) — `LocalExecutor::{run_until_idle,has_ready}` stay
        // genuinely `async fn` (its own doc: "run_until_idle handles Pending without ever yielding
        // its own future" — matches `⚛️reactor/💼️jobs`'s identical use of this exact bridge).
        let more_work = REACTOR_EXECUTOR.with(|executor| executor.run_until_deadline(64, 256 * 1_024, std::time::Instant::now() + std::time::Duration::from_millis(8)));
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
        let more_work = more_work || reconcile_work || resumes_remain || REACTOR_EXECUTOR.with(|executor| executor.has_pending()) || COMMAND_INGRESS.with(|ingress| ingress.borrow().iter().any(Option::is_some));

        let mut ui_patches = semio_framework::kernel::UiTurnPatches::default();
        if let Some(patch) = PENDING_PATCHES.with(|pending| pending.borrow_mut().take_one()) {
            if let Err(patch) = ui_patches.try_push_ui_patch(patch) {
                PENDING_PATCHES.with(|pending| pending.borrow_mut().hand_back_turn(patch));
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

        Ok(semio_framework::kernel::TurnResult { ui_patches, effects, presence, next_wake: ARMED_TIMERS.with(|timers| timers.borrow().first()), status, fuel_used: 0, command_ingress })
    }

    fn route_exchange_output(instance: u32, output: crate::plugin_runtime::PluginExchangeOutput, effects: &mut Vec<Effect>) {
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

    fn advance_command_cursor(mut cursor: semio_framework::kernel::CommandPageCursor) -> Result<semio_framework::kernel::CommandPageCursor, semio_framework::kernel::CommandPageCursor> {
        let Some(page_index) = cursor.page_index.checked_add(1) else { return Err(cursor) };
        cursor.page_index = page_index;
        Ok(cursor)
    }

    const PENDING_PATCH_CAPACITY: usize = semio_framework_ui_runtime::SURFACE_RECONCILE_ADMISSION_SLOTS;

    enum PendingPatchOwner {
        Reconcile(SurfaceReconcileReadyPatch),
        External(Option<UiPatch>),
        Published(SurfaceReconcilePublishedPatch),
    }

    struct PendingPatchSlot {
        sequence: u64,
        owner: PendingPatchOwner,
    }

    struct PendingPatchAuthority {
        slots: [Option<PendingPatchSlot>; PENDING_PATCH_CAPACITY],
        closing_instances: [Option<u32>; PENDING_PATCH_CAPACITY],
        turn_handback: Option<UiPatch>,
        next_sequence: u64,
        exhausted: bool,
    }

    impl PendingPatchAuthority {
        fn new() -> Self {
            Self { slots: std::array::from_fn(|_| None), closing_instances: [None; PENDING_PATCH_CAPACITY], turn_handback: None, next_sequence: 0, exhausted: false }
        }

        fn reserve_sequence(&mut self) -> Option<u64> {
            if self.exhausted || self.slots.iter().all(Option::is_some) {
                return None;
            }
            let sequence = self.next_sequence.checked_add(1)?;
            self.next_sequence = sequence;
            self.exhausted = sequence == u64::MAX;
            Some(sequence)
        }

        fn has_capacity(&self) -> bool {
            !self.exhausted && self.slots.iter().any(Option::is_none)
        }

        fn push_reconcile(&mut self, owner: SurfaceReconcileReadyPatch) -> Result<(), SurfaceReconcileReadyPatch> {
            let Some(index) = self.slots.iter().position(Option::is_none) else { return Err(owner) };
            let Some(sequence) = self.reserve_sequence() else { return Err(owner) };
            self.slots[index] = Some(PendingPatchSlot { sequence, owner: PendingPatchOwner::Reconcile(owner) });
            Ok(())
        }

        fn push_external(&mut self, patch: UiPatch) -> Result<(), UiPatch> {
            let Some(index) = self.slots.iter().position(Option::is_none) else { return Err(patch) };
            let Some(sequence) = self.reserve_sequence() else { return Err(patch) };
            self.slots[index] = Some(PendingPatchSlot { sequence, owner: PendingPatchOwner::External(Some(patch)) });
            Ok(())
        }

        fn take_one(&mut self) -> Option<UiPatch> {
            if let Some(patch) = self.turn_handback.take() {
                return Some(patch);
            }
            let index =
                self.slots.iter().enumerate().filter(|(_, slot)| slot.as_ref().is_some_and(|slot| !matches!(&slot.owner, PendingPatchOwner::Published(_)))).min_by_key(|(_, slot)| slot.as_ref().map(|slot| slot.sequence)).map(|(index, _)| index)?;
            let slot = self.slots[index].take()?;
            match slot.owner {
                PendingPatchOwner::Reconcile(owner) => {
                    let (patch, published) = owner.publish()?;
                    self.slots[index] = Some(PendingPatchSlot { sequence: slot.sequence, owner: PendingPatchOwner::Published(published) });
                    Some(patch)
                }
                PendingPatchOwner::External(mut patch) => patch.take(),
                PendingPatchOwner::Published(published) => {
                    self.slots[index] = Some(PendingPatchSlot { sequence: slot.sequence, owner: PendingPatchOwner::Published(published) });
                    None
                }
            }
        }

        fn hand_back_turn(&mut self, patch: UiPatch) {
            self.turn_handback = Some(patch);
        }

        fn apply_published_ack(&mut self, surface: &str, revision: u64, advance: impl FnOnce(semio_framework_ui_runtime::SurfaceReconcilePublishedAck) -> Result<(), semio_framework_ui_runtime::SurfaceReconcilePublishedAck>) -> bool {
            let Some(index) = self.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| matches!(&slot.owner, PendingPatchOwner::Published(owner) if owner.matches(surface, revision)))) else {
                return false;
            };
            let Some(slot) = self.slots[index].take() else { return false };
            let published = match slot.owner {
                PendingPatchOwner::Published(owner) => owner,
                owner => {
                    self.slots[index] = Some(PendingPatchSlot { sequence: slot.sequence, owner });
                    return false;
                }
            };
            let ack = match published.acknowledge(surface, revision) {
                Ok(ack) => ack,
                Err(published) => {
                    self.slots[index] = Some(PendingPatchSlot { sequence: slot.sequence, owner: PendingPatchOwner::Published(published) });
                    return false;
                }
            };
            match advance(ack) {
                Ok(()) => true,
                Err(ack) => {
                    self.slots[index] = Some(PendingPatchSlot { sequence: slot.sequence, owner: PendingPatchOwner::Published(ack.into_published()) });
                    false
                }
            }
        }

        fn close_instance_one(&mut self, instance: u32) -> bool {
            if self.turn_handback.as_ref().and_then(|patch| parse_surface_instance(&patch.surface.0)) == Some(instance) {
                if self.turn_handback.as_mut().is_some_and(|patch| patch.ops.pop().is_some()) {
                    return false;
                }
                self.turn_handback.take();
                return false;
            }
            let Some(index) = self.slots.iter().position(|slot| {
                slot.as_ref().is_some_and(|slot| match &slot.owner {
                    PendingPatchOwner::Reconcile(owner) => owner.surface().and_then(|surface| parse_surface_instance(&surface.0)) == Some(instance),
                    PendingPatchOwner::External(patch) => patch.as_ref().and_then(|patch| parse_surface_instance(&patch.surface.0)) == Some(instance),
                    PendingPatchOwner::Published(owner) => parse_surface_instance(&owner.surface().0) == Some(instance),
                })
            }) else {
                return true;
            };
            let complete = match self.slots[index].as_mut().map(|slot| &mut slot.owner) {
                Some(PendingPatchOwner::Reconcile(owner)) => owner.close_step(),
                Some(PendingPatchOwner::External(patch)) => {
                    if patch.as_mut().is_some_and(|patch| patch.ops.pop().is_some()) {
                        false
                    } else {
                        patch.take();
                        true
                    }
                }
                Some(PendingPatchOwner::Published(_)) => true,
                None => true,
            };
            if complete {
                self.slots[index] = None;
            }
            false
        }

        fn begin_close_instance(&mut self, instance: u32) {
            if self.closing_instances.iter().flatten().any(|closing| *closing == instance) {
                return;
            }
            if let Some(slot) = self.closing_instances.iter_mut().find(|slot| slot.is_none()) {
                *slot = Some(instance);
            }
        }

        fn close_step(&mut self) -> bool {
            let Some(index) = self.closing_instances.iter().position(Option::is_some) else { return true };
            let Some(instance) = self.closing_instances[index] else { return false };
            if self.close_instance_one(instance) {
                self.closing_instances[index] = None;
            }
            false
        }

        fn has_unpublished(&self) -> bool {
            self.turn_handback.is_some() || self.slots.iter().flatten().any(|slot| !matches!(&slot.owner, PendingPatchOwner::Published(_))) || self.closing_instances.iter().any(Option::is_some)
        }
    }

    crate::component_persistent_local! {
        static PENDING_PATCHES: RefCell<PendingPatchAuthority> = RefCell::new(PendingPatchAuthority::new());
    }

    /// 🪪️ Surfaces are named `"<instance>:<body-key>"` in this wave (no dedicated `surface-ref`
    /// bookkeeping table yet — `ui.wit`'s `surface-ref` record exists at the WIT boundary, but the
    /// Rust-side `kernel::UiPatch.surface` is still a plain `String` per A3's landed shape).
    fn parse_surface_instance(surface: &str) -> Option<u32> {
        surface.split(':').next()?.parse().ok()
    }

    /// 🔀️ `AppFrame::UiPatch` → a real `kernel::UiPatch` passthrough into `PENDING_PATCHES` (the wire
    /// frame is already `UiPatch`-shaped field-for-field — channel v12/A4 — so this is a decode, not a
    /// render); `AppFrame::Effects`/`Events` no longer exist as frames (`poll` decodes
    /// `plugin_exchange`'s `PluginExchangeOutput.effects`/`.events` directly instead — see there);
    /// `AppFrame::UiSnapshotEnd` has no consumer yet in this wave (patches apply incrementally, no
    /// snapshot-boundary bookkeeping); everything else → `Effect::SendMessage` to the shell, matching
    /// design-abi.md §2's table verbatim.
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
                let Ok(ops) = dsl::from_dsl_value::<ui_contract::UiPatchOps>(ops_value) else { return };
                let Ok(surface) = ui_contract::SurfaceId::try_from(surface) else { return };
                let patch = UiPatch { surface, base_revision: ui_contract::UiRevision(base_revision), revision: ui_contract::UiRevision(revision), ops };
                if let Err(patch) = PENDING_PATCHES.with(|pending| pending.borrow_mut().push_external(patch)) {
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
            let input = match resume.outcome {
                TaskResumeOutcome::Fault(fault) => {
                    effects.push(Effect::SendMessage { target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(resume.instance.to_string()) }, payload: dsl::encode_fault_bytes(&fault) });
                    continue;
                }
                TaskResumeOutcome::Command(bytes) => crate::plugin_runtime::TaskResumeInput::Command(bytes),
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
        dsl::from_dsl_value(value).map_err(|_| ())
    }

    fn push_admitted_effect(effects: &mut Vec<Effect>, instance: u32, effect: Effect) {
        if let Effect::SetTimer { id, .. } = &effect {
            if ARMED_TIMERS.with(|timers| timers.borrow_mut().insert(instance, *id)).is_err() {
                let fault = semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.timer-capacity"), "fixed timer authority is saturated or collided");
                effects.push(Effect::SendMessage { target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) }, payload: dsl::encode_fault_bytes(&fault) });
                return;
            }
        }
        if let Effect::SpawnJob { job, .. } = &effect {
            if JOB_RENDER_BINDINGS.with(|bindings| bindings.borrow_mut().bind(instance, *job)).is_err() {
                let fault = semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.job-render-binding-capacity"), "fixed job-to-surface render authority is saturated or collided");
                effects.push(Effect::SendMessage { target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) }, payload: dsl::encode_fault_bytes(&fault) });
                return;
            }
        }
        effects.push(effect);
    }

    fn decode_wire_app_event(bytes: &[u8]) -> Result<semio_framework::kernel::AppEvent, ()> {
        let value = store::pack_rt::decode_wire_value(bytes).map_err(|_| ())?;
        dsl::from_dsl_value(value).map_err(|_| ())
    }

    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: decodes `instance-open-event.quotas` (a wire
    /// `pack`) into the real `QuotaSchema` — see `wit_event_to_kernel`'s `InstanceOpen` arm. Falls
    /// back to `default()` (no field set — read as "no limit declared" by every reader, e.g.
    /// `instance_task_quota`'s `unwrap_or(16)`) on a decode failure rather than failing `InstanceOpen`
    /// outright.
    fn wit_command_page_to_kernel(page: crate::component::component::exports::semio::framework::reactor::CommandIngressPage) -> Result<(semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage), semio_framework::Fault> {
        if page.length as usize > semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES {
            return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.command-page-lift-cap"), "fixed command page declares more than 4096 bytes"));
        }
        let mut bytes = [0; semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES];
        let mut offset = 0usize;
        for block in [
            &page.block_00,
            &page.block_01,
            &page.block_02,
            &page.block_03,
            &page.block_04,
            &page.block_05,
            &page.block_06,
            &page.block_07,
            &page.block_08,
            &page.block_09,
            &page.block_10,
            &page.block_11,
            &page.block_12,
            &page.block_13,
            &page.block_14,
            &page.block_15,
            &page.block_16,
            &page.block_17,
            &page.block_18,
            &page.block_19,
            &page.block_20,
            &page.block_21,
            &page.block_22,
            &page.block_23,
            &page.block_24,
            &page.block_25,
            &page.block_26,
            &page.block_27,
            &page.block_28,
            &page.block_29,
            &page.block_30,
            &page.block_31,
            &page.block_32,
            &page.block_33,
            &page.block_34,
            &page.block_35,
            &page.block_36,
            &page.block_37,
            &page.block_38,
            &page.block_39,
            &page.block_40,
            &page.block_41,
            &page.block_42,
            &page.block_43,
            &page.block_44,
            &page.block_45,
            &page.block_46,
            &page.block_47,
            &page.block_48,
            &page.block_49,
            &page.block_50,
            &page.block_51,
            &page.block_52,
            &page.block_53,
            &page.block_54,
            &page.block_55,
            &page.block_56,
            &page.block_57,
            &page.block_58,
            &page.block_59,
            &page.block_60,
            &page.block_61,
            &page.block_62,
            &page.block_63,
        ] {
            for word in [block.word_0, block.word_1, block.word_2, block.word_3, block.word_4, block.word_5, block.word_6, block.word_7] {
                let fixed = word.to_le_bytes();
                bytes[offset..offset + fixed.len()].copy_from_slice(&fixed);
                offset += fixed.len();
            }
        }
        let cursor = page.cursor;
        let bytes = semio_framework::kernel::FixedCommandPage::try_from_array(bytes, page.length)?;
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

    fn kernel_command_cursor_to_wit(cursor: semio_framework::kernel::CommandPageCursor) -> crate::component::component::exports::semio::framework::reactor::CommandPageCursor {
        use crate::component::component::exports::semio::framework::reactor as wit;
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
    fn idle_command_cursor_to_wit() -> crate::component::component::exports::semio::framework::reactor::CommandPageCursor {
        use crate::component::component::exports::semio::framework::reactor as wit;
        wit::CommandPageCursor { owner: 0, generation: 0, command_index: 0, command_count: 0, instance: 0, seq: 0, kind: 0, page_index: 0, page_count: 0, item_count: 0, metadata: 0 }
    }

    /// 🔢️ Kernel command ingress → scalar WIT record, avoiding nested variant discriminants in async results.
    fn kernel_command_ingress_to_wit(status: semio_framework::kernel::CommandIngressStatus) -> crate::component::component::exports::semio::framework::reactor::CommandIngressStatus {
        use crate::component::component::exports::semio::framework::reactor as wit;
        let (kind, cursor, fault) = match status {
            semio_framework::kernel::CommandIngressStatus::Idle => (0, idle_command_cursor_to_wit(), Vec::new()),
            semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor) => (1, kernel_command_cursor_to_wit(cursor), Vec::new()),
            semio_framework::kernel::CommandIngressStatus::Backpressure(cursor) => (2, kernel_command_cursor_to_wit(cursor), Vec::new()),
            semio_framework::kernel::CommandIngressStatus::CommandPending(cursor) => (3, kernel_command_cursor_to_wit(cursor), Vec::new()),
            semio_framework::kernel::CommandIngressStatus::CommandComplete(cursor) => (4, kernel_command_cursor_to_wit(cursor), Vec::new()),
            semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault } => (5, kernel_command_cursor_to_wit(cursor), fault),
        };
        wit::CommandIngressStatus { kind, cursor, fault }
    }

    fn decode_wire_quotas(bytes: &[u8]) -> semio_framework::kernel::QuotaSchema {
        store::pack_rt::decode_wire_value(bytes).ok().and_then(|value| dsl::from_dsl_value(value).ok()).unwrap_or_default()
    }

    /// 🔀️ WIT `event` → kernel `Event`. Thin field-for-field translation — the WIT side already
    /// mirrors the kernel shape (see `📓️design-abi.md` §2 / `events.wit`'s own doc comments).
    fn wit_event_to_kernel(event: crate::component::component::exports::semio::framework::reactor::Event) -> Event {
        use crate::component::component::exports::semio::framework::reactor::Event as W;
        match event {
            W::InstanceOpen(payload) => Event::InstanceOpen {
                instance: semio_framework::kernel::PluginInstanceId(payload.instance.to_string()),
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
            W::InstanceClose(_) => Event::InstanceClose,
            W::Activate(payload) => Event::Activate { reason: wit_activation_to_kernel(payload.reason) },
            W::SuspendRequest(_) => Event::SuspendRequest,
            W::CapabilityChanged(_) => Event::SuspendRequest,
            W::QuotaChanged(_) => Event::SuspendRequest,
            // 🎬️ `wit-flip` (26/08/20): UI intents no longer masquerade as `app-command` — see kernel
            // `Event::UiIntent`'s own doc.
            W::UiIntent(payload) => Event::UiIntent { instance: semio_framework::kernel::PluginInstanceId(payload.instance.to_string()), intent: payload.intent },
            W::SurfaceVisible(payload) => Event::SurfaceVisible { surface: format!("{}:{}", payload.surface.instance, "window") },
            W::SurfaceHidden(payload) => Event::SurfaceHidden { surface: format!("{}:{}", payload.surface.instance, "window") },
            W::SurfaceResized(payload) => Event::SurfaceResized { surface: format!("{}:{}", payload.surface.instance, "window"), width: payload.width, height: payload.height },
            W::PatchAck(payload) => Event::PatchAck { surface: format!("{}:{}", payload.surface.instance, "window"), revision: payload.revision },
            W::PatchRejected(payload) => Event::PatchRejected { surface: format!("{}:{}", payload.surface.instance, "window"), revision: payload.revision, reason: payload.reason },
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
        result: semio_framework::kernel::TurnResult,
        _budget: crate::component::component::exports::semio::framework::reactor::Budget,
    ) -> Result<crate::component::component::exports::semio::framework::reactor::TurnResult, semio_framework::Fault> {
        use crate::component::component::exports::semio::framework::reactor as wit;
        Ok(wit::TurnResult {
            ui_patches: result.ui_patches.into_iter().map(kernel_ui_patch_to_wit).collect(),
            effects: result.effects.into_iter().map(kernel_effect_to_wit).collect::<Result<Vec<_>, _>>()?,
            presence: result.presence.into_iter().map(kernel_presence_update_to_wit).collect(),
            next_wake: result.next_wake,
            status: match result.status {
                TurnStatus::Idle => wit::TurnStatus::Idle,
                TurnStatus::MoreWork => wit::TurnStatus::MoreWork,
                TurnStatus::CheckpointReady { checkpoint } => wit::TurnStatus::CheckpointReady(wit::JobCheckpoint { state: checkpoint.state, applied_progress: checkpoint.applied_progress }),
                TurnStatus::Faulted(bytes) => wit::TurnStatus::Faulted(bytes),
            },
            fuel_used: result.fuel_used,
            command_ingress: kernel_command_ingress_to_wit(result.command_ingress),
        })
    }

    /// 👥️ M2: pack-encodes a whole `ui_contract::PresenceUpdate` into the WIT `presence-update.update`
    /// field — same `pack_patch_field` helper every `patch-op` variant already uses, since a
    /// render-plane presence update is exactly as opaque to the WIT boundary as a patch op's payload.
    fn kernel_presence_update_to_wit(update: ui_contract::PresenceUpdate) -> crate::component::component::exports::semio::framework::reactor::PresenceUpdate {
        use crate::component::component::exports::semio::framework::reactor as wit;
        wit::PresenceUpdate { update: pack_patch_field(&update) }
    }

    fn kernel_ui_patch_to_wit(patch: UiPatch) -> crate::component::component::exports::semio::framework::reactor::UiPatch {
        use crate::component::component::exports::semio::framework::reactor as wit;
        let instance: u32 = patch.surface.0.split(':').next().and_then(|s| s.parse().ok()).unwrap_or(0);
        wit::UiPatch { surface: wit_ui::SurfaceRef { instance, surface: 0 }, revision: patch.revision.0, base_revision: patch.base_revision.0, ops: patch.ops.into_iter().map(kernel_patch_op_to_wit).collect() }
    }

    /// 🩹️ Packs any `Serialize` payload the same way `kernel_effect_to_wit`'s own `pack` helper does —
    /// shared here rather than duplicated because every `patch-op` variant but `Remove`/`SetRoot` carries
    /// exactly one `pack`-encoded field.
    // 🚫️async: E5 executor bridge — `store::pack_rt::encode_wire_value` is genuinely `async fn` (out of
    // this packet's `path_scope`, `🏪️store/**`); safe to resolve synchronously here for the same "world
    // actor has no host-async import" reason as this file's other WIT-boundary bridges.
    fn pack_patch_field<T: serde::Serialize>(value: &T) -> Vec<u8> {
        store::pack_rt::encode_wire_value(&dsl::to_dsl_value(value).unwrap_or(dsl::DslValue::Null))
    }

    /// 🩹️ Wire payload for `patch-set-activity`'s `activity: pack` field. `component.wit`'s
    /// `patch-set-activity` (`wit-flip`, 26/08/20) carries only `activity: pack`, with no sibling
    /// `disabled: bool`, while the contract crate's own `UiPatchOp::SetActivity` carries `disabled` as a
    /// peer field (`🧬️contract/🦀️document.rs:148`) — a real WIT/contract mismatch, flagged rather than
    /// silently resolved by `📓️terra-wit-flip-report.md`'s decisions section, and resolved HERE (the
    /// first packet to actually encode this op) by folding `disabled` into the packed payload instead of
    /// dropping it. A future packet editing `component.wit` may add a `disabled: bool` field there
    /// instead, at which point this wrapper — and its symmetric decode on the host side — goes away.
    #[derive(serde::Serialize)]
    struct ActivityPatchPayload<'a> {
        activity: &'a ui_contract::Activity,
        disabled: bool,
    }

    fn kernel_patch_op_to_wit(op: UiPatchOp) -> wit_ui::PatchOp {
        match op {
            UiPatchOp::Upsert(record) => wit_ui::PatchOp::Upsert(wit_ui::PatchUpsert { node: pack_patch_field(&record) }),
            UiPatchOp::SetComponent { id, component } => wit_ui::PatchOp::SetComponent(wit_ui::PatchSetComponent { node: id.0, component: pack_patch_field(&component) }),
            UiPatchOp::SetLayout { id, layout } => wit_ui::PatchOp::SetLayout(wit_ui::PatchSetLayout { node: id.0, layout: pack_patch_field(&layout) }),
            UiPatchOp::SetActivity { id, activity, disabled } => wit_ui::PatchOp::SetActivity(wit_ui::PatchSetActivity { node: id.0, activity: pack_patch_field(&ActivityPatchPayload { activity: &activity, disabled }) }),
            UiPatchOp::SetChildren { id, children } => wit_ui::PatchOp::SetChildren(wit_ui::PatchSetChildren { node: id.0, children: children.into_iter().map(|child| child.0).collect() }),
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
    fn kernel_effect_to_wit(effect: Effect) -> Result<crate::component::component::exports::semio::framework::reactor::Effect, semio_framework::Fault> {
        use crate::component::component::exports::semio::framework::reactor as wit;
        // 🚫️async: E5 executor bridge — `store::pack_rt::encode_wire_value` is genuinely `async fn`
        // (out of this packet's `path_scope`, `🏪️store/**`), but every caller in this match below is
        // itself sync (R9: `kernel_effect_to_wit`'s only consumer is the WIT-fixed sync `world actor`
        // boundary, no suspension point of its own) — `resolve_ready` is safe here because `world
        // actor` imports no `host-async`, so this store call never has anything real to suspend on.
        fn pack<T: serde::Serialize>(value: &T) -> Vec<u8> {
            store::pack_rt::encode_wire_value(&dsl::to_dsl_value(value).unwrap_or(dsl::DslValue::Null))
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
            Effect::PatchWorld3dChrome { selection_json, vortices_json, document_selected_ids, document_highlighted_ids } => {
                wit::Effect::PatchWorld3dChrome(wit_effects::PatchWorld3dChromeEffect { selection_json, vortices_json, document_selected_ids, document_highlighted_ids })
            }
            Effect::InvokeExtension { req, extension_id, capability, request_json } => {
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
} // mod wit_bridge

/// 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: test-only hooks into this actor's UNGATED
/// per-actor state (`EXECUTOR`/`REGISTRY`/`TASK_RESUMES`) — `poll`'s real event routing
/// (`Event::Completed` → `REGISTRY::resolve`, `run_until_idle`, `drain_task_resumes`) lives inside
/// `wit_bridge`, which is wasm32-wasip2-only and cannot be exercised by a native `cargo test`. This
/// module gives native tests (this crate's own, and `plugin_runtime`'s `test_push_instance`-backed
/// integration tests) the SAME three primitives `poll` itself drives, so the mechanism — spawn,
/// park, get resolved by an injected completion, resume — is exercised end-to-end without needing
/// a wasm32-wasip2 build. `pub(crate)`, `#[cfg(test)]`-gated: never part of the real API surface.
#[cfg(test)]
pub(crate) mod test_support {
    use super::*;

    /// ▶️ The exact `run_until_idle` call `poll` makes after routing events, exposed directly.
    pub(crate) async fn run_until_idle(max_iterations: u32) -> bool {
        TEST_FUTURE_EXECUTOR.with(|executor| executor.run_until_idle(max_iterations))
    }

    /// ✅️ The exact `REGISTRY::resolve` call `poll`'s `Event::Completed` arm makes, exposed
    /// directly — the native stand-in for "an injected `Event::Completed`".
    pub(crate) async fn resolve_request(id: u64, result: Result<Vec<u8>, semio_framework::Fault>) {
        REGISTRY.with(|registry| registry.resolve(semio_framework::kernel::RequestId(id), result));
    }

    /// 📬️ Pops one `TASK_RESUMES` entry, erased to `plugin_runtime::TaskResumeInput` (a `Fault`
    /// resolution surfaces as `Err` here instead — `drain_task_resumes` frames that straight to
    /// the shell without ever reaching `plugin_runtime`, so a test asserting on it never needs
    /// `plugin_resume_task` at all).
    pub(crate) async fn pop_task_resume() -> Option<(u32, crate::app::ActionMeta, Result<crate::plugin_runtime::TaskResumeInput, semio_framework::Fault>)> {
        TASK_RESUMES.with(|resumes| resumes.borrow_mut().pop()).map(|resume| {
            let input = match resume.outcome {
                TaskResumeOutcome::Command(bytes) => Ok(crate::plugin_runtime::TaskResumeInput::Command(bytes)),
                TaskResumeOutcome::Emit { artifact_ops, config_ops, draft_ops } => Ok(crate::plugin_runtime::TaskResumeInput::Emit { artifact_ops, config_ops, draft_ops }),
                TaskResumeOutcome::Fault(fault) => Err(fault),
            };
            (resume.instance, resume.meta, input)
        })
    }

    pub(crate) async fn task_count_for_instance(instance: u32) -> usize {
        TASK_RECORDS.with(|records| records.borrow().count_instance(instance))
    }

    pub(crate) async fn task_key_is_live(instance: u32, key: &str) -> bool {
        TASK_RECORDS.with(|records| records.borrow().find_key(instance, key).is_some())
    }

    pub(crate) async fn set_instance_quota(instance: u32, outstanding_requests: u64) {
        INSTANCE_METADATA.with(|metadata| {
            let mut metadata = metadata.borrow_mut();
            if let Some(entry) = metadata.slots.get_mut(InstanceMetadataRegistry::index(instance)).filter(|entry| entry.instance == instance) {
                entry.quota.outstanding_requests = Some(outstanding_requests);
            } else {
                let _ = metadata.insert(instance, "test".into(), semio_framework::kernel::QuotaSchema { outstanding_requests: Some(outstanding_requests), ..Default::default() });
            }
        });
    }

    pub(crate) async fn pending_request_count() -> usize {
        REGISTRY.with(|registry| registry.pending_ids().len())
    }

    /// 🚫️ The exact `RequestRegistry::cancel_instance` call `poll`'s `Event::InstanceClose` arm
    /// makes right after `cancel_instance_tasks`, exposed directly for a test to run the SAME
    /// two-step sequence natively.
    pub(crate) async fn cancel_instance_registry_requests(instance: u32) -> usize {
        REGISTRY.with(|registry| {
            let before = registry.pending_ids().len();
            let mut cursor = registry.begin_cancel_instance(instance);
            while registry.cancel_instance_step(&mut cursor) != requests::RequestCloseStep::Complete {}
            before.saturating_sub(registry.pending_ids().len())
        })
    }

    //#region 🔖️M2PresenceHooks
    /// 🎯️ M1: the exact `PATCHES.revision` read `poll`'s intent-batching loop guards every
    /// `UiIntent` against, exposed directly.
    pub(crate) async fn patches_revision(surface: &str) -> ui_contract::UiRevision {
        PATCHES.with(|patches| patches.revision(surface))
    }

    /// 🩹️ Test-only completion driver over the same retained mounted authority.
    pub(crate) async fn patches_diff(surface: &str, tree: &semio_framework_ui_runtime::ComponentTree) -> Option<ui_contract::UiPatch> {
        PATCHES.with(|patches| {
            if !patches.can_begin(surface) {
                return None;
            }
            if let Err((surface, tree)) = patches.begin(surface.to_string(), tree.clone()) {
                let _ = patches.retain_unadmitted(surface, tree);
                return None;
            }
            for _ in 0..512 {
                patches.drive_one();
                if let Some(patch) = patches.take_ready_patch() {
                    return Some(patch);
                }
            }
            None
        })
    }

    /// 👥️ M2 (ticket 26/08/17 `design-unified.md`): the exact `PRESENCE.record_own` call `poll`'s
    /// dirty-render loop makes for each drained `PresenceUpdate`, exposed directly.
    pub(crate) async fn presence_record_own(surface: &str, node_key: &str, own: ui_contract::OwnPresence, ttl_ms: u32) {
        PRESENCE.with(|hub| hub.borrow_mut().record_own(ui_contract::SurfaceId::from(surface), node_key.to_string(), own, ttl_ms));
    }

    /// 👥️ M2: the exact `PRESENCE.record_peer` call, exposed directly.
    pub(crate) async fn presence_record_peer(surface: &str, node_key: &str, mark: ui_contract::PeerMark, ttl_ms: u32, now_ms: u64) {
        PRESENCE.with(|hub| hub.borrow_mut().record_peer(ui_contract::SurfaceId::from(surface), node_key.to_string(), mark, ttl_ms, now_ms));
    }

    /// 👥️ M2: the exact `expire`-then-`flush` sequence `poll` runs once per turn, exposed directly.
    pub(crate) async fn presence_expire_and_flush(now_ms: u64) -> Vec<ui_contract::PresenceUpdate> {
        PRESENCE.with(|hub| {
            let mut hub = hub.borrow_mut();
            hub.expire(now_ms);
            hub.flush()
        })
    }
    //#endregion 🔖️M2PresenceHooks
}

//#region 🧪️M1M2ReactorTests
/// 🎯️👥️ M1/M2 (ticket 26/08/17 `design-unified.md`) acceptance, driven through `test_support`'s
/// direct hooks into `PATCHES`/`PRESENCE` — the same two thread-locals `poll`'s real intent-batching
/// and dirty-render loops touch, exercised without needing a wasm32-wasip2 build (`poll` itself,
/// gated to `wit_bridge`, cannot run under a native `cargo test` — see `test_support`'s own doc).
#[cfg(test)]
mod m1_m2_reactor_tests {
    use super::test_support::*;
    use semio_framework_ui_contract as ui_contract;
    use semio_framework_ui_runtime::{ComponentTree, TreeNode};

    fn leaf(key: &str) -> ComponentTree {
        ComponentTree::new(TreeNode::new(key, ui_contract::Component::Separator(ui_contract::SeparatorProps {})))
    }

    /// 🎯️ M1: `PATCHES.revision` reads 0 for a surface `poll` has never rendered, and
    /// `ui_runtime::is_stale_intent` correctly classifies an intent at/behind/beyond the tolerance
    /// against it — the exact two calls `poll`'s `Event::UiIntent` arm chains together.
    #[semio_framework_async_macros::async_test]
    async fn revision_guard_never_rejects_an_intent_at_the_never_rendered_default() {
        let current = patches_revision("never-rendered").await;
        assert_eq!(current, ui_contract::UiRevision(0));
        assert!(!semio_framework_ui_runtime::is_stale_intent(ui_contract::UiRevision(0), current, semio_framework_ui_runtime::DEFAULT_REVISION_TOLERANCE));
    }

    /// 🎯️ M1: after one real render bumps `PATCHES`'s revision to 1, an intent stamped at revision
    /// 0 (trailing by exactly the default tolerance of 1) is NOT stale — but one that trails by 2
    /// (as if two more renders had happened since the client last saw the surface) IS. This is
    /// exactly the acceptance criterion "an intent whose revision trails by 2 produces no patch and
    /// no command" reduced to the guard `poll` evaluates before ever reaching dispatch.
    #[semio_framework_async_macros::async_test]
    async fn revision_guard_rejects_an_intent_trailing_by_more_than_the_tolerance() {
        patches_diff("s", &leaf("root")).await;
        patches_diff("s", &leaf("root2")).await;
        patches_diff("s", &leaf("root3")).await;
        let current = patches_revision("s").await;
        assert_eq!(current, ui_contract::UiRevision(3));
        assert!(!semio_framework_ui_runtime::is_stale_intent(ui_contract::UiRevision(2), current, semio_framework_ui_runtime::DEFAULT_REVISION_TOLERANCE), "trailing by exactly the tolerance must still dispatch");
        assert!(semio_framework_ui_runtime::is_stale_intent(ui_contract::UiRevision(1), current, semio_framework_ui_runtime::DEFAULT_REVISION_TOLERANCE), "trailing by 2 must be rejected — no patch, no command");
    }

    /// 👥️ M2 acceptance: a turn where only presence changed touches `PRESENCE` and leaves `PATCHES`
    /// completely untouched — no revision bump, no patch, because the two channels share no code
    /// path (by construction: `stamp_and_cache_interaction_ui` writes `pending_presence`, never
    /// `PENDING_PATCHES`/`PATCHES`).
    #[semio_framework_async_macros::async_test]
    async fn a_presence_only_turn_emits_presence_and_zero_patches() {
        let before = patches_revision("presence-only").await;
        presence_record_own("presence-only", "row-1", ui_contract::OwnPresence { selected: true, ..Default::default() }, 4_000).await;
        let updates = presence_expire_and_flush(0).await;
        assert_eq!(updates.len(), 1);
        assert!(updates[0].own.selected);
        // 🩹️ Zero ui_patches: the surface's revision is EXACTLY what it was before — nothing was ever
        // diffed against `PATCHES` for it, so there is nothing for a subsequent `poll` to have sent.
        assert_eq!(patches_revision("presence-only").await, before);
    }

    /// 👥️ M2 acceptance: a burst of same-key presence writes between two flushes coalesces to ONE
    /// update per `(surface, node_key)` — the property `PresenceHub` itself guarantees, exercised
    /// here through the reactor's own `PRESENCE` wiring rather than the hub in isolation.
    #[semio_framework_async_macros::async_test]
    async fn a_burst_of_same_key_presence_writes_between_polls_coalesces_to_one_update() {
        let mark = |selected: bool, hovered: bool| ui_contract::PeerMark { actor: "user:bob#s1".into(), color: Some(2), hovered, selected, label: "user:bob#s1".into() };
        presence_record_peer("burst", "row-1", mark(false, true), 4_000, 0).await;
        presence_record_peer("burst", "row-1", mark(true, true), 4_000, 10).await;
        presence_record_peer("burst", "row-1", mark(true, false), 4_000, 20).await;
        let updates = presence_expire_and_flush(20).await;
        assert_eq!(updates.len(), 1, "a burst on one key must cost exactly one update, got {updates:?}");
        assert_eq!(updates[0].peers.len(), 1);
        assert!(updates[0].peers[0].selected);
        assert!(!updates[0].peers[0].hovered, "must reflect the LAST write, not the first");
    }

    /// 👥️ M2 acceptance: a peer mark not refreshed within its TTL ages out with no goodbye message —
    /// the flush after expiry reports the now-empty peer list once, then the slot is forgotten.
    #[semio_framework_async_macros::async_test]
    async fn ttl_expiry_drops_a_peer_mark_with_no_goodbye_message() {
        let mark = ui_contract::PeerMark { actor: "user:carol#s1".into(), color: Some(4), hovered: true, selected: false, label: "user:carol#s1".into() };
        presence_record_peer("ttl", "row-1", mark, 1_000, 0).await;
        let first = presence_expire_and_flush(0).await;
        assert_eq!(first[0].peers.len(), 1);
        let after_expiry = presence_expire_and_flush(1_000).await;
        assert_eq!(after_expiry.len(), 1, "expiry at the TTL boundary must surface as one more update");
        assert!(after_expiry[0].peers.is_empty(), "the expired peer must be omitted, not sent as a goodbye");
        assert!(presence_expire_and_flush(2_000).await.is_empty(), "the now-empty slot was garbage-collected");
    }
}
//#endregion 🧪️M1M2ReactorTests

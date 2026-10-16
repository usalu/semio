//! ⚛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2, design-abi.md §4): the turn loop —
//! `reactor::poll`'s real implementation. Ties together `🧵️executor` (async task scheduling),
//! `🌐host/📮️requests` (the host-effect request/completion registry, owned by the `Host` handle), `🩹️patches` (revisioned UI diffing),
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
#[path = "🎟️context/🦀️.rs"]
pub mod original_actor_context;
#[path = "📥️cold-pair/🦀️.rs"]
mod cold_pair;
pub use semio_framework_plugin_actor_host::executor;
#[path = "🚪️lifetime/🦀️.rs"]
pub(crate) mod instance_lifetime;
#[path = "💼️jobs/🦀️.rs"]
pub mod jobs;
#[path = "🩹️patches/🦀️.rs"]
pub mod patches;
#[path = "📨️pending/🦀️.rs"]
mod pending;
use crate::host::requests;

pub(crate) use semio_framework_plugin_plugin_program::surface_context;

// 🧬️ Only `wit_bridge` below (wasm32-wasip2) consumes these —
// a plain native build never reaches the WIT-boundary translation code, so unlike `RefCell` these
// two must be gated identically to `wit_bridge` itself or they warn as unused on native.
use semio_framework::kernel::{ActorInstanceLifecycleReceipt, ActorUiPatchReceipt, Effect, Event, MessageEndpoint, TurnStatus, UiPatch};
// 🧬️ Same gating rationale as the `kernel` import above: only the WIT-boundary code below names the
// semantic-UI contract types (`UiIntent`, `UiRevision`, `Activity`), so an ungated alias warns as
// unused on native. ALSO enabled under `cfg(test)` (M2, ticket 26/08/17 `design-unified.md`): the
// native `reactor_driver` module below (behind its own `#[cfg(test)]`) exercises `PATCHES`/`PRESENCE`
// directly with real `ui_contract` values — `wit_bridge` still cannot run under `cargo test`
// (wasm32-wasip2-only), but its own type vocabulary can be reused for a native fixture.
use semio_framework_ui_contract as ui_contract;
// 🧬️ Same gating rationale as `ui_contract` above: `is_stale_intent`/`DEFAULT_REVISION_TOLERANCE`
// are only named inside `wit_bridge::poll`'s intent-batching loop (M1, ticket 26/08/17
// `design-unified.md`) — gated identically to that module so an ungated alias never warns as unused
// on native.
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
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
/// 🖼️ Code of the fault ONE mounted surface publishes when its own retained render terminated. The
/// host reads it to scope the report to that surface instead of failing the turn that happened to
/// carry it (`🔌️PluginRuntime/🟦️.tsx`, `SURFACE_RENDER_FAULT`).
pub const SURFACE_RENDER_FAULT_CODE: &str = "ui.surface-render";

fn shell_fault_effect(instance: u32, fault: &semio_framework::Fault) -> Effect {
    let fault = store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(fault));
    let frame = protocol::AppFrame::Error { in_reply_to: None, fault, report: Vec::new() };
    Effect::SendMessage { target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) }, payload: ::semio_framework_async::poll::resolve_ready(protocol::encode_app_frame(&frame)) }
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
    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): the actor-local executor
    /// every `AsyncTask` `spawn_task` admits runs on — the COLD half of this actor's two
    /// executors, for futures that park on a `RequestFuture` across host turns rather than
    /// burning a bounded work budget like `REACTOR_EXECUTOR`'s reducer/job tasks do. Driven once
    /// per turn by `turn::poll`, woken by `Event::Timer` and by `RequestRegistry::resolve`.
    static TASK_EXECUTOR: executor::ColdFutureExecutor = executor::ColdFutureExecutor::new();
    /// 🪪️ Every instance this actor currently has open — `(id, app_id)`, in `InstanceOpen` order.
    /// Used by `📸️checkpoint`.
    static INSTANCE_METADATA: RefCell<InstanceMetadataRegistry> = RefCell::new(InstanceMetadataRegistry::new());
    /// 📦️ The package assets the host preloaded on `Event::InstanceOpen.assets`, by declared name — one set per
    /// guest process, because a package declares the same assets for every instance it opens. See [`package_asset`].
    static PACKAGE_ASSETS: RefCell<Vec<(String, &'static [u8])>> = RefCell::new(Vec::new());
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
    /// 🛬️ The cold pair whose document still loads through the stepped archive load, stepped once per turn.
    static COLD_PAIR_DOCUMENT_LOAD: RefCell<Option<cold_pair::ColdPairDocumentLoad>> = RefCell::new(None);
    /// 📸️ Instances whose checkpoint-restored document still loads under `checkpoint::RESTORE_DOCUMENT_LOAD_OPERATION`,
    /// each stepped once per turn.
    static RESTORED_DOCUMENT_LOADS: RefCell<Vec<u32>> = RefCell::new(Vec::new());
}
#[path = "⏲️tasks/🧮️fixed-slots/🦀️.rs"]
mod tasks_fixed_slots;
pub use tasks_fixed_slots::*;
#[path = "⏲️tasks/📇️instance-metadata/🦀️.rs"]
mod tasks_instance_metadata;
pub use tasks_instance_metadata::*;
#[path = "⏲️tasks/🧵️task-lane/🦀️.rs"]
mod tasks_task_lane;
pub use tasks_task_lane::*;
#[path = "⏲️tasks/🔗️extension-continuation/🦀️.rs"]
mod tasks_extension_continuation;
pub use tasks_extension_continuation::*;
#[path = "⏲️tasks/🧷️render-bindings/🦀️.rs"]
mod tasks_render_bindings;
pub use tasks_render_bindings::*;
#[path = "📨️pending/🔒️reactor-close/🦀️.rs"]
mod life_reactor_close;
pub use life_reactor_close::*;
#[path = "🏃️runtime/📦️instances/🏷️metadata-owner/🦀️.rs"]
mod inst_metadata_owner;
pub use inst_metadata_owner::*;
#[path = "🔄️turn/🏁️close-step/🦀️.rs"]
mod turn_close_step;
pub use turn_close_step::*;
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
#[path = "🧬️wit-bridge/🪢️bridge/🦀️.rs"]
mod wit_bridge;

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
    pub(crate) allocation_admitted: bool,
}

impl FixedTimerRegistry {
    pub(crate) fn new() -> Self {
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

    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
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

    pub(crate) fn is_empty(&self) -> bool {
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


/// 📦️ Keeps the assets one `Event::InstanceOpen` delivered so app code can read them with [`package_asset`]. The first
/// delivery of a name wins: a package declares the same bytes for every instance it opens, so a later open repeating a
/// name changes nothing. The bytes live for the guest process, like the package that declared them.
pub(crate) fn admit_package_assets(assets: Vec<(String, Vec<u8>)>) {
    PACKAGE_ASSETS.with(|admitted| {
        let mut admitted = admitted.borrow_mut();
        for (name, bytes) in assets {
            if !admitted.iter().any(|(existing, _)| *existing == name) {
                admitted.push((name, Box::leak(bytes.into_boxed_slice())));
            }
        }
    });
}

/// 📦️ The bytes of the package asset `name` as the host delivered them on `Event::InstanceOpen.assets` (`📓️design-abi.md` §2,
/// the `read-asset` replacement), or `None` when the host delivered no asset of that name. Read-only; valid for the
/// guest process. An app reads it while it is created, which is after the open event's assets are admitted and before
/// the app serves its first request.
pub fn package_asset(name: &str) -> Option<&'static [u8]> {
    PACKAGE_ASSETS.with(|admitted| admitted.borrow().iter().find(|(existing, _)| existing == name).map(|(_, bytes)| *bytes))
}

/// 🧪️ What `Event::InstanceOpen` does with its assets, for an app test that has no component host to deliver them: admits
/// `assets` exactly as the open event would.
#[cfg(any(test, feature = "artifact-app-testing"))]
pub fn admit_package_assets_for_testing(assets: Vec<(String, Vec<u8>)>) {
    admit_package_assets(assets);
}

#[cfg(test)]
#[path = "🧪️tests/📦️package-assets/🦀️.rs"]
mod package_asset_tests;

/// 🔗️ The close participants the reactor core owns above the lifecycle, the patch tracker and the cold document pair ingress: the
/// lifecycle owner drives them through its [`instance_lifetime::ClosePeers`] port, which the turn supplies on every call.
pub(crate) struct ReactorClosePeers;

impl instance_lifetime::ClosePeers for ReactorClosePeers {
    fn preflight_patches(&self, key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
        PATCHES.with(|patches| patches.preflight_close_instance(key)).map_err(reactor_close_fault)
    }

    fn reserve_patches(&self, key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
        PATCHES.with(|patches| patches.reserve_close_instance(key)).map_err(reactor_close_fault)
    }

    fn activate_patches(&self, key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
        PATCHES.with(|patches| patches.activate_close_instance(key)).map_err(reactor_close_fault)
    }

    fn patches_complete(&self, key: instance_lifetime::NativeCloseKey) -> Result<bool, semio_framework::Fault> {
        PATCHES.with(|patches| patches.close_instance_complete(key)).map_err(reactor_close_fault)
    }

    fn release_patches(&self, key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
        PATCHES.with(|patches| patches.release_close_instance(key)).map_err(reactor_close_fault)
    }

    fn preflight_cold_pair(&self, key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
        COLD_PAIR_INGRESS.with(|ingress| ingress.try_borrow().map_err(|_| reactor_close_fault("cold pair close preflight busy"))?.preflight_close_instance(key).map_err(reactor_close_fault))
    }

    fn reserve_cold_pair(&self, key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
        COLD_PAIR_INGRESS.with(|ingress| ingress.try_borrow_mut().map_err(|_| reactor_close_fault("cold pair close reservation busy"))?.reserve_close_instance(key).map_err(reactor_close_fault))
    }

    fn activate_cold_pair(&self, key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
        COLD_PAIR_INGRESS.with(|ingress| ingress.try_borrow_mut().map_err(|_| reactor_close_fault("cold pair close activation busy"))?.activate_close_instance(key).map_err(reactor_close_fault))
    }

    fn cold_pair_complete(&self, key: instance_lifetime::NativeCloseKey) -> Result<bool, semio_framework::Fault> {
        COLD_PAIR_INGRESS.with(|ingress| ingress.try_borrow().map_err(|_| "cold pair close receipt busy")?.close_instance_complete(key)).map_err(reactor_close_fault)
    }

    fn release_cold_pair(&self, key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
        COLD_PAIR_INGRESS.with(|ingress| ingress.try_borrow_mut().map_err(|_| "cold pair close release busy")?.release_close_instance(key)).map_err(reactor_close_fault)
    }
}

/// 📸️ `checkpoint::checkpoint` body — unconditional (no WIT type in its signature, only
/// `Vec<u8>`/kernel types), unlike `poll`/the `wit_*`/`kernel_*_to_wit` bridge below.
///
/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): the task itself is never
/// serialized (`TASK_RECORDS`/`EXECUTOR` are process memory, not pack state) — only the
/// `restart` command bytes of every LIVE task that declared one via `.restartable(..)` survive
/// into the pack, one `TaskRestart{instance, command}` per such task.
pub async fn checkpoint_now<PA: crate::plugin_app::PluginApp>(runtime: &crate::plugin_runtime::PluginRuntime<PA>) -> Result<Vec<u8>, semio_framework::Fault> {
    let instances = INSTANCE_METADATA.with(|metadata| metadata.borrow().checkpoint_rows());
    let timers = ARMED_TIMERS.with(|timers| timers.borrow().rows());
    let pending = REGISTRY.with(|registry| registry.pending_ids().into_iter().map(|id| id.0).collect());
    let task_restarts: Vec<checkpoint::TaskRestart> =
        TASK_RECORDS.with(|records| records.borrow().iter().filter_map(|(_, record)| record.restart.as_ref().map(|command| checkpoint::TaskRestart { instance: record.instance, command: command.clone() })).collect());
    checkpoint::checkpoint(runtime, &instances, timers, pending, task_restarts).await
}

/// 📸️ `checkpoint::restore` body — re-arms the timer list from the restored pack and hands every admitted document load
/// to the turn, which drives it to `Ready` (`📓️api-stepped-document-load.md` §4);
/// `pending_requests` are intentionally NOT re-parked (design-abi.md §4: async tasks are marked
/// re-run-on-restore, not resumed as though the host round-trip were still in flight).
/// `task_restarts` ARE re-dispatched, though not synchronously here: each one is queued onto
/// `TASK_RESUMES` as an ordinary `Command` resume (the SAME resume path a live task's own
/// `TaskResolution::Command` takes), drained by the first `poll` after restore — restoring is a
/// pure state-load, it must not itself re-enter app dispatch.
#[expect(clippy::result_large_err, reason = "The fixed resume queue returns the admitted view snapshot, task metadata, and payload owner intact without allocating on rejection.")]
pub async fn restore_now<PA: crate::plugin_app::PluginApp>(runtime: &crate::plugin_runtime::PluginRuntime<PA>, state: &[u8]) -> Result<(), semio_framework::Fault> {
    let mut observe = |_: semio_framework_value::native_encoding::NativeEncodeProgress| true;
    let observer: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_> = &mut observe;
    let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(checkpoint::RESTORE_AUTHORITY_BYTES, observer).map_err(semio_framework::FaultFrom::into_fault)?;
    let mut accept = |_| true;
    let mut native = semio_framework_value::NativeDecodeControl::new(checkpoint::RESTORE_AUTHORITY_BYTES, &mut accept);
    let mut original = semio_framework_os_kernel::io::control::NativeSnapshotDecodeOwner::new(&mut native, runtime.mounted_owner_policy().preparation);
    let checkpoint::RestoredCheckpoint { pack, document_loads } = checkpoint::restore(runtime, state, &mut identity, &mut original).await?;
    RESTORED_DOCUMENT_LOADS.with(|loads| loads.borrow_mut().extend(document_loads));
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
        let meta = crate::app_emission::ActionMeta { actor: crate::plugin_runtime::instance_actor(runtime, restart.instance).await?.into(), instance_id: restart.instance, view_state: None };
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
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
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
/// simply do not exist outside a wasm32-wasip2
/// build (mirrors the OLD `host_port`'s per-function `#[cfg(...)]` pattern, just hoisted to one
/// module instead of repeated per function).
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub use wit_bridge::{poll, stage_cold_pair_page, stage_command_page};

#[path = "🔄️turn/🦀️.rs"]
mod turn;
pub use turn::poll_kernel;
pub use turn::retained_command_ingress_occupancy;
pub use turn::retained_extension_invocation_occupancy;
pub use turn::{last_turn_more_work_sources, TurnMoreWorkSources};

/// 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: test-only hooks into this actor's UNGATED
/// per-actor state (`EXECUTOR`/`REGISTRY`/`TASK_RESUMES`) — `poll`'s real event routing
/// (`Event::Completed` → `REGISTRY::resolve`, `run_until_idle`, `drain_task_resumes`) lives inside
/// `wit_bridge`, which is wasm32-wasip2-only and cannot be exercised by a native `cargo test`. This
/// module gives native tests (this crate's own, and `plugin_runtime`'s `test_push_instance`-backed
/// integration tests) the SAME three primitives `poll` itself drives, so the mechanism — spawn,
/// park, get resolved by an injected completion, resume — is exercised end-to-end without needing
/// a wasm32-wasip2 build. `pub(crate)`, `#[cfg(test)]`-gated: never part of the real API surface.
#[cfg(test)]
#[path = "🧪️tests/⚛️reactor-driver/🦀️.rs"]
pub(crate) mod reactor_driver;

//#region 🧪️M1M2ReactorTests
/// 🎯️👥️ M1/M2 (ticket 26/08/17 `design-unified.md`) acceptance, driven through `reactor_driver`'s
/// direct hooks into `PATCHES`/`PRESENCE` — the same two thread-locals `poll`'s real intent-batching
/// and dirty-render loops touch, exercised without needing a wasm32-wasip2 build (`poll` itself,
/// gated to `wit_bridge`, cannot run under a native `cargo test` — see `reactor_driver`'s own doc).
#[cfg(test)]
#[path = "🧪️tests/🔬️m1-m2-reactor/🦀️.rs"]
mod m1_m2_reactor_tests;
//#endregion 🧪️M1M2ReactorTests

#[cfg(test)]
#[path = "🧪️tests/🔬️reconcile-budget/🦀️.rs"]
mod reconcile_budget_tests;

//#region 🧪️ReconcileSpinTests
/// 🔁️ The crossing-count laws of the retained-surface publication ladder: what arms a turn, how many
/// host round trips N pending surfaces cost, and that neither the wall hold nor a cancellation mid
/// drive is violated. See `📓️reactor-reconcile-spin-2026-09-14.md`.
#[cfg(test)]
#[path = "🧪️tests/🔬️reconcile-spin/🦀️.rs"]
mod reconcile_spin_tests;
//#endregion 🧪️ReconcileSpinTests

//#region 🧪️TurnPatchBatchTests
/// 🧺️ The batching laws of the turn patch page: N surfaces that are ready together cost TWO
/// crossings, a batch the byte budget cuts still delivers every publication exactly once and in
/// order, the first ready patch is always admitted, and the page never becomes a contiguous guest
/// request past the declared ceiling. See `📓️ui-turn-patch-batching-2026-09-15.md`.
#[cfg(test)]
#[path = "🧪️tests/🧺️turn-patch-batch/🦀️.rs"]
mod turn_patch_batch_tests;
//#endregion 🧪️TurnPatchBatchTests

//#region 🧪️MoreWorkDriveTests
/// 🚚️ The laws of the WORKER-owned `MoreWork` drive: k silent turns cost one host crossing, a pending
/// host input interrupts the drive within one turn, a cancellation ends it on the turn it lands, the
/// drive's budget is derived from the MEASURED turn cost rather than from the reactor's own executor
/// slice, and readiness is per allocation rather than global. See
/// `📓️worker-more-work-drive-2026-09-15.md`.
#[cfg(test)]
#[path = "🧪️tests/🚚️more-work-drive/🦀️.rs"]
mod more_work_drive_tests;
//#endregion 🧪️MoreWorkDriveTests

//#region 🧪️ExtensionContinuationTests
#[cfg(test)]
#[path = "🧪️tests/🔬️extension-continuation/🦀️.rs"]
mod extension_continuation_tests;
//#endregion 🧪️ExtensionContinuationTests

#[cfg(test)]
#[path="🎟️context/🌉️call/🧪️tests/🔌️registry/🦀️.rs"]
mod original_registered_job_loan_tests;

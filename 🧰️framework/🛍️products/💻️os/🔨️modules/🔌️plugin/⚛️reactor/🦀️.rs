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

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
use semio_framework_plugin_reactor_tasks::tasks_task_lane::PendingResume;
use semio_framework_plugin_reactor_tasks::tasks_fixed_slots::ReactorFixedSlots;
use semio_framework_plugin_reactor_tasks::tasks_task_lane::TaskResumeOutcome;
use semio_framework_plugin_reactor_lifecycle::life_reactor_close::reactor_close_fault;
pub use semio_framework_plugin_reactor_surfaces::checkpoint;
pub use semio_framework_plugin_actor_host::executor;
pub use semio_framework_plugin_reactor_lifecycle::instance_lifetime;
pub use semio_framework_plugin_reactor_patches as patches;
use semio_framework_plugin_reactor_lifecycle as pending;

pub use semio_framework_plugin_plugin_program::surface_context;

// 🧬️ Only `wit_bridge` below (wasm32-wasip2) consumes these —
// a plain native build never reaches the WIT-boundary translation code, so unlike `RefCell` these
// two must be gated identically to `wit_bridge` itself or they warn as unused on native.
use semio_framework::kernel::{Effect, MessageEndpoint};
/// 👥️ M2 (ticket 26/08/17 `design-unified.md`): standalone — needs no `EntityStore`/`UiRuntime`, just
/// `record_own`/`record_peer`/`expire`/`flush` — so unlike the two imports directly above, this one
/// is ungated: the `PRESENCE` thread_local right below references it on EVERY build, matching how
/// `patches::PatchTracker` (an equally wit_bridge-only consumer) is reached through the ungated
/// `pub mod patches;` at this file's top.
use semio_framework_ui_runtime::PresenceHub;
use std::cell::RefCell;

#[doc(hidden)]
pub const RECONCILE_STEP_OPPORTUNITY_LIMIT: u64 = 1_024;

//#region 📬️ShellFaultFrame
/// 🖼️ Code of the fault ONE mounted surface publishes when its own retained render terminated. The
/// host reads it to scope the report to that surface instead of failing the turn that happened to
/// carry it (`🔌️PluginRuntime/🟦️.tsx`, `SURFACE_RENDER_FAULT`).
pub const SURFACE_RENDER_FAULT_CODE: &str = "ui.surface-render";

#[doc(hidden)]
pub fn shell_fault_effect(instance: u32, fault: &semio_framework::Fault) -> Effect {
    let fault = store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(fault));
    let frame = protocol::AppFrame::Error { in_reply_to: None, fault, report: Vec::new() };
    Effect::SendMessage { target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) }, payload: ::semio_framework_async::poll::resolve_ready(protocol::encode_app_frame(&frame)) }
}

//#endregion 📬️ShellFaultFrame

#[doc(hidden)]
pub fn reconcile_step_opportunities(fuel: u64) -> usize {
    usize::try_from(fuel.min(RECONCILE_STEP_OPPORTUNITY_LIMIT)).unwrap_or(RECONCILE_STEP_OPPORTUNITY_LIMIT as usize).max(1)
}

semio_framework_plugin_abi::component_persistent_local! {
    /// 👥️ Collects presence by concrete mounted surface and node, bound atomically with each render.
    pub static PRESENCE: RefCell<PresenceHub> = RefCell::new(PresenceHub::new());
    /// 📦️ The package assets the host preloaded on `Event::InstanceOpen.assets`, by declared name — one set per
    /// guest process, because a package declares the same assets for every instance it opens. See [`package_asset`].
    pub static PACKAGE_ASSETS: RefCell<Vec<(String, &'static [u8])>> = RefCell::new(Vec::new());
    /// ⏱️ Live timer ids this actor has armed via `Effect::SetTimer`, carried into the checkpoint
    /// pack (design-abi.md §4).
    pub static ARMED_TIMERS: RefCell<FixedTimerRegistry> = RefCell::new(FixedTimerRegistry::new());
}
pub use semio_framework_plugin_reactor_surfaces::{COLD_PAIR_DOCUMENT_LOAD, COLD_PAIR_INGRESS, RESTORED_DOCUMENT_LOADS};
pub use semio_framework_plugin_reactor_lifecycle::{REACTOR_CLOSES, REACTOR_CLOSE_CURSOR};
pub use semio_framework_plugin_reactor_patches::PATCHES;
pub use semio_framework_plugin_reactor_tasks::{INSTANCE_METADATA, JOB_RENDER_BINDINGS, REACTOR_EXECUTOR, REGISTRY, TASK_EXECUTOR, TASK_RECORDS, TASK_RESUMES};
pub use semio_framework_plugin_reactor_tasks::tasks_fixed_slots::*;
pub use semio_framework_plugin_reactor_tasks::tasks_instance_metadata::*;
pub use semio_framework_plugin_reactor_tasks::tasks_task_lane::*;
pub use semio_framework_plugin_reactor_tasks::tasks_extension_continuation::*;
pub use semio_framework_plugin_reactor_tasks::tasks_render_bindings::*;
pub use semio_framework_plugin_reactor_lifecycle::life_reactor_close::*;
pub use semio_framework_plugin_runtime_instances::inst_metadata_owner::*;

#[doc(hidden)]
pub const REACTOR_TIMER_SLOTS: usize = 1_024;

struct TimerEntry {
    pub id: u64,
    pub instance: u32,
    pub previous: Option<usize>,
    next: Option<usize>,
}

#[doc(hidden)]
pub struct FixedTimerRegistry {
    pub slots: ReactorFixedSlots<TimerEntry>,
    pub head: Option<usize>,
    pub tail: Option<usize>,
    pub live: usize,
    pub allocation_admitted: bool,
}

impl FixedTimerRegistry {
    pub fn new() -> Self {
        let slots = ReactorFixedSlots::new();
        let allocation_admitted = slots.allocation_admitted;
        Self { slots, head: None, tail: None, live: 0, allocation_admitted }
    }

    pub fn index(id: u64) -> usize {
        id as usize % REACTOR_TIMER_SLOTS
    }

    pub fn insert(&mut self, instance: u32, id: u64) -> Result<(), u64> {
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

    pub fn remove(&mut self, id: u64) -> bool {
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

    pub fn first(&self) -> Option<u64> {
        self.head.and_then(|index| self.slots.get(index)).map(|entry| entry.id)
    }

    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    pub fn contains(&self, id: u64) -> bool {
        self.slots.get(Self::index(id)).is_some_and(|entry| entry.id == id)
    }

    pub fn rows(&self) -> Vec<u64> {
        let mut rows = Vec::with_capacity(self.live);
        let mut cursor = self.head;
        while let Some(index) = cursor {
            let Some(entry) = self.slots.get(index) else { break };
            rows.push(entry.id);
            cursor = entry.next;
        }
        rows
    }

    pub fn is_empty(&self) -> bool {
        self.live == 0
    }

    pub fn cancel_instance_step(&mut self, instance: u32, cursor: &mut usize) -> bool {
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
#[doc(hidden)]
pub fn admit_package_assets(assets: Vec<(String, Vec<u8>)>) {
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


/// 🔗️ The close participants the reactor core owns above the lifecycle, the patch tracker and the cold document pair ingress: the
/// lifecycle owner drives them through its [`instance_lifetime::ClosePeers`] port, which the turn supplies on every call.
#[doc(hidden)]
pub struct ReactorClosePeers;

impl semio_framework_plugin_reactor_lifecycle::instance_lifetime::ClosePeers for ReactorClosePeers {
    fn preflight_patches(&self, key: semio_framework_plugin_reactor_lifecycle::instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
        PATCHES.with(|patches| patches.preflight_close_instance(key)).map_err(reactor_close_fault)
    }

    fn reserve_patches(&self, key: semio_framework_plugin_reactor_lifecycle::instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
        PATCHES.with(|patches| patches.reserve_close_instance(key)).map_err(reactor_close_fault)
    }

    fn activate_patches(&self, key: semio_framework_plugin_reactor_lifecycle::instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
        PATCHES.with(|patches| patches.activate_close_instance(key)).map_err(reactor_close_fault)
    }

    fn patches_complete(&self, key: semio_framework_plugin_reactor_lifecycle::instance_lifetime::NativeCloseKey) -> Result<bool, semio_framework::Fault> {
        PATCHES.with(|patches| patches.close_instance_complete(key)).map_err(reactor_close_fault)
    }

    fn release_patches(&self, key: semio_framework_plugin_reactor_lifecycle::instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
        PATCHES.with(|patches| patches.release_close_instance(key)).map_err(reactor_close_fault)
    }

    fn preflight_cold_pair(&self, key: semio_framework_plugin_reactor_lifecycle::instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
        COLD_PAIR_INGRESS.with(|ingress| ingress.try_borrow().map_err(|_| reactor_close_fault("cold pair close preflight busy"))?.preflight_close_instance(key).map_err(reactor_close_fault))
    }

    fn reserve_cold_pair(&self, key: semio_framework_plugin_reactor_lifecycle::instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
        COLD_PAIR_INGRESS.with(|ingress| ingress.try_borrow_mut().map_err(|_| reactor_close_fault("cold pair close reservation busy"))?.reserve_close_instance(key).map_err(reactor_close_fault))
    }

    fn activate_cold_pair(&self, key: semio_framework_plugin_reactor_lifecycle::instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
        COLD_PAIR_INGRESS.with(|ingress| ingress.try_borrow_mut().map_err(|_| reactor_close_fault("cold pair close activation busy"))?.activate_close_instance(key).map_err(reactor_close_fault))
    }

    fn cold_pair_complete(&self, key: semio_framework_plugin_reactor_lifecycle::instance_lifetime::NativeCloseKey) -> Result<bool, semio_framework::Fault> {
        COLD_PAIR_INGRESS.with(|ingress| ingress.try_borrow().map_err(|_| "cold pair close receipt busy")?.close_instance_complete(key)).map_err(reactor_close_fault)
    }

    fn release_cold_pair(&self, key: semio_framework_plugin_reactor_lifecycle::instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
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
pub async fn checkpoint_now<PA: semio_framework_plugin_reactor_app::PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>) -> Result<Vec<u8>, semio_framework::Fault> {
    let instances = INSTANCE_METADATA.with(|metadata| metadata.borrow().checkpoint_rows());
    let timers = ARMED_TIMERS.with(|timers| timers.borrow().rows());
    let pending = REGISTRY.with(|registry| registry.pending_ids().into_iter().map(|id| id.0).collect());
    let task_restarts: Vec<semio_framework_plugin_reactor_surfaces::checkpoint::TaskRestart> =
        TASK_RECORDS.with(|records| records.borrow().iter().filter_map(|(_, record)| record.restart.as_ref().map(|command| semio_framework_plugin_reactor_surfaces::checkpoint::TaskRestart { instance: record.instance, command: command.clone() })).collect());
    semio_framework_plugin_reactor_surfaces::checkpoint::checkpoint(runtime, &instances, timers, pending, task_restarts).await
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
pub async fn restore_now<PA: semio_framework_plugin_reactor_app::PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, state: &[u8]) -> Result<(), semio_framework::Fault> {
    let mut observe = |_: semio_framework_value::native_encoding::NativeEncodeProgress| true;
    let observer: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_> = &mut observe;
    let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(semio_framework_plugin_reactor_surfaces::checkpoint::RESTORE_AUTHORITY_BYTES, observer).map_err(semio_framework::FaultFrom::into_fault)?;
    let mut accept = |_| true;
    let mut native = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(semio_framework_plugin_reactor_surfaces::checkpoint::RESTORE_AUTHORITY_BYTES, &mut accept);
    let mut original = semio_framework_os_kernel::io::control::NativeSnapshotDecodeOwner::new(&mut native, runtime.mounted_owner_policy().preparation);
    let semio_framework_plugin_reactor_surfaces::checkpoint::RestoredCheckpoint { pack, document_loads } = semio_framework_plugin_reactor_surfaces::checkpoint::restore(runtime, state, &mut identity, &mut original).await?;
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
        let meta = semio_framework_plugin_decl_emission::ActionMeta { actor: semio_framework_plugin_runtime_instances::inst_wire_frames::instance_actor(runtime, restart.instance).await?.into(), instance_id: restart.instance, view_state: None };
        let pending = PendingResume { instance: restart.instance, meta, outcome: TaskResumeOutcome::Command(restart.command.clone()) };
        TASK_RESUMES
            .with(|resumes| resumes.borrow_mut().push(pending))
            .map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.task-resume-capacity"), "fixed task-resume authority is saturated during restore"))?;
    }
    Ok(())
}


//#region 🔁️CommandIngressTerminal
#[doc(hidden)]
pub fn advance_command_cursor(mut cursor: semio_framework::kernel::CommandPageCursor) -> Result<semio_framework::kernel::CommandPageCursor, semio_framework::kernel::CommandPageCursor> {
    let Some(page_index) = cursor.page_index.checked_add(1) else { return Err(cursor) };
    cursor.page_index = page_index;
    Ok(cursor)
}

#[doc(hidden)]
pub fn terminal_command_ingress(cursor: semio_framework::kernel::CommandPageCursor, fault: Option<Vec<u8>>) -> semio_framework::kernel::CommandIngressStatus {
    match advance_command_cursor(cursor) {
        Ok(terminal) => match fault {
            Some(fault) => semio_framework::kernel::CommandIngressStatus::Fault { cursor: terminal, fault },
            None => semio_framework::kernel::CommandIngressStatus::CommandComplete(terminal),
        },
        Err(cursor) => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-index-exhausted".to_vec() },
    }
}

//#endregion 🔁️CommandIngressTerminal





//#region 🧪️M1M2ReactorTests
//#endregion 🧪️M1M2ReactorTests


//#region 🧪️ReconcileSpinTests
//#endregion 🧪️ReconcileSpinTests

//#region 🧪️TurnPatchBatchTests
//#endregion 🧪️TurnPatchBatchTests

//#region 🧪️MoreWorkDriveTests
//#endregion 🧪️MoreWorkDriveTests

//#region 🧪️ExtensionContinuationTests
//#endregion 🧪️ExtensionContinuationTests


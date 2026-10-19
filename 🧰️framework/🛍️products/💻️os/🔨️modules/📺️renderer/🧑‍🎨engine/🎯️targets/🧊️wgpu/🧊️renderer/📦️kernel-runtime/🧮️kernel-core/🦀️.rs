//! 🧮️ Kernel core: sequence authority, job progress presentation, turn budget, extension index, requests and close submissions.
use semio_framework_ui_contract_validation::{UiDocumentLimits};
use super::*;

//#region 🎠️KernelRuntime
#[derive(Default)]
pub struct RendererSequenceAuthority {
    pub committed: u64,
    pub reserved: Option<u64>,
    pub exhausted: bool,
}

impl RendererSequenceAuthority {
    pub fn try_reserve(&mut self) -> Result<u64, String> {
        if self.exhausted || self.reserved.is_some() {
            return Err("renderer sequence authority is exhausted or reserved".to_string());
        }
        let generation = self.committed.checked_add(1).filter(|generation| *generation != 0).ok_or_else(|| "renderer sequence authority is permanently exhausted".to_string())?;
        self.reserved = Some(generation);
        Ok(generation)
    }

    pub fn commit(&mut self, generation: u64) -> Result<(), String> {
        if self.reserved != Some(generation) || self.committed.checked_add(1) != Some(generation) {
            return Err("renderer sequence reservation is stale".to_string());
        }
        self.reserved = None;
        self.committed = generation;
        self.exhausted = generation == u64::MAX;
        Ok(())
    }

    pub fn rollback(&mut self, generation: u64) {
        if self.reserved == Some(generation) {
            self.reserved = None;
        }
    }
}

struct RendererSequenceReservation {
    generation: u64,
    active: bool,
}

impl RendererSequenceReservation {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn commit(mut self) -> Result<u64, String> {
        let mut authority = renderer_sequence_authority().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        authority.commit(self.generation)?;
        self.active = false;
        Ok(self.generation)
    }
}

impl Drop for RendererSequenceReservation {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        let mut authority = renderer_sequence_authority().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        authority.rollback(self.generation);
    }
}

fn renderer_sequence_authority() -> &'static Mutex<RendererSequenceAuthority> {
    static AUTHORITY: OnceLock<Mutex<RendererSequenceAuthority>> = OnceLock::new();
    AUTHORITY.get_or_init(|| Mutex::new(RendererSequenceAuthority::default()))
}

pub fn reserve_seq() -> Result<RendererSequenceReservation, String> {
    let mut authority = renderer_sequence_authority().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let generation = authority.try_reserve()?;
    Ok(RendererSequenceReservation { generation, active: true })
}

pub const JOB_PROGRESS_PRESENTATION_CAPACITY: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum JobProgressPresentationState {
    Vacant,
    Reserved,
    Ready,
    CheckedOut,
    Presented,
}

#[derive(Clone, Copy, Debug)]
struct JobProgressPresentationSlot {
    pub epoch: u64,
    admission_sequence: u64,
    state: JobProgressPresentationState,
    pub identity: JobProgressIdentity,
    kind: JobProgressKind,
    pub applied_progress: u64,
}

impl JobProgressPresentationSlot {
    fn vacant() -> Self {
        Self {
            epoch: 0,
            admission_sequence: 0,
            state: JobProgressPresentationState::Vacant,
            identity: JobProgressIdentity { actor: ActorId(0), job: 0, operation: 0, base_revision: 0, generation: 0, step_sequence: 0, preview_sequence: 0 },
            kind: JobProgressKind::Yield,
            applied_progress: 0,
        }
    }
}

/// 🔌️ The inversion seam of the lease: the kernel pool installs how a presented progress token is handed back to the pool, so this file never names the pool's client.
pub type JobProgressAcknowledge = fn(JobProgressPresentationToken) -> bool;

pub struct JobProgressPresentationBridge {
    pub slots: [JobProgressPresentationSlot; JOB_PROGRESS_PRESENTATION_CAPACITY],
    reserve_cursor: usize,
    next_admission_sequence: u64,
    acknowledge: Option<JobProgressAcknowledge>,
}

impl JobProgressPresentationBridge {
    pub fn new() -> Self {
        Self { slots: [JobProgressPresentationSlot::vacant(); JOB_PROGRESS_PRESENTATION_CAPACITY], reserve_cursor: 0, next_admission_sequence: 1, acknowledge: None }
    }

    pub fn install_acknowledge(&mut self, acknowledge: JobProgressAcknowledge) {
        self.acknowledge = Some(acknowledge);
    }

    pub fn reserve(&mut self, identity: JobProgressIdentity, kind: JobProgressKind, applied_progress: u64) -> Option<JobProgressPresentationToken> {
        let index = (0..JOB_PROGRESS_PRESENTATION_CAPACITY)
            .map(|offset| (self.reserve_cursor + offset) % JOB_PROGRESS_PRESENTATION_CAPACITY)
            .find(|index| self.slots[*index].state == JobProgressPresentationState::Vacant && self.slots[*index].epoch != u64::MAX)?;
        let admission_sequence = self.next_admission_sequence;
        self.next_admission_sequence = self.next_admission_sequence.checked_add(1)?;
        let slot = &mut self.slots[index];
        slot.epoch = slot.epoch.checked_add(1)?;
        slot.admission_sequence = admission_sequence;
        slot.state = JobProgressPresentationState::Reserved;
        slot.identity = identity;
        slot.kind = kind;
        slot.applied_progress = applied_progress;
        self.reserve_cursor = (index + 1) % JOB_PROGRESS_PRESENTATION_CAPACITY;
        Some(JobProgressPresentationToken { index, epoch: slot.epoch })
    }

    pub fn publish(&mut self, token: JobProgressPresentationToken) -> bool {
        let Some(slot) = self.slots.get_mut(token.index) else { return false };
        if slot.epoch != token.epoch || slot.state != JobProgressPresentationState::Reserved {
            return false;
        }
        slot.state = JobProgressPresentationState::Ready;
        true
    }

    pub fn cancel(&mut self, token: JobProgressPresentationToken) -> bool {
        let Some(slot) = self.slots.get_mut(token.index) else { return false };
        if slot.epoch != token.epoch || !matches!(slot.state, JobProgressPresentationState::Reserved | JobProgressPresentationState::Ready) {
            return false;
        }
        slot.state = JobProgressPresentationState::Vacant;
        true
    }

    pub fn can_cancel(&self, token: JobProgressPresentationToken) -> bool {
        self.slots.get(token.index).is_some_and(|slot| slot.epoch == token.epoch && matches!(slot.state, JobProgressPresentationState::Reserved | JobProgressPresentationState::Ready))
    }

    pub fn take(&mut self) -> Option<JobProgressPresentationLease> {
        let index = self.oldest_ready_index()?;
        let slot = &mut self.slots[index];
        slot.state = JobProgressPresentationState::CheckedOut;
        Some(JobProgressPresentationLease { token: JobProgressPresentationToken { index, epoch: slot.epoch }, kind: slot.kind, applied_progress: slot.applied_progress, terminal: false, acknowledge: self.acknowledge })
    }

    fn oldest_ready_index(&self) -> Option<usize> {
        self.slots.iter().enumerate().filter(|(_, slot)| slot.state == JobProgressPresentationState::Ready).min_by_key(|(_, slot)| slot.admission_sequence).map(|(index, _)| index)
    }

    pub fn return_lease(&mut self, token: JobProgressPresentationToken) -> bool {
        let Some(slot) = self.slots.get_mut(token.index) else { return false };
        if slot.epoch != token.epoch || slot.state != JobProgressPresentationState::CheckedOut {
            return false;
        }
        slot.state = JobProgressPresentationState::Ready;
        true
    }

    pub fn presented(&mut self, token: JobProgressPresentationToken) -> bool {
        let Some(slot) = self.slots.get_mut(token.index) else { return false };
        if slot.epoch != token.epoch || slot.state != JobProgressPresentationState::CheckedOut {
            return false;
        }
        slot.state = JobProgressPresentationState::Presented;
        true
    }

    pub fn release_presented(&mut self, token: JobProgressPresentationToken) -> bool {
        let Some(slot) = self.slots.get_mut(token.index) else { return false };
        if slot.epoch != token.epoch || slot.state != JobProgressPresentationState::Presented {
            return false;
        }
        slot.state = JobProgressPresentationState::Vacant;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.slots.iter().all(|slot| slot.state == JobProgressPresentationState::Vacant)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JobProgressPresentationToken {
    pub index: usize,
    pub epoch: u64,
}

pub struct JobProgressPresentationLease {
    pub token: JobProgressPresentationToken,
    kind: JobProgressKind,
    pub applied_progress: u64,
    pub terminal: bool,
    acknowledge: Option<JobProgressAcknowledge>,
}

impl JobProgressPresentationLease {
    pub fn visual(&self) -> (JobProgressKind, u64) {
        (self.kind, self.applied_progress)
    }

    pub fn acknowledge_presented(&mut self) -> bool {
        if self.terminal {
            return false;
        }
        let admitted = self.acknowledge.is_some_and(|acknowledge| acknowledge(self.token));
        self.terminal = admitted;
        admitted
    }
}

impl Drop for JobProgressPresentationLease {
    fn drop(&mut self) {
        if !self.terminal {
            let returned = job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").return_lease(self.token);
            assert!(returned, "job-progress presentation lease must return to its exact generation slot");
        }
    }
}

pub fn job_progress_presentation_bridge() -> &'static Mutex<JobProgressPresentationBridge> {
    static BRIDGE: OnceLock<Mutex<JobProgressPresentationBridge>> = OnceLock::new();
    BRIDGE.get_or_init(|| Mutex::new(JobProgressPresentationBridge::new()))
}

pub fn install_job_progress_acknowledge(acknowledge: JobProgressAcknowledge) {
    job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").install_acknowledge(acknowledge);
}

pub fn take_job_progress_presentation() -> Option<JobProgressPresentationLease> {
    job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").take()
}

pub fn next_seq() -> Result<u64, String> {
    reserve_seq()?.commit()
}

/// 🧾️ Decodes one actor turn back into the kernel's shape. A turn that painted nothing crosses
/// with EMPTY `ui_patches` bytes — the plugin-host bridge (`🧵️shard`'s `to_actor_turn_result`)
/// publishes a transport token only for a non-empty patch owner — so empty bytes are the empty
/// owner, never a malformed token.
pub fn decode_actor_turn_result(result: &semio_framework_actor::TurnResult, session: u64) -> Result<TurnResult, String> {
    let status = match &result.status {
        semio_framework_actor::TurnStatus::Idle => semio_framework::kernel::TurnStatus::Idle,
        semio_framework_actor::TurnStatus::MoreWork => semio_framework::kernel::TurnStatus::MoreWork,
        semio_framework_actor::TurnStatus::CheckpointReady { checkpoint } => semio_framework::kernel::TurnStatus::CheckpointReady { checkpoint: checkpoint.clone() },
        semio_framework_actor::TurnStatus::Faulted { detail } => semio_framework::kernel::TurnStatus::Faulted(detail.clone()),
        status => return Err(format!("kernel: unexpected job status in reactor turn: {status:?}")),
    };
    Ok(TurnResult {
        ui_patches: if result.ui_patches.is_empty() {
            semio_framework::kernel::UiTurnPatches::default()
        } else {
            semio_framework::kernel::UiTurnPatchTransportLease::try_from_token(&result.ui_patches, session)
                .map_err(|error| format!("kernel: decode ui patch transport: {error}"))?
                .take_owner()
                .map_err(|_| "kernel: turn patch transport lease lost its exact owner".to_string())?
        },
        effects: serde_json::from_slice(&result.effects).map_err(|error| format!("kernel: decode effects: {error}"))?,
        presence: Vec::new(),
        next_wake: result.next_wake,
        status,
        fuel_used: result.usage.fuel,
        command_ingress: serde_json::from_slice(&result.command_ingress).map_err(|error| format!("kernel: decode command ingress: {error}"))?,
        cold_pair_ingress: result.cold_pair_ingress.clone(),
        lifecycle_receipt: result.lifecycle_receipt,
        ui_patch_receipt: result.ui_patch_receipt,
    })
}

/// ⛽️ One generous constant turn budget until the DRR scheduler threads a real per-lane one
/// through (same honestly-flagged gap `PluginInstanceHandle`'s `RELAY_JOB_BUDGET` already
/// documents on the host side for jobs — this is its `reactor::poll` turn-budget twin).
pub const TURN_BUDGET: TurnBudget = TurnBudget { fuel: 50_000_000, deadline_ms: 100, max_effects: 64, max_patch_bytes: 1 << 20, max_frames: 8 };

/// 🎟️ The retained grant one job-replay-log step is funded with: the pages, copy, capacity and release of one operation's state and output payloads (the declared actor bridge grant), shared by the pool and the retained-state recovery close.
pub const JOB_REPLAY_STEP_GRANT: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: semio_framework_actor::JOB_REPLAY_PROCESS_PAGE_CAPACITY,
    maximum_copy_bytes: 2 * semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES,
    maximum_capacity_bytes: 2 * semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES,
    maximum_release_bytes: 2 * semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES,
    maximum_depth: 64,
};

/// ⏳️ terra-kernel-loop: same tripwire shape as `scale_bench`'s own `PUMP_OUTCOME_TIMEOUT` —
/// how long `run_turn`'s tick loop waits for a granted turn's `ShardOutcome` before giving up.
pub const RUN_TURN_OUTCOME_TIMEOUT: Duration = Duration::from_secs(5);

/// ⏳️ How long one [`KernelPoolState::run_turn`] request keeps settling a guest that COMPLETED its turn
/// answering `MoreWork`: past it the guest is handed back settled-as-far-as-it-got, never mid-call. A
/// guest preempted MID-call is never bounded by wall-clock time: it yields its slice to the other
/// actors ([`KernelPoolState::yield_slice`]) and only an explicit cancel ends it
/// ([`KernelPoolState::turn_cancelled`]).
pub const RUN_TURN_SETTLE_BUDGET: Duration = Duration::from_secs(30);

/// ⚖️ How many queued requests of other instances run between two slices of a mid-flight turn — the
/// fairness contract's `requestsBetweenSlices` (`🧑‍🎨engine/🧫️fixtures/🧵️kernel-pool-future` `turnFairness`); a
/// slice itself is [`TURN_BUDGET`]'s fuel and wall.
pub const TURN_REQUESTS_BETWEEN_SLICES: usize = 1;

/// 🧩️ How many compiled components the kernel keeps for reuse by content hash — a reopened kind, a second
/// window of one app — before the oldest is dropped (its live instances keep their own handle).
pub const COMPILED_COMPONENT_CAPACITY: usize = 8;

/// 🧪️ How many mid-flight turns their owners cancelled in this test process ([`KernelPoolState::retire_cancelled_turn`]).
#[cfg(any(test, feature = "testkit"))]
pub static CANCELLED_KERNEL_TURNS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// 🤫️ Consecutive continuation turns that carried nothing after which [`KernelPoolState::run_turn`]
/// stops settling a guest that still answers `MoreWork`: that guest is waiting on a host round trip
/// no empty turn delivers. Derived exactly like the React host's `PLUGIN_UI_QUIESCENT_CONTINUATIONS`
/// (`🔌️PluginRuntime/🟦️.tsx`) — the patch pages one surface may take
/// (`max_patch_bytes / max_text_bytes`, `🖱️ui/🧬️contract/🛡️limits`) times its continuation batch of 8.
/// 🧵️ The turns one continuation takes: every owed event (a lifecycle ACK, a typed-operation ACK)
/// on a turn of its own — a guest left owning ingress by the first must not receive the second in
/// the same grant — or one empty-event turn for a guest that only answered `MoreWork`.
pub fn run_turn_continuation_turns(continuation: Option<Vec<Event>>) -> std::collections::VecDeque<Vec<Event>> {
    match continuation {
        None => std::collections::VecDeque::new(),
        Some(events) if events.is_empty() => std::collections::VecDeque::from([Vec::new()]),
        Some(events) => events.into_iter().map(|event| vec![event]).collect(),
    }
}

pub fn run_turn_quiescent_continuations() -> usize {
    let limits = UiDocumentLimits::default();
    limits.max_patch_bytes.div_ceil(limits.max_text_bytes) * 8
}

/// 🧵️ P1e (INTERACTIVE-JOB-RUNTIME-REFACTOR, one-pool-worker-runtime): sized from
/// `semio_framework_async::worker_count_for` — the SAME formula [`crate::renderer_worker_pool`]
/// itself sizes its one process-wide `WorkerPool` from — rather than a fresh ad-hoc formula,
/// keeping "no component sizes itself per-CPU" true even though a shard count is minted before
/// the pool object itself is touched here (`ShardExecutor` count, not thread count — shards are a
/// pure scheduling/affinity unit post-P1c, never a thread per shard). `available_parallelism()`
/// failing (rare; a sandboxed/exotic host) falls back to `4` cores' worth of shards rather than
/// faulting the kernel pool task before it can start.
pub fn native_shard_count() -> u16 {
    let cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    semio_framework_async::worker_count_for(semio_framework_async::ProcessKind::InteractiveNative, cores) as u16
}

//#region 🔖️ExtensionIndex
/// 🧩️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): the descriptor-driven
/// source of truth for "which extensions activate alongside plugin X" — `📓️design-unified.md`
/// §M6. Embedded at COMPILE time via `include_str!` (never a runtime path lookup into `🤖️
/// generated/**`, which is gitignored and has no stable runtime location once packaged) — the
/// same registry `🖥️hosts/🦀️.rs` a few lines above this module already mounts as real Rust source,
/// just read as data here instead of compiled as code. READ-ONLY: this file is `🤖️generated/**`,
/// registrar-owned; never edited by this packet.


/// 🧩️ The handful of fields this host actually needs out of one registry entry — every other
/// field (`hashes`, `dependsOn`, `activationEvents`, …) is irrelevant to activation and left
/// unparsed (serde ignores unknown JSON fields by default).
#[derive(serde::Deserialize)]
struct PluginDescriptorJson {
    #[serde(rename = "pluginId")]
    plugin_id: String,
    role: String,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    extends: Option<String>,
}

/// 🧩️ One extension's activation-relevant identity — `extension_id`/`package` are the SAME
/// string (the extension crate's own `pluginId`, deliberately distinct from its parent's
/// `PackageId` — see `activate_extensions_of`'s own doc for why this matters for package-wide
/// quarantine isolation). `capability_requests` mirrors the registry's `capabilities` array
/// verbatim as unscoped [`CapabilityGrant`]s (`scope: None` — this host has no capability broker
/// populating scopes for ANY actor kind yet, extension or plugin).
#[derive(Clone)]
struct ExtensionRecord {
    pub extension_id: String,
    pub package: PackageId,
    pub capability_requests: Vec<CapabilityGrant>,
}

/// 🧩️ `by_parent[plugin_id]` = every installed extension whose descriptor `extends == plugin_id`
/// — exactly the "descriptors carry `extends`... zero special-casing" design requirement.
struct ExtensionIndex {
    by_parent: HashMap<String, Vec<ExtensionRecord>>,
}

impl ExtensionIndex {
    fn load() -> Self {
        let mut by_parent: HashMap<String, Vec<ExtensionRecord>> = HashMap::new();
        let entries=crate::plugin_catalog::plugin_catalog_v1().plugins.iter().cloned();
        for entry in entries {
            if entry.role != "extension" {
                continue;
            }
            let Some(parent) = entry.extends else { continue };
            let record = ExtensionRecord { extension_id: entry.plugin_id.clone(), package: PackageId(entry.plugin_id), capability_requests: entry.capabilities.into_iter().map(|capability| CapabilityGrant { capability, scope: None }).collect() };
            by_parent.entry(parent).or_default().push(record);
        }
        Self { by_parent }
    }

    pub fn extensions_of(&self, plugin_id: &str) -> &[ExtensionRecord] {
        self.by_parent.get(plugin_id).map(Vec::as_slice).unwrap_or(&[])
    }
}

/// 🧩️ Parsed once, lazily — the registry JSON is fixed at compile time (`include_str!`), so
/// there is nothing to invalidate or re-read across the process's lifetime.
pub fn extension_index() -> &'static ExtensionIndex {
    static INDEX: OnceLock<ExtensionIndex> = OnceLock::new();
    INDEX.get_or_init(ExtensionIndex::load)
}

/// 🧩️ Mirrors `program_bridge::load_wasm_plugins`'s own "first `.wasm` file directly inside the
/// plugin's own directory" convention — kept as a small local helper rather than importing that
/// module's version, which is embedded inside its own `find`-style closure, not a reusable fn.
pub async fn find_wasm_artifact(dir:&std::path::Path)->Result<Option<semio_framework_os_services::OriginalSuppliedNativePath>,crate::RendererSourceIoError>{crate::find_renderer_wasm_path(dir).await}
//#endregion 🔖️ExtensionIndex

//#region 🔖️Requests/Outcomes
#[path="🐣️create-app/🎟️source/🦀️.rs"]
mod create_app_source;
pub use create_app_source::{CreateAppBorrowed,CreateAppRequestOwner,CreateAppSourceFailure,CreateAppSourceFailureKind,open_create_app_source};

#[derive(Clone, Copy)]
pub enum QueuedKernelEventKind {
    SurfaceVisible,
    MessageShell,
    MessageBackbone,
}

pub struct QueuedKernelEvent {
    pub kind: Option<QueuedKernelEventKind>,
    pub surface_visible: Option<String>,
    pub surface_body_key: Option<String>,
    pub surface_view_state: Option<Vec<u8>>,
    pub message_endpoint: Option<String>,
    pub message_payload: Option<Vec<u8>>,
}

impl QueuedKernelEvent {
    pub fn try_from_events(events: Vec<Event>) -> Result<Self, RejectedKernelEvents> {
        let mut events = std::collections::VecDeque::from(events);
        if events.len() != 1 {
            return Err(RejectedKernelEvents { events });
        }
        match events.pop_front().expect("one queued event is present") {
            Event::SurfaceVisible { surface, body_key, view_state } => {
                Ok(Self { kind: Some(QueuedKernelEventKind::SurfaceVisible), surface_visible: Some(surface), surface_body_key: Some(body_key), surface_view_state: Some(view_state), message_endpoint: None, message_payload: None })
            }
            Event::Message { source: MessageEndpoint::Shell { instance }, payload } if !payload.is_empty() && payload.len() <= semio_framework_plugin::document_backbone_binding::DOCUMENT_BACKBONE_BINDING_CONTROL_MAXIMUM_BYTES => {
                Ok(Self { kind: Some(QueuedKernelEventKind::MessageShell), surface_visible: None, surface_body_key: None, surface_view_state: None, message_endpoint: Some(instance.0), message_payload: Some(payload) })
            }
            Event::Message { source: MessageEndpoint::Backbone { uri }, payload } if !payload.is_empty() && payload.len() <= store_sync::os_store::backbone::BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES => {
                Ok(Self { kind: Some(QueuedKernelEventKind::MessageBackbone), surface_visible: None, surface_body_key: None, surface_view_state: None, message_endpoint: Some(uri), message_payload: Some(payload) })
            }
            rejected => {
                events.push_front(rejected);
                Err(RejectedKernelEvents { events })
            }
        }
    }

    pub fn into_event(mut self) -> Event {
        match self.kind.take().expect("queued event kind is present") {
            QueuedKernelEventKind::SurfaceVisible => Event::SurfaceVisible {
                surface: self.surface_visible.take().expect("queued surface-visible surface is present"),
                body_key: self.surface_body_key.take().expect("queued surface-visible body key is present"),
                view_state: self.surface_view_state.take().expect("queued surface-visible view state is present"),
            },
            QueuedKernelEventKind::MessageShell => Event::Message {
                source: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(self.message_endpoint.take().expect("queued shell-message instance is present")) },
                payload: self.message_payload.take().expect("queued shell-message payload is present"),
            },
            QueuedKernelEventKind::MessageBackbone => {
                Event::Message { source: MessageEndpoint::Backbone { uri: self.message_endpoint.take().expect("queued backbone-message uri is present") }, payload: self.message_payload.take().expect("queued backbone-message payload is present") }
            }
        }
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if let Some(length) = self.surface_visible.as_ref().map(String::len) {
            if length > maximum_bytes {
                return (false, 0, 0);
            }
            drop(self.surface_visible.take().expect("queued surface-visible surface is present"));
            return (self.remaining_bytes() == 0, 1, length);
        }
        if let Some(length) = self.surface_body_key.as_ref().map(String::len) {
            if length > maximum_bytes {
                return (false, 0, 0);
            }
            drop(self.surface_body_key.take().expect("queued surface-visible body key is present"));
            return (self.remaining_bytes() == 0, 1, length);
        }
        if let Some(length) = self.surface_view_state.as_ref().map(Vec::len) {
            if length > maximum_bytes {
                return (false, 0, 0);
            }
            drop(self.surface_view_state.take().expect("queued surface-visible view state is present"));
            return (self.remaining_bytes() == 0, 1, length);
        }
        if let Some(length) = self.message_endpoint.as_ref().map(String::len) {
            if length > maximum_bytes {
                return (false, 0, 0);
            }
            drop(self.message_endpoint.take().expect("queued message endpoint is present"));
            return (self.remaining_bytes() == 0, 1, length);
        }
        let Some(length) = self.message_payload.as_ref().map(Vec::len) else {
            self.kind = None;
            return (true, 0, 0);
        };
        if length > maximum_bytes {
            return (false, 0, 0);
        }
        drop(self.message_payload.take().expect("queued message payload is present"));
        self.kind = None;
        (true, 1, length)
    }

    fn remaining_bytes(&self) -> usize {
        self.surface_visible.as_ref().map_or(0, String::len)
            + self.surface_body_key.as_ref().map_or(0, String::len)
            + self.surface_view_state.as_ref().map_or(0, Vec::len)
            + self.message_endpoint.as_ref().map_or(0, String::len)
            + self.message_payload.as_ref().map_or(0, Vec::len)
    }
}

pub struct RejectedKernelEvents {
    pub events: std::collections::VecDeque<Event>,
}

impl RejectedKernelEvents {
    pub fn close_step(&mut self) -> (bool, usize) {
        let Some(event) = self.events.pop_front() else {
            return (true, 0);
        };
        drop(event);
        (self.events.is_empty(), 1)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

pub enum KernelRequest {
    CreateApp {
        owner: CreateAppRequestOwner,
    },
    DestroyApp {
        owner: Arc<KernelCloseSubmission>,
    },
    /// 📡️ Non-command reactor events. Command bytes use [`KernelRequest::ExchangeCommands`]
    /// so the retained host batch lowers exactly one admitted page per turn.
    Exchange {
        instance: u32,
        event: QueuedKernelEvent,
    },
    AdvanceRetained {
        instance: u32,
        surface: SurfaceId,
    },
    MountProductReplay {
        owner: MountedProductReplayRequest,
    },
    RetireProductReplay {
        owner: MountedProductReplayRequest,
    },
    RetireProductReplayRefusal {
        owner: RefusedProductReplay,
    },
    AdvanceProductReplay {
        instance: u32,
    },
    ExchangeCommands {
        instance: u32,
        driver: semio_framework::kernel::CommandBatchDriver,
    },
    CloseRejectedCommandBuild {
        key: u64,
        owner: semio_framework::kernel::RejectedCommandBuild,
    },
    CloseRejectedEvents {
        owner: RejectedKernelEvents,
    },
    CloseRealm {
        owner: Arc<KernelCloseSubmission>,
    },
    AcknowledgeJobProgress {
        token: JobProgressPresentationToken,
    },
}

/// ⚖️ Whether `request` may be served between two slices of a turn that is mid-flight on instance `busy`
/// (the fairness contract `🧑‍🎨engine/🧫️fixtures/🧵️kernel-turn-fairness`): every request of ANOTHER instance
/// that owns its outcome, and a new app; never one for the busy instance itself (its guest owns the next
/// call), never a close or a retained-rejection drain (their maintenance invariants hold only between
/// whole requests).
pub fn kernel_request_interleavable(request: &KernelRequest, busy: u32) -> bool {
    match request {
        KernelRequest::Exchange { instance, .. } | KernelRequest::AdvanceRetained { instance, .. } | KernelRequest::AdvanceProductReplay { instance } | KernelRequest::ExchangeCommands { instance, .. } => *instance != busy,
        KernelRequest::MountProductReplay { owner } | KernelRequest::RetireProductReplay { owner } => owner.instance != busy,
        KernelRequest::CreateApp { .. } | KernelRequest::AcknowledgeJobProgress { .. } => true,
        KernelRequest::DestroyApp { .. } | KernelRequest::CloseRealm { .. } | KernelRequest::RetireProductReplayRefusal { .. } | KernelRequest::CloseRejectedCommandBuild { .. } | KernelRequest::CloseRejectedEvents { .. } => false,
    }
}

impl KernelRequest {
    pub fn command_credits(&self) -> (usize, usize) {
        match self {
            Self::ExchangeCommands { driver, .. } => (driver.remaining_pages(), driver.remaining_bytes()),
            Self::CloseRejectedCommandBuild { owner, .. } => (owner.remaining_pages(), owner.remaining_bytes()),
            Self::CreateApp { .. } => (0, 0),
            Self::Exchange { event, .. } => (0, event.remaining_bytes()),
            Self::AdvanceRetained { .. }
            | Self::MountProductReplay { .. }
            | Self::RetireProductReplay { .. }
            | Self::RetireProductReplayRefusal { .. }
            | Self::AdvanceProductReplay { .. }
            | Self::DestroyApp { .. }
            | Self::CloseRealm { .. }
            | Self::CloseRejectedEvents { .. }
            | Self::AcknowledgeJobProgress { .. } => (0, 0),
        }
    }
}

pub const KERNEL_CLOSE_SUBMISSION_CAPACITY: usize = 64;
pub const KERNEL_CLOSE_UNADMITTED: u8 = 0;
const KERNEL_CLOSE_ADMITTING: u8 = 1;
pub const KERNEL_CLOSE_READY: u8 = 2;
const KERNEL_CLOSE_SCHEDULED: u8 = 3;
const KERNEL_CLOSE_QUEUEING: u8 = 4;
pub const KERNEL_CLOSE_QUEUED: u8 = 5;
const KERNEL_CLOSE_COMPLETE: u8 = 6;
const KERNEL_CLOSE_FAULT: u8 = 7;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KernelCloseStatus {
    AdmissionBlocked,
    Pending,
    Complete,
    Fault,
}

pub struct KernelCloseSubmissionRegistry {
    pub slots: Mutex<[Option<(u32, u64, Arc<KernelCloseSubmission>)>; KERNEL_CLOSE_SUBMISSION_CAPACITY]>,
}

impl KernelCloseSubmissionRegistry {
    pub fn new() -> Self {
        Self { slots: Mutex::new(std::array::from_fn(|_| None)) }
    }

    pub fn slot(instance: u32) -> usize {
        instance as usize % KERNEL_CLOSE_SUBMISSION_CAPACITY
    }

    fn try_insert(&self, instance: u32, generation: u64, owner: Arc<KernelCloseSubmission>) -> Result<(), Arc<KernelCloseSubmission>> {
        let Ok(mut slots) = self.slots.try_lock() else {
            return Err(owner);
        };
        let slot = &mut slots[Self::slot(instance)];
        if let Some((_, _, retained)) = slot {
            if retained.terminal_status().is_some() {
                let terminal = slot.take().expect("terminal kernel close submission is retained");
                drop(terminal);
            }
        }
        if slot.is_some() {
            return Err(owner);
        }
        *slot = Some((instance, generation, owner));
        Ok(())
    }

    fn try_remove(&self, instance: u32, generation: u64) -> bool {
        let Ok(mut slots) = self.slots.try_lock() else {
            return false;
        };
        let slot = &mut slots[Self::slot(instance)];
        if !matches!(slot, Some((retained_instance, retained_generation, _)) if *retained_instance == instance && *retained_generation == generation) {
            return false;
        }
        let terminal = slot.take().expect("exact terminal kernel close submission is retained");
        drop(terminal);
        true
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn contains(&self, instance: u32, generation: u64) -> bool {
        self.slots.try_lock().is_ok_and(|slots| matches!(&slots[Self::slot(instance)], Some((retained_instance, retained_generation, _)) if *retained_instance == instance && *retained_generation == generation))
    }
}

pub struct KernelCloseSubmission {
    pub instance: u32,
    pub realm: bool,
    pub generation: u64,
    pub queue: Arc<KernelRequestQueue>,
    pub pool: semio_framework_async::WorkerPool,
    pub registry: std::sync::Weak<KernelCloseSubmissionRegistry>,
    pub phase: std::sync::atomic::AtomicU8,
}

impl KernelCloseSubmission {
    pub fn terminal_status(&self) -> Option<KernelCloseStatus> {
        match self.phase.load(std::sync::atomic::Ordering::Acquire) {
            KERNEL_CLOSE_COMPLETE => Some(KernelCloseStatus::Complete),
            KERNEL_CLOSE_FAULT => Some(KernelCloseStatus::Fault),
            _ => None,
        }
    }

    pub fn try_admit(self: &Arc<Self>) -> bool {
        if self.phase.compare_exchange(KERNEL_CLOSE_UNADMITTED, KERNEL_CLOSE_ADMITTING, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return self.phase.load(std::sync::atomic::Ordering::Acquire) >= KERNEL_CLOSE_READY;
        }
        let Some(registry) = self.registry.upgrade() else {
            self.phase.store(KERNEL_CLOSE_FAULT, std::sync::atomic::Ordering::Release);
            return false;
        };
        match registry.try_insert(self.instance, self.generation, self.clone()) {
            Ok(()) => {
                self.phase.store(KERNEL_CLOSE_READY, std::sync::atomic::Ordering::Release);
                true
            }
            Err(rejected) => {
                drop(rejected);
                self.phase.store(KERNEL_CLOSE_UNADMITTED, std::sync::atomic::Ordering::Release);
                false
            }
        }
    }

    pub fn try_schedule(self: &Arc<Self>) -> bool {
        if self.phase.compare_exchange(KERNEL_CLOSE_READY, KERNEL_CLOSE_SCHEDULED, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return false;
        }
        let scheduled = self.clone();
        let job: semio_framework_async::Job = Box::new(move || scheduled.run_queue_admission());
        match self.pool.try_submit(semio_framework_async::Lane::Maintenance, job) {
            Ok(()) => true,
            Err(error) => {
                self.phase.compare_exchange(KERNEL_CLOSE_SCHEDULED, KERNEL_CLOSE_READY, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).ok();
                drop(error.into_job());
                false
            }
        }
    }

    fn run_queue_admission(self: Arc<Self>) {
        if self.phase.compare_exchange(KERNEL_CLOSE_SCHEDULED, KERNEL_CLOSE_QUEUEING, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return;
        }
        let wake = Waker::from(self.clone());
        self.phase.store(KERNEL_CLOSE_QUEUED, std::sync::atomic::Ordering::Release);
        let request = if self.realm { KernelRequest::CloseRealm { owner: self.clone() } } else { KernelRequest::DestroyApp { owner: self.clone() } };
        match self.queue.try_push(request, Arc::new(ResponseSlot::default()), Some(&wake)) {
            Ok(()) => {}
            Err((request, slot)) => {
                drop(request);
                drop(slot);
                if self.phase.compare_exchange(KERNEL_CLOSE_QUEUED, KERNEL_CLOSE_READY, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok() {
                    let _ = self.try_schedule();
                }
            }
        }
    }

    pub fn finish(&self, status: KernelCloseStatus) {
        let phase = match status {
            KernelCloseStatus::Complete => KERNEL_CLOSE_COMPLETE,
            KernelCloseStatus::Fault => KERNEL_CLOSE_FAULT,
            KernelCloseStatus::AdmissionBlocked | KernelCloseStatus::Pending => return,
        };
        self.phase.store(phase, std::sync::atomic::Ordering::Release);
        if let Some(registry) = self.registry.upgrade() {
            let _ = registry.try_remove(self.instance, self.generation);
        }
    }
}

impl std::task::Wake for KernelCloseSubmission {
    fn wake(self: Arc<Self>) {
        let _ = self.try_schedule();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        let _ = self.try_schedule();
    }
}

#[must_use = "kernel close ownership must be polled through terminal completion"]
pub struct KernelCloseHandle {
    pub owner: Arc<KernelCloseSubmission>,
}

impl KernelCloseHandle {
    pub fn poll(&self) -> KernelCloseStatus {
        if let Some(status) = self.owner.terminal_status() {
            if let Some(registry) = self.owner.registry.upgrade() {
                let _ = registry.try_remove(self.owner.instance, self.owner.generation);
            }
            return status;
        }
        if !self.owner.try_admit() {
            return self.owner.terminal_status().unwrap_or(KernelCloseStatus::AdmissionBlocked);
        }
        let _ = self.owner.try_schedule();
        KernelCloseStatus::Pending
    }
}
//#endregion
//#endregion 🎠️KernelRuntime

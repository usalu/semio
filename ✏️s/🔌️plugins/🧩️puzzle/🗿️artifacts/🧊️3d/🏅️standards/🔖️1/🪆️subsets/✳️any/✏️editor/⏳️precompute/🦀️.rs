//! ⏳️ Puzzle 3d play app — the precompute session: the scene the host syncs in, the registered
//! collision meshes, the two independent background lanes (brush-candidate caching and fill
//! planning), and `dispatch`, which drives `Puzzle3dEngineCommand`/`Puzzle3dEngineOutcome` (schema
//! types, `crate::standards::v1::subsets::any::schema`) through the session. The rules the lanes consult live
//! in `🖌️brush/🦀️.rs`, the geometry in `📐️geometry/🦀️.rs`, the fill plan's own state
//! in `🪣️fill/🦀️.rs`. Rehomed from the former `⚙️engine/⏳️session` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): a puzzle-3d artifact is a schema plus an io
//! system, never an engine — this interactive brush/fill session is the app's own state machine over
//! that schema, not artifact behaviour.

//#region 🔖️Reexports
pub use crate::editor::puzzle3d::precompute::brush::apply_brush_placement_to_fixture;
//#endregion 🔖️Reexports

//#region 🔖️Constants
/// ⏳️ Default cap on how many objects one fill session may plan — was `⚙️engine`'s own
/// `FILL_COUNT_MAX`; distinct from (and not to be confused with) the UI-facing
/// `crate::editor::puzzle3d::PUZZLE3D_FILL_COUNT_MAX` slider clamp.
pub(crate) const FILL_COUNT_MAX: usize = 1000;
//#endregion 🔖️Constants

use crate::editor::puzzle3d::precompute::brush::{
    brush_candidate_suggestion_weight, brush_compatible_candidates, brush_preview_from_candidate, brush_target_vortex_allows_suggestion, resolve_object_kind_mesh_url, vortex_world_from_object, AttractionVortexContext, TargetVortexWorld,
};
use crate::editor::puzzle3d::precompute::fill::{FillBuilder, FillBuilderOwnerCensusCursor, FillBuilderOwnerCensusStep, FillBuilderRetirementCursor, FillPreparationRoots, FillPreviewJsonStep, PlacedCollisionEntry};
use crate::editor::puzzle3d::precompute::geometry::{
    pose_isometry, world_bounds, CollisionAabb, CollisionBody, CollisionIndexMutation, CollisionIndexOwner, CollisionIndexRemoval, CollisionMutationStep, CollisionOverlapState, CollisionQueryStep, CollisionSpatialIndex, CollisionStepContext,
    CollisionStepResult,
};
use crate::standards::v1::subsets::any::schema::{
    puzzle3d_vortex_full_id, BrushCollisionFreeResult, BrushCompatibleCandidate, BrushPlacePayload, BrushPreviewState, FillBuildProgress, FillProgressSummary, Fixture, KindCatalogBundle, PrecomputeLane, Puzzle3dEngineCommand, Puzzle3dEngineOutcome,
    SceneConfig,
};
use crate::Puzzle3dError;
use semio_framework_job::{default_now_us, root_cancel_token, CancelToken, Generation, InteractiveJob, InteractiveJobCloseStep, InteractiveStage, Operation, RevisionId, StepOutcome};
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

//#region 💼️FillJobBridge
pub const FILL_JOB_KIND: &str = "semio.puzzle3d.fill";
const FILL_ENVELOPE_PAGE_BYTES: usize = 16 * 1024;
const FILL_ENVELOPE_MAX_PAGES: usize = 256;
pub(crate) const FILL_ENVELOPE_MAX_BYTES: usize = FILL_ENVELOPE_PAGE_BYTES * FILL_ENVELOPE_MAX_PAGES;
pub(crate) const FILL_ENVELOPE_MAX_ITEMS: usize = 65_536;
const FILL_ENVELOPE_MAX_OPERATIONS: usize = 4;
const FILL_ENVELOPE_PROCESS_BYTES: usize = FILL_ENVELOPE_MAX_BYTES * FILL_ENVELOPE_MAX_OPERATIONS;
const FILL_ENVELOPE_TOKEN_BYTES: usize = 56;
const FILL_ENVELOPE_AUTHORITY_ITEMS: usize = 2;
const FILL_ENVELOPE_AUTHORITY_BYTES: usize = FILL_ENVELOPE_PAGE_BYTES + FILL_ENVELOPE_TOKEN_BYTES;
const FILL_ENVELOPE_MAGIC: [u8; 8] = *b"P3FILL04";
const FILL_WORKER_MAX_MESHES: usize = 64;
const FILL_WORKER_MAX_MESH_VALUES: usize = 196_608;
const FILL_WORKER_MAX_URL_BYTES: usize = 4 * 1024;

#[derive(Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
struct FillJobRequest {
    job: u64,
    operation: u64,
    generation: u64,
    base_revision: u64,
    slot: u8,
    registry_generation: u64,
}

struct FillEnvelopeTokenCursor {
    bytes: Vec<u8>,
    request: FillJobRequest,
    field: u8,
}

impl FillEnvelopeTokenCursor {
    fn new(bytes: Vec<u8>) -> Self {
        Self { bytes, request: FillJobRequest::default(), field: 0 }
    }

    fn step(&mut self) -> Result<Option<FillJobRequest>, &'static str> {
        if self.bytes.len() != FILL_ENVELOPE_TOKEN_BYTES {
            return Err("fill worker token has invalid length");
        }
        let read = |start| u64::from_le_bytes(self.bytes[start..start + 8].try_into().expect("fixed token range"));
        match self.field {
            0 => {
                if self.bytes[..8] != FILL_ENVELOPE_MAGIC || usize::from(self.bytes[8]) >= FILL_ENVELOPE_MAX_OPERATIONS || self.bytes[9..16].iter().any(|byte| *byte != 0) {
                    return Err("fill worker token header is malformed");
                }
                self.request.slot = self.bytes[8];
            }
            1 => {
                self.request.registry_generation = read(16);
                if self.request.registry_generation == 0 {
                    return Err("fill worker registry generation is zero");
                }
            }
            2 => {
                self.request.job = read(24);
                if self.request.job == 0 {
                    return Err("fill worker job identity is zero");
                }
            }
            3 => {
                self.request.operation = read(32);
                if self.request.operation == 0 {
                    return Err("fill worker operation identity is zero");
                }
            }
            4 => {
                self.request.generation = read(40);
                if self.request.generation == 0 {
                    return Err("fill worker operation generation is zero");
                }
            }
            5 => {
                self.request.base_revision = read(48);
                if self.request.base_revision == 0 {
                    return Err("fill worker base revision is zero");
                }
            }
            _ => return Ok(Some(self.request.clone())),
        }
        self.field += 1;
        Ok(None)
    }
}

fn fill_envelope_raw_request(bytes: &[u8]) -> Option<FillJobRequest> {
    if bytes.len() != FILL_ENVELOPE_TOKEN_BYTES {
        return None;
    }
    let slot = bytes[8];
    if usize::from(slot) >= FILL_ENVELOPE_MAX_OPERATIONS {
        return None;
    }
    let read = |start: usize| -> Option<u64> { Some(u64::from_le_bytes(bytes[start..start + 8].try_into().ok()?)) };
    Some(FillJobRequest { job: read(24)?, operation: read(32)?, generation: read(40)?, base_revision: read(48)?, slot, registry_generation: read(16)? })
}

fn decode_fill_envelope_request(bytes: &[u8]) -> Option<FillJobRequest> {
    if bytes.get(..8) != Some(FILL_ENVELOPE_MAGIC.as_slice()) || bytes.get(9..16)?.iter().any(|byte| *byte != 0) {
        return None;
    }
    let request = fill_envelope_raw_request(bytes)?;
    (request.registry_generation != 0 && request.job != 0 && request.operation != 0 && request.generation != 0 && request.base_revision != 0).then_some(request)
}

#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue)]
struct FillWorkerMesh {
    url: String,
    positions: Vec<f32>,
    indices: Vec<u32>,
    fallback: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
struct FillObservation {
    generation: u64,
    sequence: u64,
    available: u32,
    done: bool,
}

struct FillJobSlice {
    progress: Option<FillObservation>,
    done: bool,
}

type SharedFillBuilder = Arc<Mutex<FillBuilder>>;

struct SharedFillWorkerJob {
    fill: Option<SharedFillBuilder>,
}

impl SharedFillWorkerJob {
    fn new(fill: SharedFillBuilder) -> Self {
        Self { fill: Some(fill) }
    }
}

impl InteractiveJob for SharedFillWorkerJob {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> StepOutcome {
        let Some(fill) = &self.fill else { return StepOutcome::Yield };
        let Ok(mut fill) = fill.try_lock() else { return StepOutcome::Yield };
        fill.step(cx)
    }

    fn begin_close(&mut self) {}

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
        if self.fill.is_none() {
            return InteractiveJobCloseStep::Complete;
        }
        if maximum_items == 0 {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        self.fill.take();
        InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
    }

    fn terminal_is_empty(&self) -> bool {
        self.fill.is_none()
    }
}

/// 🧵️ The fill planner is driven ON THE CALLER, never handed to a worker pool: one
/// `FillBuilder::step` is a single bounded cursor unit, and a pooled submission costs a thread
/// round-trip per unit that no interactive step (nor the single-threaded wasm guest, which has no
/// pool at all) can wait for — measured at ≈4 000 idle `precompute_step_lane` calls per planner step
/// before this became [`semio_framework_job::BatchJobSession`].
type MountedFillWorker = semio_framework_job::BatchJobSession<SharedFillWorkerJob>;
type RejectedFillWorker = semio_framework_job::WorkerJobSessionAdmissionRejected<SharedFillWorkerJob>;

fn mount_fill_worker(fill: SharedFillBuilder, operation: Operation, cancel: CancelToken) -> Result<MountedFillWorker, RejectedFillWorker> {
    semio_framework_job::BatchJobSession::try_new(
        SharedFillWorkerJob::new(fill),
        semio_framework_job::BatchJobParams {
            operation: operation.operation,
            generation: operation.generation,
            cancel,
            config: semio_framework_job::BatchDriveConfig { site: "puzzle3d.fill.mounted", stage: InteractiveStage::BackgroundStep, fuel_per_step: 1, step_budget_us: 2000 },
            now_us: default_now_us,
        },
    )
}

/// 📏️ Exactly the turns a rejected fill-worker admission needs to reach terminal emptiness:
/// `SharedFillWorkerJob::close_step` releases its one `Arc` in a single turn and then reports
/// `Complete`, and `WorkerJobSessionAdmissionRejected::close_step` spends one further turn each
/// dropping the job shell, the batch params and the fault page — five, plus one to observe `Complete`.
const REJECTED_FILL_WORKER_CLOSE_TURNS: usize = 6;

/// ♻️ Runs a rejected fill-worker admission through its own incremental close before the owner is
/// released. `WorkerJobSessionAdmissionRejected::drop` asserts terminal emptiness
/// (`🧵️job/🦀️.rs`: *"rejected worker session admission requires exact incremental close"*), so
/// dropping one after a single `close_step` — or by a plain `= None` — aborted the whole process from
/// a destructor and hid every following test's verdict.
fn retire_rejected_fill_worker(mut rejected: RejectedFillWorker) {
    rejected.begin_close();
    for _ in 0..REJECTED_FILL_WORKER_CLOSE_TURNS {
        if rejected.terminal_is_empty() {
            break;
        }
        rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
    debug_assert!(rejected.terminal_is_empty(), "rejected fill worker must reach terminal emptiness within its own declared close turns");
}

struct FillEnvelopeAdmissionCursor {
    request: FillJobRequest,
    census: FillBuilderOwnerCensusCursor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FillEnvelopeTerminalReason {
    Complete,
    Cancelled,
    Fault,
    Closed,
}

impl FillEnvelopeTerminalReason {
    fn code(self) -> u8 {
        match self {
            Self::Complete => 1,
            Self::Cancelled => 2,
            Self::Fault => 3,
            Self::Closed => 4,
        }
    }

    fn from_code(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Complete),
            2 => Some(Self::Cancelled),
            3 => Some(Self::Fault),
            4 => Some(Self::Closed),
            _ => None,
        }
    }
}

struct FillEnvelopeTerminalIntent {
    job: AtomicU64,
    registry_generation: AtomicU64,
    reason: AtomicU8,
}

fn fill_envelope_terminal_intents() -> &'static [FillEnvelopeTerminalIntent; FILL_ENVELOPE_MAX_OPERATIONS] {
    static INTENTS: OnceLock<[FillEnvelopeTerminalIntent; FILL_ENVELOPE_MAX_OPERATIONS]> = OnceLock::new();
    INTENTS.get_or_init(|| std::array::from_fn(|_| FillEnvelopeTerminalIntent { job: AtomicU64::new(0), registry_generation: AtomicU64::new(0), reason: AtomicU8::new(0) }))
}

fn register_fill_envelope_terminal_intent(request: &FillJobRequest) {
    let intent = &fill_envelope_terminal_intents()[usize::from(request.slot)];
    intent.reason.store(0, Ordering::Release);
    intent.registry_generation.store(request.registry_generation, Ordering::Release);
    intent.job.store(request.job, Ordering::Release);
}

fn request_fill_envelope_terminal(request: &FillJobRequest, reason: FillEnvelopeTerminalReason) {
    let intent = &fill_envelope_terminal_intents()[usize::from(request.slot)];
    if intent.job.load(Ordering::Acquire) == request.job && intent.registry_generation.load(Ordering::Acquire) == request.registry_generation {
        intent.reason.fetch_max(reason.code(), Ordering::AcqRel);
    }
}

fn apply_fill_envelope_terminal_intent(authority: &mut FillEnvelopeAuthority) -> bool {
    let intent = &fill_envelope_terminal_intents()[usize::from(authority.request.slot)];
    if intent.job.load(Ordering::Acquire) != authority.request.job || intent.registry_generation.load(Ordering::Acquire) != authority.request.registry_generation {
        return false;
    }
    let Some(reason) = FillEnvelopeTerminalReason::from_code(intent.reason.swap(0, Ordering::AcqRel)) else {
        return false;
    };
    if !matches!(authority.phase, FillEnvelopePhase::Closing) {
        authority.phase = FillEnvelopePhase::Terminal(reason);
        authority.observation.done = true;
    }
    true
}

fn release_fill_envelope_terminal_intent(request: &FillJobRequest) {
    let intent = &fill_envelope_terminal_intents()[usize::from(request.slot)];
    if intent.job.load(Ordering::Acquire) == request.job && intent.registry_generation.load(Ordering::Acquire) == request.registry_generation {
        intent.reason.store(0, Ordering::Release);
        intent.registry_generation.store(0, Ordering::Release);
        intent.job.store(0, Ordering::Release);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FillEnvelopePhase {
    Measuring,
    Admitted,
    Terminal(FillEnvelopeTerminalReason),
    Closing,
}

struct FillEnvelopeAuthority {
    request: FillJobRequest,
    fill: Option<SharedFillBuilder>,
    worker: Option<MountedFillWorker>,
    worker_outcome: Option<StepOutcome>,
    worker_terminal: bool,
    fill_retirement: Option<FillBuilderRetirementCursor>,
    cancel: Option<CancelToken>,
    steps_remaining: usize,
    preview_sequence: u64,
    observation: FillObservation,
    phase: FillEnvelopePhase,
    token_page: Option<Box<[u8; FILL_ENVELOPE_PAGE_BYTES]>>,
    token_len: usize,
    checked_out: Arc<AtomicBool>,
    close_cursor: usize,
    reserved_items: usize,
    reserved_bytes: usize,
}

struct FillEnvelopeMeasurementOwners {
    fill: SharedFillBuilder,
    worker: MountedFillWorker,
}

struct FillEnvelopeMeasurementRequest {
    job: u64,
    operation: Operation,
    fill: SharedFillBuilder,
    worker: MountedFillWorker,
    cancel: CancelToken,
    steps_remaining: usize,
    preview_sequence: u64,
    observation: FillObservation,
}

struct FillEnvelopeRegistry {
    slots: [Option<FillEnvelopeAuthority>; FILL_ENVELOPE_MAX_OPERATIONS],
    generations: [u64; FILL_ENVELOPE_MAX_OPERATIONS],
    next_slot: usize,
    aggregate_bytes: usize,
}

impl Default for FillEnvelopeRegistry {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| None), generations: [0; FILL_ENVELOPE_MAX_OPERATIONS], next_slot: 0, aggregate_bytes: 0 }
    }
}

fn fill_envelope_registry() -> &'static Mutex<FillEnvelopeRegistry> {
    static REGISTRY: OnceLock<Mutex<FillEnvelopeRegistry>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(FillEnvelopeRegistry::default()))
}

fn fill_envelope_token(request: &FillJobRequest) -> [u8; FILL_ENVELOPE_TOKEN_BYTES] {
    let mut token = [0_u8; FILL_ENVELOPE_TOKEN_BYTES];
    token[..8].copy_from_slice(&FILL_ENVELOPE_MAGIC);
    token[8] = request.slot;
    token[16..24].copy_from_slice(&request.registry_generation.to_le_bytes());
    token[24..32].copy_from_slice(&request.job.to_le_bytes());
    token[32..40].copy_from_slice(&request.operation.to_le_bytes());
    token[40..48].copy_from_slice(&request.generation.to_le_bytes());
    token[48..56].copy_from_slice(&request.base_revision.to_le_bytes());
    token
}

#[cfg(test)]
fn decode_fill_envelope_token(bytes: &[u8]) -> Option<FillJobRequest> {
    decode_fill_envelope_request(bytes)
}

impl FillEnvelopeRegistry {
    fn begin_measurement(&mut self, request: FillEnvelopeMeasurementRequest) -> Result<FillJobRequest, FillEnvelopeMeasurementOwners> {
        let FillEnvelopeMeasurementRequest { job, operation, fill, worker, cancel, steps_remaining, preview_sequence, observation } = request;
        let candidates = [self.next_slot, (self.next_slot + 1) % FILL_ENVELOPE_MAX_OPERATIONS, (self.next_slot + 2) % FILL_ENVELOPE_MAX_OPERATIONS, (self.next_slot + 3) % FILL_ENVELOPE_MAX_OPERATIONS];
        let Some(slot) = candidates.into_iter().find(|slot| self.slots[*slot].is_none() && self.generations[*slot] != u64::MAX) else {
            return Err(FillEnvelopeMeasurementOwners { fill, worker });
        };
        let Some(registry_generation) = self.generations[slot].checked_add(1).filter(|generation| *generation != u64::MAX) else {
            return Err(FillEnvelopeMeasurementOwners { fill, worker });
        };
        self.generations[slot] = registry_generation;
        let request = FillJobRequest { job, operation: operation.operation.0, generation: operation.generation.0, base_revision: operation.base_revision.0, slot: slot as u8, registry_generation };
        self.slots[slot] = Some(FillEnvelopeAuthority {
            request: request.clone(),
            fill: Some(fill),
            worker: Some(worker),
            worker_outcome: None,
            worker_terminal: false,
            fill_retirement: None,
            cancel: Some(cancel),
            steps_remaining,
            preview_sequence,
            observation,
            phase: FillEnvelopePhase::Measuring,
            token_page: None,
            token_len: 0,
            checked_out: Arc::new(AtomicBool::new(false)),
            close_cursor: 0,
            reserved_items: 0,
            reserved_bytes: 0,
        });
        register_fill_envelope_terminal_intent(&request);
        self.next_slot = (slot + 1) % FILL_ENVELOPE_MAX_OPERATIONS;
        Ok(request)
    }

    fn finish_measurement(&mut self, request: &FillJobRequest, requested_items: usize, requested_bytes: usize) -> Option<Vec<u8>> {
        let admitted_items = requested_items.checked_add(FILL_ENVELOPE_AUTHORITY_ITEMS);
        let admitted_bytes = requested_bytes.checked_add(FILL_ENVELOPE_AUTHORITY_BYTES);
        if requested_items == 0
            || admitted_items.is_none_or(|items| items > FILL_ENVELOPE_MAX_ITEMS)
            || requested_bytes == 0
            || admitted_bytes.is_none_or(|bytes| bytes > FILL_ENVELOPE_MAX_BYTES)
            || admitted_bytes.and_then(|bytes| self.aggregate_bytes.checked_add(bytes)).is_none_or(|bytes| bytes > FILL_ENVELOPE_PROCESS_BYTES)
        {
            if let Some(authority) = self.authority_mut(request) {
                authority.phase = FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Fault);
                authority.observation.done = true;
            }
            return None;
        }
        let authority = self.authority_mut(request)?;
        if !matches!(authority.phase, FillEnvelopePhase::Measuring) {
            return None;
        }
        let admitted_items = admitted_items.expect("validated item credit");
        let admitted_bytes = admitted_bytes.expect("validated byte credit");
        let token = fill_envelope_token(request);
        let mut token_page = Box::new([0_u8; FILL_ENVELOPE_PAGE_BYTES]);
        token_page[..FILL_ENVELOPE_TOKEN_BYTES].copy_from_slice(&token);
        authority.token_page = Some(token_page);
        authority.token_len = FILL_ENVELOPE_TOKEN_BYTES;
        authority.reserved_items = admitted_items;
        authority.reserved_bytes = admitted_bytes;
        authority.phase = FillEnvelopePhase::Admitted;
        self.aggregate_bytes += admitted_bytes;
        Some(token.to_vec())
    }

    #[cfg(test)]
    fn reserve(
        &mut self,
        job: u64,
        operation: Operation,
        requested_items: usize,
        requested_bytes: usize,
        fill: SharedFillBuilder,
        cancel: CancelToken,
        steps_remaining: usize,
        preview_sequence: u64,
        observation: FillObservation,
    ) -> Result<(FillJobRequest, Vec<u8>), SharedFillBuilder> {
        let worker = match mount_fill_worker(Arc::clone(&fill), operation, cancel.clone()) {
            Ok(worker) => worker,
            Err(rejected) => {
                retire_rejected_fill_worker(rejected);
                return Err(fill);
            }
        };
        let request = self.begin_measurement(FillEnvelopeMeasurementRequest { job, operation, fill, worker, cancel, steps_remaining, preview_sequence, observation }).map_err(|owners| owners.fill)?;
        match self.finish_measurement(&request, requested_items, requested_bytes) {
            Some(token) => Ok((request, token)),
            None => {
                let slot = usize::from(request.slot);
                let authority = self.slots[slot].take().expect("fresh measurement authority");
                release_fill_envelope_terminal_intent(&request);
                if let Some(mut worker) = authority.worker {
                    worker.begin_close();
                }
                Err(authority.fill.expect("measurement retained exact fill"))
            }
        }
    }

    fn authority_mut(&mut self, request: &FillJobRequest) -> Option<&mut FillEnvelopeAuthority> {
        self.slots.get_mut(usize::from(request.slot))?.as_mut().filter(|authority| authority.request == *request && authority.request.registry_generation == request.registry_generation)
    }

    fn token(&self, request: &FillJobRequest) -> Option<Vec<u8>> {
        let authority = self.slots.get(usize::from(request.slot))?.as_ref()?;
        (authority.request == *request).then(|| authority.token_page.as_ref().map(|page| page[..authority.token_len].to_vec())).flatten()
    }

    fn observation(&self, request: &FillJobRequest) -> Option<FillObservation> {
        let authority = self.slots.get(usize::from(request.slot))?.as_ref()?;
        (authority.request == *request).then_some(authority.observation)
    }

    fn take_closed(&mut self) -> Option<FillEnvelopeTerminalHandle> {
        for authority in self.slots.iter_mut().flatten() {
            apply_fill_envelope_terminal_intent(authority);
        }
        let authority = self.slots.iter_mut().flatten().find(|authority| {
            matches!(authority.phase, FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Closed) | FillEnvelopePhase::Closing) && authority.checked_out.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_ok()
        })?;
        Some(FillEnvelopeTerminalHandle { request: authority.request.clone(), checked_out: authority.checked_out.clone(), returned: false })
    }
}

enum FillEnvelopeDrive {
    Advanced(FillJobSlice),
    Blocked,
    Stale,
}

fn terminalize_fill_envelope(request: &FillJobRequest, reason: FillEnvelopeTerminalReason) {
    request_fill_envelope_terminal(request, reason);
}

struct FillEnvelopeWorkerFaultGuard {
    request: Option<FillJobRequest>,
}

impl FillEnvelopeWorkerFaultGuard {
    fn new(input: &[u8]) -> Self {
        Self { request: fill_envelope_raw_request(input) }
    }

    fn disarm(&mut self) {
        self.request = None;
    }
}

impl Drop for FillEnvelopeWorkerFaultGuard {
    fn drop(&mut self) {
        if let Some(request) = &self.request {
            request_fill_envelope_terminal(request, FillEnvelopeTerminalReason::Fault);
        }
    }
}

struct FillEnvelopeJobEntryCursor {
    context_job: u64,
    terminal_guard: FillEnvelopeWorkerFaultGuard,
    token: FillEnvelopeTokenCursor,
}

impl FillEnvelopeJobEntryCursor {
    fn new(context_job: u64, input: Vec<u8>) -> Self {
        let terminal_guard = FillEnvelopeWorkerFaultGuard::new(&input);
        Self { context_job, terminal_guard, token: FillEnvelopeTokenCursor::new(input) }
    }

    fn step(&mut self) -> Result<Option<FillJobRequest>, &'static str> {
        self.token.step()
    }

    fn bind(&self, request: &FillJobRequest) -> Result<(), &'static str> {
        if request.job != self.context_job {
            return Err("fill worker context job does not match the decoded request owner");
        }
        if self.terminal_guard.request.as_ref() != Some(request) {
            return Err("fill worker decoded request does not match the envelope owner");
        }
        let registry = fill_envelope_registry().try_lock().map_err(|_| "fill worker envelope owner is contended")?;
        if !registry.slots.get(usize::from(request.slot)).and_then(Option::as_ref).is_some_and(|authority| authority.request == *request) {
            return Err("fill worker envelope owner is stale");
        }
        Ok(())
    }

    fn into_guard(mut self) -> FillEnvelopeWorkerFaultGuard {
        std::mem::replace(&mut self.terminal_guard, FillEnvelopeWorkerFaultGuard { request: None })
    }
}

fn drive_fill_envelope(request: &FillJobRequest) -> FillEnvelopeDrive {
    let Ok(mut registry) = fill_envelope_registry().try_lock() else {
        return FillEnvelopeDrive::Blocked;
    };
    let Some(authority) = registry.authority_mut(request) else {
        return FillEnvelopeDrive::Stale;
    };
    apply_fill_envelope_terminal_intent(authority);
    if authority.checked_out.load(Ordering::Acquire) || !matches!(authority.phase, FillEnvelopePhase::Admitted) {
        return FillEnvelopeDrive::Blocked;
    }
    let Some(cancel) = authority.cancel.clone() else {
        return FillEnvelopeDrive::Blocked;
    };
    if cancel.is_cancelled_now() {
        if let Some(worker) = authority.worker.as_mut() {
            worker.begin_close();
        }
        authority.phase = FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Cancelled);
        authority.observation.done = true;
        return FillEnvelopeDrive::Advanced(FillJobSlice { progress: Some(authority.observation), done: true });
    }
    let Some(fill_owner) = authority.fill.as_ref() else {
        return FillEnvelopeDrive::Blocked;
    };
    {
        let Ok(fill) = fill_owner.try_lock() else {
            return FillEnvelopeDrive::Blocked;
        };
        if fill.operation.operation.0 != request.operation || fill.operation.generation.0 != request.generation || fill.operation.base_revision.0 != request.base_revision {
            if let Some(worker) = authority.worker.as_mut() {
                worker.begin_close();
            }
            authority.phase = FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Fault);
            authority.observation.done = true;
            return FillEnvelopeDrive::Stale;
        }
    }
    let previous = authority.observation;
    if let Some(outcome) = authority.worker_outcome.as_mut() {
        let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        if !outcome.terminal_is_empty() {
            return FillEnvelopeDrive::Advanced(FillJobSlice { progress: None, done: false });
        }
        authority.worker_outcome.take();
        if authority.worker_terminal {
            if let Some(worker) = authority.worker.as_mut() {
                worker.begin_close();
            }
        } else if authority.worker.as_mut().is_none_or(|worker| worker.resume().is_err()) {
            authority.worker_terminal = true;
            authority.phase = FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Fault);
            authority.observation.done = true;
        }
    } else {
        let Some(worker) = authority.worker.as_mut() else {
            authority.phase = FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Fault);
            authority.observation.done = true;
            return FillEnvelopeDrive::Stale;
        };
        match worker.step() {
            Ok(_) => {
                let Some(outcome) = worker.take_outcome() else {
                    authority.worker_terminal = true;
                    authority.phase = FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Fault);
                    authority.observation.done = true;
                    return FillEnvelopeDrive::Stale;
                };
                match &outcome {
                    StepOutcome::CheckpointReady(_) => authority.steps_remaining = authority.steps_remaining.saturating_sub(1),
                    StepOutcome::Complete(_) => {
                        authority.steps_remaining = 0;
                        authority.worker_terminal = true;
                        authority.phase = FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Complete);
                    }
                    StepOutcome::Cancelled => {
                        authority.steps_remaining = 0;
                        authority.worker_terminal = true;
                        authority.phase = FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Cancelled);
                    }
                    StepOutcome::Fault(_) => {
                        authority.steps_remaining = 0;
                        authority.worker_terminal = true;
                        authority.phase = FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Fault);
                    }
                    StepOutcome::Yield | StepOutcome::PreviewReady(_) => {}
                }
                authority.worker_outcome = Some(outcome);
            }
            Err(_) => {
                authority.worker_terminal = true;
                authority.phase = FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Fault);
                authority.observation.done = true;
            }
        }
    }
    let Some((generation, sequence, available, fill_done)) = authority.fill.as_ref().and_then(|fill| {
        let fill = fill.try_lock().ok()?;
        Some((fill.preview.generation, fill.preview.sequence, fill.sequence.len() as u32, fill.stalled || fill.sequence.len() >= fill.max_count))
    }) else {
        return FillEnvelopeDrive::Advanced(FillJobSlice { progress: None, done: authority.observation.done });
    };
    let done = authority.steps_remaining == 0 || fill_done;
    if done && matches!(authority.phase, FillEnvelopePhase::Admitted) {
        authority.phase = FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Complete);
        authority.worker_terminal = true;
    }
    authority.observation = FillObservation { generation, sequence, available, done };
    let progress = (authority.observation != previous).then_some(authority.observation);
    FillEnvelopeDrive::Advanced(FillJobSlice { progress, done })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillEnvelopeCloseStep {
    Pending,
    Blocked,
    Complete,
    Stale,
}

pub struct FillEnvelopeTerminalHandle {
    request: FillJobRequest,
    checked_out: Arc<AtomicBool>,
    returned: bool,
}

impl FillEnvelopeTerminalHandle {
    pub fn reason(&self) -> Option<&'static str> {
        let registry = fill_envelope_registry().try_lock().ok()?;
        let authority = registry.slots.get(usize::from(self.request.slot))?.as_ref().filter(|authority| authority.request == self.request)?;
        match authority.phase {
            FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Complete) => Some("complete"),
            FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Cancelled) => Some("cancelled"),
            FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Fault) => Some("fault"),
            FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Closed) | FillEnvelopePhase::Closing => Some("closed"),
            FillEnvelopePhase::Measuring | FillEnvelopePhase::Admitted => None,
        }
    }

    pub fn resume(mut self) -> Result<Vec<u8>, Self> {
        let Ok(mut registry) = fill_envelope_registry().try_lock() else {
            return Err(self);
        };
        let Some(authority) = registry.authority_mut(&self.request) else {
            self.returned = true;
            return Err(self);
        };
        if matches!(authority.phase, FillEnvelopePhase::Closing) || authority.fill_retirement.is_some() {
            return Err(self);
        }
        let Some(token) = authority.token_page.as_ref().map(|page| page[..authority.token_len].to_vec()) else {
            return Err(self);
        };
        authority.phase = FillEnvelopePhase::Admitted;
        authority.checked_out.store(false, Ordering::Release);
        self.returned = true;
        Ok(token)
    }

    pub fn close_step(&mut self) -> FillEnvelopeCloseStep {
        let Ok(mut registry) = fill_envelope_registry().try_lock() else {
            return FillEnvelopeCloseStep::Blocked;
        };
        let slot = usize::from(self.request.slot);
        let Some(authority) = registry.authority_mut(&self.request) else {
            self.returned = true;
            return FillEnvelopeCloseStep::Stale;
        };
        authority.phase = FillEnvelopePhase::Closing;
        match authority.close_cursor {
            0 => {
                let Some(outcome) = authority.worker_outcome.as_mut() else {
                    authority.close_cursor = 1;
                    return FillEnvelopeCloseStep::Pending;
                };
                let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                if outcome.terminal_is_empty() {
                    authority.worker_outcome.take();
                    authority.close_cursor = 1;
                }
                return FillEnvelopeCloseStep::Pending;
            }
            1 => {
                let Some(worker) = authority.worker.as_mut() else {
                    authority.close_cursor = 2;
                    return FillEnvelopeCloseStep::Pending;
                };
                worker.begin_close();
                if matches!(worker.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::WorkerJobCloseStep::Complete) && worker.terminal_is_empty() {
                    authority.worker.take();
                    authority.close_cursor = 2;
                }
                return FillEnvelopeCloseStep::Pending;
            }
            2 => {
                if authority.fill_retirement.is_none() {
                    let Some(fill) = authority.fill.take() else {
                        authority.close_cursor = 3;
                        return FillEnvelopeCloseStep::Pending;
                    };
                    let fill = match Arc::try_unwrap(fill) {
                        Ok(fill) => fill,
                        Err(fill) => {
                            authority.fill = Some(fill);
                            return FillEnvelopeCloseStep::Blocked;
                        }
                    };
                    let fill = fill.into_inner().unwrap_or_else(std::sync::PoisonError::into_inner);
                    authority.fill_retirement = Some(FillBuilderRetirementCursor::new(fill));
                    return FillEnvelopeCloseStep::Pending;
                }
                let retired = authority.fill_retirement.as_mut().is_some_and(FillBuilderRetirementCursor::retire_one);
                if retired {
                    authority.fill_retirement.take();
                    authority.close_cursor = 3;
                }
                return FillEnvelopeCloseStep::Pending;
            }
            3 => {
                authority.cancel.take();
                authority.close_cursor = 4;
                return FillEnvelopeCloseStep::Pending;
            }
            4 => {
                authority.token_page.take();
                authority.token_len = 0;
                authority.close_cursor = 5;
                return FillEnvelopeCloseStep::Pending;
            }
            5 => {
                authority.reserved_items = 0;
                authority.close_cursor = 6;
                return FillEnvelopeCloseStep::Pending;
            }
            _ => {}
        }
        let Some(authority) = registry.slots[slot].take() else {
            return FillEnvelopeCloseStep::Stale;
        };
        registry.aggregate_bytes = registry.aggregate_bytes.checked_sub(authority.reserved_bytes).expect("reserved fill byte credit");
        release_fill_envelope_terminal_intent(&authority.request);
        self.returned = true;
        FillEnvelopeCloseStep::Complete
    }

    pub fn terminal_is_empty(&self) -> bool {
        let Ok(registry) = fill_envelope_registry().try_lock() else {
            return false;
        };
        registry.slots.get(usize::from(self.request.slot)).is_none_or(Option::is_none)
    }
}

impl Drop for FillEnvelopeTerminalHandle {
    fn drop(&mut self) {
        if self.returned {
            return;
        }
        request_fill_envelope_terminal(&self.request, FillEnvelopeTerminalReason::Closed);
        self.checked_out.store(false, Ordering::Release);
    }
}
//#endregion 💼️FillJobBridge

//#region 🔖️Clock
/// ⏱️ Uses the same checked real clock authority as retained jobs on every target.
fn puzzle3d_deadline(duration_us: u64) -> Option<u64> {
    default_now_us()?.checked_add(duration_us)
}

/// 🪫️ Admission deadline for one precompute turn, including its first task. It has to fit INSIDE
/// the retained step it is spent from, not equal it — and with room for the overshoot its own last
/// task unit costs. Handed the whole 2 000 µs this artifact budgets per interactive step, one
/// `refresh_brush_candidates` overshot it to a measured 2.04 ms on the 180-object Nakagin document:
/// the entire step's budget, spent on one of its halves, and still over. Halved it still reached
/// 1.54 ms, because the overshoot is one whole narrow-phase candidate (~0.5 ms there). A QUARTER
/// leaves the dispatch turn three quarters of its own step for the rest of its work, which is the
/// margin the measured laws in `✏️editor/🧪️tests/🔬️unit/🦀️.rs` need to hold. The lane redrives whatever
/// this deadline leaves unfinished, so a smaller budget only ever costs turns, never results.
/// Ticket 26/09/02/PUZZLE-3D-END-TO-END W-P3.
const PUZZLE3D_PRECOMPUTE_STEP_BUDGET_US: u64 = 500;
//#endregion 🔖️Clock

//#region 🥽️SharedBrushMeshes
/// 🥽️ Content-addressed brush-mesh decode kernel — a pure `(url, positions, indices)` bytes-in,
/// validated-geometry-bytes-out compute, so one mesh identity decodes exactly once per process and
/// every document instance reads the identical derived page instead of re-uploading it. This is the
/// kernel the WIT `engine-derive`/`engine-read` host route registers once it is threaded through
/// exchange (`🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️.rs`), with no plugin-side change.
struct Puzzle3dMeshDecodeEngine;

/// ⚖️ Byte budget of the derived-geometry LRU: `FILL_WORKER_MAX_MESHES` document-scale meshes at the
/// `FILL_WORKER_MAX_MESH_VALUES` ceiling, four bytes per value, halved because positions and indices
/// never both saturate.
const BRUSH_MESH_CACHE_BYTES: usize = FILL_WORKER_MAX_MESHES * FILL_WORKER_MAX_MESH_VALUES * 4;

impl store::Engine for Puzzle3dMeshDecodeEngine {
    const ENGINE_ID: &'static str = "puzzle3d.mesh-decode";

    fn compute(&self, input: &[u8]) -> Result<Vec<u8>, store::EngineFault> {
        let geometry = brush_mesh_request_geometry(input).ok_or_else(|| store::EngineFault::InvalidInput("puzzle3d brush mesh request".into()))?;
        let (positions, indices) = decode_brush_mesh_geometry(geometry).ok_or_else(|| store::EngineFault::InvalidInput("puzzle3d brush mesh geometry".into()))?;
        brush_mesh_geometry_is_admissible(&positions, &indices).then(|| geometry.to_vec()).ok_or_else(|| store::EngineFault::Compute("puzzle3d brush mesh geometry is not a closed indexed triangle page".into()))
    }
}

/// 🔗️ Little-endian request wire: `url_len | url | position_count | index_count | positions | indices`.
/// The url is part of the cache key on purpose — two urls carrying byte-identical geometry stay two
/// distinct mesh identities, exactly as the collision engine's own `meshes` map keys them.
fn encode_brush_mesh_request(url: &str, positions: &[f32], indices: &[u32]) -> Option<Vec<u8>> {
    let mut bytes = u32::try_from(url.len()).ok()?.to_le_bytes().to_vec();
    bytes.extend_from_slice(url.as_bytes());
    bytes.extend_from_slice(&u32::try_from(positions.len()).ok()?.to_le_bytes());
    bytes.extend_from_slice(&u32::try_from(indices.len()).ok()?.to_le_bytes());
    bytes.extend(positions.iter().flat_map(|value| value.to_le_bytes()));
    bytes.extend(indices.iter().flat_map(|value| value.to_le_bytes()));
    Some(bytes)
}

fn brush_mesh_request_geometry(input: &[u8]) -> Option<&[u8]> {
    let url_len = usize::try_from(u32::from_le_bytes(input.get(..4)?.try_into().ok()?)).ok()?;
    input.get(url_len.checked_add(4)?..)
}

fn decode_brush_mesh_geometry(geometry: &[u8]) -> Option<(Vec<f32>, Vec<u32>)> {
    let position_bytes = usize::try_from(u32::from_le_bytes(geometry.get(..4)?.try_into().ok()?)).ok()?.checked_mul(4)?;
    let index_bytes = usize::try_from(u32::from_le_bytes(geometry.get(4..8)?.try_into().ok()?)).ok()?.checked_mul(4)?;
    let indices_at = position_bytes.checked_add(8)?;
    let positions = geometry.get(8..indices_at)?;
    let indices = geometry.get(indices_at..indices_at.checked_add(index_bytes)?)?;
    Some((positions.as_chunks::<4>().0.iter().map(|bytes| f32::from_le_bytes(*bytes)).collect(), indices.as_chunks::<4>().0.iter().map(|bytes| u32::from_le_bytes(*bytes)).collect()))
}

fn brush_mesh_geometry_is_admissible(positions: &[f32], indices: &[u32]) -> bool {
    let vertices = u32::try_from(positions.len() / 3).unwrap_or(u32::MAX);
    positions.len() >= 9
        && indices.len() >= 3
        && positions.len().is_multiple_of(3)
        && indices.len().is_multiple_of(3)
        && positions.len() <= FILL_WORKER_MAX_MESH_VALUES
        && indices.len() <= FILL_WORKER_MAX_MESH_VALUES
        && positions.iter().all(|value| value.is_finite())
        && indices.iter().all(|index| *index < vertices)
}

/// 🗄️ Process-wide derived-mesh authority: the framework's own content-addressed LRU plus the
/// url→handle index that lets a later command — or an entirely different open document — reach a mesh
/// the client uploaded once, so the wire only ever needs to carry the mesh id again.
struct Puzzle3dBrushMeshStore {
    engines: store::EngineCache,
    handles: HashMap<String, store::EngineHandle>,
}

fn brush_mesh_store() -> &'static Mutex<Puzzle3dBrushMeshStore> {
    static STORE: OnceLock<Mutex<Puzzle3dBrushMeshStore>> = OnceLock::new();
    STORE.get_or_init(|| {
        let mut engines = store::EngineCache::new(BRUSH_MESH_CACHE_BYTES);
        engines.register(Puzzle3dMeshDecodeEngine);
        Mutex::new(Puzzle3dBrushMeshStore { engines, handles: HashMap::new() })
    })
}

/// 🧮️ Derives one uploaded mesh into the process-wide store and hands back the validated geometry —
/// a second document registering the same identity hits the cache instead of decoding again.
pub(crate) fn derive_brush_mesh(url: &str, positions: &[f32], indices: &[u32]) -> Option<(Vec<f32>, Vec<u32>)> {
    let request = encode_brush_mesh_request(url, positions, indices)?;
    let mut store = brush_mesh_store().try_lock().ok()?;
    let handle = store.engines.derive(<Puzzle3dMeshDecodeEngine as store::Engine>::ENGINE_ID, &request).ok()?;
    let geometry = store.engines.read(&handle).ok()?;
    store.handles.insert(url.to_string(), handle);
    decode_brush_mesh_geometry(&geometry)
}

/// 📖 Geometry this process already derived for one mesh id, or `None` once the LRU evicted it — a
/// miss is a cache miss, never a correctness change: the client is asked for the bytes again.
pub(crate) fn shared_brush_mesh(url: &str) -> Option<(Vec<f32>, Vec<u32>)> {
    let store = brush_mesh_store().try_lock().ok()?;
    let geometry = store.engines.read(store.handles.get(url)?).ok()?;
    decode_brush_mesh_geometry(&geometry)
}
//#endregion 🥽️SharedBrushMeshes

//#region 🧩️PagedBrushMeshUploads
/// 📏️ Values — `f32` positions and `u32` indices counted together — one `registerBrushMesh` page
/// carries. 1 024 values are 4 096 little-endian payload bytes, 5 464 base64 characters; the shared
/// retained wire admits [`PUZZLE_COMMAND_RAW_BYTES`] (8 192) bytes per command, which leaves the
/// envelope (`surfaceId`, `url`, `page`, `pageCount`, `digest`, two key names) 2 700 bytes of head
/// room. The host's own pager (`🌐️World3dHost/🟦️.tsx`) shrinks a page below this only when a long
/// mesh id would otherwise push the envelope over the same limit.
///
/// [`PUZZLE_COMMAND_RAW_BYTES`]: crate::retained_command::PUZZLE_COMMAND_RAW_BYTES
pub const PUZZLE3D_MESH_PAGE_VALUES: usize = 1_024;

/// 🔤️ Base64 characters one page's payload string may hold — [`PUZZLE3D_MESH_PAGE_VALUES`] values as
/// four bytes each, padded to the codec's four-character group.
pub const PUZZLE3D_MESH_PAGE_BASE64_CHARS: usize = (PUZZLE3D_MESH_PAGE_VALUES * 4).div_ceil(3).div_ceil(4) * 4;

/// 📦️ Partial uploads the process stages at once. A sequence is opened and closed by one mesh
/// identity's own page run, so four covers every world window a desktop layout opens at once, and a
/// fifth identity retires the least recently advanced slot instead of growing the staging area.
const PUZZLE3D_MESH_UPLOAD_SLOTS: usize = 4;

/// 🧮️ Longest page run one mesh identity may claim: both arrays at the engine's own
/// [`FILL_WORKER_MAX_MESH_VALUES`] ceiling, paged at [`PUZZLE3D_MESH_PAGE_VALUES`]. The largest mesh
/// this repo ships — `🧊️placeholder.glb`, 25 344 positions plus 48 384 indices — is 72 pages.
const PUZZLE3D_MESH_UPLOAD_MAX_PAGES: u32 = (FILL_WORKER_MAX_MESH_VALUES * 2).div_ceil(PUZZLE3D_MESH_PAGE_VALUES) as u32;

/// 🚫️ Why one `registerBrushMesh` page was refused. Every arm is a wire fault the client can act on,
/// never a silent drop: the command arm turns it into a shell notification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle3dMeshUploadFault {
    /// 🏷️ `page`/`pageCount` are absent, zero, out of order or beyond [`PUZZLE3D_MESH_UPLOAD_MAX_PAGES`].
    Envelope,
    /// 🔤️ A payload string is not base64, or carries a value count above [`PUZZLE3D_MESH_PAGE_VALUES`].
    Payload,
    /// 🕳️ The page does not continue the staged run for this identity.
    Gap,
    /// 📦️ The run would exceed [`FILL_WORKER_MAX_MESH_VALUES`] in one of its two arrays.
    Capacity,
    /// #️⃣ The closed run does not hash to the digest the client declared.
    Digest,
    /// 📐️ The closed run is not a finite, closed, in-range indexed triangle page.
    Geometry,
}

impl Puzzle3dMeshUploadFault {
    /// 🏷️ Stable wire code, mirrored by the `registerBrushMesh` notification and the unit laws.
    pub fn code(self) -> &'static str {
        match self {
            Self::Envelope => "puzzle3d-register-mesh-envelope",
            Self::Payload => "puzzle3d-register-mesh-payload",
            Self::Gap => "puzzle3d-register-mesh-gap",
            Self::Capacity => "puzzle3d-register-mesh-capacity",
            Self::Digest => "puzzle3d-register-mesh-digest",
            Self::Geometry => "puzzle3d-register-mesh-geometry",
        }
    }
}

/// 🧱️ What one accepted page did to its identity's run.
#[derive(Clone, Debug, PartialEq)]
pub enum Puzzle3dMeshUploadStep {
    /// 🧱️ The page landed and the run is still open.
    Staged { next_page: u32, page_count: u32 },
    /// ✅️ The page closed the run; the geometry is the client's mesh, digest-verified.
    Complete(Vec<f32>, Vec<u32>),
}

/// 🧵️ One mesh identity's open page run.
struct Puzzle3dMeshUploadSlot {
    url: String,
    digest: String,
    page_count: u32,
    next_page: u32,
    positions: Vec<f32>,
    indices: Vec<u32>,
    touched: u64,
}

/// 🗄️ Process-wide staging area for page runs that have not closed yet. Fixed capacity in slots and,
/// per slot, in values — an abandoned run costs the process one slot until the next session retirement
/// sweeps it, never unbounded memory.
struct Puzzle3dBrushMeshUploads {
    slots: Vec<Puzzle3dMeshUploadSlot>,
    sequence: u64,
    swept: u64,
}

fn brush_mesh_uploads() -> &'static Mutex<Puzzle3dBrushMeshUploads> {
    static UPLOADS: OnceLock<Mutex<Puzzle3dBrushMeshUploads>> = OnceLock::new();
    UPLOADS.get_or_init(|| Mutex::new(Puzzle3dBrushMeshUploads { slots: Vec::new(), sequence: 0, swept: 0 }))
}

/// #️⃣ The identity a client declares for a mesh: unkeyed BLAKE3 over the positions' little-endian
/// bytes followed by the indices' — byte-identical to the renderer's own `puzzle3dBrushMeshDigest`
/// (`🌐️World3dHost/🟦️.tsx`), which hashes the same two typed arrays through the framework's
/// first-party `blake3Hex`.
pub fn brush_mesh_digest(positions: &[f32], indices: &[u32]) -> String {
    let mut bytes = Vec::with_capacity(positions.len().saturating_add(indices.len()).saturating_mul(4));
    bytes.extend(positions.iter().flat_map(|value| value.to_le_bytes()));
    bytes.extend(indices.iter().flat_map(|value| value.to_le_bytes()));
    semio_framework_hash::hash_bytes(&bytes)
}

/// 🔤️ One page's payload string as the four-byte values it carries — base64 of little-endian `f32`
/// positions or `u32` indices. Refuses a string longer than one page may carry, a byte run that is not
/// whole values, and a value count above the caller's remaining page budget.
pub fn decode_brush_mesh_page_values(encoded: &str, budget: usize) -> Option<Vec<[u8; 4]>> {
    if encoded.len() > PUZZLE3D_MESH_PAGE_BASE64_CHARS || budget > PUZZLE3D_MESH_PAGE_VALUES {
        return None;
    }
    let bytes = semio_framework_io_base64::base64_standard_decode(encoded).ok()?;
    (bytes.len().is_multiple_of(4) && bytes.len() / 4 <= budget).then(|| bytes.as_chunks::<4>().0.to_vec())
}

/// 🧩️ Admits one page of a mesh identity's upload run into the staging area, and hands back the whole
/// geometry the moment the run closes. A run is keyed by `(url, digest)`: a client that restarts an
/// upload with different bytes opens a different run instead of corrupting the staged one, and the
/// closed run is refused unless it hashes to the digest that keyed it.
pub fn stage_brush_mesh_page(url: &str, digest: &str, page: u32, page_count: u32, positions: &[f32], indices: &[u32]) -> Result<Puzzle3dMeshUploadStep, Puzzle3dMeshUploadFault> {
    if digest.is_empty() || page_count == 0 || page_count > PUZZLE3D_MESH_UPLOAD_MAX_PAGES || page >= page_count {
        return Err(Puzzle3dMeshUploadFault::Envelope);
    }
    if positions.len().saturating_add(indices.len()) > PUZZLE3D_MESH_PAGE_VALUES {
        return Err(Puzzle3dMeshUploadFault::Payload);
    }
    let mut uploads = brush_mesh_uploads().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    uploads.sequence = uploads.sequence.saturating_add(1);
    let sequence = uploads.sequence;
    let held = uploads.slots.iter().position(|slot| slot.url == url && slot.digest == digest);
    let index = match held {
        Some(index) if uploads.slots[index].next_page == page && uploads.slots[index].page_count == page_count => index,
        Some(_) if page != 0 => return Err(Puzzle3dMeshUploadFault::Gap),
        Some(index) => {
            uploads.slots[index] = Puzzle3dMeshUploadSlot { url: url.to_string(), digest: digest.to_string(), page_count, next_page: 0, positions: Vec::new(), indices: Vec::new(), touched: sequence };
            index
        }
        None if page != 0 => return Err(Puzzle3dMeshUploadFault::Gap),
        None => {
            if uploads.slots.len() >= PUZZLE3D_MESH_UPLOAD_SLOTS {
                let stale = uploads.slots.iter().enumerate().min_by_key(|(_, slot)| slot.touched).map(|(index, _)| index).ok_or(Puzzle3dMeshUploadFault::Capacity)?;
                uploads.slots.swap_remove(stale);
            }
            uploads.slots.push(Puzzle3dMeshUploadSlot { url: url.to_string(), digest: digest.to_string(), page_count, next_page: 0, positions: Vec::new(), indices: Vec::new(), touched: sequence });
            uploads.slots.len() - 1
        }
    };
    let slot = &mut uploads.slots[index];
    if slot.positions.len().saturating_add(positions.len()) > FILL_WORKER_MAX_MESH_VALUES || slot.indices.len().saturating_add(indices.len()) > FILL_WORKER_MAX_MESH_VALUES {
        uploads.slots.swap_remove(index);
        return Err(Puzzle3dMeshUploadFault::Capacity);
    }
    slot.positions.extend_from_slice(positions);
    slot.indices.extend_from_slice(indices);
    slot.next_page = page.saturating_add(1);
    slot.touched = sequence;
    if slot.next_page < slot.page_count {
        return Ok(Puzzle3dMeshUploadStep::Staged { next_page: slot.next_page, page_count: slot.page_count });
    }
    let closed = uploads.slots.swap_remove(index);
    if brush_mesh_digest(&closed.positions, &closed.indices) != closed.digest {
        return Err(Puzzle3dMeshUploadFault::Digest);
    }
    if !brush_mesh_geometry_is_admissible(&closed.positions, &closed.indices) {
        return Err(Puzzle3dMeshUploadFault::Geometry);
    }
    Ok(Puzzle3dMeshUploadStep::Complete(closed.positions, closed.indices))
}

/// 🧹️ Drops every run that did not advance across a whole session-retirement cycle — the page runs a
/// closed document instance abandoned. A run that is still being paged carries a `touched` newer than
/// the watermark and survives.
pub fn retire_abandoned_brush_mesh_uploads() {
    let mut uploads = brush_mesh_uploads().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let watermark = uploads.swept;
    uploads.slots.retain(|slot| slot.touched > watermark);
    uploads.swept = uploads.sequence;
}

/// 🔎️ Open page runs, for the unit laws and the staging census.
pub fn staged_brush_mesh_uploads() -> Vec<(String, String, u32, u32)> {
    let uploads = brush_mesh_uploads().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    uploads.slots.iter().map(|slot| (slot.url.clone(), slot.digest.clone(), slot.next_page, slot.page_count)).collect()
}
//#endregion 🧩️PagedBrushMeshUploads

//#region 🔖️Engine
/// 🗺️ Cell edge of the interactive brush broad phase — the same 8.0 world units the bulk fill planner
/// indexes with, so one document's two lanes bucket identically.
const BRUSH_INDEX_CELL_SIZE: f32 = 8.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[derive(Default)]
enum BrushIndexSyncStage {
    #[default]
    Objects,
    CollectStale,
    Removals,
}

/// 🔁️ Resumable, step-budgeted reconciliation of the persistent brush broad-phase index against the
/// scene the host last synced in: one object replacement per step, then one withdrawal per stale cell,
/// so a document-scale edit never rebuilds the index and no single step exceeds the lane budget.
#[derive(Default)]
struct BrushIndexSync {
    stage: BrushIndexSyncStage,
    object_cursor: usize,
    seen: std::collections::HashSet<String>,
    stale: Vec<String>,
    mutation: Option<CollisionIndexMutation>,
    removal: Option<CollisionIndexRemoval>,
}



pub(crate) struct Puzzle3dCollision {
    pub(crate) scene: Option<Arc<SceneConfig>>,
    /// 🧊️ The last scene this engine was synced FROM, so a resync with an identical config (every
    /// action re-syncs the session, see the app's `sync_precompute_session`) can skip `rebuild_queue`
    /// instead of wiping `brush_cache`/`fill`/`queue` and restarting suggestion+fill precompute from
    /// zero. Held as the decoded value, never as its JSON text: normalizing a 180-object document to a
    /// string to compare it cost 13 ms per round trip and three round trips per sync — 39 ms of the
    /// 44 ms every command spent here, over the framework's whole 8 ms interactive step ceiling on its
    /// own.
    scene_synced: Option<Arc<SceneConfig>>,
    meshes: Arc<HashMap<String, CollisionBody>>,
    mesh_is_fallback: HashMap<String, bool>,
    mesh_sources: HashMap<String, FillWorkerMesh>,
    pub(crate) brush_cache: HashMap<String, BrushCollisionFreeResult>,
    pub(crate) brush_queue: VecDeque<String>,
    brush_prepare_object_cursor: usize,
    brush_prepare_vortex_cursor: usize,
    brush_queue_preparing: bool,
    /// 🗺️ Persistent broad phase of the interactive brush lane — reconciled incrementally from the
    /// scene the host syncs in, never rebuilt per collision check, and carried across worker hops by
    /// the app's own session slot.
    brush_index: CollisionSpatialIndex,
    brush_index_owner: CollisionIndexOwner,
    brush_index_sync: Option<BrushIndexSync>,
    brush_index_ready: bool,
    /// 🧊️ World placement of every indexed owner, so a broad-phase candidate id resolves to its pose
    /// and mesh in constant time instead of a scan over the fixture.
    brush_placed: HashMap<String, PlacedCollisionEntry>,
    fill_steps_remaining: usize,
    pub(crate) fill: Option<SharedFillBuilder>,
    fill_worker: Option<MountedFillWorker>,
    fill_rejected_worker: Option<RejectedFillWorker>,
    fill_worker_outcome: Option<StepOutcome>,
    fill_worker_terminal: bool,
    fill_cancel: CancelToken,
    fill_revision: u64,
    fill_generation: u64,
    fill_preview_sequence: u64,
}

/// ♻️ The rejected fill-worker admission is an owner, not a flag: `WorkerJobSessionAdmissionRejected`
/// asserts its exact incremental close in `Drop` (`🧵️job/🦀️.rs`), so every path that lets a collision
/// engine go — the session's own Drop, a standalone engine in a test, a replaced engine — must retire
/// it here rather than each caller remembering to.
impl Drop for Puzzle3dCollision {
    fn drop(&mut self) {
        if let Some(rejected) = self.fill_rejected_worker.take() {
            retire_rejected_fill_worker(rejected);
        }
    }
}

impl Puzzle3dCollision {
    pub(crate) fn new() -> Self {
        Self {
            scene: None,
            scene_synced: None,
            meshes: Arc::new(HashMap::new()),
            mesh_is_fallback: HashMap::new(),
            mesh_sources: HashMap::new(),
            brush_cache: HashMap::new(),
            brush_queue: VecDeque::new(),
            brush_prepare_object_cursor: 0,
            brush_prepare_vortex_cursor: 0,
            brush_queue_preparing: false,
            brush_index: CollisionSpatialIndex::new(BRUSH_INDEX_CELL_SIZE),
            brush_index_owner: CollisionIndexOwner { operation: 1, generation: 1 },
            brush_index_sync: None,
            brush_index_ready: false,
            brush_placed: HashMap::new(),
            fill_steps_remaining: 0,
            fill: None,
            fill_worker: None,
            fill_rejected_worker: None,
            fill_worker_outcome: None,
            fill_worker_terminal: false,
            fill_cancel: root_cancel_token(),
            fill_revision: 0,
            fill_generation: 0,
            fill_preview_sequence: 0,
        }
    }

    fn fill_lane_active(&self) -> bool {
        self.fill.is_some() && self.fill_steps_remaining > 0
    }

    fn brush_lane_active(&self) -> bool {
        self.brush_queue_preparing || !self.brush_queue.is_empty()
    }

    fn re_enqueue_brush_targets(&mut self) {
        self.brush_prepare_object_cursor = 0;
        self.brush_prepare_vortex_cursor = 0;
        self.brush_queue_preparing = self.scene.is_some();
        self.begin_brush_index_sync();
    }

    /// 🔁️ Arms one incremental reconciliation pass of the persistent brush broad phase and bumps the
    /// index generation, so any query or replacement still in flight from the previous scene goes
    /// `Stale` instead of mixing two scenes' owners.
    fn begin_brush_index_sync(&mut self) {
        let Some(generation) = self.brush_index_owner.generation.checked_add(1) else {
            self.brush_index = CollisionSpatialIndex::new(BRUSH_INDEX_CELL_SIZE);
            self.brush_placed.clear();
            self.brush_index_sync = None;
            self.brush_index_ready = false;
            return;
        };
        self.brush_index_owner = CollisionIndexOwner { operation: self.brush_index_owner.operation, generation };
        self.brush_index_sync = self.scene.is_some().then(BrushIndexSync::default);
        self.brush_index_ready = false;
    }

    /// 🔁️ Brings the persistent broad phase up to date with the installed scene, within the caller's own
    /// deadline. A query is only ever answered against a reconciled index; if the budget runs out first the
    /// caller sees `unknown_pending` and the vortex is re-queued, which is the lane's existing resume
    /// contract. Arms a pass itself, so a scene installed without a lane tick is still indexed.
    fn reconcile_brush_index_until(&mut self, deadline_us: u64) {
        if !self.brush_index_ready && self.brush_index_sync.is_none() {
            self.begin_brush_index_sync();
        }
        while self.brush_index_sync.is_some() && default_now_us().is_some_and(|now| now < deadline_us) {
            self.step_brush_index();
        }
    }

    /// 🗺️ One unit of broad-phase reconciliation: one object replacement cell, one stale-owner scan, or
    /// one withdrawal cell. Returns whether more work is pending, which is what keeps the brush lane
    /// ticking under its own 2 ms budget instead of stalling a worker turn.
    fn step_brush_index(&mut self) -> bool {
        let Some(mut sync) = self.brush_index_sync.take() else { return false };
        let Some(scene) = self.scene.clone() else {
            self.brush_index_ready = false;
            return false;
        };
        let pending = match sync.stage {
            BrushIndexSyncStage::Objects => self.step_brush_index_objects(&mut sync, &scene),
            BrushIndexSyncStage::CollectStale => {
                let seen = std::mem::take(&mut sync.seen);
                sync.stale = self.brush_index.entry_ids().filter(|id| !seen.contains(id.as_str())).cloned().collect();
                sync.stage = BrushIndexSyncStage::Removals;
                true
            }
            BrushIndexSyncStage::Removals => self.step_brush_index_removals(&mut sync),
        };
        if pending {
            self.brush_index_sync = Some(sync);
        } else {
            self.brush_index_ready = true;
        }
        pending
    }

    fn step_brush_index_objects(&mut self, sync: &mut BrushIndexSync, scene: &SceneConfig) -> bool {
        let owner = self.brush_index_owner;
        if let Some(mutation) = sync.mutation.as_mut() {
            match self.brush_index.step_replacement(mutation, owner) {
                CollisionMutationStep::Pending => return true,
                CollisionMutationStep::Rejected(mut rejected) => while !rejected.retire_one() {},
                CollisionMutationStep::Complete | CollisionMutationStep::Stale => {}
            }
            sync.mutation = None;
            return true;
        }
        let Some(object) = scene.fixture.objects.get(sync.object_cursor) else {
            sync.stage = BrushIndexSyncStage::CollectStale;
            return true;
        };
        sync.object_cursor += 1;
        let empty_catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
        let catalogs = scene.kind_catalogs.as_ref().unwrap_or(&empty_catalogs);
        let Some(mesh_url) = resolve_object_kind_mesh_url(object.object_kind.as_deref().unwrap_or(""), catalogs, &scene.fixture) else { return true };
        let world = pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale);
        let Some(bounds) = self.meshes.get(&mesh_url).map(|body| CollisionAabb::from_body(body, &world)) else { return true };
        sync.seen.insert(object.id.clone());
        if self.brush_index.entry_bounds(object.id.as_str()) != Some(&bounds) {
            sync.mutation = Some(self.brush_index.begin_replacement(owner, object.id.clone(), bounds));
        }
        self.brush_placed.insert(object.id.clone(), PlacedCollisionEntry { object_id: object.id.clone(), mesh_url, world });
        true
    }

    fn step_brush_index_removals(&mut self, sync: &mut BrushIndexSync) -> bool {
        let owner = self.brush_index_owner;
        if let Some(removal) = sync.removal.as_mut() {
            if matches!(self.brush_index.step_removal(removal, owner), CollisionMutationStep::Pending) {
                return true;
            }
            sync.removal = None;
            return true;
        }
        let Some(id) = sync.stale.pop() else { return false };
        self.brush_placed.remove(id.as_str());
        sync.removal = self.brush_index.begin_removal(owner, id);
        true
    }

    fn prepare_one_brush_target(&mut self) {
        let Some(scene) = &self.scene else {
            self.brush_queue_preparing = false;
            return;
        };
        let Some(object) = scene.fixture.objects.get(self.brush_prepare_object_cursor) else {
            self.brush_queue_preparing = false;
            return;
        };
        let Some(vortex) = object.vortices.get(self.brush_prepare_vortex_cursor) else {
            self.brush_prepare_object_cursor += 1;
            self.brush_prepare_vortex_cursor = 0;
            return;
        };
        self.brush_prepare_vortex_cursor += 1;
        let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
        if !self.brush_cache.contains_key(&full_id) {
            self.brush_queue.push_back(full_id);
        }
    }

    fn allocate_fill_identity(&mut self, advance_revision: bool) -> Option<(RevisionId, Generation)> {
        let revision = if advance_revision { self.fill_revision.checked_add(1)? } else { self.fill_revision };
        let generation = self.fill_generation.checked_add(1)?;
        if revision == 0 || generation == 0 {
            return None;
        }
        self.fill_revision = revision;
        self.fill_generation = generation;
        Some((RevisionId(revision), Generation(generation)))
    }

    fn rebuild_queue(&mut self) {
        self.start_fill_preparation(true);
    }

    fn start_fill_preparation(&mut self, advance_revision: bool) {
        let Some((revision, generation)) = self.allocate_fill_identity(advance_revision) else {
            return;
        };
        self.fill_cancel.cancel_now();
        self.fill_cancel = root_cancel_token();
        self.fill_preview_sequence = 0;
        self.brush_queue.clear();
        self.brush_cache.clear();
        self.re_enqueue_brush_targets();
        self.fill_steps_remaining = 0;
        if let Some(scene) = self.scene.clone() {
            self.fill_steps_remaining = FILL_COUNT_MAX;
            let operation = Operation::new(semio_framework_job::allocate_operation_id(), revision, generation, scene.seed as u64);
            let fill = FillBuilder::begin_preparation(FillPreparationRoots::new(scene, self.meshes.clone()), operation);
            self.fill = Some(Arc::new(Mutex::new(fill)));
            let fill = Arc::clone(self.fill.as_ref().expect("fresh fill owner"));
            match mount_fill_worker(fill, operation, self.fill_cancel.clone()) {
                Ok(worker) => self.fill_worker = Some(worker),
                Err(mut rejected) => {
                    rejected.begin_close();
                    if let Some(previous) = self.fill_rejected_worker.replace(rejected) {
                        retire_rejected_fill_worker(previous);
                    }
                    self.fill_steps_remaining = 0;
                }
            }
            self.fill_worker_outcome = None;
            self.fill_worker_terminal = false;
        } else {
            self.fill = None;
            self.fill_worker = None;
            if let Some(rejected) = self.fill_rejected_worker.take() {
                retire_rejected_fill_worker(rejected);
            }
            self.fill_worker_outcome = None;
            self.fill_worker_terminal = false;
        }
    }

    /// 🎚️ Distribution-weight edits must not `rebuild_queue()` — applied fill objects stay, only the
    /// unapplied planning tail is discarded and re-enqueued for background `fillBuildTick` planning.
    /// The live builder is replanned IN PLACE (see [`FillBuilder::begin_soft_replan`]); constructing a
    /// fresh one instead threw away every applied placement the moment a slider moved.
    fn soft_replan_fill_tail(&mut self) {
        let Some(scene) = self.scene.clone() else {
            return;
        };
        let Some(owner) = self.fill.clone() else {
            return;
        };
        let Ok(mut fill) = owner.try_lock() else {
            return;
        };
        fill.begin_soft_replan(&scene.weights.object_weights, &scene.weights.vortex_weights);
        self.fill_steps_remaining = fill.max_count.saturating_sub(fill.applied_count);
        drop(fill);
        self.re_enqueue_brush_targets();
    }

    fn refresh_fill_job(&mut self, _refresh_meshes: bool) {
        self.start_fill_preparation(false);
    }

    pub(crate) fn update_kind_weights(&mut self, object_weights: std::collections::BTreeMap<String, f64>, vortex_weights: std::collections::BTreeMap<String, f64>) {
        if let Some(scene) = &mut self.scene {
            let scene = Arc::make_mut(scene);
            scene.weights.object_weights = object_weights;
            scene.weights.vortex_weights = vortex_weights;
            self.scene_synced = Some(Arc::new(scene.clone()));
        }
        self.brush_cache.clear();
        if self.fill.is_none() {
            self.rebuild_queue();
        } else {
            self.soft_replan_fill_tail();
        }
    }

    /// 🪣️ True when `fixture` is the fill plan's base plus zero-or-more applied fill objects — i.e. the
    /// live document after `setFillCount`, which must NOT rebuild the precompute session or the slider
    /// loses its ability to remove/replan those objects.
    fn is_fill_applied_projection(fixture: &Fixture, fill: &FillBuilder) -> bool {
        let plan_objects: std::collections::HashSet<&str> = fill.appended_objects.iter().map(|object| object.id.as_str()).collect();
        let plan_attractions: std::collections::HashSet<&str> = fill.appended_attractions.iter().map(|attraction| attraction.id.as_str()).collect();
        let base_objects: std::collections::HashSet<&str> = fill.base.objects.iter().map(|object| object.id.as_str()).collect();
        let base_attractions: std::collections::HashSet<&str> = fill.base.attractions.iter().map(|attraction| attraction.id.as_str()).collect();
        let base_volumes: std::collections::HashSet<&str> = fill.base.target_volumes.iter().map(|volume| volume.id.as_str()).collect();
        let incoming_objects: std::collections::HashSet<&str> = fixture.objects.iter().map(|object| object.id.as_str()).filter(|id| !plan_objects.contains(id)).collect();
        let incoming_attractions: std::collections::HashSet<&str> = fixture.attractions.iter().map(|attraction| attraction.id.as_str()).filter(|id| !plan_attractions.contains(id)).collect();
        let incoming_volumes: std::collections::HashSet<&str> = fixture.target_volumes.iter().map(|volume| volume.id.as_str()).collect();
        incoming_objects == base_objects && incoming_attractions == base_attractions && incoming_volumes == base_volumes
    }

    fn strip_fill_plan_from_fixture(fixture: &mut Fixture, fill: &FillBuilder) {
        let plan_objects: std::collections::HashSet<&str> = fill.appended_objects.iter().map(|object| object.id.as_str()).collect();
        let plan_attractions: std::collections::HashSet<&str> = fill.appended_attractions.iter().map(|attraction| attraction.id.as_str()).collect();
        fixture.objects.retain(|object| !plan_objects.contains(object.id.as_str()));
        fixture.attractions.retain(|attraction| !plan_attractions.contains(attraction.id.as_str()));
    }

    pub(crate) fn set_scene(&mut self, json: &str) -> Result<(), Puzzle3dError> {
        self.set_scene_config(dsl::os_pack::json::from_json_str(json)?);
        Ok(())
    }

    /// 🧊️ The typed sync every caller actually takes: the app already holds a decoded `SceneConfig`,
    /// so routing it through a JSON string and back is three whole-document round trips of pure loss.
    pub(crate) fn set_scene_config(&mut self, scene: SceneConfig) {
        let mut scene = scene;
        // 🪣️ After the fill slider materializes objects into the document, every incidental action
        // (hover, pick, mesh register sync, …) re-feeds that applied projection here. Treating it as a
        // brand-new scene used to `rebuild_queue()` and bake the filled objects into `fill.base`, after
        // which the slider could neither remove them nor replan a fresh tail.
        let applied_projection = self.fill.as_ref().and_then(|fill| fill.try_lock().ok()).is_some_and(|fill| Self::is_fill_applied_projection(&scene.fixture, &fill));
        if applied_projection {
            if let Some(fill) = self.fill.as_ref().and_then(|fill| fill.try_lock().ok()) {
                Self::strip_fill_plan_from_fixture(&mut scene.fixture, &fill);
            }
            if let Some(current) = &mut self.scene {
                let current = Arc::make_mut(current);
                current.overlap_budget = scene.overlap_budget;
                current.seed = scene.seed;
                current.weights = scene.weights.clone();
                current.kind_catalogs = scene.kind_catalogs.clone();
                current.kind_compatibility = scene.kind_compatibility.clone();
                current.host_rules = scene.host_rules.clone();
            }
            self.scene_synced = Some(Arc::new(scene));
            return;
        }
        if self.scene_synced.as_deref() == Some(&scene) {
            return;
        }
        let scene = Arc::new(scene);
        self.scene = Some(Arc::clone(&scene));
        self.scene_synced = Some(scene);
        self.rebuild_queue();
    }

    /// 🥽️ Real geometry for one mesh identity, derived once per process through the content-addressed
    /// decode kernel — the id-only wire path a second document (or a later command on the same one)
    /// takes instead of re-uploading buffers.
    pub(crate) fn adopt_shared_mesh(&mut self, url: &str, digest: Option<&str>) -> bool {
        if self.mesh_is_fallback.get(url) == Some(&false) && digest.is_none_or(|digest| self.mesh_sources.get(url).is_some_and(|mesh| brush_mesh_digest(&mesh.positions, &mesh.indices) == digest)) {
            return true;
        }
        let Some((positions, indices)) = shared_brush_mesh(url) else { return false };
        if digest.is_some_and(|digest| brush_mesh_digest(&positions, &indices) != digest) {
            return false;
        }
        self.install_collision_mesh(url.to_string(), &positions, &indices, false);
        true
    }

    fn place_collision_mesh(&mut self, url: String, positions: &[f32], indices: &[u32], is_fallback: bool) -> bool {
        if url.len() > FILL_WORKER_MAX_URL_BYTES || positions.len() > FILL_WORKER_MAX_MESH_VALUES || indices.len() > FILL_WORKER_MAX_MESH_VALUES {
            return false;
        }
        if !self.mesh_sources.contains_key(&url) && self.mesh_sources.len() >= FILL_WORKER_MAX_MESHES {
            return false;
        }
        let Some(body) = crate::editor::puzzle3d::precompute::geometry::collision_body_from_buffers(positions, indices) else {
            return false;
        };
        if self.mesh_is_fallback.get(&url) == Some(&false) {
            return false;
        }
        Arc::make_mut(&mut self.meshes).insert(url.clone(), body);
        self.mesh_is_fallback.insert(url.clone(), is_fallback);
        self.mesh_sources.insert(url.clone(), FillWorkerMesh { url, positions: positions.to_vec(), indices: indices.to_vec(), fallback: is_fallback });
        true
    }

    fn install_collision_mesh(&mut self, url: String, positions: &[f32], indices: &[u32], is_fallback: bool) {
        if !self.place_collision_mesh(url, positions, indices, is_fallback) {
            return;
        }
        self.brush_cache.clear();
        if self.fill.is_none() {
            self.rebuild_queue();
        } else {
            self.refresh_fill_job(true);
        }
        self.re_enqueue_brush_targets();
    }

    pub(crate) fn register_mesh_fallback(&mut self, url: String, positions: &[f32], indices: &[u32]) {
        self.install_collision_mesh(url, positions, indices, true);
    }

    /// 🥽️ Uploaded geometry for one mesh identity: derived into the process-wide content-addressed
    /// store first, so every other open document and every later command reaches it by id alone.
    pub(crate) fn register_mesh(&mut self, url: String, positions: &[f32], indices: &[u32]) {
        match derive_brush_mesh(&url, positions, indices) {
            Some((positions, indices)) => self.install_collision_mesh(url, &positions, &indices, false),
            None => self.install_collision_mesh(url, positions, indices, false),
        }
    }

    pub(crate) fn has_mesh(&self, url: &str) -> bool {
        self.meshes.contains_key(url)
    }

    /// 🧊️ Drops a cached brush-candidate entry and re-queues that vortex at the front so a just-opened
    /// suggestion popup is not stuck on a stale empty / pending result.
    pub(crate) fn invalidate_brush_target(&mut self, vortex_full_id: &str) {
        self.brush_cache.remove(vortex_full_id);
        self.brush_queue.retain(|id| id != vortex_full_id);
        self.brush_queue.push_front(vortex_full_id.to_string());
    }

    pub(crate) fn enqueue_brush_target(&mut self, vortex_full_id: &str) {
        if !self.brush_queue.iter().any(|id| id == vortex_full_id) {
            self.brush_queue.push_back(vortex_full_id.to_string());
        }
    }

    /// 🧊️ Recomputes and caches brush candidates for one vortex immediately (used when opening / accepting
    /// the suggestion popup so the UI does not wait on the background queue).
    pub(crate) fn refresh_brush_candidates(&mut self, vortex_full_id: &str) {
        let Some(deadline) = puzzle3d_deadline(PUZZLE3D_PRECOMPUTE_STEP_BUDGET_US) else {
            return;
        };
        let prior = self.brush_cache.get(vortex_full_id).cloned();
        let resume_from = prior.as_ref().map_or(0, |entry| entry.resume_candidate_index);
        let prior_free = prior.map(|entry| entry.free).unwrap_or_default();
        let result = self.compute_brush_cache_entry_partial(vortex_full_id, resume_from, prior_free, deadline);
        // 🖌️ A NON-TERMINAL entry is owed another slice, whatever its resume cursor says. Gating the
        // re-queue on `resume_candidate_index > 0` livelocked every target that could not clear its
        // broad-phase index (`brush_index_ready`, `brush_collision_free_until`) or its very first
        // narrow-phase candidate inside one 500 µs slice: the entry landed in `brush_cache` pending,
        // which also stops `prepare_one_brush_target` re-enqueuing it, so the lane could never come
        // back to it and the suggestion popup opened empty forever.
        if result.unknown_pending && !self.brush_queue.iter().any(|id| id == vortex_full_id) {
            self.brush_queue.push_front(vortex_full_id.to_string());
        }
        self.brush_cache.insert(vortex_full_id.to_string(), result);
    }

    fn preview_collides(meshes: &HashMap<String, CollisionBody>, preview: &BrushPreviewState, placed: &[PlacedCollisionEntry], overlap_budget: f64, sample_count: usize, deadline_us: u64) -> Option<bool> {
        struct BrushCollisionContext {
            deadline_us: u64,
        }
        impl CollisionStepContext for BrushCollisionContext {
            fn is_cancelled(&self) -> bool {
                false
            }
            fn should_yield(&self) -> bool {
                default_now_us().is_none_or(|now| now >= self.deadline_us)
            }
            fn consume_fuel(&mut self, _units: u64) {}
        }
        let preview_body = meshes.get(&preview.mesh_url)?;
        let preview_world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
        let (pmin, pmax) = world_bounds(preview_body, &preview_world);
        let mut context = BrushCollisionContext { deadline_us };
        for entry in placed {
            let other = meshes.get(&entry.mesh_url)?;
            let (omin, omax) = world_bounds(other, &entry.world);
            if pmax.x() < omin.x() || pmin.x() > omax.x() || pmax.y() < omin.y() || pmin.y() > omax.y() || pmax.z() < omin.z() || pmin.z() > omax.z() {
                continue;
            }
            let mut collision = CollisionOverlapState::new(sample_count, 8, overlap_budget);
            loop {
                match collision.step(&mut context, preview_body, &preview_world, other, &entry.world) {
                    CollisionStepResult::Pending if context.should_yield() => return None,
                    CollisionStepResult::Pending => {}
                    CollisionStepResult::Cancelled => return None,
                    CollisionStepResult::Complete { overlap, .. } if overlap > overlap_budget => return Some(true),
                    CollisionStepResult::Complete { .. } => break,
                }
            }
        }
        Some(false)
    }

    fn brush_collision_free_until(&mut self, target_full_id: &str, candidates: &[BrushCompatibleCandidate], overlap_budget: f64, resume_from: usize, mut free: Vec<BrushCompatibleCandidate>, deadline_us: u64) -> BrushCollisionFreeResult {
        self.reconcile_brush_index_until(deadline_us);
        let Some(scene) = self.scene.clone() else {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: true, resume_candidate_index: resume_from };
        };
        let empty_catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
        let catalogs = scene.kind_catalogs.as_ref().unwrap_or(&empty_catalogs);
        let target_obj = scene.fixture.objects.iter().find_map(|o| {
            o.vortices.iter().enumerate().find_map(|(i, v)| {
                let full_id = puzzle3d_vortex_full_id(&o.id, &v.id);
                if full_id == target_full_id {
                    Some((o, i, v))
                } else {
                    None
                }
            })
        });
        let Some((host, vortex_index, _)) = target_obj else {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: false, resume_candidate_index: 0 };
        };
        let Some((position, direction)) = vortex_world_from_object(host, vortex_index) else {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: false, resume_candidate_index: 0 };
        };
        let target_ctx = AttractionVortexContext { object_kind: host.object_kind.clone(), vortex_kind: host.vortices[vortex_index].vortex_kind.clone() };
        let host_id = host.id.clone();
        if !self.brush_index_ready {
            eprintln!("[DEBUG] free_until {target_full_id} index_not_ready candidates={}", candidates.len());
            return BrushCollisionFreeResult { free, unknown_pending: true, resume_candidate_index: resume_from };
        }
        let mut unknown_pending = false;
        let mut no_preview = 0usize;
        let mut no_mesh = 0usize;
        let mut collided = 0usize;
        for (index, candidate) in candidates.iter().enumerate().skip(resume_from) {
            if default_now_us().is_none_or(|now| now >= deadline_us) {
                return BrushCollisionFreeResult { free, unknown_pending: true, resume_candidate_index: index };
            }
            let world = TargetVortexWorld { position, direction, reference_orientation: host.orientation };
            let Some(preview) = brush_preview_from_candidate(target_full_id, candidate, &target_ctx, world, catalogs, &scene.fixture) else {
                no_preview += 1;
                continue;
            };
            if !self.meshes.contains_key(&preview.mesh_url) {
                eprintln!("[DEBUG] free_until {target_full_id} missing mesh {}", preview.mesh_url);
                no_mesh += 1;
                unknown_pending = true;
                continue;
            }
            match self.preview_collides_indexed(&preview, &host_id, overlap_budget, 1024, deadline_us) {
                None => unknown_pending = true,
                Some(true) => collided += 1,
                Some(false) => free.push(candidate.clone()),
            }
        }
        eprintln!("[DEBUG] free_until {target_full_id} candidates={} free={} no_preview={no_preview} no_mesh={no_mesh} collided={collided} pending={unknown_pending}", candidates.len(), free.len());
        BrushCollisionFreeResult { free, unknown_pending, resume_candidate_index: 0 }
    }

    /// 🗺️ Broad phase for ONE brush preview: the persistent spatial index resolves the candidate page
    /// its own world bounds actually overlap, so the narrow phase never sees an object from a distant
    /// cell. Replaces the former full-fixture rebuild plus linear scan per candidate.
    fn preview_collides_indexed(&self, preview: &BrushPreviewState, host_id: &str, overlap_budget: f64, sample_count: usize, deadline_us: u64) -> Option<bool> {
        let (page, _) = self.brush_broad_phase_page(preview, host_id, deadline_us)?;
        Self::preview_collides(&self.meshes, preview, &page, overlap_budget, sample_count, deadline_us)
    }

    /// 📏️ The queried candidate page plus the `(cells, members)` the cursor actually examined — the
    /// witness a document-scale test reads to prove the query stayed inside its own cells.
    fn brush_broad_phase_page(&self, preview: &BrushPreviewState, host_id: &str, deadline_us: u64) -> Option<(Vec<PlacedCollisionEntry>, (usize, usize))> {
        let preview_body = self.meshes.get(&preview.mesh_url)?;
        let preview_world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
        let owner = self.brush_index_owner;
        let mut query = self.brush_index.begin_query(owner, CollisionAabb::from_body(preview_body, &preview_world));
        loop {
            match self.brush_index.step_query(&mut query, owner) {
                CollisionQueryStep::Pending if default_now_us().is_none_or(|now| now >= deadline_us) => return None,
                CollisionQueryStep::Pending => {}
                CollisionQueryStep::Stale => return None,
                CollisionQueryStep::Complete => break,
            }
        }
        let page = (0..query.len()).filter_map(|index| query.candidate(index)).filter(|id| id.as_str() != host_id).filter_map(|id| self.brush_placed.get(id.as_str()).cloned()).collect();
        Some((page, query.examined()))
    }

    #[cfg(test)]
    fn brush_collision_free(&mut self, target_full_id: &str, candidates: &[BrushCompatibleCandidate], overlap_budget: f64) -> BrushCollisionFreeResult {
        let Some(deadline) = puzzle3d_deadline(PUZZLE3D_PRECOMPUTE_STEP_BUDGET_US * 8) else {
            return BrushCollisionFreeResult { free: Vec::new(), unknown_pending: true, resume_candidate_index: 0 };
        };
        self.brush_collision_free_until(target_full_id, candidates, overlap_budget, 0, Vec::new(), deadline)
    }

    #[cfg(test)]
    fn compute_brush_cache_entry(&mut self, target_full_id: &str) -> BrushCollisionFreeResult {
        let Some(scene) = &self.scene else {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: true, resume_candidate_index: 0 };
        };
        let catalogs = scene.kind_catalogs.as_ref().cloned().unwrap_or(KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] });
        let target_obj = scene.fixture.objects.iter().find_map(|o| {
            o.vortices.iter().enumerate().find_map(|(i, v)| {
                let full_id = puzzle3d_vortex_full_id(&o.id, &v.id);
                if full_id == target_full_id {
                    Some((o, i, v))
                } else {
                    None
                }
            })
        });
        let Some((host, _, vortex)) = target_obj else {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: false, resume_candidate_index: 0 };
        };
        let target_ctx = AttractionVortexContext { object_kind: host.object_kind.clone(), vortex_kind: vortex.vortex_kind.clone() };
        if !brush_target_vortex_allows_suggestion(vortex.vortex_kind.as_deref(), &scene.weights) {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: false, resume_candidate_index: 0 };
        }
        let compatible = brush_compatible_candidates(&target_ctx, &catalogs, &scene.kind_compatibility, &scene.host_rules);
        let compatible: Vec<BrushCompatibleCandidate> = compatible.into_iter().filter(|candidate| brush_candidate_suggestion_weight(candidate, &scene.weights, &catalogs) > 0.0).collect();
        self.brush_collision_free(target_full_id, &compatible, scene.overlap_budget)
    }

    pub(crate) fn brush_preview(&self, target_full_id: &str, candidate_index: usize) -> Option<BrushPreviewState> {
        let scene = self.scene.as_ref()?;
        let result = self.brush_cache.get(target_full_id)?;
        if result.unknown_pending && result.free.is_empty() {
            return None;
        }
        if result.free.is_empty() {
            return None;
        }
        let candidate = &result.free[candidate_index % result.free.len()];
        let catalogs = scene.kind_catalogs.as_ref().cloned().unwrap_or(KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] });
        let target_obj = scene.fixture.objects.iter().find_map(|object| {
            object.vortices.iter().enumerate().find_map(|(index, vortex)| {
                let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
                if full_id == target_full_id {
                    Some((object, index))
                } else {
                    None
                }
            })
        })?;
        let (host, vortex_index) = target_obj;
        let (position, direction) = vortex_world_from_object(host, vortex_index)?;
        let target_ctx = AttractionVortexContext { object_kind: host.object_kind.clone(), vortex_kind: host.vortices[vortex_index].vortex_kind.clone() };
        let world = TargetVortexWorld { position, direction, reference_orientation: host.orientation };
        brush_preview_from_candidate(target_full_id, candidate, &target_ctx, world, &catalogs, &scene.fixture)
    }

    pub(crate) fn precompute_step_lane(&mut self, lane: PrecomputeLane, budget: u32) -> bool {
        let Some(deadline) = puzzle3d_deadline(PUZZLE3D_PRECOMPUTE_STEP_BUDGET_US) else {
            return match lane {
                PrecomputeLane::Brush => self.brush_lane_active(),
                PrecomputeLane::Fill => self.fill_lane_active(),
            };
        };
        let mut remaining = budget as usize;
        while remaining > 0 {
            if default_now_us().is_none_or(|now| now >= deadline) {
                break;
            }
            match lane {
                PrecomputeLane::Brush => {
                    if self.brush_queue_preparing {
                        self.prepare_one_brush_target();
                        remaining -= 1;
                        continue;
                    }
                    let Some(full_id) = self.brush_queue.pop_front() else {
                        break;
                    };
                    let prior = self.brush_cache.get(&full_id).cloned();
                    let resume_from = prior.as_ref().map_or(0, |entry| entry.resume_candidate_index);
                    let prior_free = prior.map(|entry| entry.free).unwrap_or_default();
                    let result = self.compute_brush_cache_entry_partial(&full_id, resume_from, prior_free, deadline);
                    // 🖌️ Same law as `refresh_brush_candidates`: pending IS the owed signal.
                    let needs_resume = result.unknown_pending;
                    if needs_resume {
                        self.brush_queue.push_front(full_id.clone());
                    }
                    self.brush_cache.insert(full_id, result);
                }
                PrecomputeLane::Fill => {
                    if self.fill_steps_remaining == 0 {
                        break;
                    }
                    if let Some(rejected) = self.fill_rejected_worker.as_mut() {
                        rejected.begin_close();
                        if matches!(rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), InteractiveJobCloseStep::Complete) && rejected.terminal_is_empty() {
                            self.fill_rejected_worker.take();
                            self.fill_steps_remaining = 0;
                        }
                    } else if let Some(outcome) = self.fill_worker_outcome.as_mut() {
                        let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                        if outcome.terminal_is_empty() {
                            self.fill_worker_outcome.take();
                            if self.fill_worker_terminal || self.fill_worker.as_mut().is_none_or(|worker| worker.resume().is_err()) {
                                self.fill_steps_remaining = 0;
                            }
                        }
                    } else if let Some(worker) = self.fill_worker.as_mut() {
                        match worker.step() {
                            Ok(_) => {
                                let Some(outcome) = worker.take_outcome() else {
                                    self.fill_steps_remaining = 0;
                                    break;
                                };
                                match &outcome {
                                    StepOutcome::CheckpointReady(_) => self.fill_steps_remaining = self.fill_steps_remaining.saturating_sub(1),
                                    StepOutcome::Complete(_) | StepOutcome::Cancelled | StepOutcome::Fault(_) => self.fill_worker_terminal = true,
                                    StepOutcome::Yield | StepOutcome::PreviewReady(_) => {}
                                }
                                self.fill_worker_outcome = Some(outcome);
                            }
                            Err(_) => self.fill_steps_remaining = 0,
                        }
                    } else {
                        self.fill_steps_remaining = 0;
                    }
                }
            }
            remaining -= 1;
        }
        match lane {
            PrecomputeLane::Brush => self.brush_lane_active(),
            PrecomputeLane::Fill => self.fill_lane_active(),
        }
    }

    fn compute_brush_cache_entry_partial(&mut self, target_full_id: &str, resume_from: usize, prior_free: Vec<BrushCompatibleCandidate>, deadline_us: u64) -> BrushCollisionFreeResult {
        eprintln!("[DEBUG] partial {target_full_id} scene_none={} meshes={} objects={}", self.scene.is_none(), self.meshes.len(), self.scene.as_ref().map_or(0, |s| s.fixture.objects.len()));
        let Some(scene) = &self.scene else {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: true, resume_candidate_index: resume_from };
        };
        let catalogs = scene.kind_catalogs.as_ref().cloned().unwrap_or(KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] });
        let target_obj = scene.fixture.objects.iter().find_map(|o| {
            o.vortices.iter().enumerate().find_map(|(i, v)| {
                let full_id = puzzle3d_vortex_full_id(&o.id, &v.id);
                if full_id == target_full_id {
                    Some((o, i, v))
                } else {
                    None
                }
            })
        });
        let Some((host, _, vortex)) = target_obj else {
            return BrushCollisionFreeResult { free: prior_free, unknown_pending: false, resume_candidate_index: 0 };
        };
        let target_ctx = AttractionVortexContext { object_kind: host.object_kind.clone(), vortex_kind: vortex.vortex_kind.clone() };
        if !brush_target_vortex_allows_suggestion(vortex.vortex_kind.as_deref(), &scene.weights) {
            return BrushCollisionFreeResult { free: prior_free, unknown_pending: false, resume_candidate_index: 0 };
        }
        let compatible = brush_compatible_candidates(&target_ctx, &catalogs, &scene.kind_compatibility, &scene.host_rules);
        let compatible: Vec<BrushCompatibleCandidate> = compatible.into_iter().filter(|candidate| brush_candidate_suggestion_weight(candidate, &scene.weights, &catalogs) > 0.0).collect();
        eprintln!("[DEBUG] partial {target_full_id} compatible={}", compatible.len());
        self.brush_collision_free_until(target_full_id, &compatible, scene.overlap_budget, resume_from, prior_free, deadline_us)
    }

    pub(crate) fn precompute_step(&mut self, budget: u32) -> bool {
        let half = (budget / 2).max(1);
        let fill = self.precompute_step_lane(PrecomputeLane::Fill, half);
        let brush = self.precompute_step_lane(PrecomputeLane::Brush, budget.saturating_sub(half));
        fill || brush || self.fill_lane_active() || self.brush_lane_active()
    }

    #[cfg(test)]
    pub(crate) fn fill_progress_summary(&self) -> FillProgressSummary {
        self.fill.as_ref().and_then(|fill| fill.try_lock().ok()).map_or(FillProgressSummary { count: 0, applied_count: 0, max_count: FILL_COUNT_MAX, done: true }, |fill| FillProgressSummary {
            count: fill.sequence.len(),
            applied_count: fill.applied_count,
            max_count: fill.max_count,
            done: fill.stalled || fill.sequence.len() >= fill.max_count,
        })
    }

    #[cfg(test)]
    pub(crate) fn work_pending_for_test(&self) -> usize {
        self.brush_queue.len() + self.fill_steps_remaining
    }

    /// 📈️ Monotone witness of precompute work ACTUALLY completed — resolved suggestion targets,
    /// reconciled broad-phase owners and planner transitions. Both lanes are cursorized, so a queue
    /// length is no longer a progress measure: preparing one brush target moves it from the
    /// preparation cursor INTO the queue, leaving `work_pending_for_test` unchanged or larger.
    #[cfg(test)]
    pub(crate) fn precompute_progress_for_test(&self) -> usize {
        self.brush_cache.len() + self.brush_index.entry_len() + self.fill.as_ref().and_then(|fill| fill.try_lock().ok()).map_or(0, |fill| fill.transition_count as usize)
    }

    #[cfg(test)]
    pub(crate) fn fill_steps_pending_for_test(&self) -> usize {
        self.fill_steps_remaining
    }

    /// 🔽️ Moving the count down (or up) only changes which prefix of the already-planned sequence is
    /// applied to the document — the plan (`sequence`/`appended_*`/`placed`/`fixture`) is prefix-stable
    /// and is never discarded here, so a jittery drag can never force expensive replanning.
    #[cfg(test)]
    pub(crate) fn apply_fill_count(&mut self, count: usize) -> Option<Fixture> {
        let mut fill = self.fill.as_ref()?.try_lock().ok()?;
        let count = count.min(fill.sequence.len());
        fill.applied_count = count;
        let mut fixture = fill.base_fixture();
        // 🪣️ `revealIndex` is a live-viewport-only hint (see `compose_fill_display`) — never persist it
        // to the committed document projection.
        fixture.objects.extend(fill.appended_objects.iter().take(count).cloned().map(|mut object| {
            object.reveal_index = None;
            object
        }));
        fixture.attractions.extend(fill.appended_attractions.iter().take(count).cloned());
        Some(fixture)
    }

    /// 🪣️ Read-only prefix of the precomputed fill plan for live viewport show/hide — does not mutate
    /// `applied_count`, the queue, or the document projection.
    #[cfg(test)]
    pub(crate) fn compose_fill_display(&self, count: usize) -> Option<Fixture> {
        let fill = self.fill.as_ref()?.try_lock().ok()?;
        let visible = count.min(fill.sequence.len());
        let mut fixture = fill.base_fixture();
        fixture.objects.extend(fill.appended_objects.iter().take(visible).cloned());
        fixture.attractions.extend(fill.appended_attractions.iter().take(visible).cloned());
        Some(fixture)
    }

    pub(crate) fn apply_brush_placement(&mut self, payload: &BrushPlacePayload) -> Option<Fixture> {
        let catalogs = self.scene.as_ref()?.kind_catalogs.as_ref()?.clone();
        let fixture = &self.scene.as_ref()?.fixture;
        let next = apply_brush_placement_to_fixture(fixture, payload, &catalogs);
        if next.objects.len() == fixture.objects.len() {
            return None;
        }
        if let Some(scene) = &mut self.scene {
            Arc::make_mut(scene).fixture = next.clone();
        }
        self.rebuild_queue();
        Some(next)
    }

    fn take_session(&mut self) -> Puzzle3dCollisionSession {
        Puzzle3dCollisionSession {
            scene: self.scene.take(),
            scene_synced: self.scene_synced.take(),
            meshes: std::mem::replace(&mut self.meshes, Arc::new(HashMap::new())),
            mesh_is_fallback: std::mem::take(&mut self.mesh_is_fallback),
            mesh_sources: std::mem::take(&mut self.mesh_sources),
            brush_cache: std::mem::take(&mut self.brush_cache),
            brush_queue: std::mem::take(&mut self.brush_queue),
            brush_prepare_object_cursor: self.brush_prepare_object_cursor,
            brush_prepare_vortex_cursor: self.brush_prepare_vortex_cursor,
            brush_queue_preparing: std::mem::take(&mut self.brush_queue_preparing),
            brush_index: std::mem::replace(&mut self.brush_index, CollisionSpatialIndex::new(BRUSH_INDEX_CELL_SIZE)),
            brush_index_owner: self.brush_index_owner,
            brush_index_sync: self.brush_index_sync.take(),
            brush_index_ready: std::mem::take(&mut self.brush_index_ready),
            brush_placed: std::mem::take(&mut self.brush_placed),
        }
    }

    fn install_session(&mut self, session: Puzzle3dCollisionSession) {
        self.scene = session.scene;
        self.scene_synced = session.scene_synced;
        self.meshes = session.meshes;
        self.mesh_is_fallback = session.mesh_is_fallback;
        self.mesh_sources = session.mesh_sources;
        self.brush_cache = session.brush_cache;
        self.brush_queue = session.brush_queue;
        self.brush_prepare_object_cursor = session.brush_prepare_object_cursor;
        self.brush_prepare_vortex_cursor = session.brush_prepare_vortex_cursor;
        self.brush_queue_preparing = session.brush_queue_preparing;
        self.brush_index = session.brush_index;
        self.brush_index_owner = session.brush_index_owner;
        self.brush_index_sync = session.brush_index_sync;
        self.brush_index_ready = session.brush_index_ready;
        self.brush_placed = session.brush_placed;
    }
}

/// 🧵️ The brush-lane half of one document instance's collision engine: the scene it was last synced
/// from, the registered mesh geometry, the suggestion cache and the persistent broad phase. This is
/// everything the app's session slot carries across a dispatch or a worker hop — the fill lane is
/// deliberately absent, because `fill_envelope_registry` is its one authority.
pub(crate) struct Puzzle3dCollisionSession {
    scene: Option<Arc<SceneConfig>>,
    scene_synced: Option<Arc<SceneConfig>>,
    meshes: Arc<HashMap<String, CollisionBody>>,
    mesh_is_fallback: HashMap<String, bool>,
    mesh_sources: HashMap<String, FillWorkerMesh>,
    brush_cache: HashMap<String, BrushCollisionFreeResult>,
    brush_queue: VecDeque<String>,
    brush_prepare_object_cursor: usize,
    brush_prepare_vortex_cursor: usize,
    brush_queue_preparing: bool,
    brush_index: CollisionSpatialIndex,
    brush_index_owner: CollisionIndexOwner,
    brush_index_sync: Option<BrushIndexSync>,
    brush_index_ready: bool,
    brush_placed: HashMap<String, PlacedCollisionEntry>,
}

impl Puzzle3dCollisionSession {
    /// 🧮️ Resident bytes this slot keeps alive, so the app's session registry can bound the whole
    /// process against a census instead of trusting a slot count.
    pub(crate) fn bytes(&self) -> usize {
        let meshes = self.mesh_sources.values().map(|mesh| mesh.url.len().saturating_add(mesh.positions.len().saturating_mul(4)).saturating_add(mesh.indices.len().saturating_mul(4))).fold(0_usize, usize::saturating_add);
        self.scene_synced
            .as_ref()
            .map_or(0, |scene| scene.fixture.objects.len().saturating_mul(size_of::<crate::standards::v1::subsets::any::schema::FixtureObject>()))
            .saturating_add(meshes)
            .saturating_add(self.brush_placed.len().saturating_mul(size_of::<PlacedCollisionEntry>()))
            .saturating_add(self.brush_index.entry_len().saturating_mul(size_of::<CollisionAabb>()))
    }
}
//#endregion 🔖️Engine

//#region 🔖️Session
pub struct Puzzle3dPrecomputeSession {
    engine: Puzzle3dCollision,
    fill_job: Option<FillJobRequest>,
    fill_admission: Option<FillEnvelopeAdmissionCursor>,
    fill_terminal: Option<FillEnvelopeTerminalHandle>,
    fill_observation: FillObservation,
    fill_applied_count: u32,
    last_emitted_fill_checkpoint: RefCell<Vec<u8>>,
}

/// 🪣️ One fixed semantic prefix transition for resumable fill materialization.
pub(crate) struct FillApplyChunk {
    pub(crate) applied_count: u32,
    pub(crate) added_objects: Vec<crate::standards::v1::subsets::any::schema::FixtureObject>,
    pub(crate) added_attractions: Vec<crate::standards::v1::subsets::any::schema::AttractionProps>,
    pub(crate) removed_object_ids: Vec<String>,
}

impl Default for Puzzle3dPrecomputeSession {
    fn default() -> Self {
        Self::new()
    }
}

impl Puzzle3dPrecomputeSession {
    pub fn new() -> Self {
        Self { engine: Puzzle3dCollision::new(), fill_job: None, fill_admission: None, fill_terminal: None, fill_observation: FillObservation::default(), fill_applied_count: 0, last_emitted_fill_checkpoint: RefCell::new(Vec::new()) }
    }

    fn read_fill<R>(&self, read: impl FnOnce(&FillBuilder) -> R) -> Option<R> {
        if let Some(request) = &self.fill_job {
            let registry = fill_envelope_registry().try_lock().ok()?;
            let authority = registry.slots.get(usize::from(request.slot))?.as_ref().filter(|authority| authority.request == *request)?;
            let fill = authority.fill.as_ref()?.try_lock().ok()?;
            return Some(read(&fill));
        }
        let fill = self.engine.fill.as_ref()?.try_lock().ok()?;
        Some(read(&fill))
    }

    fn write_fill<R>(&self, write: impl FnOnce(&mut FillBuilder) -> R) -> Option<R> {
        if let Some(request) = &self.fill_job {
            let registry = fill_envelope_registry().try_lock().ok()?;
            let authority = registry.slots.get(usize::from(request.slot))?.as_ref().filter(|authority| authority.request == *request)?;
            let mut fill = authority.fill.as_ref()?.try_lock().ok()?;
            return Some(write(&mut fill));
        }
        let mut fill = self.engine.fill.as_ref()?.try_lock().ok()?;
        Some(write(&mut fill))
    }

    fn supersede_admitted_fill(&mut self) {
        let Some(request) = &self.fill_job else { return };
        self.engine.fill_cancel.cancel_now();
        request_fill_envelope_terminal(request, FillEnvelopeTerminalReason::Closed);
        self.fill_admission = None;
    }

    pub fn set_scene(&mut self, json: &str) -> Result<(), Puzzle3dError> {
        self.install_scene(|engine| engine.set_scene(json))
    }

    /// 🧊️ Typed sync — see [`Puzzle3dCollision::set_scene_config`].
    pub(crate) fn set_scene_config(&mut self, scene: SceneConfig) -> Result<(), Puzzle3dError> {
        self.install_scene(|engine| {
            engine.set_scene_config(scene);
            Ok(())
        })
    }

    fn install_scene(&mut self, install: impl FnOnce(&mut Puzzle3dCollision) -> Result<(), Puzzle3dError>) -> Result<(), Puzzle3dError> {
        self.supersede_admitted_fill();
        let result = install(&mut self.engine);
        if result.is_ok() {
            if self.engine.fill.is_none() {
                self.engine.rebuild_queue();
            }
            self.last_emitted_fill_checkpoint.borrow_mut().clear();
        }
        result
    }

    pub fn register_mesh(&mut self, url: &str, positions: &[f32], indices: &[u32]) {
        self.supersede_admitted_fill();
        self.engine.register_mesh(url.to_string(), positions, indices);
    }

    pub fn register_mesh_fallback(&mut self, url: &str, positions: &[f32], indices: &[u32]) {
        self.supersede_admitted_fill();
        self.engine.register_mesh_fallback(url.to_string(), positions, indices);
    }

    /// 🧩️ Admits one page of a client's mesh upload run and installs the geometry the moment the run
    /// closes — the paged half of the `registerBrushMesh` wire contract, which is how a document-scale
    /// GLB reaches this session at all: one whole mesh never fitted in the shared retained command's
    /// 8 192 raw bytes. `Ok(None)` is a closed, digest-verified, installed mesh; `Ok(Some(next))` is an
    /// open run waiting for page `next`. See [`stage_brush_mesh_page`].
    pub fn stage_mesh_page(&mut self, url: &str, digest: &str, page: u32, page_count: u32, positions: &[f32], indices: &[u32]) -> Result<Option<u32>, Puzzle3dMeshUploadFault> {
        match stage_brush_mesh_page(url, digest, page, page_count, positions, indices)? {
            Puzzle3dMeshUploadStep::Staged { next_page, .. } => Ok(Some(next_page)),
            Puzzle3dMeshUploadStep::Complete(positions, indices) => {
                self.register_mesh(url, &positions, &indices);
                Ok(None)
            }
        }
    }

    /// 🧵️ Hands the brush lane out to the app's session slot, leaving this engine empty — the check-in
    /// half of the per-instance session. Never carries fill state.
    pub(crate) fn take_collision_session(&mut self) -> Puzzle3dCollisionSession {
        self.engine.take_session()
    }

    /// 🧵️ Adopts a brush lane a previous call (possibly on another worker) left in the session slot.
    pub(crate) fn install_collision_session(&mut self, session: Puzzle3dCollisionSession) {
        self.engine.install_session(session);
    }

    /// 🥽️ Installs real geometry for one mesh identity out of the process-wide derived-mesh store —
    /// the id-only wire path, so a mesh uploaded once serves every document instance.
    pub fn adopt_shared_mesh(&mut self, url: &str, digest: Option<&str>) -> bool {
        self.supersede_admitted_fill();
        self.engine.adopt_shared_mesh(url, digest)
    }

    pub fn has_mesh(&self, url: &str) -> bool {
        self.engine.has_mesh(url)
    }

    pub fn precompute_step(&mut self, budget: u32) -> bool {
        self.engine.precompute_step(budget)
    }

    pub fn precompute_step_lane(&mut self, lane: PrecomputeLane, budget: u32) -> bool {
        self.engine.precompute_step_lane(lane, budget)
    }

    pub fn enqueue_brush_target(&mut self, vortex_full_id: &str) {
        self.engine.enqueue_brush_target(vortex_full_id);
    }

    pub fn invalidate_brush_target(&mut self, vortex_full_id: &str) {
        self.engine.invalidate_brush_target(vortex_full_id);
    }

    pub fn refresh_brush_candidates(&mut self, vortex_full_id: &str) {
        self.engine.refresh_brush_candidates(vortex_full_id);
    }

    /// 🎯️ Typed readout — was a JSON string before the headless-engine-law fix; the app now reads
    /// `.free`/`.unknown_pending` directly.
    pub fn brush_candidates(&self, vortex_full_id: &str) -> BrushCollisionFreeResult {
        self.engine.brush_cache.get(vortex_full_id).cloned().unwrap_or(BrushCollisionFreeResult { free: vec![], unknown_pending: true, resume_candidate_index: 0 })
    }

    pub fn brush_preview(&self, vortex_full_id: &str, candidate_index: usize) -> Option<BrushPreviewState> {
        self.engine.brush_preview(vortex_full_id, candidate_index)
    }

    pub fn fill_progress(&self) -> FillBuildProgress {
        self.read_fill(FillBuilder::progress).map_or(FillBuildProgress { count: 0, applied_count: 0, max_count: FILL_COUNT_MAX, done: true, appended_objects: vec![], appended_attractions: vec![], sequence: vec![], preview: None }, |mut progress| {
            progress.applied_count = (self.fill_applied_count as usize).min(progress.count);
            progress
        })
    }

    pub fn fill_progress_summary(&self) -> FillProgressSummary {
        self.read_fill(|fill| FillProgressSummary { count: fill.sequence.len(), applied_count: (self.fill_applied_count as usize).min(fill.sequence.len()), max_count: fill.max_count, done: fill.stalled || fill.sequence.len() >= fill.max_count })
            .unwrap_or(FillProgressSummary { count: 0, applied_count: 0, max_count: FILL_COUNT_MAX, done: true })
    }

    /// 🔭️ Advances a fixed number of one-unit preview JSON grants and returns the retained last
    /// valid page while a newer generation is still being censused or encoded.
    pub fn fill_preview_json_page(&self, color: &str, status_label: &str) -> Option<String> {
        const GRANTS_PER_FRAME: usize = 256;
        let mut previous_us = default_now_us()?;
        let deadline = previous_us.checked_add(2_000)?;
        let cancelled = self.engine.fill_cancel.is_cancelled_now();
        self.write_fill(|fill| {
            if fill.preview.stage == "complete" {
                return None;
            }
            for _ in 0..GRANTS_PER_FRAME {
                let now_us = default_now_us()?;
                if now_us < previous_us {
                    return fill.preview_json_ready().map(ToOwned::to_owned);
                }
                previous_us = now_us;
                let mut fuel = 1;
                let step = fill.preview_json_step(color, status_label, &mut fuel, cancelled, now_us >= deadline);
                if matches!(step, FillPreviewJsonStep::Ready | FillPreviewJsonStep::Rejected | FillPreviewJsonStep::Cancelled | FillPreviewJsonStep::Terminal) {
                    break;
                }
            }
            fill.preview_json_ready().map(ToOwned::to_owned)
        })?
    }

    pub fn fill_preview_object_kind(&self) -> Option<String> {
        self.read_fill(|fill| fill.preview.candidate_ghost.as_ref().map(|ghost| ghost.object_kind_id.clone()))?
    }

    /// 🪣️ O(1) planned-count readout for the render/tick hot path — avoids a `fill_progress` round
    /// trip just to read `sequence.len()`.
    pub fn fill_available_count(&self) -> u32 {
        self.fill_job.as_ref().map_or_else(|| self.read_fill(|fill| fill.sequence.len() as u32).unwrap_or(0), |_| self.fill_observation.available)
    }

    /// 🪣️ Restores the small persisted cursor independently of the immutable fill-plan checkpoint.
    pub(crate) fn set_fill_applied_count(&mut self, count: u32) {
        self.fill_applied_count = count.min(self.fill_available_count());
    }

    /// 🧵️ Advances only one bounded plan-prefix delta and checkpoints the new applied cursor.
    pub(crate) fn apply_fill_count_chunk(&mut self, requested: u32, max_delta: usize) -> Option<FillApplyChunk> {
        let current = self.fill_applied_count;
        let chunk = self.read_fill(|fill| {
            let target = (requested as usize).min(fill.sequence.len());
            let current = (current as usize).min(fill.sequence.len());
            let next = if target > current { current.saturating_add(max_delta).min(target) } else { current.saturating_sub(max_delta).max(target) };
            let (added_objects, added_attractions, removed_object_ids) = if next > current {
                (fill.appended_objects[current..next].to_vec(), fill.appended_attractions[current..next].to_vec(), Vec::new())
            } else {
                (Vec::new(), Vec::new(), fill.appended_objects[next..current].iter().rev().map(|object| object.id.clone()).collect())
            };
            FillApplyChunk { applied_count: next as u32, added_objects, added_attractions, removed_object_ids }
        })?;
        self.fill_applied_count = chunk.applied_count;
        Some(chunk)
    }

    pub fn fill_is_done(&self) -> bool {
        self.fill_job.as_ref().map_or_else(|| self.read_fill(|fill| fill.stalled || fill.sequence.len() >= fill.max_count).unwrap_or(true), |_| self.fill_observation.done)
    }

    pub fn fill_checkpoint_bytes(&self) -> Vec<u8> {
        let token = self.fill_job.as_ref().and_then(|request| fill_envelope_registry().try_lock().ok()?.token(request));
        token.unwrap_or_else(|| self.last_emitted_fill_checkpoint.borrow().clone())
    }

    pub fn restore_persisted_fill(&mut self, checkpoint: &[u8]) -> bool {
        let Some(request) = decode_fill_envelope_request(checkpoint) else {
            return false;
        };
        let Ok(registry) = fill_envelope_registry().try_lock() else {
            return false;
        };
        let live = |candidate: &FillJobRequest| registry.slots.get(usize::from(candidate.slot)).and_then(Option::as_ref).is_some_and(|authority| authority.request == *candidate);
        if self.fill_job.as_ref().is_some_and(|current| *current != request && live(current)) || self.fill_terminal.as_ref().is_some_and(|terminal| terminal.request != request && live(&terminal.request)) {
            return false;
        }
        let Some(authority) = registry
            .slots
            .get(usize::from(request.slot))
            .and_then(Option::as_ref)
            .filter(|authority| authority.request == request && authority.token_page.as_ref().is_some_and(|page| page[..authority.token_len] == *checkpoint) && !authority.checked_out.load(Ordering::Acquire))
        else {
            return false;
        };
        let Some(cancel) = authority.cancel.clone() else {
            return false;
        };
        let steps_remaining = authority.steps_remaining;
        let preview_sequence = authority.preview_sequence;
        let observation = authority.observation;
        self.engine.fill = None;
        self.engine.fill_cancel = cancel;
        self.engine.fill_steps_remaining = steps_remaining;
        self.engine.fill_preview_sequence = preview_sequence;
        self.fill_job = Some(request);
        self.fill_admission = None;
        self.fill_observation = observation;
        *self.last_emitted_fill_checkpoint.borrow_mut() = checkpoint.to_vec();
        true
    }

    //#region 💼️FillJobBridge
    pub fn enqueue_fill_job(&mut self) -> Option<(u64, Vec<u8>)> {
        if self.fill_admission.is_none() {
            if let Some(request) = &self.fill_job {
                let Ok(registry) = fill_envelope_registry().try_lock() else {
                    return None;
                };
                if registry.observation(request).is_some() {
                    return None;
                }
                self.fill_job = None;
            }
        }
        if self.fill_admission.is_none() {
            if self.fill_job.is_none() && !self.engine.fill_lane_active() {
                self.engine.start_fill_preparation(true);
            }
            if !self.engine.fill_lane_active() {
                return None;
            }
            let fill = self.engine.fill.take()?;
            let observed = {
                match fill.try_lock() {
                    Ok(fill) => Some((fill.operation, FillObservation { generation: fill.preview.generation, sequence: fill.preview.sequence, available: fill.sequence.len() as u32, done: fill.stalled || fill.sequence.len() >= fill.max_count })),
                    Err(_) => None,
                }
            };
            let Some((operation, observation)) = observed else {
                self.engine.fill = Some(fill);
                return None;
            };
            let job = semio_framework_job::allocate_operation_id().0;
            let Ok(mut registry) = fill_envelope_registry().try_lock() else {
                self.engine.fill = Some(fill);
                return None;
            };
            if self.engine.fill_worker_outcome.is_some() || self.engine.fill_rejected_worker.is_some() {
                self.engine.fill = Some(fill);
                return None;
            }
            let Some(worker) = self.engine.fill_worker.take() else {
                self.engine.fill = Some(fill);
                return None;
            };
            let request = match registry.begin_measurement(FillEnvelopeMeasurementRequest { job, operation, fill, worker, cancel: self.engine.fill_cancel.clone(), steps_remaining: self.engine.fill_steps_remaining, preview_sequence: self.engine.fill_preview_sequence, observation }) {
                Ok(request) => request,
                Err(owners) => {
                    self.engine.fill_worker = Some(owners.worker);
                    self.engine.fill = Some(owners.fill);
                    return None;
                }
            };
            self.engine.fill_worker_terminal = false;
            if let Some(fill) = registry.authority_mut(&request).and_then(|authority| authority.fill.as_ref()) {
                if let Ok(mut fill) = fill.try_lock() {
                    fill.preview.registry_generation = request.registry_generation;
                }
            }
            self.fill_job = Some(request.clone());
            self.fill_admission = Some(FillEnvelopeAdmissionCursor { request, census: FillBuilderOwnerCensusCursor::default() });
        }
        let census = {
            let admission = self.fill_admission.as_mut()?;
            let Ok(registry) = fill_envelope_registry().try_lock() else {
                return None;
            };
            let fill = registry.slots.get(usize::from(admission.request.slot))?.as_ref().filter(|authority| authority.request == admission.request)?.fill.as_ref()?;
            let Ok(fill) = fill.try_lock() else { return None };
            admission.census.step(&fill, FILL_ENVELOPE_MAX_ITEMS, FILL_ENVELOPE_MAX_BYTES)
        };
        let credit = match census {
            FillBuilderOwnerCensusStep::Pending => return None,
            FillBuilderOwnerCensusStep::Rejected => {
                let admission = self.fill_admission.take()?;
                terminalize_fill_envelope(&admission.request, FillEnvelopeTerminalReason::Fault);
                return None;
            }
            FillBuilderOwnerCensusStep::Complete(credit) => credit,
        };
        let admission = self.fill_admission.take()?;
        let Ok(mut registry) = fill_envelope_registry().try_lock() else {
            self.fill_admission = Some(admission);
            return None;
        };
        let token = registry.finish_measurement(&admission.request, credit.items, credit.bytes)?;
        self.fill_observation = registry.observation(&admission.request)?;
        Some((admission.request.job, token))
    }

    pub fn poll_fill_job(&mut self) -> bool {
        if self.pump_fill_terminal_step() {
            return true;
        }
        let Some(request) = &self.fill_job else {
            return false;
        };
        let Ok(registry) = fill_envelope_registry().try_lock() else {
            return false;
        };
        let Some(current) = registry.observation(request) else {
            return false;
        };
        let changed = current != self.fill_observation;
        self.fill_observation = current;
        changed
    }

    fn pump_fill_terminal_step(&mut self) -> bool {
        if self.fill_terminal.is_none() {
            if let Some(terminal) = self.take_terminal_fill_job() {
                self.fill_terminal = Some(terminal);
                return true;
            }
            let Ok(mut registry) = fill_envelope_registry().try_lock() else {
                return false;
            };
            let Some(terminal) = registry.take_closed() else {
                return false;
            };
            self.fill_terminal = Some(terminal);
            return true;
        }
        let closes_current = self.fill_terminal.as_ref().is_some_and(|terminal| self.fill_job.as_ref().is_some_and(|request| *request == terminal.request));
        let outcome = self.fill_terminal.as_mut().map(FillEnvelopeTerminalHandle::close_step);
        if matches!(outcome, Some(FillEnvelopeCloseStep::Complete | FillEnvelopeCloseStep::Stale)) {
            self.fill_terminal.take();
            if closes_current {
                self.fill_job = None;
            }
        }
        true
    }

    pub fn take_terminal_fill_job(&mut self) -> Option<FillEnvelopeTerminalHandle> {
        let request = self.fill_job.clone()?;
        let Ok(mut registry) = fill_envelope_registry().try_lock() else {
            return None;
        };
        let authority = registry.authority_mut(&request)?;
        apply_fill_envelope_terminal_intent(authority);
        if !matches!(authority.phase, FillEnvelopePhase::Terminal(_) | FillEnvelopePhase::Closing) || authority.checked_out.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return None;
        }
        Some(FillEnvelopeTerminalHandle { request, checked_out: authority.checked_out.clone(), returned: false })
    }

    /// 🧾️ Identity of the fill job this session currently drives — `(job, operation, generation)`. The
    /// fill tool publishes it into its cancel action's args so a cancel can never reach a superseded run.
    pub fn fill_job_identity(&self) -> Option<(u64, u64, u64)> {
        self.fill_job.as_ref().map(|request| (request.job, request.operation, request.generation))
    }

    /// 🛑 Cancels the live fill job only when the caller names it exactly; a stale identity is a no-op,
    /// mirroring `🔋️energy`'s own `request_identity_args` guard on `cancel-energy-simulation`.
    pub fn cancel_fill_job_for(&mut self, job: u64, operation: u64, generation: u64) -> bool {
        if self.fill_job_identity() != Some((job, operation, generation)) {
            return false;
        }
        self.cancel_fill_job()
    }

    pub fn cancel_fill_job(&mut self) -> bool {
        let Some(request) = &self.fill_job else {
            return false;
        };
        let Ok(mut registry) = fill_envelope_registry().try_lock() else {
            return false;
        };
        let Some(authority) = registry.authority_mut(request) else {
            return false;
        };
        let Some(cancel) = &authority.cancel else {
            return false;
        };
        cancel.cancel_now();
        true
    }

    #[cfg(test)]
    fn drive_fill_job(&self, request: &FillJobRequest) -> Option<FillJobSlice> {
        match drive_fill_envelope(request) {
            FillEnvelopeDrive::Advanced(slice) => Some(slice),
            FillEnvelopeDrive::Blocked => Some(FillJobSlice { progress: None, done: false }),
            FillEnvelopeDrive::Stale => None,
        }
    }

    #[cfg(test)]
    pub(crate) fn drive_enqueued_fill_job_for_test(&mut self, slices: usize) {
        for _ in 0..slices {
            let Some(request) = self.fill_job.clone() else { break };
            if self.drive_fill_job(&request).is_none() {
                break;
            }
        }
    }

    //#endregion 💼️FillJobBridge

    fn compose_fill_projection(&self, count: usize, persisted: bool) -> Option<Fixture> {
        self.read_fill(|fill| {
            let visible = count.min(fill.sequence.len());
            let mut fixture = fill.base_fixture();
            fixture.objects.extend(fill.appended_objects.iter().take(visible).cloned().map(|mut object| {
                if persisted {
                    object.reveal_index = None;
                }
                object
            }));
            fixture.attractions.extend(fill.appended_attractions.iter().take(visible).cloned());
            fixture
        })
    }

    fn apply_fill_count_projection(&mut self, count: usize) -> Option<Fixture> {
        let visible = count.min(self.fill_available_count() as usize);
        let fixture = self.compose_fill_projection(visible, true)?;
        self.fill_applied_count = visible as u32;
        Some(fixture)
    }

    /// 🪣️ Read-only prefix of the precomputed fill plan for live viewport show/hide — a query, so it
    /// stays a plain `&self` method rather than routing through `dispatch` (which is `&mut self`,
    /// uniform for the small number of genuinely mutating actions). `Puzzle3dEngineCommand::
    /// ComposeFillDisplay` still exists as a `dispatch`-able alias of this same call for command-log/
    /// wasm-bindgen-wrapper callers that only ever hold `&mut Puzzle3dPrecomputeSession`.
    pub fn compose_fill_display(&self, count: u32) -> Option<Fixture> {
        self.compose_fill_projection(count as usize, false)
    }

    /// 🎯️ Single typed entry point for every mutating (or JSON-carrying-before-this-fix) engine
    /// action — the headless replacement for the old per-action `apply_brush_placement_json`/
    /// `apply_fill_count`/`compose_fill_display`/`update_kind_weights`/`brush_preview_json`
    /// wasm-bindgen methods. Each arm calls the SAME underlying typed `Puzzle3dCollision` method those
    /// JSON wrappers always delegated to — no reimplementation.
    pub fn dispatch(&mut self, command: Puzzle3dEngineCommand) -> Result<Puzzle3dEngineOutcome, Puzzle3dError> {
        match command {
            Puzzle3dEngineCommand::SetScene { scene } => {
                self.set_scene_config(scene)?;
                Ok(Puzzle3dEngineOutcome::Unit)
            }
            Puzzle3dEngineCommand::ApplyBrushPlacement { payload } => {
                self.supersede_admitted_fill();
                let fixture = self.engine.apply_brush_placement(&payload).ok_or(Puzzle3dError::BrushPlacementRejected)?;
                Ok(Puzzle3dEngineOutcome::Fixture(fixture))
            }
            Puzzle3dEngineCommand::ApplyFillCount { count } => {
                let fixture = self.apply_fill_count_projection(count as usize).ok_or(Puzzle3dError::FillSessionUnavailable)?;
                Ok(Puzzle3dEngineOutcome::Fixture(fixture))
            }
            Puzzle3dEngineCommand::ComposeFillDisplay { count } => {
                let fixture = self.compose_fill_projection(count as usize, false).ok_or(Puzzle3dError::FillSessionUnavailable)?;
                Ok(Puzzle3dEngineOutcome::Fixture(fixture))
            }
            Puzzle3dEngineCommand::UpdateKindWeights { object_weights, vortex_weights } => {
                self.supersede_admitted_fill();
                self.engine.update_kind_weights(object_weights, vortex_weights);
                Ok(Puzzle3dEngineOutcome::Unit)
            }
            Puzzle3dEngineCommand::BrushPreview { vortex_full_id, candidate_index } => Ok(Puzzle3dEngineOutcome::BrushPreview(self.engine.brush_preview(&vortex_full_id, candidate_index as usize))),
        }
    }
}
//#endregion 🔖️Session

/// 📏️ The turns one session's Drop may spend returning the process-global fill envelope slots. Derived,
/// not chosen: `FILL_ENVELOPE_MAX_OPERATIONS` envelopes, each at most `FILL_ENVELOPE_MAX_ITEMS` retained
/// owners — the very census ceiling `finish_measurement` admitted them against — plus the eight
/// `FillEnvelopeTerminalHandle::close_step` cursor stages and its checkout turn.
const FILL_ENVELOPE_SESSION_CLOSE_TURNS: usize = FILL_ENVELOPE_MAX_OPERATIONS * (FILL_ENVELOPE_MAX_ITEMS + 9);

impl Drop for Puzzle3dPrecomputeSession {
    /// ♻️ Returns this session's fill envelope slot to the process-global registry. Requesting the
    /// `Closed` terminal is NOT enough: only `FillEnvelopeTerminalHandle::close_step` takes the slot
    /// back, so a session that merely asked and died left one of the four slots occupied forever —
    /// after four such sessions `begin_measurement` finds no candidate and every later
    /// `enqueue_fill_job` returns `None`, which is how a fill test that passes alone fails in a shared
    /// process. `pump_fill_terminal_step` also collects envelopes earlier sessions abandoned, so this
    /// drains until the registry is quiet rather than only until this session's own slot is back.
    fn drop(&mut self) {
        self.engine.fill_cancel.cancel_now();
        if let Some(request) = &self.fill_job {
            terminalize_fill_envelope(request, FillEnvelopeTerminalReason::Closed);
        }
        for _ in 0..FILL_ENVELOPE_SESSION_CLOSE_TURNS {
            if !self.pump_fill_terminal_step() {
                break;
            }
        }
    }
}

//#region 💼️SharedPluginJob
pub fn fill_job(context: semio_framework_plugin::reactor::jobs::JobCtx, input: Vec<u8>, restored: Option<Vec<u8>>) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, semio_framework::Fault>>>> {
    Box::pin(async move {
        let context_job = context.id().await;
        let mut admitted_cursor = FillEnvelopeJobEntryCursor::new(context_job, input);
        let admitted_request = loop {
            context.tick().await;
            match admitted_cursor.step() {
                Ok(Some(request)) => break request,
                Ok(None) => {}
                Err(error) => return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("puzzle3d.fill-job.decode"), error)),
            }
        };
        if let Err(error) = admitted_cursor.bind(&admitted_request) {
            return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("puzzle3d.fill-job.identity"), error));
        }
        let mut terminal_guard = admitted_cursor.into_guard();
        let request = if let Some(checkpoint) = restored {
            let mut restored_cursor = FillEnvelopeTokenCursor::new(checkpoint);
            loop {
                context.tick().await;
                match restored_cursor.step() {
                    Ok(Some(request)) => break request,
                    Ok(None) => {}
                    Err(error) => return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("puzzle3d.fill-job.restore"), error)),
                }
            }
        } else {
            admitted_request.clone()
        };
        if request != admitted_request {
            return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("puzzle3d.fill-job.stale"), "restored fill worker operation does not match admitted operation"));
        }
        loop {
            context.tick().await;
            match drive_fill_envelope(&request) {
                FillEnvelopeDrive::Blocked => continue,
                FillEnvelopeDrive::Stale => {
                    return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("puzzle3d.fill-job.stale"), "fill job no longer matches the live operation"));
                }
                FillEnvelopeDrive::Advanced(slice) => {
                    let token = fill_envelope_registry()
                        .try_lock()
                        .ok()
                        .and_then(|registry| registry.token(&request))
                        .ok_or_else(|| semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("puzzle3d.fill-job.owner"), "fill job terminal owner is unavailable"))?;
                    if slice.progress.is_some() {
                        context.progress(token.clone()).await;
                    }
                    if slice.done {
                        terminal_guard.disarm();
                        return Ok(token);
                    }
                    context.checkpoint(token).await;
                }
            }
        }
    })
}
//#endregion 💼️SharedPluginJob

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

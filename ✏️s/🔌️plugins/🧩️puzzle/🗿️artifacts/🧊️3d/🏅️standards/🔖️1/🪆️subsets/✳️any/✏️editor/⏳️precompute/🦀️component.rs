//! ⏳️ Puzzle 3d play app — the precompute session: the scene the host syncs in, the registered
//! collision meshes, the two independent background lanes (brush-candidate caching and fill
//! planning), and `dispatch`, which drives `Puzzle3dEngineCommand`/`Puzzle3dEngineOutcome` (schema
//! types, `crate::artifacts::puzzle3d::schema`) through the session. The rules the lanes consult live
//! in `🖌️brush/🦀️component.rs`, the geometry in `📐️geometry/🦀️component.rs`, the fill plan's own state
//! in `🪣️fill/🦀️component.rs`. Rehomed from the former `⚙️engine/⏳️session` (ticket
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

use crate::artifacts::puzzle3d::schema::{
    puzzle3d_vortex_full_id, BrushCollisionFreeResult, BrushCompatibleCandidate, BrushPlacePayload, BrushPreviewState, FillBuildProgress, FillProgressSummary, Fixture, KindCatalogBundle, PrecomputeLane, Puzzle3dEngineCommand, Puzzle3dEngineOutcome,
    SceneConfig,
};
use crate::artifacts::puzzle3d::Puzzle3dError;
use crate::editor::puzzle3d::precompute::brush::{
    brush_candidate_suggestion_weight, brush_compatible_candidates, brush_preview_from_candidate, brush_target_vortex_allows_suggestion, resolve_object_kind_mesh_url, vortex_world_from_object, AttractionVortexContext, TargetVortexWorld,
};
use crate::editor::puzzle3d::precompute::fill::{FillBuilder, FillBuilderOwnerCensusCursor, FillBuilderOwnerCensusStep, FillBuilderRetirementCursor, FillJobStage, FillPreparationRoots, FillPreviewJsonStep, PlacedCollisionEntry};
use crate::editor::puzzle3d::precompute::geometry::{pose_isometry, world_bounds, CollisionBody, CollisionOverlapState, CollisionStepContext, CollisionStepResult, FIXED_OWNER_SLOTS};
use semio_framework_job::{default_now_us, root_cancel_token, CancelToken, Generation, InteractiveJob, InteractiveJobCloseStep, InteractiveStage, Operation, RevisionId, StepOutcome};
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

//#region 💼️FillJobBridge
pub(crate) const FILL_JOB_KIND: &str = "semio.puzzle3d.fill";
const FILL_ENVELOPE_PAGE_BYTES: usize = 16 * 1024;
const FILL_ENVELOPE_MAX_PAGES: usize = 256;
const FILL_ENVELOPE_MAX_BYTES: usize = FILL_ENVELOPE_PAGE_BYTES * FILL_ENVELOPE_MAX_PAGES;
const FILL_ENVELOPE_MAX_ITEMS: usize = 65_536;
const FILL_ENVELOPE_MAX_OPERATIONS: usize = 4;
const FILL_ENVELOPE_PROCESS_BYTES: usize = FILL_ENVELOPE_MAX_BYTES * FILL_ENVELOPE_MAX_OPERATIONS;
const FILL_ENVELOPE_TOKEN_BYTES: usize = 56;
const FILL_ENVELOPE_AUTHORITY_ITEMS: usize = 2;
const FILL_ENVELOPE_AUTHORITY_BYTES: usize = FILL_ENVELOPE_PAGE_BYTES + FILL_ENVELOPE_TOKEN_BYTES;
const FILL_ENVELOPE_MAGIC: [u8; 8] = *b"P3FILL04";
const FILL_WORKER_MAX_MESHES: usize = 64;
const FILL_WORKER_MAX_MESH_VALUES: usize = 196_608;
const FILL_WORKER_MAX_URL_BYTES: usize = 4 * 1024;

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct FillWorkerMesh {
    url: String,
    positions: Vec<f32>,
    indices: Vec<u32>,
    fallback: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
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

type MountedFillWorker = semio_framework_job::MountedWorkerJobSession<SharedFillWorkerJob>;
type RejectedFillWorker = semio_framework_job::WorkerJobSessionAdmissionRejected<SharedFillWorkerJob>;

fn mount_fill_worker(fill: SharedFillBuilder, operation: Operation, cancel: CancelToken) -> Result<MountedFillWorker, RejectedFillWorker> {
    semio_framework_job::MountedWorkerJobSession::try_new(
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

fn fill_worker_pool() -> semio_framework_async::WorkerPool {
    let workers = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, workers))
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
    fn begin_measurement(
        &mut self,
        job: u64,
        operation: Operation,
        fill: SharedFillBuilder,
        worker: MountedFillWorker,
        cancel: CancelToken,
        steps_remaining: usize,
        preview_sequence: u64,
        observation: FillObservation,
    ) -> Result<FillJobRequest, FillEnvelopeMeasurementOwners> {
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
            Err(mut rejected) => {
                rejected.begin_close();
                let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                return Err(fill);
            }
        };
        let request = self.begin_measurement(job, operation, fill, worker, cancel, steps_remaining, preview_sequence, observation).map_err(|owners| owners.fill)?;
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
        match worker.pump_one(&fill_worker_pool(), semio_framework_async::Lane::Background) {
            Ok(semio_framework_job::WorkerJobPoll::Outcome | semio_framework_job::WorkerJobPoll::Terminal) => {
                let Some(outcome) = worker.take_checked_out_outcome() else {
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
            Ok(
                semio_framework_job::WorkerJobPoll::Idle
                | semio_framework_job::WorkerJobPoll::Submitted
                | semio_framework_job::WorkerJobPoll::Rejected
                | semio_framework_job::WorkerJobPoll::CheckedOut
                | semio_framework_job::WorkerJobPoll::Closing
                | semio_framework_job::WorkerJobPoll::TerminalEmpty,
            ) => {}
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
    semio_framework_job::default_now_us()?.checked_add(duration_us)
}

/// 🪫️ Admission deadline for one precompute turn, including its first task.
const PUZZLE3D_PRECOMPUTE_STEP_BUDGET_US: u64 = 2_000;
//#endregion 🔖️Clock

//#region 🔖️Engine
pub(crate) struct Puzzle3dCollision {
    pub(crate) scene: Option<Arc<SceneConfig>>,
    /// 🧊️ Raw JSON of the last `set_scene` call, so a resync with byte-identical config (every action
    /// re-syncs the session, see the app's `sync_precompute_session`) can skip `rebuild_queue` instead
    /// of wiping `brush_cache`/`fill`/`queue` and restarting suggestion+fill precompute from zero.
    scene_json: Option<String>,
    meshes: Arc<HashMap<String, CollisionBody>>,
    mesh_is_fallback: HashMap<String, bool>,
    mesh_sources: HashMap<String, FillWorkerMesh>,
    pub(crate) brush_cache: HashMap<String, BrushCollisionFreeResult>,
    pub(crate) brush_queue: VecDeque<String>,
    brush_prepare_object_cursor: usize,
    brush_prepare_vortex_cursor: usize,
    brush_queue_preparing: bool,
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

impl Puzzle3dCollision {
    pub(crate) fn new() -> Self {
        Self {
            scene: None,
            scene_json: None,
            meshes: Arc::new(HashMap::new()),
            mesh_is_fallback: HashMap::new(),
            mesh_sources: HashMap::new(),
            brush_cache: HashMap::new(),
            brush_queue: VecDeque::new(),
            brush_prepare_object_cursor: 0,
            brush_prepare_vortex_cursor: 0,
            brush_queue_preparing: false,
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
                    self.fill_rejected_worker = Some(rejected);
                    self.fill_steps_remaining = 0;
                }
            }
            self.fill_worker_outcome = None;
            self.fill_worker_terminal = false;
        } else {
            self.fill = None;
            self.fill_worker = None;
            self.fill_rejected_worker = None;
            self.fill_worker_outcome = None;
            self.fill_worker_terminal = false;
        }
    }

    /// 🎚️ Distribution-weight edits must not `rebuild_queue()` — applied fill objects stay, only the
    /// unapplied planning tail is discarded and re-enqueued for background `fillBuildTick` planning.
    fn soft_replan_fill_tail(&mut self) {
        self.start_fill_preparation(false);
    }

    fn refresh_fill_job(&mut self, _refresh_meshes: bool) {
        self.start_fill_preparation(false);
    }

    pub(crate) fn update_kind_weights(&mut self, object_weights: std::collections::BTreeMap<String, f64>, vortex_weights: std::collections::BTreeMap<String, f64>) {
        if let Some(scene) = &mut self.scene {
            let scene = Arc::make_mut(scene);
            scene.weights.object_weights = object_weights;
            scene.weights.vortex_weights = vortex_weights;
            if let Ok(normalized) = serde_json::to_string(&*scene) {
                self.scene_json = Some(normalized);
            }
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
        let mut scene: SceneConfig = serde_json::from_str(json)?;
        // 🪣️ After the fill slider materializes objects into the document, every incidental action
        // (hover, pick, mesh register sync, …) re-feeds that applied projection here. Treating it as a
        // brand-new scene used to `rebuild_queue()` and bake the filled objects into `fill.base`, after
        // which the slider could neither remove them nor replan a fresh tail.
        let applied_projection = self.fill.as_ref().and_then(|fill| fill.try_lock().ok()).is_some_and(|fill| Self::is_fill_applied_projection(&scene.fixture, &fill));
        if applied_projection {
            if let Some(fill) = self.fill.as_ref().and_then(|fill| fill.try_lock().ok()) {
                Self::strip_fill_plan_from_fixture(&mut scene.fixture, &fill);
            }
            let normalized = serde_json::to_string(&scene)?;
            if let Some(current) = &mut self.scene {
                let current = Arc::make_mut(current);
                current.overlap_budget = scene.overlap_budget;
                current.seed = scene.seed;
                current.weights = scene.weights;
                current.kind_catalogs = scene.kind_catalogs;
                current.kind_compatibility = scene.kind_compatibility;
                current.host_rules = scene.host_rules;
            }
            self.scene_json = Some(normalized);
            return Ok(());
        }
        let normalized = serde_json::to_string(&scene)?;
        if self.scene_json.as_deref() == Some(normalized.as_str()) {
            return Ok(());
        }
        self.scene = Some(Arc::new(scene));
        self.scene_json = Some(normalized);
        self.rebuild_queue();
        Ok(())
    }

    fn install_collision_mesh(&mut self, url: String, positions: &[f32], indices: &[u32], is_fallback: bool) {
        if url.len() > FILL_WORKER_MAX_URL_BYTES || positions.len() > FILL_WORKER_MAX_MESH_VALUES || indices.len() > FILL_WORKER_MAX_MESH_VALUES {
            return;
        }
        if !self.mesh_sources.contains_key(&url) && self.mesh_sources.len() >= FILL_WORKER_MAX_MESHES {
            return;
        }
        let Some(body) = crate::editor::puzzle3d::precompute::geometry::collision_body_from_buffers(positions, indices) else {
            return;
        };
        if !is_fallback && self.mesh_is_fallback.get(&url) == Some(&false) {
            return;
        }
        if is_fallback && self.mesh_is_fallback.get(&url) == Some(&false) {
            return;
        }
        Arc::make_mut(&mut self.meshes).insert(url.clone(), body);
        self.mesh_is_fallback.insert(url.clone(), is_fallback);
        self.mesh_sources.insert(url.clone(), FillWorkerMesh { url, positions: positions.to_vec(), indices: indices.to_vec(), fallback: is_fallback });
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

    pub(crate) fn register_mesh(&mut self, url: String, positions: &[f32], indices: &[u32]) {
        self.install_collision_mesh(url, positions, indices, false);
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
        let Some(deadline) = puzzle3d_deadline(PUZZLE3D_PRECOMPUTE_STEP_BUDGET_US) else { return; };
        let prior = self.brush_cache.get(vortex_full_id).cloned();
        let resume_from = prior.as_ref().map_or(0, |entry| entry.resume_candidate_index);
        let prior_free = prior.map(|entry| entry.free).unwrap_or_default();
        let result = self.compute_brush_cache_entry_partial(vortex_full_id, resume_from, prior_free, deadline);
        if result.unknown_pending && result.resume_candidate_index > 0 && !self.brush_queue.iter().any(|id| id == vortex_full_id) {
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
                semio_framework_job::default_now_us().is_none_or(|now| now >= self.deadline_us)
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

    fn brush_collision_free_until(&self, target_full_id: &str, candidates: &[BrushCompatibleCandidate], overlap_budget: f64, resume_from: usize, mut free: Vec<BrushCompatibleCandidate>, deadline_us: u64) -> BrushCollisionFreeResult {
        let Some(scene) = &self.scene else {
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
        let placed: Vec<PlacedCollisionEntry> = scene
            .fixture
            .objects
            .iter()
            .filter(|obj| obj.id != host_id)
            .filter_map(|obj| {
                let mesh_url = resolve_object_kind_mesh_url(obj.object_kind.as_deref().unwrap_or(""), catalogs, &scene.fixture)?;
                if !self.meshes.contains_key(&mesh_url) {
                    return None;
                }
                Some(PlacedCollisionEntry { object_id: obj.id.clone(), mesh_url, world: pose_isometry(obj.origin, obj.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &obj.scale) })
            })
            .collect();
        let mut unknown_pending = false;
        for (index, candidate) in candidates.iter().enumerate().skip(resume_from) {
            if semio_framework_job::default_now_us().is_none_or(|now| now >= deadline_us) {
                return BrushCollisionFreeResult { free, unknown_pending: true, resume_candidate_index: index };
            }
            let world = TargetVortexWorld { position, direction, reference_orientation: host.orientation };
            let Some(preview) = brush_preview_from_candidate(target_full_id, candidate, &target_ctx, world, catalogs, &scene.fixture) else {
                continue;
            };
            if !self.meshes.contains_key(&preview.mesh_url) {
                unknown_pending = true;
                continue;
            }
            match Self::preview_collides(&self.meshes, &preview, &placed, overlap_budget, 1024, deadline_us) {
                None => unknown_pending = true,
                Some(true) => {}
                Some(false) => free.push(candidate.clone()),
            }
        }
        BrushCollisionFreeResult { free, unknown_pending, resume_candidate_index: 0 }
    }

    fn brush_collision_free(&self, target_full_id: &str, candidates: &[BrushCompatibleCandidate], overlap_budget: f64) -> BrushCollisionFreeResult {
        let Some(deadline) = puzzle3d_deadline(PUZZLE3D_PRECOMPUTE_STEP_BUDGET_US * 8) else { return BrushCollisionFreeResult { free: Vec::new(), unknown_pending: true, resume_candidate_index: 0 }; };
        self.brush_collision_free_until(target_full_id, candidates, overlap_budget, 0, Vec::new(), deadline)
    }

    fn compute_brush_cache_entry(&self, target_full_id: &str) -> BrushCollisionFreeResult {
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
        let Some(deadline) = puzzle3d_deadline(PUZZLE3D_PRECOMPUTE_STEP_BUDGET_US) else { return match lane { PrecomputeLane::Brush => self.brush_lane_active(), PrecomputeLane::Fill => self.fill_lane_active() }; };
        let mut remaining = budget as usize;
        while remaining > 0 {
            if semio_framework_job::default_now_us().is_none_or(|now| now >= deadline) {
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
                    let needs_resume = result.unknown_pending && result.resume_candidate_index > 0;
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
                            if self.fill_worker_terminal {
                                self.fill_steps_remaining = 0;
                            } else if self.fill_worker.as_mut().is_none_or(|worker| worker.resume().is_err()) {
                                self.fill_steps_remaining = 0;
                            }
                        }
                    } else if let Some(worker) = self.fill_worker.as_mut() {
                        match worker.pump_one(&fill_worker_pool(), semio_framework_async::Lane::Background) {
                            Ok(semio_framework_job::WorkerJobPoll::Outcome | semio_framework_job::WorkerJobPoll::Terminal) => {
                                let Some(outcome) = worker.take_checked_out_outcome() else {
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
                            Ok(semio_framework_job::WorkerJobPoll::Idle | semio_framework_job::WorkerJobPoll::Submitted | semio_framework_job::WorkerJobPoll::Rejected) => {}
                            Ok(semio_framework_job::WorkerJobPoll::CheckedOut | semio_framework_job::WorkerJobPoll::Closing | semio_framework_job::WorkerJobPoll::TerminalEmpty) | Err(_) => self.fill_steps_remaining = 0,
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

    fn compute_brush_cache_entry_partial(&self, target_full_id: &str, resume_from: usize, prior_free: Vec<BrushCompatibleCandidate>, deadline_us: u64) -> BrushCollisionFreeResult {
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
        self.brush_collision_free_until(target_full_id, &compatible, scene.overlap_budget, resume_from, prior_free, deadline_us)
    }

    pub(crate) fn precompute_step(&mut self, budget: u32) -> bool {
        let half = (budget / 2).max(1);
        let fill = self.precompute_step_lane(PrecomputeLane::Fill, half);
        let brush = self.precompute_step_lane(PrecomputeLane::Brush, budget.saturating_sub(half));
        fill || brush || self.fill_lane_active() || self.brush_lane_active()
    }

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

    #[cfg(test)]
    pub(crate) fn fill_steps_pending_for_test(&self) -> usize {
        self.fill_steps_remaining
    }

    /// 🔽️ Moving the count down (or up) only changes which prefix of the already-planned sequence is
    /// applied to the document — the plan (`sequence`/`appended_*`/`placed`/`fixture`) is prefix-stable
    /// and is never discarded here, so a jittery drag can never force expensive replanning.
    pub(crate) fn apply_fill_count(&mut self, count: usize) -> Option<Fixture> {
        let mut fill = self.fill.as_ref()?.try_lock().ok()?;
        let count = count.min(fill.sequence.len());
        fill.applied_count = count;
        let mut fixture = fill.base.snapshot();
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
    pub(crate) fn compose_fill_display(&self, count: usize) -> Option<Fixture> {
        let fill = self.fill.as_ref()?.try_lock().ok()?;
        let visible = count.min(fill.sequence.len());
        let mut fixture = fill.base.snapshot();
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
    pub(crate) added_objects: Vec<crate::artifacts::puzzle3d::schema::FixtureObject>,
    pub(crate) added_attractions: Vec<crate::artifacts::puzzle3d::schema::AttractionProps>,
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
        self.supersede_admitted_fill();
        let result = self.engine.set_scene(json);
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
        self.read_fill(FillBuilder::progress)
            .map(|mut progress| {
                progress.applied_count = (self.fill_applied_count as usize).min(progress.count);
                progress
            })
            .unwrap_or(FillBuildProgress { count: 0, applied_count: 0, max_count: FILL_COUNT_MAX, done: true, appended_objects: vec![], appended_attractions: vec![], sequence: vec![], preview: None })
    }

    pub fn fill_progress_summary(&self) -> FillProgressSummary {
        self.read_fill(|fill| FillProgressSummary { count: fill.sequence.len(), applied_count: (self.fill_applied_count as usize).min(fill.sequence.len()), max_count: fill.max_count, done: fill.stalled || fill.sequence.len() >= fill.max_count })
            .unwrap_or(FillProgressSummary { count: 0, applied_count: 0, max_count: FILL_COUNT_MAX, done: true })
    }

    /// 🔭️ Advances a fixed number of one-unit preview JSON grants and returns the retained last
    /// valid page while a newer generation is still being censused or encoded.
    pub fn fill_preview_json_page(&self, color: &str, status_label: &str) -> Option<String> {
        const GRANTS_PER_FRAME: usize = 256;
        let deadline = default_now_us()?.checked_add(2_000)?;
        let cancelled = self.engine.fill_cancel.is_cancelled_now();
        self.write_fill(|fill| {
            if fill.preview.stage == "complete" {
                return None;
            }
            for _ in 0..GRANTS_PER_FRAME {
                let mut fuel = 1;
                let step = fill.preview_json_step(color, status_label, &mut fuel, cancelled, default_now_us().is_none_or(|now_us| now_us >= deadline));
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
            let request = match registry.begin_measurement(job, operation, fill, worker, self.engine.fill_cancel.clone(), self.engine.fill_steps_remaining, self.engine.fill_preview_sequence, observation) {
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
            let mut fixture = fill.base.snapshot();
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
                let json = serde_json::to_string(&scene)?;
                self.set_scene(&json)?;
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

impl Drop for Puzzle3dPrecomputeSession {
    fn drop(&mut self) {
        self.engine.fill_cancel.cancel_now();
        if let Some(request) = &self.fill_job {
            terminalize_fill_envelope(request, FillEnvelopeTerminalReason::Closed);
        }
    }
}

//#region 💼️SharedPluginJob
pub(crate) fn fill_job(context: semio_framework_plugin::reactor::jobs::JobCtx, input: Vec<u8>, restored: Option<Vec<u8>>) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, semio_framework::Fault>>>> {
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
mod tests {
    use super::*;
    use crate::artifacts::puzzle3d::schema::testkit::*;
    use crate::artifacts::puzzle3d::schema::{BrushHostRules, BrushKindWeights, CableKindCatalog, FixtureObject, KindCompatEntry, ObjectKind, ObjectKindRepresentation, ObjectKindVortexTemplate, VortexKindCatalog, VortexProps};
    use semio_framework_job::StepOutcome;
    use std::time::{Duration, Instant};

    fn fill_capable_engine() -> Puzzle3dCollision {
        let mut engine = Puzzle3dCollision::new();
        let (positions, indices) = unit_cube_mesh_buffers();
        engine.register_mesh("/test/host.glb".to_string(), &positions, &indices);
        engine.register_mesh("/test/candidate.glb".to_string(), &positions, &indices);
        let scene = SceneConfig {
            fixture: Fixture {
                attractions: vec![],
                target_volumes: vec![],
                objects: vec![FixtureObject {
                    id: "host".to_string(),
                    object_kind: Some("Host".to_string()),
                    anchor: Default::default(),
                    mesh_url: Some("/test/host.glb".to_string()),
                    origin: [0.0, 0.0, 0.0],
                    orientation: Some([0.0, 0.0, 0.0, 1.0]),
                    scale: None,
                    vortices: vec![VortexProps { id: "v0".to_string(), vortex_kind: Some("port-a".to_string()), position: [4.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
                    reveal_index: None,
                }],
            },
            kind_catalogs: Some(KindCatalogBundle {
                objects: vec![
                    ObjectKind {
                        id: "Host".to_string(),
                        representations: vec![ObjectKindRepresentation { id: "host".into(), name: String::new(), url: "/test/host.glb".to_string(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
                        scale: None,
                        vortices: vec![],
                    },
                    ObjectKind {
                        id: "Candidate".to_string(),
                        representations: vec![ObjectKindRepresentation { id: "candidate".into(), name: String::new(), url: "/test/candidate.glb".to_string(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
                        scale: None,
                        vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("port-b".to_string()), point: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, 1.0]), ..Default::default() }],
                    },
                ],
                vortices: vec![VortexKindCatalog { id: "port-a".to_string(), default_cable_kind: None, ..Default::default() }, VortexKindCatalog { id: "port-b".to_string(), default_cable_kind: None, ..Default::default() }],
                cables: vec![],
            }),
            kind_compatibility: vec![KindCompatEntry { source: "port-b".to_string(), target: "port-a".to_string(), bidirectional: true, important: false, specificity: Some("vortex".to_string()) }],
            overlap_budget: DEFAULT_OVERLAP_BUDGET,
            seed: 1,
            host_rules: BrushHostRules::default(),
            weights: BrushKindWeights::default(),
        };
        engine.set_scene(&serde_json::to_string(&scene).expect("fill scene")).expect("set fill scene");
        engine.fill.as_ref().expect("fill").lock().expect("fill lock").max_count = 1;
        engine
    }

    fn fill_builder_for_test(base: Fixture, seed: u32, catalogs: &KindCatalogBundle) -> FillBuilder {
        let scene = Arc::new(SceneConfig { fixture: base, kind_catalogs: Some(catalogs.clone()), kind_compatibility: Vec::new(), overlap_budget: 0.0, seed, host_rules: BrushHostRules::default(), weights: BrushKindWeights::default() });
        let mut fill = FillBuilder::begin_preparation(FillPreparationRoots::new(scene, Arc::new(HashMap::new())), Operation::new(semio_framework_job::allocate_operation_id(), RevisionId(1), Generation(1), seed as u64));
        while matches!(
            fill.stage,
            FillJobStage::PrepareFixture | FillJobStage::PrepareCatalogs | FillJobStage::PrepareMeshes | FillJobStage::PrepareEntries | FillJobStage::PrepareSpatial | FillJobStage::PrepareLookup | FillJobStage::PrepareConfiguration
        ) {
            fill.prepare_one();
        }
        fill
    }

    #[test]
    fn brush_candidates_allow_separated_boxes() {
        let mut engine = Puzzle3dCollision::new();
        let positions: Vec<f32> = vec![-4.0, -4.0, -4.0, 4.0, -4.0, -4.0, 4.0, 4.0, -4.0, -4.0, 4.0, -4.0, -4.0, -4.0, 4.0, 4.0, -4.0, 4.0, 4.0, 4.0, 4.0, -4.0, 4.0, 4.0, 4.0];
        let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2];
        engine.register_mesh("/test/obstacle.glb".to_string(), &positions, &indices);
        engine.register_mesh("/test/preview.glb".to_string(), &positions, &indices);
        let scene = SceneConfig {
            fixture: Fixture {
                attractions: vec![],
                target_volumes: vec![],
                objects: vec![
                    FixtureObject {
                        id: "obstacle".to_string(),
                        object_kind: Some("Kind".to_string()),
                        anchor: Default::default(),
                        mesh_url: Some("/test/obstacle.glb".to_string()),
                        origin: [0.0, 0.0, 0.0],
                        orientation: Some([0.0, 0.0, 0.0, 1.0]),
                        scale: None,
                        vortices: vec![VortexProps { id: "v0".to_string(), vortex_kind: Some("port-a".to_string()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
                        reveal_index: None,
                    },
                    FixtureObject {
                        id: "host".to_string(),
                        object_kind: Some("Host".to_string()),
                        anchor: Default::default(),
                        mesh_url: Some("/test/unregistered.glb".to_string()),
                        origin: [12.0, 0.0, 0.0],
                        orientation: Some([0.0, 0.0, 0.0, 1.0]),
                        scale: None,
                        vortices: vec![VortexProps { id: "v0".to_string(), vortex_kind: Some("port-a".to_string()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
                        reveal_index: None,
                    },
                ],
            },
            kind_catalogs: Some(KindCatalogBundle {
                objects: vec![ObjectKind {
                    id: "Kind".to_string(),
                    representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/test/preview.glb".to_string(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
                    scale: None,
                    vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("port-b".to_string()), point: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]), ..Default::default() }],
                }],
                vortices: vec![VortexKindCatalog { id: "port-a".to_string(), default_cable_kind: None, ..Default::default() }, VortexKindCatalog { id: "port-b".to_string(), default_cable_kind: None, ..Default::default() }],
                cables: vec![CableKindCatalog { id: "cable.link".to_string(), default_attraction_kind: None, ..Default::default() }],
            }),
            kind_compatibility: vec![KindCompatEntry { source: "port-b".to_string(), target: "port-a".to_string(), bidirectional: true, important: false, specificity: Some("vortex".to_string()) }],
            overlap_budget: DEFAULT_OVERLAP_BUDGET,
            seed: 1,
            host_rules: BrushHostRules::default(),
            weights: BrushKindWeights::default(),
        };
        engine.scene = Some(Arc::new(scene));
        let result = engine.compute_brush_cache_entry("host:v0");
        assert!(!result.unknown_pending, "expected mesh-ready result");
        assert_eq!(result.free.len(), 1, "expected one collision-free candidate");
    }

    /// 🪪️ Regression: `set_scene` used to unconditionally `rebuild_queue()`, wiping `brush_cache`/`fill`
    /// progress on every resync — the app's `sync_precompute_session` calls `set_scene` on *every*
    /// action, so this made suggestion/fill precompute restart from zero on every single tick, freezing
    /// the UI. A resync with byte-identical scene JSON must be a no-operation.
    #[test]
    fn compose_fill_display_is_read_only_and_matches_apply_prefix() {
        let base = Fixture { objects: vec![fill_plan_object("base")], attractions: vec![], target_volumes: vec![] };
        let catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
        let mut fill = fill_builder_for_test(base, 7, &catalogs);
        fill.applied_count = 2;
        fill.sequence = (0..5).map(fill_plan_payload).collect();
        fill.appended_objects = (0..5).map(|index| fill_plan_object(&format!("p{index}"))).collect();
        fill.appended_attractions = (0..5).map(fill_plan_attraction).collect();
        let mut engine = Puzzle3dCollision::new();
        engine.fill = Some(Arc::new(Mutex::new(fill)));

        let display = engine.compose_fill_display(4).expect("semio_compose_rs display");
        assert_eq!(display.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base", "p0", "p1", "p2", "p3"]);
        assert_eq!(engine.fill.as_ref().expect("fill").lock().expect("fill lock").applied_count, 2, "semio_compose_rs must not mutate applied_count");

        let applied = engine.apply_fill_count(4).expect("apply fill count");
        assert_eq!(applied.objects.len(), display.objects.len());
        assert_eq!(engine.fill.as_ref().expect("fill").lock().expect("fill lock").applied_count, 4);
    }

    #[test]
    fn fill_options_paths_are_millisecond_scale() {
        let base = Fixture { objects: vec![fill_plan_object("base")], attractions: vec![], target_volumes: vec![] };
        let catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
        let mut fill = fill_builder_for_test(base.clone(), 7, &catalogs);
        fill.applied_count = 0;
        fill.sequence = (0..10).map(fill_plan_payload).collect();
        fill.appended_objects = (0..10).map(|index| fill_plan_object(&format!("p{index}"))).collect();
        fill.appended_attractions = (0..10).map(fill_plan_attraction).collect();

        let mut engine = Puzzle3dCollision::new();
        let base_scene = SceneConfig { fixture: base, kind_catalogs: Some(catalogs), kind_compatibility: vec![], overlap_budget: 0.0, seed: 7, host_rules: BrushHostRules::default(), weights: BrushKindWeights::default() };
        engine.set_scene(&serde_json::to_string(&base_scene).unwrap()).expect("seed");
        engine.fill = Some(Arc::new(Mutex::new(fill)));

        let count_start = std::time::Instant::now();
        let _ = engine.apply_fill_count(5).expect("apply fill count");
        let count_ms = count_start.elapsed().as_secs_f64() * 1000.0;
        assert!(count_ms < 5.0, "fill count apply took {count_ms}ms");
        assert_eq!(engine.fill.as_ref().expect("fill").lock().expect("fill lock").applied_count, 5);

        let weight_start = std::time::Instant::now();
        let mut object_weights = std::collections::BTreeMap::new();
        object_weights.insert("Placed".to_string(), 1.0);
        let mut vortex_weights = std::collections::BTreeMap::new();
        vortex_weights.insert("c-b".to_string(), 0.5);
        vortex_weights.insert("b-s".to_string(), 0.5);
        engine.update_kind_weights(object_weights, vortex_weights);
        let weight_ms = weight_start.elapsed().as_secs_f64() * 1000.0;
        assert!(weight_ms < 50.0, "weight update took {weight_ms}ms");
        let fill_owner = engine.fill.as_ref().expect("fill").clone();
        let fill = fill_owner.lock().expect("fill lock");
        let fill_steps = engine.fill_steps_pending_for_test();
        assert_eq!(fill_steps, fill.max_count - fill.applied_count, "weight update must soft-replan the tail without a full queue wipe");
        assert_eq!(fill.applied_count, 5, "applied fill objects must survive weight edits");
    }

    #[test]
    fn apply_fill_count_downward_move_keeps_the_plan_intact() {
        // 🔽️ Moving the count DOWN must never discard the already-planned sequence/appended objects/
        // placed entries or re-enqueue FillSteps — only `applied_count` (and the returned document-prefix
        // fixture) may change. Otherwise a jittery drag forces expensive replanning on every dip.
        let base = Fixture { objects: vec![fill_plan_object("base")], attractions: vec![], target_volumes: vec![] };
        let catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
        let mut fill = fill_builder_for_test(base.clone(), 7, &catalogs);
        fill.applied_count = 0;
        fill.sequence = (0..10).map(fill_plan_payload).collect();
        fill.appended_objects = (0..10).map(|index| fill_plan_object(&format!("p{index}"))).collect();
        fill.appended_attractions = (0..10).map(fill_plan_attraction).collect();
        fill.placed = fill
            .appended_objects
            .iter()
            .map(|object| PlacedCollisionEntry { object_id: object.id.clone(), mesh_url: "/test/placed.glb".into(), world: pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale) })
            .collect();

        let mut engine = Puzzle3dCollision::new();
        let base_scene = SceneConfig { fixture: base.clone(), kind_catalogs: Some(catalogs), kind_compatibility: vec![], overlap_budget: 0.0, seed: 7, host_rules: BrushHostRules::default(), weights: BrushKindWeights::default() };
        engine.set_scene(&serde_json::to_string(&base_scene).unwrap()).expect("seed");
        engine.fill = Some(Arc::new(Mutex::new(fill)));

        engine.apply_fill_count(8).expect("apply up to 8");
        let queue_before = engine.work_pending_for_test();
        let placed_before = engine.fill.as_ref().unwrap().lock().expect("fill lock").placed.len();
        let sequence_before = engine.fill.as_ref().unwrap().lock().expect("fill lock").sequence.len();

        engine.apply_fill_count(3).expect("apply down to 3");
        let fill_owner = engine.fill.as_ref().expect("fill").clone();
        let fill = fill_owner.lock().expect("fill lock");
        assert_eq!(fill.applied_count, 3);
        assert_eq!(fill.sequence.len(), sequence_before, "the plan is prefix-stable — downward moves never truncate it");
        assert_eq!(fill.appended_objects.len(), sequence_before);
        assert_eq!(fill.appended_attractions.len(), sequence_before);
        assert_eq!(fill.placed.len(), placed_before, "placed collision entries survive a downward move");
        drop(fill);
        assert_eq!(engine.work_pending_for_test(), queue_before, "no FillSteps get re-enqueued on a downward move");

        let fixture = engine.apply_fill_count(7).expect("apply back up to 7");
        assert_eq!(fixture.objects.len(), base.objects.len() + 7, "moving back up is instant — the plan was never discarded");
    }

    #[test]
    fn update_kind_weights_soft_replans_tail_without_rebuilding_queue() {
        let mut engine = Puzzle3dCollision::new();
        let json = single_object_scene_json();
        engine.set_scene(&json).expect("seed scene");
        let queue_len_after_seed = engine.work_pending_for_test();
        engine.precompute_step(8);
        let queue_len_after_step = engine.work_pending_for_test();
        assert!(queue_len_after_step < queue_len_after_seed);

        let mut object_weights = std::collections::BTreeMap::new();
        object_weights.insert("Host".to_string(), 0.25);
        object_weights.insert("Placed".to_string(), 0.75);
        let mut vortex_weights = std::collections::BTreeMap::new();
        vortex_weights.insert("c-b".to_string(), 0.5);
        vortex_weights.insert("b-s".to_string(), 0.5);
        engine.update_kind_weights(object_weights, vortex_weights);

        assert_eq!(engine.fill.as_ref().and_then(|fill| fill.lock().ok()).map_or(0, |fill| fill.applied_count), 0, "weight-only edits must not change applied count");
        assert_eq!(engine.fill.as_ref().and_then(|fill| fill.lock().ok()).map_or(0, |fill| fill.sequence.len()), 0, "planned tail must be discarded for replanning");
        assert!(engine.work_pending_for_test() >= queue_len_after_step, "fill steps must be re-enqueued without a full queue wipe");
        assert!(engine.fill_steps_pending_for_test() > 0, "fill planning must continue after weight edits");
    }

    #[test]
    fn set_scene_with_identical_json_preserves_precompute_progress() {
        let mut engine = Puzzle3dCollision::new();
        let json = single_object_scene_json();
        engine.set_scene(&json).expect("first set_scene should succeed");
        let queue_len_before = engine.work_pending_for_test();
        assert!(queue_len_before > 0, "rebuild_queue should have enqueued at least the fill steps");
        engine.precompute_step(4);
        let queue_len_after_step = engine.work_pending_for_test();
        assert!(queue_len_after_step < queue_len_before, "precompute_step should have drained some queue items");

        engine.set_scene(&json).expect("resync with identical json should succeed");
        assert_eq!(engine.work_pending_for_test(), queue_len_after_step, "identical scene JSON must not rebuild (wipe) the queue");

        // A genuinely different scene (different object count) must still rebuild.
        let mut scene: serde_json::Value = serde_json::from_str(&json).unwrap();
        scene["fixture"]["objects"].as_array_mut().unwrap().push(serde_json::json!({ "id": "extra", "objectKind": "Host", "meshUrl": "/test/host.glb", "origin": [5.0, 0.0, 0.0], "orientation": [0.0, 0.0, 0.0, 1.0], "vortices": [] }));
        let changed_json = serde_json::to_string(&scene).unwrap();
        engine.set_scene(&changed_json).expect("set_scene with a genuinely different scene should succeed");
        assert_ne!(engine.work_pending_for_test(), queue_len_after_step, "a changed scene must rebuild the queue");
    }

    #[test]
    fn decreasing_fill_count_keeps_the_plan_intact_and_does_not_replan() {
        // 🔽️ Downward moves are prefix-stable (see `apply_fill_count`) — the plan/sequence/appended
        // objects/queue must never be discarded or re-enqueued just because the applied prefix shrank;
        // that used to force expensive replanning on every jittery drag dip.
        let base = Fixture { objects: vec![fill_plan_object("base")], attractions: vec![], target_volumes: vec![] };
        let catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
        let mut fill = fill_builder_for_test(base, 7, &catalogs);
        fill.applied_count = 3;
        fill.sequence = (0..3).map(fill_plan_payload).collect();
        fill.appended_objects = (0..3).map(|index| fill_plan_object(&format!("p{index}"))).collect();
        fill.appended_attractions = (0..3).map(fill_plan_attraction).collect();
        fill.stalled = true;
        let rng_state = fill.rng_state;
        let mut engine = Puzzle3dCollision::new();
        engine.fill = Some(Arc::new(Mutex::new(fill)));

        let fixture = engine.apply_fill_count(1).expect("fill session");
        assert_eq!(fixture.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base", "p0"], "the returned document prefix reflects the new applied count");
        let fill_owner = engine.fill.as_ref().expect("fill builder").clone();
        let fill = fill_owner.lock().expect("fill lock");
        assert_eq!(fill.appended_objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["p0", "p1", "p2"], "the full plan survives — a downward move never discards the tail");
        assert_eq!(fill.sequence.len(), 3, "the planned sequence is never truncated by a downward move");
        assert_eq!(fill.applied_count, 1);
        assert!(fill.stalled, "apply_fill_count never touches stalled — only actual planning does");
        assert_eq!(fill.rng_state, rng_state, "no replanning happens, so the random stream is untouched");
        assert_eq!(engine.fill_steps_pending_for_test(), 0, "no FillSteps get enqueued by a downward move");
        drop(fill);

        let fixture = engine.apply_fill_count(0).expect("zero fill count");
        assert_eq!(fixture.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base"], "zero applies nothing to the document");
        assert_eq!(engine.fill.as_ref().expect("fill builder").lock().expect("fill lock").sequence.len(), 3, "even at count 0, the plan is preserved for instant re-apply");
    }

    #[test]
    fn set_scene_with_applied_fill_projection_preserves_slider_session() {
        let base = Fixture { objects: vec![fill_plan_object("base")], attractions: vec![], target_volumes: vec![] };
        let catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
        let mut fill = fill_builder_for_test(base.clone(), 7, &catalogs);
        fill.applied_count = 3;
        fill.sequence = (0..3).map(fill_plan_payload).collect();
        fill.appended_objects = (0..3).map(|index| fill_plan_object(&format!("p{index}"))).collect();
        fill.appended_attractions = (0..3).map(fill_plan_attraction).collect();
        fill.stalled = true;

        let mut engine = Puzzle3dCollision::new();
        let base_scene = SceneConfig { fixture: base, kind_catalogs: Some(catalogs), kind_compatibility: vec![], overlap_budget: 0.0, seed: 7, host_rules: BrushHostRules::default(), weights: BrushKindWeights::default() };
        let base_json = serde_json::to_string(&base_scene).unwrap();
        engine.set_scene(&base_json).expect("seed base scene");
        // 🪣️ Replace the fresh FillBuilder from rebuild_queue with the already-applied session under test.
        engine.fill = Some(Arc::new(Mutex::new(fill)));

        let mut applied_scene = base_scene;
        applied_scene.fixture.objects.extend((0..3).map(|index| fill_plan_object(&format!("p{index}"))));
        applied_scene.fixture.attractions.extend((0..3).map(fill_plan_attraction));
        // 🪪️ Pose drift on the base object (attraction rederive) must not count as a new scene.
        applied_scene.fixture.objects[0].origin = [1.0, 2.0, 3.0];
        let applied_json = serde_json::to_string(&applied_scene).unwrap();
        engine.set_scene(&applied_json).expect("re-syncing the applied fill projection must succeed");

        let fill_owner = engine.fill.as_ref().expect("fill session must survive the applied-projection re-sync").clone();
        let fill = fill_owner.lock().expect("fill lock");
        assert_eq!(fill.applied_count, 3, "applied fill count must survive incidental set_scene syncs");
        assert_eq!(fill.sequence.len(), 3, "planned fill sequence must survive incidental set_scene syncs");
        assert_eq!(fill.base.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base"]);

        let reduced = engine.apply_fill_count(1).expect("decreasing after sync");
        assert_eq!(reduced.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base", "p0"], "slider must still be able to remove fill objects after a document re-sync");
        let cleared = engine.apply_fill_count(0).expect("clear after sync");
        assert_eq!(cleared.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base"]);
    }

    /// 🪪️ Regression: registering a mesh must invalidate any cached brush candidates computed against a
    /// different (e.g. fallback-box) body for the same url, but a no-operation re-registration must not matter
    /// once the cache already reflects the current mesh set (the everyday case: every action re-seeds the
    /// fallback body, and the app's `sync_precompute_session` already guards that with `has_mesh`).
    #[test]
    fn register_mesh_invalidates_cached_precompute_state() {
        let mut engine = Puzzle3dCollision::new();
        engine.set_scene(&single_object_scene_json()).expect("set_scene should succeed");
        let applied_before = engine.fill.as_ref().and_then(|fill| fill.lock().ok()).map_or(0, |fill| fill.applied_count);
        let positions: Vec<f32> = vec![-1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0];
        let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2];
        engine.register_mesh("/test/host.glb".to_string(), &positions, &indices);
        assert!(engine.brush_cache.is_empty(), "mesh registration must invalidate stale brush cache entries");
        assert_eq!(engine.fill.as_ref().and_then(|fill| fill.lock().ok()).map(|fill| fill.applied_count), Some(applied_before), "mesh registration must not reset applied fill count");
    }

    #[test]
    fn engine_precompute_step_is_false_with_no_scene() {
        let mut engine = Puzzle3dCollision::new();
        assert!(!engine.precompute_step(10));
        assert!(engine.fill.is_none());
    }

    #[test]
    fn engine_apply_brush_placement_none_without_scene_or_catalogs() {
        let mut engine = Puzzle3dCollision::new();
        let payload = BrushPlacePayload { target_vortex_full_id: "host:v0".into(), object_kind_id: "Kind".into(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        assert!(engine.apply_brush_placement(&payload).is_none(), "no scene means no placement");

        engine.set_scene(&single_object_scene_json()).expect("seed");
        if let Some(scene) = &mut engine.scene {
            Arc::make_mut(scene).kind_catalogs = None;
        }
        assert!(engine.apply_brush_placement(&payload).is_none(), "no catalogs means no placement");
    }

    #[test]
    fn engine_has_mesh_invalidate_and_refresh_brush_candidates() {
        let mut engine = Puzzle3dCollision::new();
        engine.set_scene(&single_object_scene_json()).expect("seed");
        assert!(!engine.has_mesh("/test/host.glb"));
        let (positions, indices) = unit_cube_mesh_buffers();
        engine.register_mesh("/test/host.glb".to_string(), &positions, &indices);
        assert!(engine.has_mesh("/test/host.glb"));

        engine.invalidate_brush_target("host:v0");
        assert_eq!(engine.brush_queue.front().map(String::as_str), Some("host:v0"), "invalidated brush target must be requeued at the front");
        assert!(!engine.brush_cache.contains_key("host:v0"));

        engine.refresh_brush_candidates("host:v0");
        assert!(engine.brush_cache.contains_key("host:v0"));
        assert_eq!(engine.brush_preview("host:v0", 0), None, "the catalog's Host kind has no vortices, so there are no free candidates");
    }

    #[test]
    fn precompute_session_native_wrapper_exercises_public_methods() {
        let mut session = Puzzle3dPrecomputeSession::default();
        session.set_scene(&single_object_scene_json()).expect("set_scene");
        assert!(!session.has_mesh("/test/host.glb"));
        let (positions, indices) = unit_cube_mesh_buffers();
        session.register_mesh("/test/host.glb", &positions, &indices);
        assert!(session.has_mesh("/test/host.glb"));
        assert!(!session.fill_is_done(), "a freshly (re)seeded fill session has not stalled or hit max_count yet");

        session.precompute_step(50);
        session.invalidate_brush_target("host:v0");
        session.refresh_brush_candidates("host:v0");
        let _candidates: BrushCollisionFreeResult = session.brush_candidates("host:v0");
        assert!(session.brush_preview("host:v0", 0).is_none());

        assert_eq!(session.fill_progress().max_count, FILL_COUNT_MAX);
        assert_eq!(session.fill_available_count(), 0);

        let mut object_weights = std::collections::BTreeMap::new();
        object_weights.insert("Host".to_string(), 1.0);
        session.dispatch(Puzzle3dEngineCommand::UpdateKindWeights { object_weights, vortex_weights: std::collections::BTreeMap::new() }).expect("update kind weights");

        let missing_payload = BrushPlacePayload { target_vortex_full_id: "missing:v0".to_string(), object_kind_id: "Nonexistent".to_string(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        assert!(session.dispatch(Puzzle3dEngineCommand::ApplyBrushPlacement { payload: missing_payload }).is_err());

        let outcome = session.dispatch(Puzzle3dEngineCommand::ApplyFillCount { count: 0 }).expect("fill session available");
        let Puzzle3dEngineOutcome::Fixture(fixture) = outcome else { panic!("expected a Fixture outcome") };
        assert!(fixture.objects.iter().any(|object| object.id == "host"));
        let outcome = session.dispatch(Puzzle3dEngineCommand::ComposeFillDisplay { count: 0 }).expect("fill session available");
        let Puzzle3dEngineOutcome::Fixture(fixture) = outcome else { panic!("expected a Fixture outcome") };
        assert!(fixture.objects.iter().any(|object| object.id == "host"));
    }

    fn fill_worker_session(seed: u32) -> Puzzle3dPrecomputeSession {
        let mut engine = fill_capable_engine();
        if seed != 1 {
            let mut scene = (*engine.scene.clone().expect("scene")).clone();
            scene.seed = seed;
            engine.set_scene(&serde_json::to_string(&scene).expect("scene json")).expect("reseed scene");
        }
        engine.fill.as_ref().expect("fill").lock().expect("fill lock").max_count = FILL_COUNT_MAX;
        Puzzle3dPrecomputeSession { engine, fill_job: None, fill_admission: None, fill_terminal: None, fill_observation: FillObservation::default(), fill_applied_count: 0, last_emitted_fill_checkpoint: RefCell::new(Vec::new()) }
    }

    fn close_fill_envelope(session: &mut Puzzle3dPrecomputeSession) {
        let request = session.fill_job.clone().expect("fill request");
        assert!(session.cancel_fill_job());
        let _ = session.drive_fill_job(&request);
        let mut terminal = session.take_terminal_fill_job().expect("terminal handle");
        while !matches!(terminal.close_step(), FillEnvelopeCloseStep::Complete) {}
        assert!(terminal.terminal_is_empty());
    }

    fn enqueue_measured_fill_job(session: &mut Puzzle3dPrecomputeSession) -> Option<(u64, Vec<u8>)> {
        for _ in 0..FILL_ENVELOPE_MAX_ITEMS {
            if let Some(admitted) = session.enqueue_fill_job() {
                return Some(admitted);
            }
        }
        None
    }

    fn fill_envelope_test_guard() -> std::sync::MutexGuard<'static, ()> {
        static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        GUARD.get_or_init(|| Mutex::new(())).lock().expect("fill envelope test guard")
    }

    fn drain_orphaned_fill_envelope(request: &FillJobRequest) {
        let mut mounted = Puzzle3dPrecomputeSession::new();
        for _ in 0..FILL_ENVELOPE_MAX_ITEMS {
            mounted.poll_fill_job();
            if fill_envelope_registry().lock().expect("registry").slots[usize::from(request.slot)].is_none() {
                break;
            }
        }
        let mut registry = fill_envelope_registry().lock().expect("registry");
        assert!(registry.slots[usize::from(request.slot)].is_none(), "mounted close retires the exact orphan to terminal empty");
        assert!(registry.take_closed().is_none(), "the same terminal intent cannot mount twice after readiness is cleared");
    }

    #[test]
    fn fill_worker_token_reopens_the_exact_retained_owner_and_drives_one_turn() {
        let _guard = fill_envelope_test_guard();
        let mut admitted = fill_worker_session(7);
        let source = admitted.engine.fill.as_ref().expect("fill owner").clone();
        let source_pointer = Arc::as_ptr(&source);
        let (_, input) = enqueue_measured_fill_job(&mut admitted).expect("fill job");
        assert_eq!(input.len(), FILL_ENVELOPE_TOKEN_BYTES);
        let request = decode_fill_envelope_token(&input).expect("fixed token");
        let registry = fill_envelope_registry().lock().expect("registry");
        let retained = registry.slots[usize::from(request.slot)].as_ref().and_then(|authority| authority.fill.as_ref()).expect("retained fill owner");
        assert_eq!(Arc::as_ptr(retained), source_pointer, "admission moves the same FillBuilder authority without a whole-state clone");
        drop(registry);
        let mut reopened = Puzzle3dPrecomputeSession::new();
        assert!(reopened.restore_persisted_fill(&input));
        assert!(reopened.engine.fill.is_none(), "restore mounts only the immutable registry authority and cannot recreate a mutable engine alias");
        let before = reopened.fill_observation;
        let slice = reopened.drive_fill_job(&request).expect("one retained worker turn");
        assert!(slice.progress.is_none() || reopened.poll_fill_job() || reopened.fill_observation != before);
        drop(source);
        close_fill_envelope(&mut reopened);
    }

    #[test]
    fn fill_worker_cross_generation_restore_rejects_measuring_and_every_live_terminal_phase() {
        let _guard = fill_envelope_test_guard();
        let mut measuring = fill_worker_session(45);
        assert!(measuring.enqueue_fill_job().is_none());
        let measuring_request = measuring.fill_job.clone().expect("measuring request");
        let measuring_cursor = measuring.fill_admission.as_ref().map(|admission| admission.request.clone()).expect("measurement cursor");
        let mut producer = fill_worker_session(47);
        let (_, producer_token) = enqueue_measured_fill_job(&mut producer).expect("producer request");
        let producer_request = decode_fill_envelope_token(&producer_token).expect("producer identity");
        assert!(!measuring.restore_persisted_fill(&producer_token));
        assert_eq!(measuring.fill_job.as_ref(), Some(&measuring_request));
        assert_eq!(measuring.fill_admission.as_ref().map(|admission| &admission.request), Some(&measuring_cursor));
        drop(measuring);
        drop(producer);
        drain_orphaned_fill_envelope(&measuring_request);
        drain_orphaned_fill_envelope(&producer_request);

        for phase in [
            FillEnvelopePhase::Admitted,
            FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Complete),
            FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Cancelled),
            FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Fault),
            FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Closed),
        ] {
            let mut mounted = fill_worker_session(49);
            let (_, mounted_token) = enqueue_measured_fill_job(&mut mounted).expect("mounted request");
            let mounted_request = decode_fill_envelope_token(&mounted_token).expect("mounted identity");
            let mut other = fill_worker_session(51);
            let (_, other_token) = enqueue_measured_fill_job(&mut other).expect("other request");
            let other_request = decode_fill_envelope_token(&other_token).expect("other identity");
            let (mounted_observation, aggregate_before) = {
                let mut registry = fill_envelope_registry().lock().expect("registry");
                let authority = registry.authority_mut(&mounted_request).expect("mounted authority");
                authority.phase = phase;
                if matches!(phase, FillEnvelopePhase::Terminal(_)) {
                    authority.observation.done = true;
                }
                (authority.observation, registry.aggregate_bytes)
            };
            assert!(!mounted.restore_persisted_fill(&other_token));
            assert_eq!(mounted.fill_job.as_ref(), Some(&mounted_request), "restore cannot replace the exact mounted producer");
            assert_eq!(mounted.fill_observation, mounted_observation, "restore rejection leaves mounted observation unchanged");
            let registry = fill_envelope_registry().lock().expect("registry");
            let authority = registry.slots[usize::from(mounted_request.slot)].as_ref().expect("mounted authority remains registered");
            assert_eq!(authority.request, mounted_request);
            assert_eq!(authority.phase, phase);
            assert_eq!(registry.aggregate_bytes, aggregate_before, "restore rejection neither reserves nor releases credit");
            drop(registry);
            drop(mounted);
            drop(other);
            drain_orphaned_fill_envelope(&mounted_request);
            drain_orphaned_fill_envelope(&other_request);
            let registry = fill_envelope_registry().lock().expect("registry");
            assert!(registry.slots.iter().all(Option::is_none));
            assert_eq!(registry.aggregate_bytes, 0, "both producers close once to exact zero aggregate credit");
        }
    }

    #[test]
    fn fill_worker_cross_generation_restore_preserves_dropped_closing_handle_and_zero_credit() {
        let _guard = fill_envelope_test_guard();
        let mut mounted = fill_worker_session(53);
        let (_, mounted_token) = enqueue_measured_fill_job(&mut mounted).expect("mounted request");
        let mounted_request = decode_fill_envelope_token(&mounted_token).expect("mounted identity");
        let mut other = fill_worker_session(55);
        let (_, other_token) = enqueue_measured_fill_job(&mut other).expect("other request");
        let other_request = decode_fill_envelope_token(&other_token).expect("other identity");
        terminalize_fill_envelope(&mounted_request, FillEnvelopeTerminalReason::Closed);
        let mut terminal = mounted.take_terminal_fill_job().expect("mounted terminal");
        assert_eq!(terminal.close_step(), FillEnvelopeCloseStep::Pending);
        let retirement_pointer = fill_envelope_registry()
            .lock()
            .expect("registry")
            .authority_mut(&mounted_request)
            .and_then(|authority| authority.fill_retirement.as_ref())
            .map(|cursor| cursor as *const FillBuilderRetirementCursor as usize)
            .expect("retained close cursor");
        drop(terminal);
        assert!(!mounted.restore_persisted_fill(&other_token));
        let registry = fill_envelope_registry().lock().expect("registry");
        let authority = registry.slots[usize::from(mounted_request.slot)].as_ref().expect("closing authority");
        assert_eq!(authority.request, mounted_request);
        assert!(matches!(authority.phase, FillEnvelopePhase::Closing));
        assert!(!authority.checked_out.load(Ordering::Acquire));
        assert_eq!(authority.fill_retirement.as_ref().map(|cursor| cursor as *const FillBuilderRetirementCursor as usize), Some(retirement_pointer));
        drop(registry);
        drop(mounted);
        drop(other);
        drain_orphaned_fill_envelope(&mounted_request);
        drain_orphaned_fill_envelope(&other_request);
        let registry = fill_envelope_registry().lock().expect("registry");
        assert!(registry.slots.iter().all(Option::is_none));
        assert_eq!(registry.aggregate_bytes, 0);
    }

    #[test]
    fn fill_worker_fixed_cap_rejects_plus_one_and_reused_slot_rejects_aba() {
        let _guard = fill_envelope_test_guard();
        let mut sessions = [fill_worker_session(11), fill_worker_session(13), fill_worker_session(17), fill_worker_session(19), fill_worker_session(23)];
        let tokens = [0, 1, 2, 3].map(|index| enqueue_measured_fill_job(&mut sessions[index]).expect("within fixed operation cap").1);
        let rejected_pointer = Arc::as_ptr(sessions[4].engine.fill.as_ref().expect("rejected source remains in session"));
        assert!(enqueue_measured_fill_job(&mut sessions[4]).is_none(), "operation cap + 1 is rejected before ownership transfer");
        assert_eq!(Arc::as_ptr(sessions[4].engine.fill.as_ref().expect("exact rejected owner")), rejected_pointer);

        let first = decode_fill_envelope_token(&tokens[0]).expect("first token");
        assert!(sessions[0].cancel_fill_job());
        let _ = sessions[0].drive_fill_job(&first);
        let returned = sessions[0].take_terminal_fill_job().expect("cancelled terminal owner");
        assert_eq!(returned.reason(), Some("cancelled"));
        drop(returned);
        let mut terminal = sessions[0].take_terminal_fill_job().expect("Drop atomically returns the checked-out terminal authority");
        while !matches!(terminal.close_step(), FillEnvelopeCloseStep::Complete) {}

        let replacement = enqueue_measured_fill_job(&mut sessions[4]).expect("capacity re-arms after exact close").1;
        let replacement = decode_fill_envelope_token(&replacement).expect("replacement token");
        assert_eq!(replacement.slot, first.slot);
        assert_ne!(replacement.registry_generation, first.registry_generation);
        assert!(matches!(drive_fill_envelope(&first), FillEnvelopeDrive::Stale), "a stale generation cannot consume the reused slot");
        close_fill_envelope(&mut sessions[1]);
        close_fill_envelope(&mut sessions[2]);
        close_fill_envelope(&mut sessions[3]);
        close_fill_envelope(&mut sessions[4]);
    }

    #[test]
    fn fill_worker_item_and_byte_plus_one_reject_before_owner_transfer() {
        let mut engine = fill_capable_engine();
        let fill = engine.fill.take().expect("fill owner");
        let pointer = Arc::as_ptr(&fill);
        let operation = fill.lock().expect("fill lock").operation;
        let mut registry = FillEnvelopeRegistry::default();
        let fill = registry.reserve(1, operation, FILL_ENVELOPE_MAX_ITEMS + 1, FILL_ENVELOPE_MAX_BYTES, fill, root_cancel_token(), 1, 0, FillObservation::default()).expect_err("item cap + 1");
        assert_eq!(Arc::as_ptr(&fill), pointer);
        let fill = registry.reserve(2, operation, FILL_ENVELOPE_MAX_ITEMS, FILL_ENVELOPE_MAX_BYTES + 1, fill, root_cancel_token(), 1, 0, FillObservation::default()).expect_err("byte cap + 1");
        assert_eq!(Arc::as_ptr(&fill), pointer, "both preflight failures return the exact source authority");
    }

    #[test]
    fn fill_worker_actual_owner_census_rejects_cap_plus_one_with_exact_handback() {
        let _guard = fill_envelope_test_guard();
        let mut session = fill_worker_session(29);
        let source = session.engine.fill.as_ref().expect("fill owner").clone();
        let pointer = Arc::as_ptr(&source);
        source.lock().expect("fill lock").inject_nested_owner_page_plus_one_for_test();
        assert!(enqueue_measured_fill_job(&mut session).is_none());
        assert!(session.engine.fill.is_none(), "registered fault ownership cannot recreate a mutable engine alias");
        assert!(session.fill_admission.is_none(), "rejected census cannot strand a partially measured owner");
        let request = session.fill_job.clone().expect("rejected registered owner");
        let mut registry = fill_envelope_registry().lock().expect("registry");
        let rejected = registry.authority_mut(&request).and_then(|authority| authority.fill.as_ref()).expect("registered rejected owner");
        assert_eq!(Arc::as_ptr(rejected), pointer, "nested ObjectKind backing cap + 1 keeps the exact source authority in the registered fault owner");
        drop(registry);
        drop(source);
        drop(session);
        drain_orphaned_fill_envelope(&request);
    }

    #[test]
    fn fill_worker_admitted_fixed_pages_survive_replan_and_mesh_supersession_until_retained_close() {
        let _guard = fill_envelope_test_guard();
        let mut session = fill_worker_session(43);
        let (_, token) = enqueue_measured_fill_job(&mut session).expect("fill job");
        let request = decode_fill_envelope_token(&token).expect("request");
        let (admitted_pointer, admitted_credit, admitted_backing) = {
            let registry = fill_envelope_registry().lock().expect("registry");
            let authority = registry.slots[usize::from(request.slot)].as_ref().filter(|authority| authority.request == request).expect("admitted authority");
            let fill = authority.fill.as_ref().expect("exclusive fill").lock().expect("fill lock");
            (Arc::as_ptr(authority.fill.as_ref().expect("fill")), (authority.reserved_items, authority.reserved_bytes), fill.fixed_backing_witness_for_test())
        };
        assert!(session.engine.fill.is_none(), "admission removes the last mutable session alias");

        let mut object_weights = std::collections::BTreeMap::new();
        object_weights.insert("Host".to_string(), 3.0);
        session.dispatch(Puzzle3dEngineCommand::UpdateKindWeights { object_weights, vortex_weights: std::collections::BTreeMap::new() }).expect("superseding weights");
        let weight_replacement_pointer = Arc::as_ptr(session.engine.fill.as_ref().expect("separate weight replacement candidate"));
        assert_ne!(weight_replacement_pointer, admitted_pointer, "weight supersession builds a distinct unadmitted candidate");
        let (positions, indices) = unit_cube_mesh_buffers();
        session.register_mesh("/test/superseding.glb", &positions, &indices);
        let replacement_pointer = Arc::as_ptr(session.engine.fill.as_ref().expect("separate mesh replacement candidate"));
        assert_ne!(replacement_pointer, weight_replacement_pointer, "mesh supersession replaces the unadmitted weight candidate without touching the admitted owner");

        let registry = fill_envelope_registry().lock().expect("registry");
        let authority = registry.slots[usize::from(request.slot)].as_ref().filter(|authority| authority.request == request).expect("old admitted authority");
        let fill = authority.fill.as_ref().expect("old exclusive fill").lock().expect("fill lock");
        assert_eq!(Arc::as_ptr(authority.fill.as_ref().expect("fill")), admitted_pointer);
        assert_eq!((authority.reserved_items, authority.reserved_bytes), admitted_credit, "the old exact admission credit remains assigned until close");
        assert_eq!(fill.fixed_backing_witness_for_test(), admitted_backing, "weight and mesh refresh cannot clear, replace, or drop any admitted fixed page or semantic entry");
        drop(fill);
        drop(registry);

        close_fill_envelope(&mut session);
        assert_eq!(Arc::as_ptr(session.engine.fill.as_ref().expect("replacement survives old close")), replacement_pointer);
        let (_, replacement_token) = enqueue_measured_fill_job(&mut session).expect("replacement is independently re-censused and admitted");
        assert_ne!(decode_fill_envelope_token(&replacement_token).expect("replacement request").registry_generation, request.registry_generation);
        close_fill_envelope(&mut session);
    }

    #[test]
    fn fill_worker_session_drop_during_measurement_mounts_the_same_terminal_once() {
        let _guard = fill_envelope_test_guard();
        let mut session = fill_worker_session(31);
        assert!(session.enqueue_fill_job().is_none(), "the first grant only begins exact owner measurement");
        let request = session.fill_job.clone().expect("measurement has a registered exact owner");
        assert!(session.fill_admission.is_some());
        let registry = fill_envelope_registry().lock().expect("registry contention");
        drop(session);
        assert_eq!(fill_envelope_terminal_intents()[usize::from(request.slot)].reason.load(Ordering::Acquire), FillEnvelopeTerminalReason::Closed.code());
        drop(registry);
        drain_orphaned_fill_envelope(&request);
    }

    #[test]
    fn fill_worker_completed_before_session_drop_is_reclassified_and_mounted_once() {
        let _guard = fill_envelope_test_guard();
        let mut session = fill_worker_session(33);
        let (_, token) = enqueue_measured_fill_job(&mut session).expect("fill job");
        let request = decode_fill_envelope_token(&token).expect("request");
        {
            let mut registry = fill_envelope_registry().lock().expect("registry");
            let authority = registry.authority_mut(&request).expect("authority");
            authority.phase = FillEnvelopePhase::Terminal(FillEnvelopeTerminalReason::Complete);
            authority.observation.done = true;
        }
        drop(session);
        let mut mounted = Puzzle3dPrecomputeSession::new();
        assert!(mounted.poll_fill_job(), "a mounted caller consumes the retained close intent");
        assert_eq!(mounted.fill_terminal.as_ref().and_then(FillEnvelopeTerminalHandle::reason), Some("closed"));
        drop(mounted);
        drain_orphaned_fill_envelope(&request);
    }

    #[test]
    fn fill_worker_session_drop_during_partial_close_rearms_the_same_cursor_once() {
        let _guard = fill_envelope_test_guard();
        let mut session = fill_worker_session(34);
        let (_, token) = enqueue_measured_fill_job(&mut session).expect("fill job");
        let request = decode_fill_envelope_token(&token).expect("request");
        terminalize_fill_envelope(&request, FillEnvelopeTerminalReason::Closed);
        let mut terminal = session.take_terminal_fill_job().expect("terminal handle");
        assert_eq!(terminal.close_step(), FillEnvelopeCloseStep::Pending, "one close grant only moves the admitted fill into its retirement cursor");
        let registry = fill_envelope_registry().lock().expect("registry contention");
        let authority = registry.slots[usize::from(request.slot)].as_ref().expect("closing authority");
        assert!(matches!(authority.phase, FillEnvelopePhase::Closing));
        let retirement_pointer = authority.fill_retirement.as_ref().map(|cursor| cursor as *const FillBuilderRetirementCursor as usize).expect("partial retirement cursor");
        session.fill_terminal = Some(terminal);
        drop(session);
        drop(registry);

        let mut mounted = Puzzle3dPrecomputeSession::new();
        assert!(mounted.poll_fill_job(), "the durable close intent re-arms the abandoned Closing generation");
        assert_eq!(mounted.fill_terminal.as_ref().map(|terminal| &terminal.request), Some(&request));
        let registry = fill_envelope_registry().lock().expect("registry");
        let authority = registry.slots[usize::from(request.slot)].as_ref().expect("same closing authority");
        assert_eq!(authority.fill_retirement.as_ref().map(|cursor| cursor as *const FillBuilderRetirementCursor as usize), Some(retirement_pointer), "reclamation resumes rather than resets or duplicates the cursor");
        drop(registry);
        for _ in 0..FILL_ENVELOPE_MAX_ITEMS {
            mounted.poll_fill_job();
            if fill_envelope_registry().lock().expect("registry").slots[usize::from(request.slot)].is_none() {
                break;
            }
        }
        let mut registry = fill_envelope_registry().lock().expect("registry");
        assert!(registry.slots[usize::from(request.slot)].is_none(), "the resumed close reaches terminal empty");
        assert!(registry.take_closed().is_none(), "the same Closing generation is never rediscovered twice");
    }

    #[test]
    fn fill_worker_terminal_resume_contention_returns_then_rearms_the_exact_owner() {
        let _guard = fill_envelope_test_guard();
        let mut session = fill_worker_session(35);
        let (_, token) = enqueue_measured_fill_job(&mut session).expect("fill job");
        let request = decode_fill_envelope_token(&token).expect("request");
        terminalize_fill_envelope(&request, FillEnvelopeTerminalReason::Fault);
        let terminal = session.take_terminal_fill_job().expect("terminal owner");
        let registry = fill_envelope_registry().lock().expect("contended registry");
        let terminal = terminal.resume().expect_err("contention returns the same checked-out terminal handle");
        assert_eq!(terminal.request, request);
        drop(registry);
        let resumed = match terminal.resume() {
            Ok(token) => token,
            Err(_) => panic!("capacity change must re-arm the exact owner once"),
        };
        assert_eq!(resumed, token);
        terminalize_fill_envelope(&request, FillEnvelopeTerminalReason::Closed);
        drop(session);
        drain_orphaned_fill_envelope(&request);
    }

    #[test]
    fn fill_worker_malformed_token_faults_exact_raw_owner_not_wrong_context_owner() {
        let _guard = fill_envelope_test_guard();
        let mut session = fill_worker_session(39);
        let (_, token) = enqueue_measured_fill_job(&mut session).expect("fill job");
        let request = decode_fill_envelope_token(&token).expect("request");
        let mut unrelated = fill_worker_session(57);
        let (_, unrelated_token) = enqueue_measured_fill_job(&mut unrelated).expect("unrelated job");
        let unrelated_request = decode_fill_envelope_token(&unrelated_token).expect("unrelated request");
        let mut malformed = token;
        malformed[0] ^= 0xff;
        let mut entry = FillEnvelopeJobEntryCursor::new(unrelated_request.job, malformed);
        assert_eq!(entry.step(), Err("fill worker token header is malformed"));
        drop(entry);
        assert_eq!(session.take_terminal_fill_job().and_then(|terminal| terminal.reason()), Some("fault"), "malformed production ingress preserves the exact registered owner before returning the fault");
        let registry = fill_envelope_registry().lock().expect("registry");
        assert!(matches!(registry.slots[usize::from(unrelated_request.slot)].as_ref().map(|authority| authority.phase), Some(FillEnvelopePhase::Admitted)), "malformed ingress cannot fault the wrong context owner");
        drop(registry);
        drop(session);
        drop(unrelated);
        drain_orphaned_fill_envelope(&request);
        drain_orphaned_fill_envelope(&unrelated_request);
    }

    #[test]
    fn fill_worker_wrong_context_identity_faults_decoded_producer_before_drive() {
        let _guard = fill_envelope_test_guard();
        let mut producer = fill_worker_session(59);
        let (_, token) = enqueue_measured_fill_job(&mut producer).expect("producer job");
        let request = decode_fill_envelope_token(&token).expect("producer request");
        let mut unrelated = fill_worker_session(61);
        let (_, unrelated_token) = enqueue_measured_fill_job(&mut unrelated).expect("unrelated job");
        let unrelated_request = decode_fill_envelope_token(&unrelated_token).expect("unrelated request");
        let mut entry = FillEnvelopeJobEntryCursor::new(unrelated_request.job, token);
        let decoded = loop {
            if let Some(decoded) = entry.step().expect("one-field decode") {
                break decoded;
            }
        };
        assert_eq!(decoded, request);
        assert_eq!(entry.bind(&decoded), Err("fill worker context job does not match the decoded request owner"));
        drop(entry);
        assert_eq!(producer.take_terminal_fill_job().and_then(|terminal| terminal.reason()), Some("fault"));
        let registry = fill_envelope_registry().lock().expect("registry");
        assert!(matches!(registry.slots[usize::from(unrelated_request.slot)].as_ref().map(|authority| authority.phase), Some(FillEnvelopePhase::Admitted)), "wrong context identity cannot transition another producer");
        drop(registry);
        drop(producer);
        drop(unrelated);
        drain_orphaned_fill_envelope(&request);
        drain_orphaned_fill_envelope(&unrelated_request);
    }

    #[test]
    fn fill_worker_stale_envelope_identity_is_rejected_without_faulting_replacement() {
        let _guard = fill_envelope_test_guard();
        let mut stale = fill_worker_session(63);
        let (_, stale_token) = enqueue_measured_fill_job(&mut stale).expect("stale job");
        let stale_request = decode_fill_envelope_token(&stale_token).expect("stale request");
        close_fill_envelope(&mut stale);
        let mut replacement = fill_worker_session(65);
        let (_, replacement_token) = enqueue_measured_fill_job(&mut replacement).expect("replacement job");
        let replacement_request = decode_fill_envelope_token(&replacement_token).expect("replacement request");
        let mut entry = FillEnvelopeJobEntryCursor::new(stale_request.job, stale_token);
        let decoded = loop {
            if let Some(decoded) = entry.step().expect("one-field decode") {
                break decoded;
            }
        };
        assert_eq!(entry.bind(&decoded), Err("fill worker envelope owner is stale"));
        drop(entry);
        let registry = fill_envelope_registry().lock().expect("registry");
        assert!(matches!(registry.slots[usize::from(replacement_request.slot)].as_ref().map(|authority| authority.phase), Some(FillEnvelopePhase::Admitted)), "stale identity cannot fault the live replacement or any no-owner slot");
        drop(registry);
        close_fill_envelope(&mut replacement);
    }

    #[test]
    fn fill_worker_mounted_terminal_pump_closes_completed_slot_and_rearms_capacity() {
        let _guard = fill_envelope_test_guard();
        let mut session = fill_worker_session(37);
        let (_, token) = enqueue_measured_fill_job(&mut session).expect("fill job");
        let request = decode_fill_envelope_token(&token).expect("request");
        assert!(session.cancel_fill_job());
        let _ = session.drive_fill_job(&request);
        for _ in 0..4096 {
            session.poll_fill_job();
            if session.fill_job.is_none() && session.fill_terminal.is_none() {
                break;
            }
        }
        assert!(session.fill_job.is_none());
        assert!(session.fill_terminal.is_none());
        assert!(fill_envelope_registry().lock().expect("registry").slots[usize::from(request.slot)].is_none());
    }

    #[test]
    fn fill_worker_early_fault_guard_terminalizes_and_deep_retirement_is_incremental() {
        let _guard = fill_envelope_test_guard();
        let mut session = fill_worker_session(41);
        let (_, token) = enqueue_measured_fill_job(&mut session).expect("fill job");
        let request = decode_fill_envelope_token(&token).expect("request");
        {
            let _fault = FillEnvelopeWorkerFaultGuard::new(&token);
        }
        let mut terminal = session.take_terminal_fill_job().expect("fault terminal");
        assert_eq!(terminal.reason(), Some("fault"));
        assert_eq!(terminal.close_step(), FillEnvelopeCloseStep::Pending, "the first close grant only transfers the final builder authority into its retirement cursor");
        assert!(!terminal.terminal_is_empty());
        let mut grants = 1;
        while !matches!(terminal.close_step(), FillEnvelopeCloseStep::Complete) {
            grants += 1;
        }
        assert!(grants > 8, "a populated builder cannot be bulk-dropped by one close grant");
        assert!(terminal.terminal_is_empty());
    }

    #[test]
    fn fill_worker_token_decode_advances_exactly_one_field_per_grant() {
        let request = FillJobRequest { job: 3, operation: 5, generation: 7, base_revision: 11, slot: 1, registry_generation: 13 };
        let mut cursor = FillEnvelopeTokenCursor::new(fill_envelope_token(&request).to_vec());
        assert_eq!(cursor.step().expect("header"), None);
        assert_eq!(cursor.step().expect("registry generation"), None);
        assert_eq!(cursor.step().expect("job"), None);
        assert_eq!(cursor.step().expect("operation"), None);
        assert_eq!(cursor.step().expect("generation"), None);
        assert_eq!(cursor.step().expect("base revision"), None);
        assert_eq!(cursor.step().expect("publish"), Some(request));
    }

    #[test]
    fn fill_operation_identity_checked_nonzero_exhaustion_permanently_refuses_aba() {
        let mut generation_engine = Puzzle3dCollision::new();
        assert_eq!(generation_engine.allocate_fill_identity(true), Some((RevisionId(1), Generation(1))), "zero counters allocate the first exact nonzero identity");
        generation_engine.fill_generation = u64::MAX - 1;
        assert_eq!(generation_engine.allocate_fill_identity(false), Some((RevisionId(1), Generation(u64::MAX))));
        assert_eq!(generation_engine.allocate_fill_identity(false), None, "generation max + 1 is permanently refused");
        assert_eq!(generation_engine.allocate_fill_identity(false), None);
        assert_eq!((generation_engine.fill_revision, generation_engine.fill_generation), (1, u64::MAX));

        let mut revision_engine = Puzzle3dCollision::new();
        revision_engine.fill_revision = u64::MAX - 1;
        revision_engine.fill_generation = 1;
        assert_eq!(revision_engine.allocate_fill_identity(true), Some((RevisionId(u64::MAX), Generation(2))));
        assert_eq!(revision_engine.allocate_fill_identity(true), None, "revision max + 1 is permanently refused without consuming a generation");
        assert_eq!(revision_engine.allocate_fill_identity(true), None);
        assert_eq!((revision_engine.fill_revision, revision_engine.fill_generation), (u64::MAX, 2));
    }

    #[test]
    fn fill_worker_zero_semantic_counters_and_exhausted_stale_tokens_never_alias() {
        let request = FillJobRequest { job: 3, operation: 5, generation: u64::MAX, base_revision: u64::MAX, slot: 1, registry_generation: 13 };
        let exhausted = fill_envelope_token(&request);
        assert_eq!(decode_fill_envelope_token(&exhausted), Some(request.clone()));
        let mut zero_generation = exhausted;
        zero_generation[40..48].copy_from_slice(&0_u64.to_le_bytes());
        assert!(decode_fill_envelope_token(&zero_generation).is_none());
        let mut zero_revision = exhausted;
        zero_revision[48..56].copy_from_slice(&0_u64.to_le_bytes());
        assert!(decode_fill_envelope_token(&zero_revision).is_none());
        assert_ne!(zero_generation, exhausted, "exhaustion cannot reset to a zero token that aliases the permanent max identity");
        assert_ne!(zero_revision, exhausted);
    }

    #[test]
    fn precompute_session_native_wrapper_errors_without_scene() {
        let mut session = Puzzle3dPrecomputeSession::new();
        assert!(session.dispatch(Puzzle3dEngineCommand::ApplyFillCount { count: 0 }).is_err());
        assert!(session.dispatch(Puzzle3dEngineCommand::ComposeFillDisplay { count: 0 }).is_err());
        let payload = BrushPlacePayload { target_vortex_full_id: "a:v0".to_string(), object_kind_id: "b".to_string(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        assert!(session.dispatch(Puzzle3dEngineCommand::ApplyBrushPlacement { payload }).is_err());
        assert!(session.fill_is_done());
        assert_eq!(session.fill_available_count(), 0);
    }

    #[test]
    fn fill_lane_advances_while_brush_targets_remain_queued() {
        let mut engine = Puzzle3dCollision::new();
        engine.set_scene(&single_object_scene_json()).expect("seed");
        assert!(engine.fill_steps_pending_for_test() > 0, "seed scene must schedule fill steps");
        assert!(!engine.brush_queue.is_empty(), "seed scene must schedule brush targets");
        let before = engine.fill_progress_summary().count;
        for _ in 0..24 {
            engine.precompute_step_lane(PrecomputeLane::Fill, 4);
        }
        let after = engine.fill_progress_summary().count;
        assert!(after > before || engine.fill_progress_summary().done, "fill lane must make planning progress without draining brush first");
    }

    #[test]
    fn brush_candidates_cold_cache_returns_pending_without_populating_cache() {
        let mut session = Puzzle3dPrecomputeSession::new();
        session.set_scene(&single_object_scene_json()).expect("seed");
        let result = session.brush_candidates("host:v0");
        assert!(result.unknown_pending, "cold cache must surface pending state: {result:?}");
        assert!(session.brush_preview("host:v0", 0).is_none());
    }

    /// 🧰️ `enqueue_brush_target` is the app-facing append (vs. `invalidate_brush_target`'s
    /// front-of-queue jump) — appending an already-queued id must be a no-operation.
    #[test]
    fn enqueue_brush_target_appends_once() {
        let mut engine = Puzzle3dCollision::new();
        engine.enqueue_brush_target("host:v0");
        engine.enqueue_brush_target("host:v0");
        assert_eq!(engine.brush_queue.len(), 1);
    }

    /// 🖐️ Compile-guard for the 🖐️5d app, which builds its own `Puzzle5dPrecomputeSession` on top of
    /// this one (relocated from the former `⚙️engine` root's `the_5d_facing_engine_surface_stays_public`,
    /// ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): every item it names must stay
    /// publicly reachable — pure data under `crate::artifacts::puzzle3d::schema::…`, the session/dispatch
    /// surface under `crate::editor::puzzle3d::precompute::…`. A rename or a visibility narrowing breaks
    /// this test long before it breaks 5d.
    #[test]
    fn the_5d_facing_precompute_surface_stays_public() {
        use crate::artifacts::puzzle3d::Puzzle3dError as GuardError;

        let mut session = Puzzle3dPrecomputeSession::new();
        assert!(session.set_scene("{ not json").is_err(), "set_scene surfaces a Puzzle3dError");
        session.register_mesh("/probe.glb", &[], &[]);
        assert!(!session.has_mesh("/probe.glb"));
        assert!(!session.precompute_step(1));
        let _: BrushCollisionFreeResult = session.brush_candidates("probe:v0");
        let _: Option<BrushPreviewState> = session.brush_preview("probe:v0", 0);
        let _: FillBuildProgress = session.fill_progress();
        assert!(session.precompute_step_lane(PrecomputeLane::Brush, 1) || true);
        let payload = BrushPlacePayload { target_vortex_full_id: "probe:v0".into(), object_kind_id: "Kind".into(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        let rejected: Result<Puzzle3dEngineOutcome, GuardError> = session.dispatch(Puzzle3dEngineCommand::ApplyBrushPlacement { payload });
        assert!(matches!(rejected, Err(GuardError::BrushPlacementRejected)));
        assert!(session.dispatch(Puzzle3dEngineCommand::ApplyFillCount { count: 0 }).is_err());
        let _: fn(&Fixture, &BrushPlacePayload, &KindCatalogBundle) -> Fixture = apply_brush_placement_to_fixture;
    }

    /// 🔗️ Minimal scene JSON matching `SceneConfig`'s real wire shape (camelCase, per its
    /// `#[serde(rename = ...)]` attrs) — relocated from `🧬️mutations/💾️binary/🦀️component.rs`'s own
    /// `sample_scene_config` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): that file's
    /// own copy stays for the pure-data wire-format-guard tests that need no session, and this copy feeds
    /// the two dispatch tests below, since a schema-side test module must not depend on the app.
    fn sample_scene_config() -> SceneConfig {
        let json = r#"{
            "fixture": {
                "objects": [{"id": "host", "objectKind": "Host", "meshUrl": "/test/host.glb", "origin": [0,0,0], "orientation": [0,0,0,1], "vortices": [{"id": "v0", "vortexKind": "port-a", "position": [0,0,0], "direction": [0,0,-1]}]}],
                "attractions": [],
                "targetVolumes": []
            },
            "kindCatalogs": {"objects": [{"id": "Host", "representations": [{"id": "r0", "name": "default", "url": "/test/host.glb"}], "vortices": []}], "vortices": [{"id": "port-a"}], "cables": []},
            "kindCompatibility": [],
            "overlapBudget": 0.02,
            "seed": 1
        }"#;
        serde_json::from_str(json).expect("sample scene config parses")
    }

    /// 🎯️ Behavioral parity: `dispatch` must reach the exact same engine logic the old JSON-string
    /// wasm-bindgen methods delegated to — `SetScene` seeds a fill session, `ApplyFillCount`/
    /// `ComposeFillDisplay` read/apply its prefix, matching what this module's own
    /// `precompute_session_native_wrapper_exercises_public_methods` test already asserts for the
    /// pre-dispatch API. Relocated from `🧬️mutations/💾️binary/🦀️component.rs`'s
    /// `dispatch_set_scene_then_apply_and_compose_fill_count_round_trip` — that test constructed
    /// `Puzzle3dPrecomputeSession` directly, which is now an app type a schema test file must not reach.
    #[test]
    fn dispatch_set_scene_then_apply_and_compose_fill_count_round_trip() {
        let mut session = Puzzle3dPrecomputeSession::new();
        session.dispatch(Puzzle3dEngineCommand::SetScene { scene: sample_scene_config() }).expect("set scene");
        assert!(!session.fill_is_done(), "a freshly seeded fill session has not stalled or hit max_count yet");

        session.precompute_step(50);

        let outcome = session.dispatch(Puzzle3dEngineCommand::ComposeFillDisplay { count: 0 }).expect("compose fill display");
        let Puzzle3dEngineOutcome::Fixture(fixture) = outcome else { panic!("expected a Fixture outcome") };
        assert!(fixture.objects.iter().any(|object| object.id == "host"), "the base scene's host object must survive compose_fill_display(0)");

        let outcome = session.dispatch(Puzzle3dEngineCommand::ApplyFillCount { count: 0 }).expect("apply fill count");
        let Puzzle3dEngineOutcome::Fixture(fixture) = outcome else { panic!("expected a Fixture outcome") };
        assert!(fixture.objects.iter().any(|object| object.id == "host"));
    }

    /// 🎯️ Relocated from `🧬️mutations/💾️binary/🦀️component.rs`'s
    /// `dispatch_brush_preview_without_scene_returns_none` (same reason as the test above).
    #[test]
    fn dispatch_brush_preview_without_scene_returns_none() {
        let mut session = Puzzle3dPrecomputeSession::new();
        let outcome = session.dispatch(Puzzle3dEngineCommand::BrushPreview { vortex_full_id: "host:v0".to_string(), candidate_index: 0 }).expect("brush preview never errors");
        assert_eq!(outcome, Puzzle3dEngineOutcome::BrushPreview(None), "no scene means no cached brush candidates yet");
    }

    #[test]
    fn fill_job_checkpoint_is_a_fixed_generation_token_not_a_whole_state_buffer() {
        let _guard = fill_envelope_test_guard();
        let mut session = fill_worker_session(31);
        let (_, token) = enqueue_measured_fill_job(&mut session).expect("fill envelope");
        assert_eq!(token.len(), FILL_ENVELOPE_TOKEN_BYTES);
        assert_eq!(session.fill_checkpoint_bytes(), token);
        assert!(serde_json::from_slice::<SceneConfig>(&token).is_err(), "the job checkpoint cannot regress to whole-scene serde");
        close_fill_envelope(&mut session);
    }

    #[test]
    fn fill_first_substantive_preview_arrives_below_fifty_ms_and_every_step_below_eight_ms() {
        let mut engine = fill_capable_engine();
        let started = Instant::now();
        let mut first_preview = None;
        let mut completed = false;
        let mut max_step = Duration::ZERO;
        for _ in 0..10_000 {
            let step_started = Instant::now();
            let active = engine.precompute_step_lane(PrecomputeLane::Fill, 1);
            let step_elapsed = step_started.elapsed();
            max_step = max_step.max(step_elapsed);
            assert!(step_elapsed < Duration::from_millis(8), "fill resume step reached the 8ms ceiling");
            if first_preview.is_none() && engine.fill.as_ref().and_then(|fill| fill.lock().ok()).is_some_and(|fill| fill.preview.candidate_ghost.is_some()) {
                first_preview = Some(started.elapsed());
            }
            if !active {
                completed = true;
                break;
            }
        }
        assert!(
            first_preview.is_some_and(|elapsed| elapsed < Duration::from_millis(50)),
            "first substantive fill preview exceeded 50ms: {first_preview:?}; stage={:?}; rejected={}",
            engine.fill.as_ref().and_then(|fill| fill.lock().ok()).map(|fill| fill.stage),
            engine.fill.as_ref().and_then(|fill| fill.lock().ok()).map_or(0, |fill| fill.preview.rejected_count)
        );
        assert!(completed, "fill did not complete within the bounded resume budget");
    }
}
//#endregion 🧪️Tests

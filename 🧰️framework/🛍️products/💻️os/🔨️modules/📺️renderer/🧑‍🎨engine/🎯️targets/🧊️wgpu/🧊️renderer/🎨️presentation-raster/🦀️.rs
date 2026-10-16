//! 🎨️ Presentation and raster: presentation authorities, completion queue, staged reference images, native asset transport and the runtime mailbox state.
use super::*;

#[path = "🌐️native-asset-stream/🦀️.rs"]
mod native_asset_stream;
pub use native_asset_stream::*;

pub const RUNTIME_COMPLETION_CAPACITY: usize = 128;
pub const EFFECT_STORM_BUDGET: u32 = 64;

//#region 📮️RuntimeMailbox
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuntimePresentationWitness {
    pub scene_revision: u64,
    pub input_generation: u64,
}

struct RuntimePresentationAuthorityInner {
    scene_revision: AtomicU64,
    input_generation: AtomicU64,
    admitted_scene_revision: AtomicU64,
    admitted_input_generation: AtomicU64,
    admitted: AtomicBool,
}

#[derive(Clone)]
pub struct RuntimePresentationAuthority(pub Arc<RuntimePresentationAuthorityInner>);

impl RuntimePresentationAuthority {
    pub fn new() -> Self {
        Self(Arc::new(RuntimePresentationAuthorityInner {
            scene_revision: AtomicU64::new(1),
            input_generation: AtomicU64::new(0),
            admitted_scene_revision: AtomicU64::new(0),
            admitted_input_generation: AtomicU64::new(0),
            admitted: AtomicBool::new(false),
        }))
    }

    /// 🎟️ The witness ONE frame build was admitted under, written by that build when it mints its
    /// `FrameBuildCursor` and read by `AppPresenter::present_step`'s admission gate.
    ///
    /// **This is what makes the build and the presenter one authority instead of two.** `current()`
    /// is a moving target by design — `mark_scene_changed` fires from every runtime completion the
    /// host pumps, several per tick — so a presenter that re-read it at admission refused a packet
    /// that had been built, correctly, against the authority as it stood when the build started, and
    /// turned it into a SURFACE FAULT (`offscreen prepared frame admission: prepared render revision
    /// is stale: live=35, packet=27` → `worker-present-failed`, ticket
    /// 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-edit-convergence-perf-2026-09-14.md` §6). Freshness
    /// is the BUILD's decision — it supersedes itself the moment its own witness moves — and the
    /// presenter's gate is there to refuse a packet from some OTHER build, which this pair still
    /// catches exactly.
    ///
    /// One writer, one reader, no race: `poll_runtime_and_resubmit` admits at most one build at a
    /// time, and `redraw_core` admits the finished frame and takes its `BeginGpu` step inside the
    /// SAME host tick, before any later build can write here.
    pub fn admit_build(&self, witness: RuntimePresentationWitness) {
        self.0.admitted_scene_revision.store(witness.scene_revision, Ordering::Release);
        self.0.admitted_input_generation.store(witness.input_generation, Ordering::Release);
        self.0.admitted.store(true, Ordering::Release);
    }

    /// 🎟️ The pair a presentation must be admitted against — the live one until a build has ever run.
    pub fn admitted(&self) -> RuntimePresentationWitness {
        if !self.0.admitted.load(Ordering::Acquire) {
            return self.current();
        }
        RuntimePresentationWitness { scene_revision: self.0.admitted_scene_revision.load(Ordering::Acquire), input_generation: self.0.admitted_input_generation.load(Ordering::Acquire) }
    }

    pub fn mark_scene_changed(&self) {
        self.0.scene_revision.fetch_add(1, Ordering::AcqRel);
    }

    pub fn observe_input_generation(&self, generation: u64) {
        self.0.input_generation.store(generation, Ordering::Release);
    }

    pub fn current(&self) -> RuntimePresentationWitness {
        RuntimePresentationWitness { scene_revision: self.0.scene_revision.load(Ordering::Acquire), input_generation: self.0.input_generation.load(Ordering::Acquire) }
    }

    pub fn witness_for(&self, input_generation: u64) -> Option<RuntimePresentationWitness> {
        let witness = self.current();
        (witness.input_generation == input_generation).then_some(witness)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn matches(&self, scene_revision: u64, input_generation: u64) -> bool {
        self.current() == RuntimePresentationWitness { scene_revision, input_generation }
    }
}

struct RuntimeRasterOperationAuthorityInner {
    next_operation: AtomicU64,
    exhausted: AtomicBool,
    current: Mutex<Option<ui_gpu::RasterTextureWitness>>,
}

#[derive(Clone)]
pub struct RuntimeRasterOperationAuthority(pub Arc<RuntimeRasterOperationAuthorityInner>);

impl RuntimeRasterOperationAuthority {
    pub fn new() -> Self {
        Self(Arc::new(RuntimeRasterOperationAuthorityInner { next_operation: AtomicU64::new(1), exhausted: AtomicBool::new(false), current: Mutex::new(None) }))
    }

    pub fn begin(&self, scene_revision: u64, preview_generation: u64) -> Result<ui_gpu::RasterTextureWitness, &'static str> {
        let mut current = self.0.current.lock().map_err(|_| "raster operation authority was poisoned")?;
        if current.is_some() {
            return Err("raster operation authority was occupied");
        }
        let operation = loop {
            if self.0.exhausted.load(Ordering::Acquire) {
                return Err("raster operation generation exhausted");
            }
            let operation = self.0.next_operation.load(Ordering::Acquire);
            if operation == u64::MAX {
                if self.0.exhausted.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                    break operation;
                }
                continue;
            }
            if self.0.next_operation.compare_exchange(operation, operation + 1, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                break operation;
            }
        };
        let witness = ui_gpu::RasterTextureWitness { scene_revision, preview_generation, operation };
        *current = Some(witness);
        Ok(witness)
    }

    pub fn current(&self) -> Option<ui_gpu::RasterTextureWitness> {
        self.0.current.lock().ok().and_then(|current| *current)
    }

    pub fn matches(&self, witness: ui_gpu::RasterTextureWitness) -> bool {
        self.current() == Some(witness)
    }

    pub fn release(&self, witness: ui_gpu::RasterTextureWitness) -> Result<(), &'static str> {
        let mut current = self.0.current.lock().map_err(|_| "raster operation authority was poisoned")?;
        if *current != Some(witness) {
            return Err("raster operation authority was stale or duplicated");
        }
        *current = None;
        Ok(())
    }
}

pub type RuntimeCompletion = runtime_mailbox_core::Completion<RuntimeApply>;
pub type RuntimeCompletionQueue = runtime_mailbox_core::BoundedCompletionQueue<RuntimeApply, RUNTIME_COMPLETION_CAPACITY>;

/// 📮️ A completion returned through a reservation the submitting turn already took: the revision was
/// minted when the work was submitted, it never needs the interaction state to apply, and whether it
/// CARRIES that state home is read off the apply itself — the one owner of that rule
/// ([`RuntimeApply::restores_interaction`]), so no call site can disagree with the checkout ledger.
pub fn returned_completion(revision: u64, apply: RuntimeApply) -> RuntimeCompletion {
    RuntimeCompletion { key: None, revision, requires_interaction: false, restores_interaction: apply.restores_interaction(), apply }
}


/// 📮️ Publishes one completion and its scene invalidation as ONE observation. The invalidation is
/// written while the mailbox lock is still held, so a reader that takes that lock can never see a
/// ready completion under the scene revision it was already built against.
///
/// 🐛️ ticket 26/09/17/WGPU-RENDERER-REACT-PARITY wave 2–6 integration: `mark_scene_changed` used to
/// run AFTER `drop(queue)`, and a reader landing in that window read `ready.len() == 1` beside the
/// OLD witness — a completion the frame it is about to present does not know it must rebuild for.
/// `mark_scene_changed` is a single `fetch_add` on an `AtomicU64`, so it adds no lock order.
fn enqueue_runtime_completion(completions: &Mutex<RuntimeCompletionQueue>, presentation: &RuntimePresentationAuthority, waker: &Mutex<Option<RuntimeHostWaker>>, completion: RuntimeCompletion) -> bool {
    let mut queue = completions.lock().expect("runtime completion mailbox lock");
    if !queue.enqueue(completion) {
        return false;
    }
    presentation.mark_scene_changed();
    drop(queue);
    #[cfg(all(any(test, feature = "testkit"), not(target_arch = "wasm32")))]
    runtime_publication_tests::pause_after_completion_publication();
    if let Some(waker) = waker.lock().expect("runtime completion waker lock").as_ref() {
        waker();
    }
    true
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
enum StagedReferenceImage {
    Writing { token: WorldAssetRequestToken, url: String, descriptor: SceneRasterDescriptor, writer: SceneRasterWriter },
    Ready { token: WorldAssetRequestToken, url: String, lease: SceneRasterLease },
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub struct StagedReferenceImageAuthority {
    entries: std::collections::VecDeque<StagedReferenceImage>,
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl StagedReferenceImageAuthority {
    pub const BUSY: u8 = 0;
    pub const WRITER: u8 = 1;
    pub const REUSED: u8 = 2;

    pub fn new() -> Self {
        Self { entries: std::collections::VecDeque::with_capacity(4) }
    }

    pub fn begin(&mut self, pool: &SceneRasterPool, token: WorldAssetRequestToken, url: String, descriptor: SceneRasterDescriptor, owner: u64) -> Result<u8, &'static str> {
        if let Some(entry) = self.entries.iter().find(|entry| match entry {
            StagedReferenceImage::Writing { token: candidate, url: candidate_url, descriptor: candidate_descriptor, .. } => *candidate == token && candidate_url == &url && *candidate_descriptor == descriptor,
            StagedReferenceImage::Ready { token: candidate, url: candidate_url, lease } => *candidate == token && candidate_url == &url && lease.identity().descriptor() == descriptor,
        }) {
            return Ok(match entry {
                StagedReferenceImage::Writing { .. } => Self::BUSY,
                StagedReferenceImage::Ready { .. } => Self::REUSED,
            });
        }
        if self.entries.len() >= 4 {
            return Ok(Self::BUSY);
        }
        let (entry, mode) = match pool.begin(descriptor, owner, SceneRasterWriteMode::Streamed) {
            SceneRasterBegin::Writer(writer) => (StagedReferenceImage::Writing { token, url, descriptor, writer }, Self::WRITER),
            SceneRasterBegin::Reused(lease) => (StagedReferenceImage::Ready { token, url, lease }, Self::REUSED),
            SceneRasterBegin::Backpressure(_) => return Ok(Self::BUSY),
            SceneRasterBegin::Refused(fault) => return Err(fault),
        };
        self.entries.push_back(entry);
        Ok(mode)
    }

    pub fn push(&mut self, pool: &SceneRasterPool, token: WorldAssetRequestToken, offset: usize, bytes: &[u8]) -> Result<bool, &'static str> {
        let Some(entry) = self.entries.iter_mut().find(|entry| matches!(entry, StagedReferenceImage::Writing { token: candidate, .. } if *candidate == token)) else {
            return Ok(self.entries.iter().any(|entry| matches!(entry, StagedReferenceImage::Ready { token: candidate, .. } if *candidate == token)));
        };
        let StagedReferenceImage::Writing { writer, .. } = entry else { unreachable!("matched writing reference image") };
        if writer.cursor() != offset {
            return Err("reference image row cursor is stale");
        }
        *writer = pool.push(*writer, bytes)?;
        Ok(true)
    }

    pub fn seal(&mut self, pool: &SceneRasterPool, token: WorldAssetRequestToken) -> Result<bool, &'static str> {
        let Some(index) = self.entries.iter().position(|entry| matches!(entry, StagedReferenceImage::Writing { token: candidate, .. } if *candidate == token)) else {
            return Ok(self.entries.iter().any(|entry| matches!(entry, StagedReferenceImage::Ready { token: candidate, .. } if *candidate == token)));
        };
        let Some(StagedReferenceImage::Writing { token, url, writer, .. }) = self.entries.remove(index) else { unreachable!("located writing reference image") };
        match pool.seal(writer) {
            Ok(lease) => {
                self.entries.push_back(StagedReferenceImage::Ready { token, url, lease });
                Ok(true)
            }
            Err(fault) => {
                let _ = pool.cancel(writer);
                Err(fault)
            }
        }
    }

    pub fn take(&mut self, token: WorldAssetRequestToken, url: &str) -> Option<SceneRasterLease> {
        let index = self.entries.iter().position(|entry| matches!(entry, StagedReferenceImage::Ready { token: candidate, url: candidate_url, .. } if *candidate == token && candidate_url == url))?;
        let Some(StagedReferenceImage::Ready { lease, .. }) = self.entries.remove(index) else { unreachable!("located ready reference image") };
        Some(lease)
    }

    pub fn restore(&mut self, token: WorldAssetRequestToken, url: String, lease: SceneRasterLease) {
        assert!(self.entries.len() < 4, "a just-taken reference raster restores into its exact stage slot");
        self.entries.push_back(StagedReferenceImage::Ready { token, url, lease });
    }

    pub fn discard(&mut self, pool: &SceneRasterPool, token: WorldAssetRequestToken) {
        let Some(index) = self.entries.iter().position(|entry| match entry {
            StagedReferenceImage::Writing { token: candidate, .. } | StagedReferenceImage::Ready { token: candidate, .. } => *candidate == token,
        }) else {
            return;
        };
        if let Some(StagedReferenceImage::Writing { writer, .. }) = self.entries.remove(index) {
            let _ = pool.cancel(writer);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub enum NativeReferenceDecodeOutput {
    Ready(SceneRasterLease),
    Waiting(DecodedReferenceImage),
    Failed,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativeReferenceDecodeCloseStep {
    Busy,
    Pending { released_items: usize, released_bytes: usize },
    Complete,
}

#[cfg(not(target_arch = "wasm32"))]
pub struct NativeReferenceDecodeJob {
    pub token: WorldAssetRequestToken,
    pub url: String,
    pub input: Mutex<Option<Vec<u8>>>,
    pub decoded: Mutex<Option<DecodedReferenceImage>>,
    pub output: Mutex<Option<NativeReferenceDecodeOutput>>,
    pub phase: std::sync::atomic::AtomicU8,
    pub cancelled: AtomicBool,
    #[cfg(any(test, feature = "testkit"))]
    pub worker_barriers: Mutex<Option<(Arc<std::sync::Barrier>, Arc<std::sync::Barrier>)>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeReferenceDecodeJob {
    pub fn new(token: WorldAssetRequestToken, url: String, bytes: Vec<u8>) -> Self {
        Self {
            token,
            url,
            input: Mutex::new(Some(bytes)),
            decoded: Mutex::new(None),
            output: Mutex::new(None),
            phase: std::sync::atomic::AtomicU8::new(0),
            cancelled: AtomicBool::new(false),
            #[cfg(any(test, feature = "testkit"))]
            worker_barriers: Mutex::new(None),
        }
    }

    pub fn try_schedule(self: &Arc<Self>, mailbox: RuntimeMailbox) -> bool {
        if self.phase.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return false;
        }
        let scheduled = self.clone();
        let pool = mailbox.0.scene_raster_pool.clone();
        let job: semio_framework_async::Job =
            Box::new(move || {
                let bytes = scheduled.input.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).take();
                let decoded = scheduled.decoded.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).take().or_else(|| {
                    if scheduled.cancelled.load(Ordering::Acquire) {
                        None
                    } else {
                        bytes.as_deref().and_then(infinite_world::world::decode_reference_image_bytes)
                    }
                });
                #[cfg(any(test, feature = "testkit"))]
                if let Some((entered, release)) = scheduled.worker_barriers.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).as_ref().cloned() {
                    entered.wait();
                    release.wait();
                }
                let mut retained_decoded = None;
                let output = if scheduled.cancelled.load(Ordering::Acquire) {
                    retained_decoded = decoded;
                    NativeReferenceDecodeOutput::Failed
                } else if let Some(decoded) = decoded {
                    let descriptor = SceneRasterDescriptor { width: decoded.width, height: decoded.height, source_digest: decoded.source_digest, source_revision: 1, profile: SceneRasterProfile::ReferenceImageMapNoColorSpace, mesh: None };
                    let owner = decoded.source_digest[0].wrapping_add(decoded.source_digest[1]).max(1);
                    match pool.begin(descriptor, owner, SceneRasterWriteMode::Moved { capacity_bytes: decoded.pixels.capacity() }) {
                        SceneRasterBegin::Reused(lease) => {
                            retained_decoded = Some(decoded);
                            NativeReferenceDecodeOutput::Ready(lease)
                        }
                        SceneRasterBegin::Writer(writer) => {
                            let DecodedReferenceImage { width, height, source_digest, pixels } = decoded;
                            match pool.prepare_moved(writer, pixels).and_then(|prepared| pool.seal_prepared_moved(prepared)) {
                                Ok(lease) => NativeReferenceDecodeOutput::Ready(lease),
                                Err((pixels, _fault)) => {
                                    let _ = pool.cancel(writer);
                                    retained_decoded = Some(DecodedReferenceImage { width, height, source_digest, pixels });
                                    NativeReferenceDecodeOutput::Failed
                                }
                            }
                        }
                        SceneRasterBegin::Backpressure(_) => NativeReferenceDecodeOutput::Waiting(decoded),
                        SceneRasterBegin::Refused(_) => {
                            retained_decoded = Some(decoded);
                            NativeReferenceDecodeOutput::Failed
                        }
                    }
                } else {
                    NativeReferenceDecodeOutput::Failed
                };
                if let Some(bytes) = bytes {
                    let mut retained = scheduled.input.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                    assert!(retained.is_none(), "the native reference worker returns one exact encoded owner");
                    *retained = Some(bytes);
                }
                if let Some(decoded) = retained_decoded {
                    let mut retained = scheduled.decoded.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                    assert!(retained.is_none(), "the native reference worker returns one exact decoded owner");
                    *retained = Some(decoded);
                }
                *scheduled.output.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(output);
                scheduled.phase.store(if scheduled.cancelled.load(Ordering::Acquire) { 3 } else { 6 }, Ordering::Release);
                if let Some(waker) = mailbox.0.waker.lock().expect("runtime completion waker lock").as_ref() {
                    waker();
                }
            });
        match renderer_worker_pool().try_submit(semio_framework_async::Lane::Maintenance, job) {
            Ok(()) => true,
            Err(error) => {
                drop(error.into_job());
                self.phase.store(if self.cancelled.load(Ordering::Acquire) { 3 } else { 0 }, Ordering::Release);
                false
            }
        }
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
        loop {
            let phase = self.phase.load(Ordering::Acquire);
            if !matches!(phase, 0 | 2 | 6) {
                break;
            }
            if self.phase.compare_exchange(phase, 3, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                break;
            }
        }
    }

    pub fn take_output(&self) -> Option<NativeReferenceDecodeOutput> {
        if self.phase.compare_exchange(2, 5, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return None;
        }
        self.output.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).take()
    }

    pub fn rearm(&self, decoded: DecodedReferenceImage) -> bool {
        *self.decoded.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(decoded);
        let ready = !self.cancelled.load(Ordering::Acquire);
        self.phase.store(if ready { 0 } else { 3 }, Ordering::Release);
        ready
    }

    pub fn restore_output(&self, output: NativeReferenceDecodeOutput) {
        *self.output.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(output);
        self.phase.store(if self.cancelled.load(Ordering::Acquire) { 3 } else { 2 }, Ordering::Release);
    }

    pub fn retire_payload_step(&self, maximum_items: usize, maximum_bytes: usize) -> NativeReferenceDecodeCloseStep {
        if maximum_items == 0 {
            return NativeReferenceDecodeCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        {
            let mut decoded = self.decoded.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            if let Some(image) = decoded.as_mut() {
                if !image.pixels.is_empty() {
                    let released_bytes = image.pixels.len().min(maximum_bytes);
                    if released_bytes == 0 {
                        return NativeReferenceDecodeCloseStep::Pending { released_items: 0, released_bytes: 0 };
                    }
                    image.pixels.truncate(image.pixels.len() - released_bytes);
                    return NativeReferenceDecodeCloseStep::Pending { released_items: 0, released_bytes };
                }
                *decoded = None;
                return NativeReferenceDecodeCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
        }
        {
            let mut input = self.input.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            if let Some(bytes) = input.as_mut() {
                if !bytes.is_empty() {
                    let released_bytes = bytes.len().min(maximum_bytes);
                    if released_bytes == 0 {
                        return NativeReferenceDecodeCloseStep::Pending { released_items: 0, released_bytes: 0 };
                    }
                    bytes.truncate(bytes.len() - released_bytes);
                    return NativeReferenceDecodeCloseStep::Pending { released_items: 0, released_bytes };
                }
                *input = None;
                return NativeReferenceDecodeCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
        }
        NativeReferenceDecodeCloseStep::Complete
    }

    pub fn close_step(&self, maximum_items: usize, maximum_bytes: usize) -> NativeReferenceDecodeCloseStep {
        self.cancel();
        match self.phase.load(Ordering::Acquire) {
            1 | 5 => return NativeReferenceDecodeCloseStep::Busy,
            4 => return NativeReferenceDecodeCloseStep::Complete,
            0 | 2 => return NativeReferenceDecodeCloseStep::Busy,
            3 => {}
            _ => return NativeReferenceDecodeCloseStep::Busy,
        }
        if maximum_items == 0 {
            return NativeReferenceDecodeCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if let Some(output) = self.output.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).take() {
            match output {
                NativeReferenceDecodeOutput::Waiting(decoded) => {
                    let mut retained = self.decoded.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                    assert!(retained.is_none(), "one native reference decode owns at most one decoded image");
                    *retained = Some(decoded);
                    return NativeReferenceDecodeCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                NativeReferenceDecodeOutput::Ready(lease) => {
                    let _ = lease.release();
                }
                NativeReferenceDecodeOutput::Failed => {}
            }
            return NativeReferenceDecodeCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        let step = self.retire_payload_step(maximum_items, maximum_bytes);
        if !matches!(step, NativeReferenceDecodeCloseStep::Complete) {
            return step;
        }
        self.phase.store(4, Ordering::Release);
        NativeReferenceDecodeCloseStep::Complete
    }
}

pub struct NativeAssetHandoff {
    pub fetch: Option<RendererAssetFetchOwner>,
    pub payload: Option<semio_framework_job::RetainedJobPayload>,
    pub seal: bool,
    pub reject: bool,
}

#[cfg(not(target_arch = "wasm32"))]
pub struct NativeAssetTransportLease {
    pub surface_token: Option<AdmittedSurfaceToken>,
    pub request_token: WorldAssetRequestToken,
    pub cancel: semio_framework_async::CancelToken,
    pub cancellation: semio_framework_os_services::HttpBodyCancellationHandle,
    pub cancellation_step: Option<semio_framework_os_services::HttpBodyCancellationStep>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NativeAssetTransportCloseStep {
    Busy,
    Pending(semio_framework_os_services::HttpBodyCancellationStep),
    Terminal,
}

#[cfg(not(target_arch = "wasm32"))]
pub struct NativeAssetTransportGuard<R> {
    pub mailbox: RuntimeMailbox<R>,
    pub surface_token: Option<AdmittedSurfaceToken>,
    pub request_token: WorldAssetRequestToken,
    pub armed: bool,
}

#[cfg(all(any(test, feature = "testkit"), not(target_arch = "wasm32")))]
pub struct NativeAssetPageTestBarrier {
    pub surface_token: AdmittedSurfaceToken,
    pub entered: AtomicBool,
    pub release: AtomicBool,
}

#[cfg(not(target_arch = "wasm32"))]
impl<R> Drop for NativeAssetTransportGuard<R> {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        let mut slot = self.mailbox.0.native_asset_transport.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if slot.as_ref().is_some_and(|lease| lease.surface_token == self.surface_token && lease.request_token == self.request_token) {
            *slot = None;
        }
        drop(slot);
        if let Some(waker) = self.mailbox.0.waker.lock().expect("runtime completion waker lock").as_ref() {
            waker();
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub enum NativeAssetStreamFailure {
    Cancelled,
    CancelledPage { payload: semio_framework_job::RetainedJobPayload },
    Unavailable(String),
    Fault(String),
    Page { detail: &'static str, payload: semio_framework_job::RetainedJobPayload },
}

#[cfg(not(target_arch = "wasm32"))]
impl From<String> for NativeAssetStreamFailure {
    fn from(detail: String) -> Self {
        Self::Fault(detail)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<&str> for NativeAssetStreamFailure {
    fn from(detail: &str) -> Self {
        Self::Fault(detail.to_string())
    }
}

pub struct RuntimeMailboxInner<R> {
    pub runtime: Mutex<R>,
    pub presentation_authority: RuntimePresentationAuthority,
    pub raster_operation_authority: RuntimeRasterOperationAuthority,
    pub world_cursor_wake: infinite_world::world::WorldCursorWakeAuthority,
    pub completions: Mutex<RuntimeCompletionQueue>,
    pub waker: Mutex<Option<RuntimeHostWaker>>,
    pub next_revision: AtomicU64,
    pub applied_revisions: Mutex<std::collections::HashMap<&'static str, u64>>,
    pub frame_inputs: Mutex<FrameBuildInputs>,
    pub retained_control_deadline_us: AtomicU64,
    pub shell_clock_deadline_us: AtomicU64,
    pub frame_fault: Mutex<Option<String>>,
    #[cfg(not(target_arch = "wasm32"))]
    pub frame_maintenance: FrameMaintenanceAuthority,
    #[cfg(not(target_arch = "wasm32"))]
    pub frame_maintenance_executions: Arc<FrameExecutionRegistry<FrameMaintenanceOwner>>,
    #[cfg(not(target_arch = "wasm32"))]
    pub frame_deferred_executions: Arc<FrameExecutionRegistry<FrameDeferredExecutionOwner>>,
    pub world3d_close_cursor: Mutex<usize>,
    pub world3d_asset_cursor: Mutex<usize>,
    pub world3d_asset_decode_cursor: Mutex<usize>,
    pub asset_probe: Mutex<Option<RendererAssetProbe>>,
    pub asset_decode_session: Mutex<Option<RendererAssetDecodeSession>>,
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub staged_reference_images: Mutex<StagedReferenceImageAuthority>,
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub scene_raster_pool: SceneRasterPool,
    #[cfg(not(target_arch = "wasm32"))]
    pub native_reference_decode: Mutex<Option<Arc<NativeReferenceDecodeJob>>>,
    #[cfg(not(target_arch = "wasm32"))]
    pub native_asset_transport: Mutex<Option<NativeAssetTransportLease>>,
    #[cfg(all(any(test, feature = "testkit"), not(target_arch = "wasm32")))]
    pub native_asset_page_barrier: Mutex<Option<Arc<NativeAssetPageTestBarrier>>>,
    pub native_asset_fetching: AtomicBool,
    pub native_asset_handoff: Mutex<Option<NativeAssetHandoff>>,
    #[cfg(not(target_arch = "wasm32"))]
    pub native_asset_http: Arc<semio_framework_os_services::HttpPool>,
    #[cfg(not(target_arch = "wasm32"))]
    pub native_asset_https: Arc<semio_framework_os_services::HttpPool>,
    #[cfg(not(target_arch = "wasm32"))]
    pub native_asset_http_runtime: Arc<semio_framework_os_services::TokioHostRuntime>,
    #[cfg(not(target_arch = "wasm32"))]
    pub native_asset_http_scope: semio_framework_async::ScopeHandle,
    #[cfg(not(target_arch = "wasm32"))]
    pub native_asset_http_cancel: semio_framework_async::CancelToken,
}

impl<R> RuntimeMailboxInner<R> {
    pub fn try_lock(&self) -> std::sync::TryLockResult<std::sync::MutexGuard<'_, R>> {
        self.runtime.try_lock()
    }

    pub fn completion(&self, key: Option<&'static str>, requires_interaction: bool, apply: RuntimeApply) -> RuntimeCompletion {
        RuntimeCompletion { key, revision: self.next_revision.fetch_add(1, Ordering::Relaxed), requires_interaction, restores_interaction: apply.restores_interaction(), apply }
    }

    pub fn enqueue(&self, key: Option<&'static str>, requires_interaction: bool, apply: RuntimeApply) -> bool {
        let completion = self.completion(key, requires_interaction, apply);
        enqueue_runtime_completion(&self.completions, &self.presentation_authority, &self.waker, completion)
    }

    pub fn finish(&self, completion: RuntimeCompletion) {
        let mut queue = self.completions.lock().expect("runtime completion mailbox lock");
        queue.finish(completion);
        drop(queue);
        self.presentation_authority.mark_scene_changed();
        if let Some(waker) = self.waker.lock().expect("runtime completion waker lock").as_ref() {
            waker();
        }
    }
}

/// 🔌️ Everything the runtime mailbox needs of the runtime it guards (plan section 5.3, inversion I2): the interaction checkout, the two pending
/// frame cursors and the resize entry. The mailbox is generic over it so the transport crate never names the application runtime above it.
pub trait MailboxRuntime: Send + 'static {
    fn interaction(&self) -> Option<&AppInteractionState>;
    fn interaction_mut(&mut self) -> Option<&mut AppInteractionState>;
    fn interaction_available(&self) -> bool;
    fn check_out_interaction(&mut self, site: &'static str) -> Option<AppInteractionState>;
    fn return_interaction(&mut self, interaction: AppInteractionState);
    fn resize(&mut self, logical_width: f32, logical_height: f32, scale_factor: f32);
    fn pending_frame_deferred(&self) -> &Option<FrameDeferredCursor>;
    fn pending_frame_deferred_mut(&mut self) -> &mut Option<FrameDeferredCursor>;
    fn frame_actions_mut(&mut self) -> &mut FrameActionOwners;
    #[cfg(not(target_arch = "wasm32"))]
    fn pending_frame_maintenance_refusal(&self) -> &Option<FrameMaintenanceRefusal>;
    #[cfg(not(target_arch = "wasm32"))]
    fn pending_frame_maintenance_refusal_mut(&mut self) -> &mut Option<FrameMaintenanceRefusal>;
}

/// 🪪️ Submits owned work without holding the original runtime or a mutex guard across suspension.
pub type RuntimeHandle<R> = std::sync::Weak<RuntimeMailboxInner<R>>;

pub struct RuntimeMailbox<R>(pub Arc<RuntimeMailboxInner<R>>);

impl<R> Clone for RuntimeMailbox<R> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct FrameMaintenanceExecutionEnvelope<R> {
    pub mailbox: RuntimeMailbox<R>,
    pub registry: Arc<FrameExecutionRegistry<FrameMaintenanceOwner>>,
    pub generation: u64,
    pub revision: u64,
    pub armed: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl<R> FrameMaintenanceExecutionEnvelope<R> {
    pub fn new(mailbox: RuntimeMailbox<R>, registry: Arc<FrameExecutionRegistry<FrameMaintenanceOwner>>, generation: u64, revision: u64) -> Self {
        Self { mailbox, registry, generation, revision, armed: true }
    }

    pub fn begin(mut self) -> Option<FrameMaintenanceExecutionGuard<R>> {
        let owner_cell = self.registry.try_begin(self.generation)?;
        let owner = owner_cell.try_take()?;
        self.armed = false;
        Some(FrameMaintenanceExecutionGuard { mailbox: self.mailbox.clone(), registry: self.registry.clone(), owner_cell, owner: Some(owner), generation: self.generation, revision: self.revision, completed: false })
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<R> Drop for FrameMaintenanceExecutionEnvelope<R> {
    fn drop(&mut self) {
        if self.armed && self.registry.abandon(self.generation) {
            self.mailbox.0.finish(returned_completion(self.revision, RuntimeApply::ResumeFrameDeferred { interaction: None, cursor: None }));
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct FrameMaintenanceExecutionGuard<R> {
    mailbox: RuntimeMailbox<R>,
    registry: Arc<FrameExecutionRegistry<FrameMaintenanceOwner>>,
    owner_cell: Arc<FrameExecutionOwnerCell<FrameMaintenanceOwner>>,
    owner: Option<FrameMaintenanceOwner>,
    generation: u64,
    revision: u64,
    completed: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl<R> FrameMaintenanceExecutionGuard<R> {
    fn owner_mut(&mut self) -> Option<&mut FrameMaintenanceOwner> {
        self.owner.as_mut()
    }

    fn finish(mut self) -> bool {
        let Some(owner) = self.owner.as_mut() else { return false };
        let Some((interaction, cursor)) = owner.take_pair() else { return false };
        if !self.registry.complete(self.generation) {
            owner.interaction = Some(interaction);
            owner.cursor = Some(cursor);
            return false;
        }
        self.completed = true;
        self.mailbox.0.finish(returned_completion(self.revision, RuntimeApply::ResumeFrameDeferred { interaction: Some(interaction), cursor: Some(cursor) }));
        true
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<R> Drop for FrameMaintenanceExecutionGuard<R> {
    fn drop(&mut self) {
        if self.completed {
            return;
        }
        if let Some(owner) = self.owner.take() {
            self.owner_cell.restore_taken(owner);
        }
        if self.registry.abandon(self.generation) {
            self.mailbox.0.finish(returned_completion(self.revision, RuntimeApply::ResumeFrameDeferred { interaction: None, cursor: None }));
        }
    }
}

pub struct RuntimeDispatchReturn<R>{mailbox:RuntimeMailbox<R>,revision:u64,cursor:Option<RuntimeDispatchCursor>,interaction:Option<AppInteractionState>}
impl<R> RuntimeDispatchReturn<R>{
    pub async fn run(mut self){
        let cursor=self.cursor.as_mut().expect("original dispatch cursor remains owned");let event=cursor.inflight.as_mut().expect("original in-flight event remains owned");let interaction=self.interaction.as_mut().expect("original dispatch interaction remains owned");
        let _latency=frame_latency::FrameLatencyTimer::start(frame_latency::FrameLatencyAuthority::renderer_frame(cursor.generation),frame_latency::FrameLatencyStage::DispatchApply,1);dispatch_normalized_event(interaction,event).await;self.complete();
    }
    fn complete(mut self){self.mailbox.0.finish(returned_completion(self.revision,RuntimeApply::ResumeDispatch{interaction:self.interaction.take(),cursor:self.cursor.take()}));}
}
impl<R> Drop for RuntimeDispatchReturn<R>{fn drop(&mut self){if self.cursor.is_some(){self.mailbox.0.finish(returned_completion(self.revision,RuntimeApply::ResumeDispatch{interaction:self.interaction.take(),cursor:self.cursor.take()}));}}}

#[cfg(not(target_arch = "wasm32"))]
pub struct FrameDeferredExecutionEnvelope<R> {
    pub mailbox: RuntimeMailbox<R>,
    pub registry: Arc<FrameExecutionRegistry<FrameDeferredExecutionOwner>>,
    pub generation: u64,
    pub revision: u64,
    pub armed: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl<R> FrameDeferredExecutionEnvelope<R> {
    pub fn new(mailbox: RuntimeMailbox<R>, registry: Arc<FrameExecutionRegistry<FrameDeferredExecutionOwner>>, generation: u64, revision: u64) -> Self {
        Self { mailbox, registry, generation, revision, armed: true }
    }

    pub fn begin(mut self) -> Option<FrameDeferredExecutionGuard<R>> {
        let owner_cell = self.registry.try_begin(self.generation)?;
        let owner = owner_cell.try_take()?;
        self.armed = false;
        Some(FrameDeferredExecutionGuard { mailbox: self.mailbox.clone(), registry: self.registry.clone(), owner_cell, owner: Some(owner), generation: self.generation, revision: self.revision, completed: false })
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<R> Drop for FrameDeferredExecutionEnvelope<R> {
    fn drop(&mut self) {
        if self.armed && self.registry.abandon(self.generation) {
            self.mailbox.0.finish(returned_completion(self.revision, RuntimeApply::ResumeFrameDeferred { interaction: None, cursor: None }));
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct FrameDeferredExecutionGuard<R> {
    mailbox: RuntimeMailbox<R>,
    registry: Arc<FrameExecutionRegistry<FrameDeferredExecutionOwner>>,
    owner_cell: Arc<FrameExecutionOwnerCell<FrameDeferredExecutionOwner>>,
    owner: Option<FrameDeferredExecutionOwner>,
    generation: u64,
    revision: u64,
    completed: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl<R> FrameDeferredExecutionGuard<R> {
    fn owner_mut(&mut self) -> Option<&mut FrameDeferredExecutionOwner> {
        self.owner.as_mut()
    }

    fn finish(mut self) -> bool {
        let Some(owner) = self.owner.as_mut() else { return false };
        let Some((interaction, cursor)) = owner.take_pair() else { return false };
        if !self.registry.complete(self.generation) {
            owner.interaction = Some(interaction);
            owner.cursor = Some(cursor);
            return false;
        }
        self.completed = true;
        self.mailbox.0.finish(returned_completion(self.revision, RuntimeApply::ResumeFrameDeferred { interaction: Some(interaction), cursor: Some(cursor) }));
        true
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<R> Drop for FrameDeferredExecutionGuard<R> {
    fn drop(&mut self) {
        if self.completed {
            return;
        }
        if let Some(owner) = self.owner.take() {
            self.owner_cell.restore_taken(owner);
        }
        if self.registry.abandon(self.generation) {
            self.mailbox.0.finish(returned_completion(self.revision, RuntimeApply::ResumeFrameDeferred { interaction: None, cursor: None }));
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresenterInteractionStep {
    Ready,
    Restored,
    Waiting { site: &'static str, request: &'static str },
    RuntimeLock,
    Abandoned,
}
//#endregion 📮️RuntimeMailbox

pub use semio_framework_os_renderer_wgpu_host_base::RuntimeHostWaker;

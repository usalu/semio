//! 🔁️ Asset decode jobs: asset probe, decode job and session, prefix validators and the renderer worker pool with its I/O slots.
use super::*;

//#region 📡️RendererAssetAuthority
#[derive(Clone, Copy)]
pub enum RendererAssetProbePhase {
    Reading,
    Parsing,
    Semantic,
    Materializing,
    Ready,
    Closing,
}

pub struct RendererAssetProbe {
    pub owner: Option<RendererAssetFetchOwner>,
    prefix: [u8; RENDERER_ASSET_PROBE_BYTES],
    prefix_len: u8,
    pub observed_bytes: usize,
    page_cursor: RendererAssetPageCursor,
    page_index: RendererAssetPageIndex,
    pub format: Option<RendererAssetFormatCursor>,
    glb_schema: Option<GlbSchemaCursor>,
    pub glb_materialize: Option<GlbMaterializeCursor>,
    pub phase: RendererAssetProbePhase,
    pub rejection: Option<(&'static str, WorldAssetRequestKind, String)>,
}

/// 🗑️ What a probe step answers. `Reject` and `Fault` both close the response; they differ in WHOSE
/// invariant broke. `Reject` is a verdict about the RESPONSE BYTES — a decoder refused them (wrong
/// format for the kind, truncated structure, dimensions over
/// [`RENDERER_ASSET_PIXEL_BYTES`]) — so exactly one asset is missing and the frame carries on, the way
/// React's loaders leave one texture unpainted. `Fault` is a verdict about the RENDERER — a page
/// cursor lost ownership, a sealed byte witness disagreed, a mesh publication generation was stale —
/// and those quarantine the surface, because nothing downstream can be trusted after them.
pub enum RendererAssetProbeStep {
    Pending,
    Ready,
    Reject(&'static str),
    Fault(&'static str),
}

impl RendererAssetProbe {
    pub fn new(owner: RendererAssetFetchOwner) -> Self {
        Self {
            owner: Some(owner),
            prefix: [0; RENDERER_ASSET_PROBE_BYTES],
            prefix_len: 0,
            observed_bytes: 0,
            page_cursor: RendererAssetPageCursor::new(),
            page_index: RendererAssetPageIndex::new(),
            format: None,
            glb_schema: None,
            glb_materialize: None,
            phase: RendererAssetProbePhase::Reading,
            rejection: None,
        }
    }

    pub fn owner(&self) -> &RendererAssetFetchOwner {
        self.owner.as_ref().expect("asset probe owns response")
    }

    pub fn owner_mut(&mut self) -> &mut RendererAssetFetchOwner {
        self.owner.as_mut().expect("asset probe owns response")
    }

    pub fn step(&mut self) -> RendererAssetProbeStep {
        match self.phase {
            RendererAssetProbePhase::Reading => {
                let page = match self.owner.as_ref().expect("asset probe owns response").decode_page() {
                    Ok(Some(page)) => page,
                    Ok(None) => return self.finish_probe(),
                    Err(_) => return self.fault("asset response page cursor lost exact ownership"),
                };
                self.observed_bytes = match self.observed_bytes.checked_add(page.bytes().len()) {
                    Some(bytes) => bytes,
                    None => return self.fault("asset response byte count overflowed"),
                };
                if let Err(detail) = self.page_index.admit(page.bytes().len()) {
                    return self.fault(detail);
                }
                let remaining = RENDERER_ASSET_PROBE_BYTES - usize::from(self.prefix_len);
                let take = remaining.min(page.bytes().len());
                if take != 0 {
                    let start = usize::from(self.prefix_len);
                    self.prefix[start..start + take].copy_from_slice(&page.bytes()[..take]);
                    self.prefix_len += take as u8;
                }
                if self.owner_mut().advance_decode_page().is_err() {
                    return self.fault("asset response page cursor could not advance");
                }
                RendererAssetProbeStep::Pending
            }
            RendererAssetProbePhase::Parsing => {
                let owner = self.owner.as_mut().expect("asset probe owns response");
                let block = match self.page_cursor.read_block(owner) {
                    Ok(Some(block)) => block,
                    Ok(None) => {
                        if let Err(detail) = self.format.as_ref().expect("parsing asset owns format cursor").finish() {
                            return self.reject(detail);
                        }
                        if self.owner_mut().rewind_decode_pages().is_err() {
                            return self.fault("asset response could not rewind after retained structure decode");
                        }
                        self.page_cursor = RendererAssetPageCursor::new();
                        if matches!(self.owner().kind(), WorldAssetRequestKind::Glb) {
                            self.glb_schema = Some(GlbSchemaCursor::new());
                            self.phase = RendererAssetProbePhase::Semantic;
                            return RendererAssetProbeStep::Pending;
                        }
                        self.phase = RendererAssetProbePhase::Ready;
                        return RendererAssetProbeStep::Ready;
                    }
                    Err(detail) => return self.fault(detail),
                };
                if let Err(detail) = self.format.as_mut().expect("parsing asset owns format cursor").feed(&block.0[..usize::from(block.1)]) {
                    return self.reject(detail);
                }
                RendererAssetProbeStep::Pending
            }
            RendererAssetProbePhase::Semantic => {
                let mut schema = self.glb_schema.take().expect("semantic GLB probe owns schema cursor");
                let result = schema.step(self.owner_mut());
                match result {
                    Ok(false) => {
                        self.glb_schema = Some(schema);
                        RendererAssetProbeStep::Pending
                    }
                    Ok(true) => {
                        self.glb_materialize = Some(GlbMaterializeCursor::new(schema.output));
                        self.phase = RendererAssetProbePhase::Materializing;
                        RendererAssetProbeStep::Pending
                    }
                    Err(detail) => self.reject(detail),
                }
            }
            RendererAssetProbePhase::Materializing => {
                let mut materialize = self.glb_materialize.take().expect("GLB probe owns materializer");
                let result = materialize.step(self.owner.as_ref().expect("asset probe owns response"), &self.page_index);
                self.glb_materialize = Some(materialize);
                match result {
                    Ok(false) => RendererAssetProbeStep::Pending,
                    Ok(true) => {
                        self.phase = RendererAssetProbePhase::Ready;
                        RendererAssetProbeStep::Ready
                    }
                    Err(detail) => self.fault(detail),
                }
            }
            RendererAssetProbePhase::Ready => RendererAssetProbeStep::Ready,
            RendererAssetProbePhase::Closing => RendererAssetProbeStep::Pending,
        }
    }

    fn finish_probe(&mut self) -> RendererAssetProbeStep {
        if self.observed_bytes != self.owner().received_bytes() {
            return self.fault("asset response byte witness did not match its sealed claim");
        }
        let prefix_bytes = self.prefix;
        let prefix = &prefix_bytes[..usize::from(self.prefix_len)];
        let valid = match self.owner().kind() {
            WorldAssetRequestKind::Glb => {
                prefix.len() >= 12
                    && &prefix[..4] == b"glTF"
                    && u32::from_le_bytes(prefix[4..8].try_into().expect("fixed GLB version bytes")) == 2
                    && usize::try_from(u32::from_le_bytes(prefix[8..12].try_into().expect("fixed GLB length bytes"))).ok() == Some(self.observed_bytes)
            }
            WorldAssetRequestKind::ReferenceImage => renderer_asset_reference_prefix_is_valid(prefix),
            WorldAssetRequestKind::UiImage { .. } | WorldAssetRequestKind::MapTile { vector: false, .. } => renderer_asset_ui_image_prefix_is_valid(prefix),
            WorldAssetRequestKind::MapTile { vector: true, .. } | WorldAssetRequestKind::Terrain { .. } => self.observed_bytes != 0,
        };
        if !valid {
            return self.reject("asset response format probe rejected malformed input");
        }
        if self.owner_mut().rewind_decode_pages().is_err() {
            return self.fault("asset response could not rewind for its retained decoder");
        }
        self.format = match RendererAssetFormatCursor::new(self.owner().kind(), prefix, self.observed_bytes) {
            Ok(cursor) => Some(cursor),
            Err(detail) => return self.reject(detail),
        };
        self.page_cursor = RendererAssetPageCursor::new();
        self.phase = RendererAssetProbePhase::Parsing;
        RendererAssetProbeStep::Pending
    }

    pub fn fault(&mut self, detail: &'static str) -> RendererAssetProbeStep {
        self.owner_mut().begin_close();
        self.phase = RendererAssetProbePhase::Closing;
        RendererAssetProbeStep::Fault(detail)
    }

    /// 🗑️ Refuses THIS response and keeps the frame: the kind and url are captured before
    /// `begin_close` clears them, so the lane that requested the asset can record its own miss
    /// (React's `tileMiss`/loader-error shape) instead of re-fetching the same refused bytes forever.
    pub fn reject(&mut self, detail: &'static str) -> RendererAssetProbeStep {
        self.rejection = Some((detail, self.owner().kind(), self.owner().url().to_string()));
        self.owner_mut().begin_close();
        self.phase = RendererAssetProbePhase::Closing;
        RendererAssetProbeStep::Reject(detail)
    }

    pub fn take_rejection(&mut self) -> Option<(&'static str, WorldAssetRequestKind, String)> {
        self.rejection.take()
    }

    pub fn take_ready_mesh_asset(&mut self) -> Option<World3dMeshAsset> {
        (matches!(self.phase, RendererAssetProbePhase::Ready) && matches!(self.owner().kind(), WorldAssetRequestKind::Glb)).then(|| self.glb_materialize.as_mut()?.take_asset()).flatten()
    }

    pub fn restore_ready_mesh_asset(&mut self, asset: World3dMeshAsset) {
        self.glb_materialize.as_mut().expect("ready GLB probe owns its materializer").restore_asset(asset);
    }

    pub fn finish_ready_mesh(&mut self) {
        assert!(
            self.glb_materialize.as_ref().is_some_and(|materialize| materialize.lease.is_none() && materialize.appearance.is_none()),
            "published GLB probe relinquished its paged mesh and appearance owners"
        );
        self.owner_mut().begin_close();
        self.phase = RendererAssetProbePhase::Closing;
    }

    pub fn begin_close(&mut self) {
        if let Some(materialize) = self.glb_materialize.as_mut() {
            materialize.begin_close();
        }
        self.owner_mut().begin_close();
        self.phase = RendererAssetProbePhase::Closing;
    }

    pub fn close_step(&mut self) -> bool {
        self.begin_close();
        if self.glb_materialize.as_mut().is_some_and(|materialize| !materialize.close_step()) {
            return false;
        }
        self.glb_materialize = None;
        self.owner_mut().close_step()
    }

    pub fn take_terminal_owner(&mut self) -> Option<RendererAssetFetchOwner> {
        self.owner.as_ref().is_some_and(|owner| owner.owner().terminal_is_empty()).then(|| self.owner.take().expect("terminal probe owner"))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RendererAssetDecodeResult {
    Ready,
    Rejected(&'static str),
    Cancelled,
    Fault(&'static str),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RendererAssetDecodeTurnKind {
    Idle,
    Pending,
    Published,
    Cancelled,
    Fault,
}

#[derive(serde::Serialize)]
pub struct RendererAssetDecodeTurn {
    pub kind: RendererAssetDecodeTurnKind,
    pub pending: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<&'static str>,
}

/// 🧵️ Owns one response across bounded decoder turns without borrowing the application runtime.
pub struct RendererAssetDecodeJob {
    pub probe: RefCell<Option<RendererAssetProbe>>,
    pub handback: Arc<Mutex<Option<RendererAssetProbe>>>,
    pub cancelled: Arc<AtomicBool>,
    pub result: Option<RendererAssetDecodeResult>,
    pub steps: u64,
    closing: bool,
}

impl RendererAssetDecodeJob {
    pub fn new(probe: RendererAssetProbe, cancelled: Arc<AtomicBool>) -> Self {
        Self { probe: RefCell::new(Some(probe)), handback: Arc::new(Mutex::new(None)), cancelled, result: None, steps: 0, closing: false }
    }

    pub fn take_probe(&self) -> Option<RendererAssetProbe> {
        self.probe.try_borrow_mut().ok()?.take()
    }

    fn complete<'a>(&'a mut self, result: RendererAssetDecodeResult, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        let loan = semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None)?;
        if loan.is_some() { self.result = Some(result); }
        Ok(loan)
    }
}

impl semio_framework_job::InteractiveJob for RendererAssetDecodeJob {
    fn step<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        let grant = context.retained_grant();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(None); }
        if self.cancelled.load(Ordering::Acquire) || context.is_cancelled() {
            return self.complete(RendererAssetDecodeResult::Cancelled, context).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if let Some(result) = self.result { return self.complete(result, context).map_err(semio_framework_job::OriginalTurnFailure::from); }
        if context.should_yield() { return Ok(None); }
        let Some(steps) = self.steps.checked_add(1) else {
            return self.complete(RendererAssetDecodeResult::Fault("asset decode step sequence exhausted"), context).map_err(semio_framework_job::OriginalTurnFailure::from);
        };
        let Some(probe) = self.probe.get_mut().as_mut() else {
            return self.complete(RendererAssetDecodeResult::Fault("asset decode lost its exact response owner"), context).map_err(semio_framework_job::OriginalTurnFailure::from);
        };
        let result = probe.step();
        self.steps = steps;
        context.consume_fuel(1);
        self.result = match result {
            RendererAssetProbeStep::Pending => None,
            RendererAssetProbeStep::Ready => Some(RendererAssetDecodeResult::Ready),
            RendererAssetProbeStep::Reject(detail) => Some(RendererAssetDecodeResult::Rejected(detail)),
            RendererAssetProbeStep::Fault(detail) => Some(RendererAssetDecodeResult::Fault(detail)),
        };
        Ok(None)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        if self.result.is_none() { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "decoder completion requires its original held typed result")); }
        descriptor.complete(None, None)
    }

    fn begin_close(&mut self) {
        self.closing = true;
        self.result = None;
    }

    fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_job::InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> {
        use semio_framework_job::InteractiveJobCloseStep;
        self.begin_close();
        let empty = semio_framework_job::RetainedCloneProgress::default();
        if self.probe.get_mut().is_none() { return Ok(InteractiveJobCloseStep::Complete { progress: empty }); }
        let grant = cx.retained_grant();
        if cx.should_yield() || grant.maximum_items == 0 { return Ok(InteractiveJobCloseStep::Pending { progress: empty }); }
        if grant.maximum_depth == 0 { return Err(semio_framework_job::OriginalTurnFailure::debit_original(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "original renderer asset probe close requires admitted depth"), cx)); }
        let mut handback = match self.handback.try_lock() {
            Ok(handback) => handback,
            Err(std::sync::TryLockError::WouldBlock) => return Ok(InteractiveJobCloseStep::Blocked),
            Err(std::sync::TryLockError::Poisoned(_)) => return Err(semio_framework_job::OriginalTurnFailure::debit_original(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original renderer asset probe handback lock is poisoned"), cx)),
        };
        if handback.is_some() { return Ok(InteractiveJobCloseStep::Blocked); }
        let progress = semio_framework_job::RetainedCloneProgress { copied_items: 1, ..empty };
        cx.consume_retained(progress)?;
        *handback = self.probe.get_mut().take();
        cx.consume_fuel(1);
        Ok(InteractiveJobCloseStep::Pending { progress })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_close_capacity_byte_demand(&self, _: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        let probe = self.probe.try_borrow().map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "decoder original response is borrowed"))?;
        Ok(usize::from(probe.is_some() || !self.closing))
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.result.is_none() && self.probe.borrow().is_none()
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct RendererAssetDecodeWake(RuntimeHostWaker);

#[cfg(not(target_arch = "wasm32"))]
impl std::task::Wake for RendererAssetDecodeWake {
    fn wake(self: Arc<Self>) {
        (self.0)();
    }
    fn wake_by_ref(self: &Arc<Self>) {
        (self.0)();
    }
}

/// 🎟️ The original authority one asset decode runs under: the owner's identity, cancellation alias, clock and per-step budget, lent unchanged.
pub struct RendererAssetDecodeInvocation {
    params: semio_framework_job_worker_authority::BatchJobParams,
}

impl RendererAssetDecodeInvocation {
    pub fn admit(cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<Self>, semio_framework_job::OriginalTurnFailure> {
        let Some(now) = cx.now_us() else {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "original asset decode admission requires its initiating clock").into());
        };
        let Some(cancel) = cx.admit_original_cancel_alias()? else { return Ok(None) };
        Ok(Some(Self {
            params: semio_framework_job_worker_authority::BatchJobParams {
                operation: cx.operation(),
                generation: cx.generation(),
                cancel,
                now_us: cx.clock_source(),
                config: semio_framework_job_worker_authority::BatchDriveConfig {
                    retained: cx.retained_grant(),
                    site: "renderer_asset_decode",
                    stage: semio_framework_job::InteractiveStage::InteractiveStep,
                    fuel_per_step: cx.fuel_remaining(),
                    step_budget_us: cx.deadline_us().saturating_sub(now),
                },
            },
        }))
    }
}

/// 📬️ Retains the exact decoder ticket and all rejected or terminal response handbacks.
pub struct RendererAssetDecodeSession {
    job: Option<RendererAssetDecodeJob>,
    params: Option<semio_framework_job_worker_authority::BatchJobParams>,
    params_retirement: Option<semio_framework_async::CancelTokenRetirement>,
    pub session: Option<semio_framework_job_worker::WorkerJobSession<RendererAssetDecodeJob>>,
    ticket: Option<semio_framework_job_worker::WorkerJobTicket>,
    pub handback: Arc<Mutex<Option<RendererAssetProbe>>>,
    pub boundary: Option<(RendererAssetProbe, Option<RendererAssetDecodeResult>)>,
    pub cancelled: Arc<AtomicBool>,
    pub surface_token: Option<AdmittedSurfaceToken>,
    pub request_token: WorldAssetRequestToken,
    pub fault: Option<&'static str>,
    #[cfg(not(target_arch = "wasm32"))]
    pub waker: Option<std::task::Waker>,
}

impl RendererAssetDecodeSession {
    pub fn new(probe: RendererAssetProbe, wake: Option<RuntimeHostWaker>, invocation: RendererAssetDecodeInvocation) -> Self {
        let surface_token = match probe.owner() {
            RendererAssetFetchOwner::World { surface_token, .. } => Some(*surface_token),
            RendererAssetFetchOwner::Shared(_) => None,
        };
        let request_token = probe.owner().owner().token();
        let cancelled = Arc::new(AtomicBool::new(false));
        let job = RendererAssetDecodeJob::new(probe, cancelled.clone());
        let handback = job.handback.clone();
        #[cfg(target_arch = "wasm32")]
        let _ = wake;
        Self {
            job: Some(job),
            params: Some(invocation.params),
            params_retirement: None,
            session: None,
            ticket: None,
            handback,
            boundary: None,
            cancelled,
            surface_token,
            request_token,
            fault: None,
            #[cfg(not(target_arch = "wasm32"))]
            waker: wake.map(|wake| std::task::Waker::from(Arc::new(RendererAssetDecodeWake(wake)))),
        }
    }

    pub fn cancel(&self) {
        if !self.cancelled.swap(true, Ordering::AcqRel) {
            self.wake();
        }
    }

    pub fn wake(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(waker) = self.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    pub fn pump_one(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        use semio_framework_job::RetainedCloneProgress;
        use semio_framework_job_worker::{WorkerJobDemandError, WorkerJobPoll};
        if self.boundary.is_none() {
            if let Ok(mut handback) = self.handback.try_lock() {
                self.boundary = handback.take().map(|probe| (probe, Some(self.fault.map_or(RendererAssetDecodeResult::Cancelled, RendererAssetDecodeResult::Fault))));
            }
        }
        if self.session.is_none() {
            return self.pump_unadmitted(cx);
        }
        let grant = cx.retained_grant();
        if cx.retirement_should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(true);
        }
        let Some(session) = self.session.as_mut() else { return Ok(false) };
        let _ = session.take_wake();
        #[cfg(not(target_arch = "wasm32"))]
        if self.waker.as_ref().is_some_and(|waker| session.register_wake(waker).is_err()) {
            return Ok(true);
        }
        match session.poll() {
            WorkerJobPoll::Idle => {
                #[cfg(not(target_arch = "wasm32"))]
                match session.try_submit_step(&renderer_worker_pool(), semio_framework_async::Lane::Maintenance, grant) {
                    Ok(ticket) => {
                        self.ticket = Some(ticket);
                        return Ok(false);
                    }
                    Err(semio_framework_job_worker::WorkerJobSubmitFault::SequenceExhausted) => {
                        self.fault = Some("asset decoder session sequence exhausted");
                        let _ = session.begin_close();
                    }
                    Err(_) => {}
                }
                #[cfg(target_arch = "wasm32")]
                match session.try_step_on_worker(grant) {
                    Ok((ticket, _)) => self.ticket = Some(ticket),
                    Err(semio_framework_job_worker::WorkerJobContention::WakeExhausted(_)) => {
                        self.fault = Some("asset decoder session wake sequence exhausted");
                        let _ = session.begin_close();
                    }
                    Err(_) => {}
                }
            }
            WorkerJobPoll::Outcome => {
                let Some(ticket) = self.ticket else {
                    self.fault = Some("asset decoder outcome lost its exact ticket");
                    let _ = session.begin_close();
                    return Ok(true);
                };
                if let Ok(owner) = session.take_outcome(ticket) {
                    self.receive_outcome(owner, cx)?;
                }
            }
            WorkerJobPoll::Terminal => {
                if let Ok(owner) = session.take_terminal() {
                    self.receive_outcome(owner, cx)?;
                }
            }
            WorkerJobPoll::Rejected => {
                if let Ok(rejected) = session.take_rejected() {
                    match rejected.kind() {
                        semio_framework_async::WorkerSubmitErrorKind::Saturated | semio_framework_async::WorkerSubmitErrorKind::Contended => rejected.resume(),
                        _ => {
                            self.fault = Some("asset decoder worker pool refused the exact submission");
                            self.boundary = rejected.job().take_probe().map(|probe| (probe, Some(RendererAssetDecodeResult::Fault("asset decoder worker pool refused the exact submission"))));
                            rejected.begin_close();
                        }
                    }
                }
            }
            WorkerJobPoll::Closing => match session.has_original_cancel_alias_witness(cx.original_cancel_token()) {
                Ok(true) => match session.return_original_cancel_alias_step(cx.original_cancel_token(), grant) {
                    Ok(Some(step)) => {
                        cx.consume_retained(step.progress())?;
                        cx.consume_fuel(step.progress().copied_items as u64);
                    }
                    Ok(None) | Err(WorkerJobDemandError::Contention(_)) => {}
                    Err(WorkerJobDemandError::Refused(error)) => return Err(semio_framework_job::OriginalTurnFailure::debit_original(error, cx)),
                },
                Ok(false) => {
                    session.close_step(cx)?;
                }
                Err(WorkerJobDemandError::Contention(_)) => {}
                Err(WorkerJobDemandError::Refused(error)) => return Err(semio_framework_job::OriginalTurnFailure::debit_original(error, cx)),
            },
            WorkerJobPoll::TerminalEmpty => {
                cx.consume_retained(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
                cx.consume_fuel(1);
                self.session = None;
                self.ticket = None;
            }
            WorkerJobPoll::Submitted | WorkerJobPoll::CheckedOut => return Ok(false),
        }
        Ok(true)
    }

    fn pump_unadmitted(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        use semio_framework_job::{InteractiveJob as _, RetainedCloneProgress};
        if self.job.is_none() && self.params.is_none() && self.params_retirement.is_none() {
            return Ok(false);
        }
        let grant = cx.retained_grant();
        if cx.retirement_should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(true);
        }
        let paid = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        if self.job.is_some() && self.params.is_some() && !self.cancelled.load(Ordering::Acquire) && !cx.is_cancelled() {
            if let Some((session, _posted)) = semio_framework_job_worker::WorkerJobSession::try_admit_owned(&mut self.job, &mut self.params, cx)? {
                self.session = Some(session);
            }
            return Ok(true);
        }
        if let Some(job) = self.job.as_mut() {
            if self.boundary.is_none() {
                self.boundary = job.take_probe().map(|probe| (probe, Some(RendererAssetDecodeResult::Cancelled)));
            }
            job.begin_close();
            if job.terminal_is_empty() {
                cx.consume_retained(paid)?;
                cx.consume_fuel(1);
                self.job = None;
            }
            return Ok(true);
        }
        if let Some(params) = self.params.take() {
            cx.consume_retained(paid)?;
            cx.consume_fuel(1);
            self.params_retirement = Some(semio_framework_async::CancelTokenRetirement::from_token(params.cancel));
            return Ok(true);
        }
        if let Some(owner) = self.params_retirement.as_mut() {
            if !owner.terminal_is_empty() {
                match owner.return_alias_step(cx.original_cancel_token(), grant) {
                    Ok(step) => {
                        cx.consume_retained(step.progress())?;
                        cx.consume_fuel(step.progress().copied_items as u64);
                    }
                    Err(semio_framework_async::CancelTokenRetirementError::Blocked(_)) => {}
                    Err(semio_framework_async::CancelTokenRetirementError::Refused(error)) => return Err(semio_framework_job::OriginalTurnFailure::debit_original(error, cx)),
                }
                return Ok(true);
            }
            cx.consume_retained(paid)?;
            cx.consume_fuel(1);
            self.params_retirement = None;
        }
        Ok(true)
    }

    fn receive_outcome(&mut self, mut owner: semio_framework_job_worker::WorkerJobOutcome<RendererAssetDecodeJob>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
        use semio_framework_job::{JobOutcomeDescriptor, JobOutcomeKind, RetainedCloneStep};
        let Some((issued, progress)) = owner.take_retained_step_receipt() else {
            self.fault = Some("asset decoder step returned no original receipt");
            owner.begin_close();
            return Ok(());
        };
        if !progress.fits(issued) || !progress.fits(cx.retained_grant()) {
            self.fault = Some("asset decoder receipt exceeds its original receiving authority");
            owner.begin_close();
            return Ok(());
        }
        cx.consume_retained(progress)?;
        let kind = owner.outcome_descriptor().map(JobOutcomeDescriptor::kind);
        let mut acknowledged = false;
        for _ in 0..4 {
            let step = owner.acknowledge_outcome(cx.retained_grant());
            cx.consume_retained(step.progress())?;
            cx.consume_fuel(step.progress().copied_items as u64);
            if matches!(step, RetainedCloneStep::Complete(_)) {
                acknowledged = true;
                break;
            }
        }
        if !acknowledged {
            self.fault = Some("asset decoder outcome acknowledgement did not complete");
            owner.begin_close();
            return Ok(());
        }
        if kind == Some(JobOutcomeKind::Yield) {
            self.ticket = None;
            if let Err(owner) = owner.resume() {
                self.fault = Some("asset decoder yield could not resume its exact owner");
                owner.begin_close();
            }
            return Ok(());
        }
        let result = match kind {
            Some(JobOutcomeKind::Complete) => owner.job().result.unwrap_or(RendererAssetDecodeResult::Fault("asset decoder completed without its typed result")),
            Some(JobOutcomeKind::Cancelled) => RendererAssetDecodeResult::Cancelled,
            _ => RendererAssetDecodeResult::Fault("asset decoder worker terminated with a fault"),
        };
        self.boundary = owner.job().take_probe().map(|probe| (probe, Some(result)));
        self.ticket = None;
        owner.begin_close();
        Ok(())
    }

    pub fn take_boundary(&mut self) -> Option<(RendererAssetProbe, Option<RendererAssetDecodeResult>)> {
        if self.session.is_some() {
            return None;
        }
        let (probe, result) = self.boundary.take()?;
        Some((probe, if self.cancelled.load(Ordering::Acquire) && !matches!(result, Some(RendererAssetDecodeResult::Fault(_))) { Some(RendererAssetDecodeResult::Cancelled) } else { result }))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.job.is_none() && self.params.is_none() && self.params_retirement.is_none() && self.session.is_none() && self.boundary.is_none() && self.handback.try_lock().is_ok_and(|owner| owner.is_none())
    }
}

//#endregion 📡️RendererAssetAuthority


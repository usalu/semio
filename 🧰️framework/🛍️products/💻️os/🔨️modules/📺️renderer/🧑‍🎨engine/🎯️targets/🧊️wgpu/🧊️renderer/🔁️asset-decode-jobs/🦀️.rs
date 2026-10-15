//! 🔁️ Asset decode jobs: asset probe, decode job and session, prefix validators and the renderer worker pool with its I/O slots.
use super::*;

//#region 📡️RendererAssetAuthority
#[derive(Clone, Copy)]
pub(crate) enum RendererAssetProbePhase {
    Reading,
    Parsing,
    Semantic,
    Materializing,
    Ready,
    Closing,
}

pub(crate) struct RendererAssetProbe {
    pub(crate) owner: Option<RendererAssetFetchOwner>,
    prefix: [u8; RENDERER_ASSET_PROBE_BYTES],
    prefix_len: u8,
    pub(crate) observed_bytes: usize,
    page_cursor: RendererAssetPageCursor,
    page_index: RendererAssetPageIndex,
    pub(crate) format: Option<RendererAssetFormatCursor>,
    glb_schema: Option<GlbSchemaCursor>,
    pub(crate) glb_materialize: Option<GlbMaterializeCursor>,
    pub(crate) phase: RendererAssetProbePhase,
    pub(crate) rejection: Option<(&'static str, WorldAssetRequestKind, String)>,
}

/// 🗑️ What a probe step answers. `Reject` and `Fault` both close the response; they differ in WHOSE
/// invariant broke. `Reject` is a verdict about the RESPONSE BYTES — a decoder refused them (wrong
/// format for the kind, truncated structure, dimensions over
/// [`RENDERER_ASSET_PIXEL_BYTES`]) — so exactly one asset is missing and the frame carries on, the way
/// React's loaders leave one texture unpainted. `Fault` is a verdict about the RENDERER — a page
/// cursor lost ownership, a sealed byte witness disagreed, a mesh publication generation was stale —
/// and those quarantine the surface, because nothing downstream can be trusted after them.
pub(crate) enum RendererAssetProbeStep {
    Pending,
    Ready,
    Reject(&'static str),
    Fault(&'static str),
}

impl RendererAssetProbe {
    pub(crate) fn new(owner: RendererAssetFetchOwner) -> Self {
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

    pub(crate) fn owner(&self) -> &RendererAssetFetchOwner {
        self.owner.as_ref().expect("asset probe owns response")
    }

    pub(crate) fn owner_mut(&mut self) -> &mut RendererAssetFetchOwner {
        self.owner.as_mut().expect("asset probe owns response")
    }

    pub(crate) fn step(&mut self) -> RendererAssetProbeStep {
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

    pub(crate) fn fault(&mut self, detail: &'static str) -> RendererAssetProbeStep {
        self.owner_mut().begin_close();
        self.phase = RendererAssetProbePhase::Closing;
        RendererAssetProbeStep::Fault(detail)
    }

    /// 🗑️ Refuses THIS response and keeps the frame: the kind and url are captured before
    /// `begin_close` clears them, so the lane that requested the asset can record its own miss
    /// (React's `tileMiss`/loader-error shape) instead of re-fetching the same refused bytes forever.
    pub(crate) fn reject(&mut self, detail: &'static str) -> RendererAssetProbeStep {
        self.rejection = Some((detail, self.owner().kind(), self.owner().url().to_string()));
        self.owner_mut().begin_close();
        self.phase = RendererAssetProbePhase::Closing;
        RendererAssetProbeStep::Reject(detail)
    }

    pub(crate) fn take_rejection(&mut self) -> Option<(&'static str, WorldAssetRequestKind, String)> {
        self.rejection.take()
    }

    pub(crate) fn take_ready_mesh_asset(&mut self) -> Option<World3dMeshAsset> {
        (matches!(self.phase, RendererAssetProbePhase::Ready) && matches!(self.owner().kind(), WorldAssetRequestKind::Glb)).then(|| self.glb_materialize.as_mut()?.take_asset()).flatten()
    }

    pub(crate) fn restore_ready_mesh_asset(&mut self, asset: World3dMeshAsset) {
        self.glb_materialize.as_mut().expect("ready GLB probe owns its materializer").restore_asset(asset);
    }

    pub(crate) fn finish_ready_mesh(&mut self) {
        assert!(
            self.glb_materialize.as_ref().is_some_and(|materialize| materialize.lease.is_none() && materialize.appearance.is_none()),
            "published GLB probe relinquished its paged mesh and appearance owners"
        );
        self.owner_mut().begin_close();
        self.phase = RendererAssetProbePhase::Closing;
    }

    pub(crate) fn begin_close(&mut self) {
        if let Some(materialize) = self.glb_materialize.as_mut() {
            materialize.begin_close();
        }
        self.owner_mut().begin_close();
        self.phase = RendererAssetProbePhase::Closing;
    }

    pub(crate) fn close_step(&mut self) -> bool {
        self.begin_close();
        if self.glb_materialize.as_mut().is_some_and(|materialize| !materialize.close_step()) {
            return false;
        }
        self.glb_materialize = None;
        self.owner_mut().close_step()
    }

    pub(crate) fn take_terminal_owner(&mut self) -> Option<RendererAssetFetchOwner> {
        self.owner.as_ref().is_some_and(|owner| owner.owner().terminal_is_empty()).then(|| self.owner.take().expect("terminal probe owner"))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RendererAssetDecodeResult {
    Ready,
    Rejected(&'static str),
    Cancelled,
    Fault(&'static str),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum RendererAssetDecodeTurnKind {
    Idle,
    Pending,
    Published,
    Cancelled,
    Fault,
}

#[derive(serde::Serialize)]
pub(crate) struct RendererAssetDecodeTurn {
    pub(crate) kind: RendererAssetDecodeTurnKind,
    pub(crate) pending: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) detail: Option<&'static str>,
}

/// 🧵️ Owns one response across bounded decoder turns without borrowing the application runtime.
pub(crate) struct RendererAssetDecodeJob {
    pub(crate) probe: RefCell<Option<RendererAssetProbe>>,
    pub(crate) handback: Arc<Mutex<Option<RendererAssetProbe>>>,
    pub(crate) cancelled: Arc<AtomicBool>,
    pub(crate) result: Option<RendererAssetDecodeResult>,
    pub(crate) steps: u64,
    closing: bool,
}

impl RendererAssetDecodeJob {
    pub(crate) fn new(probe: RendererAssetProbe, cancelled: Arc<AtomicBool>) -> Self {
        Self { probe: RefCell::new(Some(probe)), handback: Arc::new(Mutex::new(None)), cancelled, result: None, steps: 0, closing: false }
    }

    pub(crate) fn take_probe(&self) -> Option<RendererAssetProbe> {
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

/// 📬️ Retains the exact decoder ticket and all rejected or terminal response handbacks.
pub(crate) struct RendererAssetDecodeSession {
    pub(crate) session: Option<semio_framework_job_worker::WorkerJobSession<RendererAssetDecodeJob>>,
    pub(crate) rejected: Option<semio_framework_job_worker::WorkerJobSessionAdmissionRejected<RendererAssetDecodeJob>>,
    ticket: Option<semio_framework_job_worker::WorkerJobTicket>,
    pub(crate) handback: Arc<Mutex<Option<RendererAssetProbe>>>,
    pub(crate) boundary: Option<(RendererAssetProbe, Option<RendererAssetDecodeResult>)>,
    pub(crate) cancelled: Arc<AtomicBool>,
    pub(crate) surface_token: Option<AdmittedSurfaceToken>,
    pub(crate) request_token: WorldAssetRequestToken,
    pub(crate) fault: Option<&'static str>,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) waker: Option<std::task::Waker>,
}

impl RendererAssetDecodeSession {
    pub(crate) fn new(probe: RendererAssetProbe, wake: Option<RuntimeHostWaker>) -> Self {
        use semio_framework_job::{allocate_operation_id,root_cancel_token,Generation,InteractiveStage};
use semio_framework_job_worker::{WorkerJobSession};use semio_framework_job_worker_authority::{BatchDriveConfig,BatchJobParams};
        let surface_token = match probe.owner() {
            RendererAssetFetchOwner::World { surface_token, .. } => Some(*surface_token),
            RendererAssetFetchOwner::Shared(_) => None,
        };
        let request_token = probe.owner().owner().token();
        let cancelled = Arc::new(AtomicBool::new(false));
        let job = RendererAssetDecodeJob::new(probe, cancelled.clone());
        let handback = job.handback.clone();
        let params = BatchJobParams {
            operation: allocate_operation_id(),
            generation: Generation(1),
            cancel: root_cancel_token(),
            config: BatchDriveConfig { retained: semio_framework_job::RetainedCloneGrant::default(), site: "renderer_asset_decode", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: semio_framework_job::INTERACTIVE_LANE_WALL_US },
            now_us: semio_framework_job::default_now_us,
        };
        let (session, rejected, boundary) = match WorkerJobSession::try_new(job, params) {
            Ok(session) => (Some(session), None, None),
            Err(mut rejected) => {
                let boundary = rejected.job().take_probe().map(|probe| (probe, None));
                rejected.begin_close();
                (None, Some(rejected), boundary)
            }
        };
        #[cfg(target_arch = "wasm32")]
        let _ = wake;
        Self {
            session,
            rejected,
            ticket: None,
            handback,
            boundary,
            cancelled,
            surface_token,
            request_token,
            fault: None,
            #[cfg(not(target_arch = "wasm32"))]
            waker: wake.map(|wake| std::task::Waker::from(Arc::new(RendererAssetDecodeWake(wake)))),
        }
    }

    pub(crate) fn cancel(&self) {
        if !self.cancelled.swap(true, Ordering::AcqRel) {
            self.wake();
        }
    }

    pub(crate) fn wake(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(waker) = self.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    pub(crate) fn pump_one(&mut self) -> bool {
        use semio_framework_job::{StepOutcome};
use semio_framework_job_worker::{WorkerJobPoll};
        if self.boundary.is_none() {
            if let Ok(mut handback) = self.handback.try_lock() {
                self.boundary = handback.take().map(|probe| (probe, Some(self.fault.map_or(RendererAssetDecodeResult::Cancelled, RendererAssetDecodeResult::Fault))));
            }
        }
        if let Some(rejected) = self.rejected.as_mut() {
            let demand = match rejected.retirement_demands(ui_contract::UI_WORKER_RETIREMENT_POLICY.maximum_copy_bytes) { Ok(demand) => demand, Err(_) => { self.fault = Some("original rejected job close demand refused"); return false; } };
            if !ui_contract::ui_worker_retirement_permits(demand) { self.fault = Some("original worker owner exceeds fixed retirement policy"); return false; }
            let grant = ui_contract::UI_WORKER_RETIREMENT_POLICY;
            let step = rejected.close_step(grant).admit(grant, rejected.terminal_is_empty());
            if matches!(step, semio_framework_job::InteractiveJobCloseStep::Refused { .. }) { self.fault = Some("original rejected job close receipt refused"); return false; }
            if rejected.terminal_is_empty() {
                self.rejected = None;
            }
            return true;
        }
        let Some(session) = self.session.as_ref() else { return false };
        let _ = session.take_wake();
        #[cfg(not(target_arch = "wasm32"))]
        if self.waker.as_ref().is_some_and(|waker| session.register_wake(waker).is_err()) {
            return true;
        }
        match session.poll() {
            WorkerJobPoll::Idle => {
                #[cfg(not(target_arch = "wasm32"))]
                match session.try_submit_step(&renderer_worker_pool(), semio_framework_async::Lane::Maintenance) {
                    Ok(ticket) => {
                        self.ticket = Some(ticket);
                        return false;
                    }
                    Err(semio_framework_job_worker::WorkerJobSubmitFault::SequenceExhausted) => {
                        self.fault = Some("asset decoder session sequence exhausted");
                        let _ = session.begin_close();
                    }
                    Err(_) => {}
                }
                #[cfg(target_arch = "wasm32")]
                match session.try_step_on_worker() {
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
                    return true;
                };
                if let Ok(mut owner) = session.take_outcome(ticket) {
                    self.ticket = None;
                    if matches!(owner.outcome(), StepOutcome::Yield) {
                        let _ = owner.take_outcome();
                        if let Err(owner) = owner.resume() {
                            self.fault = Some("asset decoder yield could not resume its exact owner");
                            owner.begin_close();
                        }
                    } else {
                        self.fault = Some("asset decoder exposed a nonterminal payload");
                        owner.begin_close();
                    }
                }
            }
            WorkerJobPoll::Terminal => {
                if let Ok(owner) = session.take_terminal() {
                    let result = match owner.outcome() {
                        StepOutcome::Complete(_) => owner.job().result.unwrap_or(RendererAssetDecodeResult::Fault("asset decoder completed without its typed result")),
                        StepOutcome::Cancelled => RendererAssetDecodeResult::Cancelled,
                        _ => RendererAssetDecodeResult::Fault("asset decoder worker terminated with a fault"),
                    };
                    self.boundary = owner.job().take_probe().map(|probe| (probe, Some(result)));
                    self.ticket = None;
                    owner.begin_close();
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
            WorkerJobPoll::Closing | WorkerJobPoll::TerminalEmpty => {
                let demand = match session.retirement_demands(ui_contract::UI_WORKER_RETIREMENT_POLICY.maximum_copy_bytes) { Ok(demand) => demand, Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_)) => return false, Err(_) => { self.fault = Some("original worker job close demand refused"); return false; } };
                if !ui_contract::ui_worker_retirement_permits(demand) { self.fault = Some("original worker owner exceeds fixed retirement policy"); return false; }
                let grant = ui_contract::UI_WORKER_RETIREMENT_POLICY;
            let step = session.close_step(grant);
            if !step.progress().fits(grant) || matches!(step, semio_framework_job_worker_authority::WorkerJobCloseStep::Refused(_)) { self.fault = Some("original worker job close receipt refused"); return false; }
                if session.terminal_is_empty() {
                    self.session = None;
                }
            }
            WorkerJobPoll::Submitted | WorkerJobPoll::CheckedOut => return false,
        }
        true
    }

    pub(crate) fn take_boundary(&mut self) -> Option<(RendererAssetProbe, Option<RendererAssetDecodeResult>)> {
        if self.session.is_some() || self.rejected.is_some() {
            return None;
        }
        let (probe, result) = self.boundary.take()?;
        Some((probe, if self.cancelled.load(Ordering::Acquire) && !matches!(result, Some(RendererAssetDecodeResult::Fault(_))) { Some(RendererAssetDecodeResult::Cancelled) } else { result }))
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.session.is_none() && self.rejected.is_none() && self.boundary.is_none() && self.handback.try_lock().is_ok_and(|owner| owner.is_none())
    }
}

pub(crate) fn renderer_asset_svg_prefix_is_valid(prefix: &[u8]) -> bool {
    std::str::from_utf8(prefix).ok().is_some_and(|text| {
        let text = text.trim_start_matches(|character: char| character.is_ascii_whitespace());
        text.starts_with("<svg") || text.starts_with("<?xml")
    })
}

fn renderer_asset_ui_image_prefix_is_valid(prefix: &[u8]) -> bool {
    prefix.starts_with(b"\x89PNG\r\n\x1a\n") || prefix.starts_with(&[0xff, 0xd8, 0xff]) || renderer_asset_svg_prefix_is_valid(prefix)
}

pub(crate) fn renderer_asset_reference_prefix_is_valid(prefix: &[u8]) -> bool {
    renderer_asset_ui_image_prefix_is_valid(prefix)
        || prefix.starts_with(b"GIF87a")
        || prefix.starts_with(b"GIF89a")
        || (prefix.len() >= 12 && &prefix[..4] == b"RIFF" && &prefix[8..12] == b"WEBP")
        || prefix.starts_with(b"BM")
        || prefix.starts_with(b"II*\0")
        || prefix.starts_with(b"MM\0*")
        || prefix.starts_with(b"II+\0")
        || prefix.starts_with(b"MM\0+")
        || prefix.starts_with(b"%PDF-")
        || (prefix.len() >= 12 && &prefix[4..8] == b"ftyp")
}
//#endregion 📡️RendererAssetAuthority

//#region 🧵️RendererWorkerPool
/// 🧵️ Resolves the interactive OS process's single worker pool for every renderer subsystem.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn renderer_worker_pool() -> semio_framework_async::WorkerPool {
    let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, cores))
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn renderer_worker_pool() -> semio_framework_async::WorkerPool {
    semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 1))
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct RendererIoHandle {
    slot: usize,
    pub(crate) generation: u64,
    completed: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl RendererIoHandle {
    fn try_take(&mut self) -> Option<Result<semio_framework_os_services::NativeIoValue, String>> {
        let result = renderer_io_with_node(self.slot, self.generation, |node| {
            let result = node.result.take()?;
            node.result_taken = true;
            RENDERER_IO_WAKE.store(true, Ordering::Release);
            Some(result)
        })
        .flatten();
        self.completed |= result.is_some();
        result
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Future for RendererIoHandle {
    type Output = Result<semio_framework_os_services::NativeIoValue, String>;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if let Some(result) = self.try_take() {
            return std::task::Poll::Ready(result);
        }
        let registered = renderer_io_with_node(self.slot, self.generation, |node| {
            node.waker = Some(cx.waker().clone());
            if let Some(session) = node.session.as_ref() {
                let _ = session.register_wake(cx.waker());
            }
        })
        .is_some();
        if registered {
            RENDERER_IO_WAKE.store(true, Ordering::Release);
            std::task::Poll::Pending
        } else if renderer_io_generation_live(self.slot, self.generation) {
            RENDERER_IO_WAKE.store(true, Ordering::Release);
            cx.waker().wake_by_ref();
            std::task::Poll::Pending
        } else {
            self.completed = true;
            std::task::Poll::Ready(Err("native I/O mounted session generation is stale".into()))
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for RendererIoHandle {
    fn drop(&mut self) {
        if self.completed {
            return;
        }
        if let Some(slot) = RENDERER_IO_SLOTS.get(self.slot) {
            if slot.generation.load(Ordering::Acquire) == self.generation {
                slot.cancel_requested.store(true, Ordering::Release);
            }
        }
        RENDERER_IO_WAKE.store(true, Ordering::Release);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) const RENDERER_IO_SESSION_SLOTS: usize = semio_framework_job_worker::WORKER_JOB_SESSION_SLOTS + 1;
#[cfg(not(target_arch = "wasm32"))]
const RENDERER_IO_VACANT: u8 = 0;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const RENDERER_IO_LIVE: u8 = 1;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const RENDERER_IO_CHECKED_OUT: u8 = 2;
#[cfg(not(target_arch = "wasm32"))]
const RENDERER_IO_EXHAUSTED: u8 = 3;

#[cfg(not(target_arch = "wasm32"))]
struct RendererIoSlot {
    generation: AtomicU64,
    state: std::sync::atomic::AtomicU8,
    node: std::sync::atomic::AtomicPtr<RendererIoNode>,
    cancel_requested: AtomicBool,
}

#[cfg(not(target_arch = "wasm32"))]
impl RendererIoSlot {
    const fn vacant() -> Self {
        Self { generation: AtomicU64::new(0), state: std::sync::atomic::AtomicU8::new(RENDERER_IO_VACANT), node: std::sync::atomic::AtomicPtr::new(std::ptr::null_mut()), cancel_requested: AtomicBool::new(false) }
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct RendererIoNode {
    session: Option<semio_framework_job_worker::WorkerJobSession<semio_framework_os_services::NativeIoJob>>,
    rejected: Option<semio_framework_job_worker::WorkerJobSessionAdmissionRejected<semio_framework_os_services::NativeIoJob>>,
    ticket: Option<semio_framework_job_worker::WorkerJobTicket>,
    result: Option<Result<semio_framework_os_services::NativeIoValue, String>>,
    result_taken: bool,
    detached: bool,
    cancel: semio_framework_async::CancelToken,
    waker: Option<std::task::Waker>,
}

#[cfg(not(target_arch = "wasm32"))]
impl RendererIoNode {
    fn wake(&mut self) {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }

    fn close_detached_result(&mut self) -> bool {
        if !self.detached {
            return false;
        }
        let Some(result) = self.result.as_mut() else { return false };
        match result {
            Ok(value) if !value.terminal_is_empty() => {
                let _ = value.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                return true;
            }
            Ok(_) => {}
            Err(error) if !error.is_empty() => {
                if error.len() > semio_framework_job::JOB_PAYLOAD_PAGE_BYTES {
                    return false;
                }
                error.clear();
                return true;
            }
            Err(_) => {}
        }
        self.result = None;
        self.result_taken = true;
        true
    }

    fn pump_one(&mut self) {
        if self.close_detached_result() {
            self.wake();
            return;
        }
        if let Some(rejected) = self.rejected.as_mut() {
            let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            if rejected.terminal_is_empty() {
                self.rejected = None;
            }
            self.wake();
            return;
        }
        let Some(session) = self.session.as_ref() else {
            self.wake();
            return;
        };
        let _ = session.take_wake();
        if self.result.is_some() || self.detached {
            if !matches!(session.poll(), semio_framework_job_worker::WorkerJobPoll::Closing | semio_framework_job_worker::WorkerJobPoll::TerminalEmpty) {
                let _ = session.begin_close();
                self.wake();
                return;
            }
            let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            if session.terminal_is_empty() {
                self.session = None;
            }
            self.wake();
            return;
        }
        match session.poll() {
            semio_framework_job_worker::WorkerJobPoll::Idle => match session.try_submit_step(&renderer_worker_pool(), semio_framework_async::Lane::Io) {
                Ok(ticket) => self.ticket = Some(ticket),
                Err(semio_framework_job_worker::WorkerJobSubmitFault::Pool(semio_framework_async::WorkerSubmitErrorKind::Saturated | semio_framework_async::WorkerSubmitErrorKind::Contended)) => {
                    if let Ok(rejected) = session.take_rejected() {
                        rejected.resume();
                    }
                }
                Err(error) => {
                    if let Ok(rejected) = session.take_rejected() {
                        rejected.begin_close();
                    } else {
                        let _ = session.begin_close();
                    }
                    self.result = Some(Err(format!("native I/O worker submission failed: {error:?}")));
                }
            },
            semio_framework_job_worker::WorkerJobPoll::Outcome => {
                let Some(ticket) = self.ticket.take() else {
                    self.result = Some(Err("native I/O outcome lost its exact ticket".into()));
                    let _ = session.begin_close();
                    self.wake();
                    return;
                };
                match session.take_outcome(ticket) {
                    Ok(mut owner) => {
                        if matches!(owner.take_outcome(), semio_framework_job::StepOutcome::Yield) {
                            if owner.resume().is_err() {
                                self.result = Some(Err("native I/O yield could not resume exact owner".into()));
                            }
                        } else {
                            owner.begin_close();
                            self.result = Some(Err("native I/O published a nonterminal retained payload".into()));
                        }
                    }
                    Err(_) => self.result = Some(Err("native I/O outcome handback contention".into())),
                }
            }
            semio_framework_job_worker::WorkerJobPoll::Terminal => match session.take_terminal() {
                Ok(mut owner) => {
                    self.result = owner.job_mut().take_result().or_else(|| Some(Err("native I/O terminal owner had no structured result".into())));
                    owner.begin_close();
                }
                Err(_) => self.result = Some(Err("native I/O terminal handback contention".into())),
            },
            semio_framework_job_worker::WorkerJobPoll::Rejected => {
                if let Ok(rejected) = session.take_rejected() {
                    rejected.resume();
                }
            }
            semio_framework_job_worker::WorkerJobPoll::Closing => {
                let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                if session.terminal_is_empty() {
                    self.session = None;
                }
            }
            semio_framework_job_worker::WorkerJobPoll::Submitted | semio_framework_job_worker::WorkerJobPoll::CheckedOut | semio_framework_job_worker::WorkerJobPoll::TerminalEmpty => {}
        }
        self.wake();
    }

    fn terminal_is_empty(&self) -> bool {
        self.session.is_none() && self.rejected.is_none() && self.result.is_none() && (self.detached || self.result_taken)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) static RENDERER_IO_SLOTS: [RendererIoSlot; RENDERER_IO_SESSION_SLOTS] = [const { RendererIoSlot::vacant() }; RENDERER_IO_SESSION_SLOTS];
#[cfg(not(target_arch = "wasm32"))]
pub(crate) static RENDERER_IO_WAKE: AtomicBool = AtomicBool::new(false);
#[cfg(not(target_arch = "wasm32"))]
static RENDERER_IO_PUMP_CURSOR: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn renderer_io_with_node<R>(index: usize, generation: u64, use_node: impl FnOnce(&mut RendererIoNode) -> R) -> Option<R> {
    let slot = RENDERER_IO_SLOTS.get(index)?;
    if slot.generation.load(Ordering::Acquire) != generation || slot.state.compare_exchange(RENDERER_IO_LIVE, RENDERER_IO_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_err() {
        return None;
    }
    if slot.generation.load(Ordering::Acquire) != generation {
        slot.state.store(RENDERER_IO_LIVE, Ordering::Release);
        return None;
    }
    let pointer = slot.node.load(Ordering::Acquire);
    let result = (!pointer.is_null()).then(|| use_node(unsafe { &mut *pointer }));
    slot.state.store(RENDERER_IO_LIVE, Ordering::Release);
    result
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn renderer_io_generation_live(index: usize, generation: u64) -> bool {
    RENDERER_IO_SLOTS.get(index).is_some_and(|slot| slot.generation.load(Ordering::Acquire) == generation && matches!(slot.state.load(Ordering::Acquire), RENDERER_IO_LIVE | RENDERER_IO_CHECKED_OUT) && !slot.node.load(Ordering::Acquire).is_null())
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn pump_renderer_io_sessions(maximum_sessions: usize) -> usize {
    if maximum_sessions == 0 {
        return 0;
    }
    let start = RENDERER_IO_PUMP_CURSOR.fetch_add(maximum_sessions, Ordering::AcqRel);
    let mut advanced = 0;
    for offset in 0..RENDERER_IO_SESSION_SLOTS {
        if advanced == maximum_sessions {
            break;
        }
        let index = start.wrapping_add(offset) % RENDERER_IO_SESSION_SLOTS;
        let slot = &RENDERER_IO_SLOTS[index];
        if slot.state.compare_exchange(RENDERER_IO_LIVE, RENDERER_IO_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_err() {
            continue;
        }
        let pointer = slot.node.load(Ordering::Acquire);
        if pointer.is_null() {
            slot.state.store(RENDERER_IO_VACANT, Ordering::Release);
            continue;
        }
        let node = unsafe { &mut *pointer };
        let cancel_advanced = slot.cancel_requested.swap(false, Ordering::AcqRel);
        if cancel_advanced {
            node.detached = true;
            node.cancel.cancel_now();
            if let Some(session) = node.session.as_ref() {
                let _ = session.begin_close();
            }
            if let Some(rejected) = node.rejected.as_mut() {
                rejected.begin_close();
            }
            node.wake();
        }
        if !cancel_advanced {
            node.pump_one();
        }
        if node.terminal_is_empty() {
            slot.node.store(std::ptr::null_mut(), Ordering::Release);
            slot.cancel_requested.store(false, Ordering::Release);
            drop(unsafe { Box::from_raw(pointer) });
            slot.state.store(if slot.generation.load(Ordering::Acquire) == u64::MAX { RENDERER_IO_EXHAUSTED } else { RENDERER_IO_VACANT }, Ordering::Release);
        } else {
            slot.state.store(RENDERER_IO_LIVE, Ordering::Release);
        }
        advanced += 1;
    }
    if RENDERER_IO_SLOTS.iter().any(|slot| slot.state.load(Ordering::Acquire) == RENDERER_IO_LIVE) {
        RENDERER_IO_WAKE.store(true, Ordering::Release);
    }
    advanced
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn submit_renderer_io(request: semio_framework_os_services::NativeIoRequest) -> Result<RendererIoHandle, String> {
    use semio_framework_job::{allocate_operation_id,root_cancel_token,Generation,InteractiveStage,INTERACTIVE_LANE_FUEL,INTERACTIVE_LANE_WALL_US};
use semio_framework_job_worker_authority::{BatchDriveConfig,BatchJobParams};
    let Some((slot_index, generation)) = RENDERER_IO_SLOTS.iter().enumerate().find_map(|(index, slot)| {
        if slot.state.compare_exchange(RENDERER_IO_VACANT, RENDERER_IO_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return None;
        }
        let Some(generation) = slot.generation.load(Ordering::Acquire).checked_add(1) else {
            slot.state.store(RENDERER_IO_EXHAUSTED, Ordering::Release);
            return None;
        };
        slot.generation.store(generation, Ordering::Release);
        slot.cancel_requested.store(false, Ordering::Release);
        Some((index, generation))
    }) else {
        return Err("native I/O mounted session registry is exhausted".into());
    };
    let job = semio_framework_os_services::NativeIoJob::new(request);
    let cancel = root_cancel_token();
    let params = BatchJobParams {
        operation: allocate_operation_id(),
        generation: Generation(generation),
        cancel: cancel.clone(),
        config: BatchDriveConfig { retained: semio_framework_job::RetainedCloneGrant::default(), site: "os_renderer_native_io", stage: InteractiveStage::InteractiveStep, fuel_per_step: INTERACTIVE_LANE_FUEL, step_budget_us: INTERACTIVE_LANE_WALL_US },
        now_us: semio_framework_job::default_now_us,
    };
    let (session, rejected, result) = match semio_framework_job_worker::WorkerJobSession::try_new(job, params) {
        Ok(session) => (Some(session), None, None),
        Err(mut rejected) => {
            rejected.begin_close();
            (None, Some(rejected), Some(Err("native I/O retained session registry is full".into())))
        }
    };
    let node = Box::new(RendererIoNode { session, rejected, ticket: None, result, result_taken: false, detached: false, cancel, waker: None });
    RENDERER_IO_SLOTS[slot_index].node.store(Box::into_raw(node), Ordering::Release);
    RENDERER_IO_SLOTS[slot_index].state.store(RENDERER_IO_LIVE, Ordering::Release);
    RENDERER_IO_WAKE.store(true, Ordering::Release);
    Ok(RendererIoHandle { slot: slot_index, generation, completed: false })
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn run_renderer_io(request: semio_framework_os_services::NativeIoRequest) -> Result<semio_framework_os_services::NativeIoValue, String> {
    submit_renderer_io(request)?.await
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../../🧪️tests/🔬️wgpu-renderer-renderer-io-retained/🦀️.rs"]
mod renderer_io_retained_tests;
//#endregion 🧵️RendererWorkerPool

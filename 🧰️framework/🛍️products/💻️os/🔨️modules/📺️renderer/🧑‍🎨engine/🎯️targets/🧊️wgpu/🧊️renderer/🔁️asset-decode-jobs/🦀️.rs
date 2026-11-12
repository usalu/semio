//! 🔁️ Asset decode jobs: asset probe, decode job and session, prefix validators and the renderer worker pool with its I/O slots.
use super::*;
use semio_ui_contract as ui_contract;
use semio_plugin_ownership::reserved_job_session::RefusedWorkerSource;

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

    pub fn step(&mut self,cx:&mut semio_framework_job::StepContext<'_>) -> Result<RendererAssetProbeStep,semio_framework_job::OriginalTurnFailure> {
        if matches!(self.phase,RendererAssetProbePhase::Materializing){
            let deadline=cx.deadline_us();let fuel=cx.fuel_remaining().saturating_sub(1);
            let materialize=self.glb_materialize.as_mut().expect("GLB probe owns materializer");
            let ready=cx.with_original_work_loan(fuel,deadline,|same|materialize.step(self.owner.as_ref().expect("asset probe owns response"),&self.page_index,same))?;
            if ready{self.phase=RendererAssetProbePhase::Ready;return Ok(RendererAssetProbeStep::Ready)}
            return Ok(RendererAssetProbeStep::Pending)
        }
        Ok(self.step_value())
    }

    fn step_value(&mut self) -> RendererAssetProbeStep {
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
            RendererAssetProbePhase::Materializing => self.fault("GLB materialization requires its original context"),
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

    pub fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {
        self.begin_close();
        if let Some(materialize)=self.glb_materialize.as_mut(){if !materialize.close_step(cx)?{
            return Ok(false);
        }
        }
        self.glb_materialize = None;
        Ok(self.owner_mut().close_step())
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
    pub result: Option<RendererAssetDecodeResult>,
    pub steps: u64,
    closing: bool,
}

impl RendererAssetDecodeJob {
    pub fn new(probe: RendererAssetProbe) -> Self {
        Self { probe: RefCell::new(Some(probe)), result: None, steps: 0, closing: false }
    }

    pub fn take_probe(&self) -> Option<RendererAssetProbe> {
        self.probe.try_borrow_mut().ok()?.take()
    }

    fn original_png_close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        let probe=self.probe.try_borrow().map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"decoder original response is borrowed"))?;
        match probe.as_ref().and_then(|probe|probe.glb_materialize.as_ref()){Some(materialize)=>materialize.original_png_close_demands(body),None=>Ok(Default::default())}
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
        if context.is_cancelled() {
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
        let result = probe.step(context);
        self.steps = steps;
        context.consume_fuel(1);
        let result=result?;
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
        self.begin_close();
        if let Some(probe)=self.probe.get_mut().as_mut(){
            if let Some(materialize)=probe.glb_materialize.as_mut(){if materialize.original_png_pending(){
                let before=cx.retained_progress();materialize.close_original_png(cx)?;let after=cx.retained_progress();
                let difference=|whole:usize,earlier:usize|whole.checked_sub(earlier).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original PNG close receipt moved backwards"));
                let progress=RetainedCloneProgress{copied_items:difference(after.copied_items,before.copied_items)?,copied_bytes:difference(after.copied_bytes,before.copied_bytes)?,retained_capacity_bytes:difference(after.retained_capacity_bytes,before.retained_capacity_bytes)?,released_bytes:difference(after.released_bytes,before.released_bytes)?};
                return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress});
            }}
            return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked);
        }
        Ok(semio_framework_job::InteractiveJobCloseStep::Complete { progress: Default::default() })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.original_png_close_demands(semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.original_png_close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.original_png_close_demands(semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        let probe = self.probe.try_borrow().map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "decoder original response is borrowed"))?;
        Ok(self.original_png_close_demands(semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES)?.depth.max(usize::from(probe.is_some() || !self.closing)))
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.result.is_none() && self.probe.borrow().is_none()
    }
}

/// 📬️ One mounted decoder and its exact pending source borrow the initiating caller across every poll.
pub struct RendererAssetDecodeSession {
    pub session: Option<semio_framework_job_worker::MountedWorkerJobSession<RendererAssetDecodeJob>>,
    pub source: Option<semio_plugin_ownership::reserved_job_session::RefusedWorkerSource<RendererAssetDecodeJob>>,
    pub boundary: Option<(RendererAssetProbe, Option<RendererAssetDecodeResult>)>,
    pub surface_token: Option<AdmittedSurfaceToken>,
    pub request_token: WorldAssetRequestToken,
    pub cancelled: AtomicBool,
    pub closing: bool,
}
impl RendererAssetDecodeSession {
    pub fn new(probe: RendererAssetProbe, cx: &semio_framework_job::StepContext<'_>) -> Self {
        let surface_token = match probe.owner() { RendererAssetFetchOwner::World { surface_token, .. } => Some(*surface_token), RendererAssetFetchOwner::Shared(_) => None };
        let request_token = probe.owner().owner().token();
        let job = RendererAssetDecodeJob::new(probe);
        Self { session: None, source: Some(semio_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(job, (cx.operation(), cx.generation()))), boundary: None, surface_token, request_token, cancelled: AtomicBool::new(false), closing: false }
    }
    pub fn cancel(&self) { self.cancelled.store(true, Ordering::Release); }
    pub fn pump_one(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        use semio_framework_job::{InteractiveJob, JobOutcomeKind, close_original_retained_step, close_original_effect_step};
        use semio_framework_value::{RetirementDemand, RetainedCloneStep, ValueError, ValueRefusalKind};
        self.closing |= self.cancelled.load(Ordering::Acquire) || cx.is_cancelled();
        if self.source.as_ref().is_some_and(RefusedWorkerSource::terminal_is_empty) { RefusedWorkerSource::retire_empty(&mut self.source,cx)?; return Ok(true); }
        if let Some(source) = self.source.as_mut() {
            if self.closing {
                if self.boundary.is_none() { self.boundary = source.job().and_then(RendererAssetDecodeJob::take_probe).map(|probe| (probe, Some(RendererAssetDecodeResult::Cancelled))); }
                source.close_step(cx)?;
            } else {
                let config = semio_framework_job_worker_authority::BatchDriveConfig { retained: cx.retained_grant(), site: "renderer_asset_decode", stage: semio_framework_job::InteractiveStage::InteractiveStep, fuel_per_step: cx.fuel_remaining().min(4096), step_budget_us: semio_framework_job::INTERACTIVE_LANE_WALL_US };
                source.admit_pending(config, &mut self.session, cx)?;
            }
            return Ok(true);
        }
        if self.session.as_ref().is_some_and(|session|session.terminal_is_empty()) { close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{self.session=None;semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}})?;return Ok(true); }
        let Some(session) = self.session.as_mut() else { return Ok(false); };
        if self.closing {
            if self.boundary.is_none() {
                if session.checked_out_job().is_none() { session.pump_one_for_interactive_turn(cx)?; }
                self.boundary = session.checked_out_job().and_then(RendererAssetDecodeJob::take_probe).map(|probe| (probe, Some(RendererAssetDecodeResult::Cancelled)));
                if self.boundary.is_none() { return Ok(false); }
            }
            session.begin_close();
            let witness = cx.borrowed_original_cancel_token()?;
            match session.has_original_cancel_alias_witness(witness) {
                Ok(true) => {
                    close_original_effect_step(cx, RetirementDemand { depth: 1, ..Default::default() }, |effect| {let turn=session.return_original_cancel_alias_step(witness,effect);semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:turn.result.map(|step| step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(|error| match error { semio_framework_job_worker::WorkerJobDemandError::Refused(error) => error, semio_framework_job_worker::WorkerJobDemandError::Contention(_) => ValueError::literal(ValueRefusalKind::WorkLimit, "decoder cancellation return is contended") }),work:turn.work})})?;
                    return Ok(true);
                }
                Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_)) => return Ok(false),
                Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error)) => return Err(error.into()),
                Ok(false) => {}
            }
            session.close_step(cx)?;
            return Ok(true);
        }
        if let Some(kind) = session.checked_out_outcome_descriptor().map(semio_framework_job::JobOutcomeDescriptor::kind) {
            if matches!(kind, JobOutcomeKind::Complete | JobOutcomeKind::Cancelled | JobOutcomeKind::Fault) {
                let result = session.checked_out_job().and_then(|job| job.result).unwrap_or(RendererAssetDecodeResult::Fault("decoder terminal result is absent"));
                self.boundary = session.checked_out_job().and_then(RendererAssetDecodeJob::take_probe).map(|probe| (probe, Some(result)));
                self.closing = true;
            } else {
                session.acknowledge_checked_out_outcome(cx)?;
                if session.checked_out_outcome_descriptor().is_none() { session.resume().map_err(|_| ValueError::literal(ValueRefusalKind::InvariantViolated, "decoder lost its original checked-out owner"))?; }
            }
        } else if session.checked_out_job().is_some() {
            session.resume().map_err(|_| ValueError::literal(ValueRefusalKind::InvariantViolated, "decoder lost its original empty-outcome owner"))?;
        } else { session.pump_one_for_interactive_turn(cx)?; }
        Ok(true)
    }

    pub fn take_boundary(&mut self) -> Option<(RendererAssetProbe, Option<RendererAssetDecodeResult>)> {
        if self.session.is_some() || self.source.is_some() { return None; }
        let (probe, result) = self.boundary.take()?;
        Some((probe, if self.cancelled.load(Ordering::Acquire) && !matches!(result, Some(RendererAssetDecodeResult::Fault(_))) { Some(RendererAssetDecodeResult::Cancelled) } else { result }))
    }
    pub fn terminal_is_empty(&self) -> bool { self.session.is_none() && self.source.is_none() && self.boundary.is_none() }
}

//#endregion 📡️RendererAssetAuthority


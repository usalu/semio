//! 🏃️ Run job of the reconstruction session: the run phases and remnant and the interactive reconstruction run job that advances the engine under the budget. See [`super`].

use super::*;

//#region 🧵️RunJob
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub(super) enum Owed {
    Checkpoint,
    Complete,
    Fault,
}

#[derive(semio_framework_value::RetireOwned)]
enum RunPhase {
    Ingest { stream: usize, frame: usize, ingestion: Option<FrameIngestion> },
    Pipeline,
    DenseTrace { cursor: usize },
    Products(Box<ProductPreparation>),
    Settled,
}

/// 🧺️ Every heap owner of a finished or cancelled run, retired in bounded turns through one controlled owner.
#[derive(semio_framework_value::RetireOwned)]
struct RunRemnant {
    pub(super) phase: RunPhase,
    pub(super) engine: Option<Box<ReconstructionEngine>>,
    pub(super) scene: Option<Arc<RemodelingSnapshot>>,
    pub(super) observations: Vec<EngineObservation>,
    pub(super) owed: VecDeque<Owed>,
}

/// ⏯️ The reconstruction `runJob`. One unit of fuel is one decision that produced visible evidence; engine
/// slices without evidence are transitions bounded only by the deadline. A rebuilt job whose checkpoint
/// matches its inputs replays silently to the checkpoint's decision count.
pub struct ReconstructionRunJob {
    pub(super) identity: ToolRunIdentity,
    pub(super) writer: ToolRunTickWriter,
    pub(super) scene: Option<Arc<RemodelingSnapshot>>,
    pub(super) inputs: RemodelingContentDigest,
    pub(super) engine: Option<Box<ReconstructionEngine>>,
    pub(super) phase: RunPhase,
    pub(super) observations: Vec<EngineObservation>,
    pub(super) counters: [u64; 8],
    frames_total: u64,
    frames_decided: u64,
    pub(super) gcp_count: usize,
    pub(super) decisions: u64,
    replay_to: u64,
    ops_emitted: u32,
    checkpoint_at: u64,
    pub(super) stage: ReconstructionRunStage,
    pub(super) owed: VecDeque<Owed>,
    pub(super) outbox: Outbox,
    remnant: Option<semio_framework_value::retirement::controlled::ControlledRetirement<RunRemnant>>,
    pub(super) closing: bool,
}

impl ReconstructionRunJob {
    /// 🌱️ A run over `scene`; `checkpoint` resumes it when its inputs still match, `provisional` is the
    /// ledger's provisional op count the writer continues from.
    pub fn new(identity: ToolRunIdentity, scene: Arc<RemodelingSnapshot>, checkpoint: Option<&[u8]>, provisional: u32) -> Self {
        let inputs = reconstruction_inputs(&scene);
        let mut engine = Box::new(ReconstructionEngine::new(&build_engine_params(&scene.params, &scene.calibration)));
        engine.observe();
        let mut writer = ToolRunTickWriter::with_provisional_base(identity, provisional);
        let resume = checkpoint.and_then(ReconstructionRunCheckpoint::decode).filter(|checkpoint| checkpoint.inputs == inputs && checkpoint.provisional_ops <= provisional);
        match resume {
            Some(checkpoint) => writer.retract_to(checkpoint.provisional_ops),
            None if checkpoint.is_some() || provisional > 0 => {
                writer.clear_trace();
                writer.retract_to(0);
            }
            None => {}
        }
        let frames_total = scene.streams.iter().map(|stream| stream.frames.len() as u64).sum();
        let gcp_count = scene.gcps.len();
        Self {
            identity,
            writer,
            scene: Some(scene),
            inputs,
            engine: Some(engine),
            phase: RunPhase::Ingest { stream: 0, frame: 0, ingestion: None },
            observations: Vec::new(),
            counters: [0; 8],
            frames_total,
            frames_decided: 0,
            gcp_count,
            decisions: 0,
            replay_to: resume.map_or(0, |checkpoint| checkpoint.decisions),
            ops_emitted: 0,
            checkpoint_at: 0,
            stage: ReconstructionRunStage::Ingest,
            owed: VecDeque::from(if resume.is_some() { Vec::new() } else { vec![Owed::Checkpoint] }),
            outbox: Outbox::default(),
            remnant: None,
            closing: false,
        }
    }

    /// 📍️ The checkpoint this job reports now.
    pub fn checkpoint(&self) -> ReconstructionRunCheckpoint {
        ReconstructionRunCheckpoint { base_revision: self.identity.base_revision, inputs: self.inputs, decisions: self.decisions, provisional_ops: self.ops_emitted }
    }

    /// 🔢️ Counter values in [`ReconstructionRunCounter::ALL`] order.
    pub fn counters(&self) -> [u64; 8] {
        self.counters
    }

    fn live(&self) -> bool {
        self.decisions >= self.replay_to
    }

    pub(super) fn count(&mut self, counter: ReconstructionRunCounter, delta: i64) {
        let slot = &mut self.counters[usize::from(counter.index())];
        *slot = slot.saturating_add_signed(delta);
    }

    pub(super) fn step(&mut self, reason: ReconstructionRunReason, args: &[u64]) {
        if self.live() {
            let args: Vec<ToolRunStepArg> = args.iter().map(|value| ToolRunStepArg::Unsigned(*value)).collect();
            let _ = self.writer.step(reason.step_kind(), self.stage.index(), reason.code(), None, &args);
        }
    }

    fn upsert(&mut self, key: u64, reason: ReconstructionRunReason, subject: ToolRunTraceSubject) {
        if self.live() {
            self.writer.upsert(key, reason.verdict(), reason.code(), subject);
        }
    }

    fn append(&mut self, op: &RemodelingMutation) -> Result<(), &'static str> {
        self.ops_emitted += 1;
        if self.ops_emitted > self.writer.provisional_len() {
            let bytes = protocol::OpBinary::encode_op(op).map_err(|_| "remodeling.reconstruction.op-encode")?;
            self.writer.append_op(bytes).map_err(|_| "remodeling.reconstruction.provisional-cap")?;
        }
        Ok(())
    }

    /// 🎯️ Completes one decision: consumes one unit of fuel once live, and owes a checkpoint every
    /// `CHECKPOINT_DECISIONS` decisions.
    fn decide(&mut self, cx: &mut StepContext<'_>) {
        let was_live = self.live();
        self.decisions += 1;
        if was_live {
            cx.consume_fuel(1);
            if self.decisions - self.checkpoint_at >= CHECKPOINT_DECISIONS {
                self.checkpoint_at = self.decisions;
                self.owed.push_back(Owed::Checkpoint);
            }
        } else if self.live() {
            self.step(ReconstructionRunReason::Resuming, &[self.decisions]);
        }
    }

    fn stage_progress(&self) -> (u64, Option<u64>) {
        match &self.phase {
            RunPhase::Ingest { .. } => (self.frames_decided, Some(self.frames_total)),
            RunPhase::Pipeline => self.engine.as_ref().map_or((0, None), |engine| engine.stage_progress()),
            RunPhase::DenseTrace { cursor } => (*cursor as u64, self.engine.as_ref().map(|engine| engine.dense_positions().len().min(RECONSTRUCTION_DENSE_TRACE_MAX) as u64)),
            RunPhase::Products(_) | RunPhase::Settled => (u64::from(self.ops_emitted), None),
        }
    }

    pub(super) fn progress(&self, state: ToolRunState) -> ToolRunProgress {
        let (completed, total) = self.stage_progress();
        ToolRunProgress {
            identity: self.identity,
            sequence: 0,
            state,
            stage: self.stage.index(),
            completed: total.map_or(completed, |total| completed.min(total)),
            total,
            counters: ReconstructionRunCounter::ALL.iter().zip(self.counters).map(|(counter, value)| ToolRunCounter { counter: counter.index(), value }).collect(),
            units_per_second: 0.0,
            conflicts: 0,
            steps: ToolRunStepRing::new(),
        }
    }

    /// 🔭️ Turns every engine observation of the last slice into trace records, steps and counters;
    /// `true` when the slice produced visible evidence.
    fn observe(&mut self) -> bool {
        let mut observations = std::mem::take(&mut self.observations);
        if let Some(engine) = self.engine.as_mut() {
            engine.drain_observations(&mut observations);
        }
        let evidence = !observations.is_empty();
        let (mut triangulated, mut pruned) = (0u64, 0u64);
        for observation in observations.drain(..) {
            match observation {
                EngineObservation::FeaturesExtracted { frame, keypoints } => {
                    self.count(ReconstructionRunCounter::Keypoints, keypoints as i64);
                    self.upsert(KEY_FEATURES | frame as u64, ReconstructionRunReason::FeaturesExtracted, ToolRunTraceSubject::Entity { entity: frame as u64 });
                    self.step(ReconstructionRunReason::FeaturesExtracted, &[frame as u64, keypoints as u64]);
                }
                EngineObservation::MatchAccepted { frame_a, frame_b, query, .. } => {
                    self.count(ReconstructionRunCounter::MatchesAccepted, 1);
                    self.upsert(match_trace_key(frame_a, frame_b, query), ReconstructionRunReason::MatchAccepted, ToolRunTraceSubject::Entity { entity: match_trace_key(frame_a, frame_b, query) & !KEY_MATCH });
                }
                EngineObservation::MatchRatioRejected { frame_a, frame_b, query, .. } => {
                    self.count(ReconstructionRunCounter::MatchesRejected, 1);
                    self.upsert(match_trace_key(frame_a, frame_b, query), ReconstructionRunReason::MatchAmbiguous, ToolRunTraceSubject::Entity { entity: match_trace_key(frame_a, frame_b, query) & !KEY_MATCH });
                }
                EngineObservation::MatchCrossCheckRejected { frame_a, frame_b, query, .. } => {
                    self.count(ReconstructionRunCounter::MatchesRejected, 1);
                    self.upsert(match_trace_key(frame_a, frame_b, query), ReconstructionRunReason::MatchAsymmetric, ToolRunTraceSubject::Entity { entity: match_trace_key(frame_a, frame_b, query) & !KEY_MATCH });
                }
                EngineObservation::PairMatched { frame_a, frame_b, matches } => self.step(ReconstructionRunReason::PairMatched, &[frame_a as u64, frame_b as u64, matches as u64]),
                EngineObservation::PairGeometryRejected { frame_a, frame_b, dropped } => {
                    self.count(ReconstructionRunCounter::MatchesRejected, dropped as i64);
                    self.step(ReconstructionRunReason::PairGeometryRejected, &[frame_a as u64, frame_b as u64, dropped as u64]);
                }
                EngineObservation::TracksBuilt { tracks } => self.step(ReconstructionRunReason::TracksBuilt, &[tracks as u64]),
                EngineObservation::CameraRegistered { frame, pose } => {
                    self.count(ReconstructionRunCounter::Cameras, 1);
                    let preview = camera_pose_preview(frame as u32, &pose);
                    let [w, x, y, z] = preview.rotation_wxyz;
                    self.upsert(camera_trace_key(frame), ReconstructionRunReason::CameraRegistered, ToolRunTraceSubject::Instance3d { mesh: RECONSTRUCTION_TRACE_CAMERA_MESH, position: preview.translation, rotation: [x, y, z, w], scale: RECONSTRUCTION_TRACE_CAMERA_SCALE });
                    self.step(ReconstructionRunReason::CameraRegistered, &[frame as u64]);
                }
                EngineObservation::CameraRejected { frame } => {
                    self.upsert(camera_trace_key(frame), ReconstructionRunReason::CameraRejected, ToolRunTraceSubject::Entity { entity: frame as u64 });
                    self.step(ReconstructionRunReason::CameraRejected, &[frame as u64]);
                }
                EngineObservation::SeedPairFailed { .. } => self.step(ReconstructionRunReason::SeedPairFailed, &[]),
                EngineObservation::PointTriangulated { track, point } => {
                    triangulated += 1;
                    self.count(ReconstructionRunCounter::SparsePoints, 1);
                    self.upsert(point_trace_key(track), ReconstructionRunReason::PointsTriangulated, point_subject(point));
                }
                EngineObservation::PointPruned { track, point } => {
                    pruned += 1;
                    self.count(ReconstructionRunCounter::SparsePoints, -1);
                    self.upsert(point_trace_key(track), ReconstructionRunReason::PointsPruned, point_subject(point));
                }
                EngineObservation::DepthMapEstimated { view, samples } => self.step(ReconstructionRunReason::DepthMapEstimated, &[view as u64, samples as u64]),
                EngineObservation::DenseCloudFused { points } => {
                    self.step(ReconstructionRunReason::DenseCloudFused, &[points as u64]);
                    if points > RECONSTRUCTION_DENSE_TRACE_MAX {
                        self.step(ReconstructionRunReason::DenseTraceCapped, &[RECONSTRUCTION_DENSE_TRACE_MAX as u64, points as u64]);
                    }
                }
                EngineObservation::MeshExtracted { vertices, triangles } => self.step(ReconstructionRunReason::MeshExtracted, &[vertices as u64, triangles as u64]),
            }
        }
        self.observations = observations;
        if triangulated > 0 {
            self.step(ReconstructionRunReason::PointsTriangulated, &[triangulated]);
        }
        if pruned > 0 {
            self.step(ReconstructionRunReason::PointsPruned, &[pruned]);
        }
        evidence
    }

    /// 📥️ One bounded ingestion slice; `Some(reason)` once a frame verdict was taken.
    pub(super) fn ingest(&mut self) -> Result<Option<(u64, ReconstructionRunReason)>, ()> {
        let RunPhase::Ingest { stream, frame, ingestion } = &mut self.phase else { return Err(()) };
        let scene = self.scene.as_ref().ok_or(())?;
        let Some(current) = ingestion.as_mut() else {
            let Some(stream_ref) = scene.streams.get(*stream) else {
                self.scene = None;
                self.phase = RunPhase::Pipeline;
                return Ok(None);
            };
            let Some(frame_ref) = stream_ref.frames.get(*frame) else {
                *stream += 1;
                *frame = 0;
                return Ok(None);
            };
            let index = u64::from(frame_ref.index);
            *frame += 1;
            return Ok(match frame_ingestion(scene, frame_ref) {
                Some(next) => {
                    *ingestion = Some(next);
                    None
                }
                None => Some((index, ReconstructionRunReason::FrameUnreadable)),
            });
        };
        let index = u64::from(current.frame.index);
        if let Some(decoder) = current.decoder.as_mut() {
            match decoder.advance() {
                BoundedDecodeProgress::Working => return Ok(None),
                BoundedDecodeProgress::Complete(image) => {
                    current.decoder = None;
                    current.image = Some(image);
                    return Ok(None);
                }
                BoundedDecodeProgress::Failed(_) => {
                    *ingestion = None;
                    return Ok(Some((index, ReconstructionRunReason::FrameUnreadable)));
                }
            }
        }
        if !advance_frame_sharpness(current) {
            return Ok(None);
        }
        let done = ingestion.take().expect("scored frame");
        let image = done.image.expect("decoded frame");
        let pixels = image.width as usize * image.height as usize;
        let sharpness = if pixels == 0 { 0.0 } else { (done.sharpness_sum / pixels as f64) as f32 };
        let engine = self.engine.as_mut().ok_or(())?;
        Ok(Some((
            index,
            match engine.push_frame_with_sharpness(done.frame.index, image, done.frame.timestamp_ms, sharpness) {
                FrameAcceptance::Accepted => ReconstructionRunReason::FrameAccepted,
                FrameAcceptance::RejectedStride | FrameAcceptance::RejectedMaxFrames => ReconstructionRunReason::FrameSkipped,
                FrameAcceptance::RejectedBlur => ReconstructionRunReason::FrameBlurred,
            },
        )))
    }

    fn fault(&mut self, reason: ReconstructionRunReason, args: &[u64]) {
        self.step(reason, args);
        self.phase = RunPhase::Settled;
        self.owed.push_back(Owed::Fault);
    }

    /// 🦶️ One bounded unit of the current phase; `true` when it completed a decision.
    pub(super) fn advance(&mut self) -> Result<bool, &'static str> {
        match self.phase {
            RunPhase::Ingest { .. } => self.advance_ingest(),
            RunPhase::Pipeline => self.advance_pipeline(),
            RunPhase::DenseTrace { cursor } => self.advance_dense(cursor),
            RunPhase::Products(_) => self.advance_products(),
            RunPhase::Settled => Ok(false),
        }
    }

    fn advance_ingest(&mut self) -> Result<bool, &'static str> {
        match self.ingest() {
            Ok(Some((frame, reason))) => {
                self.frames_decided += 1;
                let counter = if reason == ReconstructionRunReason::FrameAccepted { ReconstructionRunCounter::FramesAccepted } else { ReconstructionRunCounter::FramesRejected };
                self.count(counter, 1);
                self.upsert(frame_trace_key(frame), reason, ToolRunTraceSubject::Entity { entity: frame });
                self.step(reason, &[frame]);
                Ok(true)
            }
            Ok(None) => Ok(false),
            Err(()) => Err("remodeling.reconstruction.ingest"),
        }
    }

    fn advance_pipeline(&mut self) -> Result<bool, &'static str> {
        let engine = self.engine.as_mut().ok_or("remodeling.reconstruction.engine")?;
        let status = engine.advance(1);
        if engine.stage() != EngineStage::Failed {
            self.stage = ReconstructionRunStage::of(engine.stage());
        }
        let evidence = self.observe();
        match status {
            EngineStatus::Working { .. } => Ok(evidence),
            EngineStatus::Done => {
                // 📈️ The dense trace replays the fused cloud AFTER meshing finished, so naming its
                // own stage would walk the reported stage backwards (Surface -> Fusion); the run's
                // visible stage only ever moves forward.
                self.stage = self.stage.max(ReconstructionRunStage::Fusion);
                self.phase = RunPhase::DenseTrace { cursor: 0 };
                Ok(evidence)
            }
            EngineStatus::Failed(_) => {
                let accepted = self.counters[usize::from(ReconstructionRunCounter::FramesAccepted.index())];
                if accepted < 2 {
                    self.fault(ReconstructionRunReason::TooFewFrames, &[accepted]);
                } else {
                    self.fault(ReconstructionRunReason::PipelineFailed, &[u64::from(self.stage.index()) + 1]);
                }
                Ok(true)
            }
        }
    }

    fn advance_dense(&mut self, cursor: usize) -> Result<bool, &'static str> {
        let engine = self.engine.as_ref().ok_or("remodeling.reconstruction.engine")?;
        let total = engine.dense_positions().len().min(RECONSTRUCTION_DENSE_TRACE_MAX);
        if cursor >= total {
            self.stage = ReconstructionRunStage::Products;
            self.phase = RunPhase::Products(Box::new(ProductPreparation::new(self.gcp_count)));
            return Ok(false);
        }
        let end = (cursor + RECONSTRUCTION_DENSE_TRACE_BATCH).min(total);
        let batch: Vec<[f64; 3]> = engine.dense_positions()[cursor..end].to_vec();
        self.phase = RunPhase::DenseTrace { cursor: end };
        for (offset, point) in batch.into_iter().enumerate() {
            self.count(ReconstructionRunCounter::DensePoints, 1);
            self.upsert(KEY_DENSE | (cursor + offset) as u64, ReconstructionRunReason::DenseCloudFused, point_subject(point));
        }
        Ok(true)
    }

    fn advance_products(&mut self) -> Result<bool, &'static str> {
        let (RunPhase::Products(preparation), Some(engine)) = (&mut self.phase, self.engine.as_mut()) else { return Err("remodeling.reconstruction.products") };
        match preparation.advance(engine) {
            ProductYield::Working => Ok(false),
            ProductYield::Step(reason, args) => {
                self.step(reason, &args);
                Ok(false)
            }
            ProductYield::Op(op) => {
                self.append(&op)?;
                self.owed.push_back(Owed::Checkpoint);
                Ok(true)
            }
            ProductYield::Failed(reason) => {
                self.fault(reason, &[]);
                Ok(true)
            }
            ProductYield::Done => {
                self.step(ReconstructionRunReason::ResultReady, &[u64::from(self.ops_emitted)]);
                self.phase = RunPhase::Settled;
                self.owed.push_back(Owed::Checkpoint);
                self.owed.push_back(Owed::Complete);
                Ok(true)
            }
        }
    }

    fn begin_preview(&mut self, state: ToolRunState) -> bool {
        if self.writer.is_empty() || !self.live() {
            return false;
        }
        self.writer.progress(self.progress(state));
        let Some(bytes) = self.writer.finish().and_then(|tick| tick.encode().ok()) else { return false };
        self.outbox.begin(JobPublicationKind::Preview, bytes);
        true
    }

    fn settle<'a>(&'a mut self, cx: &mut StepContext<'_>, owed: Owed) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        match owed {
            Owed::Checkpoint => {
                self.outbox.begin(JobPublicationKind::Checkpoint { applied_progress: self.decisions }, self.checkpoint().encode().to_vec());
                cx.consume_fuel(1);
                Ok(None)
            }
            Owed::Complete => JobOutcomeBorrow::admit_complete(cx, None, None),
            Owed::Fault => {
                self.outbox.begin(JobPublicationKind::Fault, Vec::new());
                cx.consume_fuel(1);
                Ok(None)
            }
        }
    }
}

impl ReconstructionRunJob {
    /// 🪜️ The step body; its value refusals are lifted into the original failure at the trait boundary.
    fn step_value<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        if self.closing || cx.is_cancelled() {
            return JobOutcomeBorrow::admit_cancelled(cx);
        }
        match self.outbox.phase(cx)? {
            OutboxPhase::Building => return self.outbox.advance(cx),
            OutboxPhase::Delivered | OutboxPhase::Retiring => {
                self.outbox.retire_step(cx)?;
                return Ok(None);
            }
            OutboxPhase::Idle => {}
        }
        let mut advanced = false;
        loop {
            let terminal = self.owed.iter().any(|owed| matches!(owed, Owed::Complete | Owed::Fault));
            if !self.owed.is_empty() && (self.live() || terminal) {
                let state = if self.owed.contains(&Owed::Complete) { ToolRunState::Complete } else { ToolRunState::Running };
                if self.begin_preview(state) {
                    cx.consume_fuel(1);
                    return Ok(None);
                }
                let owed = self.owed.pop_front().expect("owed outcome");
                return self.settle(cx, owed);
            }
            if !self.live() {
                self.owed.clear();
            }
            if advanced && (cx.deadline_exceeded() || (self.live() && (cx.fuel_exhausted() || self.writer.pending_bytes() >= TICK_FLUSH_BYTES))) {
                if self.begin_preview(ToolRunState::Running) {
                    cx.consume_fuel(1);
                    return Ok(None);
                }
                return JobOutcomeBorrow::admit_yield(cx);
            }
            advanced = true;
            match self.advance() {
                Ok(true) => self.decide(cx),
                Ok(false) => {}
                Err(_) => {
                    self.phase = RunPhase::Settled;
                    self.owed.push_back(Owed::Fault);
                }
            }
        }
    }
}

impl InteractiveJob for ReconstructionRunJob {
    fn step<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{self.step_value(cx).map_err(semio_framework_job::OriginalTurnFailure::from)}

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        borrow_remodel_outcome(&self.outbox, descriptor)
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|grant|{
            self.closing = true;
            if grant.maximum_items == 0 || grant.maximum_depth == 0 {
                return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
            }
            if let Some(step) = close_outbox_turn(&mut self.outbox, grant)? {
                return Ok(step);
            }
            if let Some(owner) = self.remnant.as_mut() {
                let step = owner.step(grant);
                if owner.terminal_is_empty() {
                    self.remnant = None;
                }
                return Ok(match step {
                    Ok(RetainedCloneStep::Complete(progress)) => InteractiveJobCloseStep::Complete { progress },
                    Ok(RetainedCloneStep::Progress(progress)) => InteractiveJobCloseStep::Pending { progress },
                    Err(error) => return Err(error),
                });
            }
            if matches!(self.phase, RunPhase::Settled) && self.engine.is_none() && self.scene.is_none() && self.observations.is_empty() && self.owed.is_empty() {
                return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() });
            }
            let remnant = RunRemnant { phase: std::mem::replace(&mut self.phase, RunPhase::Settled), engine: self.engine.take(), scene: self.scene.take(), observations: std::mem::take(&mut self.observations), owed: std::mem::take(&mut self.owed) };
            Ok(match semio_framework_value::retirement::controlled::ControlledRetirement::new(remnant) {
                Ok(owner) => {
                    self.remnant = Some(owner);
                    InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } }
                }
                Err((error, remnant)) => {
                    self.phase = remnant.phase;
                    self.engine = remnant.engine;
                    self.scene = remnant.scene;
                    self.observations = remnant.observations;
                    self.owed = remnant.owed;
                    return Err(error)
                }
            })
        })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        self.remnant.as_ref().map_or_else(|| Ok(self.outbox.retirement_demands()?.copy_bytes), |owner| owner.next_copy_byte_demand())
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError> {
        self.remnant.as_ref().map_or_else(|| Ok(self.outbox.retirement_demands()?.capacity_bytes), |owner| owner.next_capacity_byte_demand(maximum_copy_bytes))
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        self.remnant.as_ref().map_or_else(|| Ok(self.outbox.retirement_demands()?.release_bytes), |owner| owner.next_release_byte_demand())
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        if let Some(owner) = self.remnant.as_ref() {
            return owner.next_depth_demand();
        }
        let publication = self.outbox.retirement_demands()?.depth;
        Ok(if publication == 0 { usize::from(!self.terminal_is_empty()) } else { publication })
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.remnant.is_none() && matches!(self.phase, RunPhase::Settled) && self.outbox.terminal_is_empty() && self.engine.is_none() && self.scene.is_none() && self.observations.is_empty() && self.owed.is_empty()
    }
}
//#endregion 🧵️RunJob

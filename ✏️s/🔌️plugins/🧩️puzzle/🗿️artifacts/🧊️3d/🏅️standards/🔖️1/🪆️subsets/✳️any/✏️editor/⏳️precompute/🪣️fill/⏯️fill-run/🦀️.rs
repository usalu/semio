//! ⏯️ Fill run: the tool run job that steps the planner, publishes every verdict as a trace upsert and keeps accepted placements as provisional ops.

use super::*;

//#region ⏯️FillRunJob
/// 🚰️ Estimated pending tick bytes past which a fill run job flushes: one tick must fit one
/// `JOB_PAYLOAD_PAGE_BYTES` job payload page with a placement's ops to spare.
pub const FILL_RUN_TICK_FLUSH_BYTES: usize = 8 * 1024;
/// 🧬️ Provisional ops one accepted placement appends: `create_object` then `connect_vortices`.
pub const FILL_RUN_OPS_PER_PLACEMENT: u32 = 2;
/// 🎯️ Trace scale of a marked vortex over the vortex marker mesh, so an exhausted vortex stands out from the open ones.
pub const FILL_RUN_MARKED_VORTEX_SCALE: f32 = 2.0;

/// 🆔️ Stable provisional entity of a placed object: the first eight little-endian bytes of its id digest.
pub fn fill_run_entity(object_id: &str) -> u64 {
    let digest = semio_framework_hash::hash(object_id.as_bytes());
    u64::from_le_bytes(digest.as_bytes()[..8].try_into().expect("eight digest bytes"))
}

/// 🧬️ The two `OpBinary` document ops one placement contributes, in append order.
pub fn fill_run_ops(object: &EngineSceneObject, attraction: &AttractionProps, peers: &[Puzzle3dObject], catalog_snapshot: &Puzzle3dSceneSnapshot) -> Option<[Vec<u8>; 2]> {
    use semio_s_artifact_puzzle_3d_mutations::io::binary::mutations::encode_op;
    use {semio_s_artifact_puzzle_3d_mutations_vortex::connect_vortices::mutation::connect_vortices, semio_s_artifact_puzzle_3d_mutations_object::create_object::mutation::create_object};

    let mut document: semio_s_artifact_puzzle_3d_model::Puzzle3dObject = semio_framework_value::FromValue::from_value(semio_framework_value::ToValue::to_value(object)).ok()?;
    let kind_id = object.object_kind.as_deref().unwrap_or("object");
    if document.label.as_deref().map(str::is_empty).unwrap_or(true) {
        document.label = Some(puzzle3d_next_object_label(peers, catalog_snapshot, kind_id));
    }
    let create = encode_op(&create_object(document, None)).ok()?;
    let connect = encode_op(&connect_vortices(attraction.id.clone(), attraction.attracting.clone(), attraction.attracted.clone(), attraction.gap, attraction.shift, attraction.rise, attraction.rotation, attraction.turn, attraction.tilt, attraction.x, attraction.y, None)).ok()?;
    Some([create, connect])
}

/// 🧱️ The placements a provisional op list holds, in op order (`create_object` then `connect_vortices`
/// per placement), keyed consecutively from `first_key` — a key range the run's trace key allocator handed out
/// (`ToolRunJobRequest::trace_keys`), since the run's own candidate keys are not recoverable from ops — with
/// subjects indexing `mesh_lane`; `None` when the list is not a whole sequence of fill placements.
pub fn fill_run_placements(provisional: &[semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation], mesh_lane: &[String], first_key: u64) -> Option<Vec<FillRunPlacement>> {
    use semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation;
    provisional
        .chunks(FILL_RUN_OPS_PER_PLACEMENT as usize)
        .enumerate()
        .map(|(index, pair)| {
            let [Puzzle3dMutation::CreateObject(create), Puzzle3dMutation::ConnectVortices(connect)] = pair else { return None };
            let object = <EngineSceneObject as semio_framework_value::FromValue>::from_value(semio_framework_value::ToValue::to_value(&create.object)).ok()?;
            let mesh = object.mesh_url.as_ref().and_then(|url| mesh_lane.iter().position(|entry| entry == url)).unwrap_or(0) as u32;
            let subject = ToolRunTraceSubject::Instance3d { mesh, position: object.origin.map(|value| value as f32), rotation: object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]).map(|value| value as f32), scale: fill_run_scale(&object.scale) };
            let attraction = AttractionProps { id: connect.id.clone(), attracting: connect.attracting.clone(), attracted: connect.attracted.clone(), gap: connect.gap, shift: connect.shift, rise: connect.rise, rotation: connect.rotation, turn: connect.turn, tilt: connect.tilt, x: connect.x, y: connect.y };
            Some(FillRunPlacement { key: first_key + index as u64, subject, entity: fill_run_entity(&object.id), object, attraction })
        })
        .collect()
}

/// 🧱️ One provisional placement of a fill run: its trace key, entity, object and attraction.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct FillRunPlacement {
    pub key: u64,
    pub subject: ToolRunTraceSubject,
    pub entity: u64,
    pub object: EngineSceneObject,
    pub attraction: AttractionProps,
}

/// 🧭️ The run stage a planner stage belongs to.
fn fill_run_stage_of(stage: FillJobStage) -> Option<FillRunStage> {
    match stage {
        FillJobStage::RetractTail => Some(FillRunStage::Retract),
        FillJobStage::PrepareScene | FillJobStage::PrepareCatalogs | FillJobStage::PrepareMeshes | FillJobStage::PrepareEntries | FillJobStage::PrepareSpatial | FillJobStage::PrepareLookup | FillJobStage::PrepareConfiguration => Some(FillRunStage::Prepare),
        FillJobStage::PrepareTargets | FillJobStage::SelectTarget | FillJobStage::PrepareCandidates | FillJobStage::SelectCandidate => Some(FillRunStage::Search),
        FillJobStage::ConstructPreview | FillJobStage::QueryBroadPhase | FillJobStage::TestCollision => Some(FillRunStage::Test),
        FillJobStage::AcceptCandidate => Some(FillRunStage::Lock),
        FillJobStage::Complete(_) => None,
    }
}

/// ⏳️ Planner transitions inside one run job step: the step's deadline and cancellation, but neither
/// its fuel — a run job's fuel counts candidates, never transitions or collision samples — nor its job
/// operation: the planner answers to its own `operation`, while the framework tool run ledger guards the
/// job by run identity and generation.
pub(super) struct FillRunTransitionContext<'a, 'b> {
    pub(super) outer: &'a mut StepContext<'b>,
    pub operation: Operation,
}

impl CollisionStepContext for FillRunTransitionContext<'_, '_> {
    fn is_cancelled(&self) -> bool {
        self.outer.is_cancelled()
    }

    fn should_yield(&self) -> bool {
        self.outer.deadline_exceeded()
    }

    fn consume_fuel(&mut self, _units: u64) {}
}

impl FillStepContext for FillRunTransitionContext<'_, '_> {
    fn operation(&self) -> OperationId {
        self.operation.operation
    }

    fn generation(&self) -> Generation {
        self.operation.generation
    }

    fn set_stage(&mut self, label: &'static str) {
        self.outer.set_stage(label);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FillRunOwed {
    Checkpoint,
    Complete,
}

/// 🧾️ Test-only record of one verdict the run reached, with what the oracle needs to recompute it.
#[cfg(any(test, feature = "testing"))]
#[derive(Clone, Debug, semio_framework_value::RetireOwned)]
pub struct FillRunVerdictRecord {
    pub key: u64,
    pub reason: FillRunReason,
    pub mesh_url: String,
    pub origin: [f64; 3],
    pub orientation: [f64; 4],
    pub host: Option<String>,
    pub placements_before: usize,
}

/// ⏯️ The puzzle 3d fill tool run job (`ToolRunDefinition.runJob`): drives [`FillBuilder`] and reports
/// every constructed candidate as a `testing` trace record that turns `danger` (collision), `warning`
/// (rule refusal) or `success` (placed); every placement appends its `create_object` +
/// `connect_vortices` ops and entity. One unit of fuel is one candidate verdict. Ticks travel only in
/// `PreviewReady` pages; `CheckpointReady` (after each placement) and `Complete` are returned by the
/// call after the tick that led to them. [`FillRunJob::resume`] with a new requested count continues the
/// same deterministic sequence (raise) or retracts the tail (lower); a job rebuilt after the framework
/// closed its predecessor reaches that checkpoint first by a silent replay ([`FillRunJob::replaying`]).
#[derive(semio_framework_value::RetireOwned)]
struct FillRunClosingOwners {
    writer: ToolRunTickWriter,
    mesh_lane: Vec<String>,
    mesh_index: semio_framework_value::retained_clone::ordered_map::RetainedOrderedMap<String, u32>,
    events: Vec<FillRunEvent>,
    deferred: Vec<FillRunEvent>,
    placement_keys: Vec<(u64, ToolRunTraceSubject)>,
    #[cfg(any(test, feature = "testing"))]
    verdicts: Vec<FillRunVerdictRecord>,
    #[cfg(any(test, feature = "testing"))]
    live_record: Option<FillRunVerdictRecord>,
}

pub struct FillRunJob {
    close_started: bool,
    close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<FillRunClosingOwners>,
    pub(super) outbox: semio_s_plugin_puzzle_jobs::JobOutbox,
    pub builder: FillBuilder,
    pub(super) writer: ToolRunTickWriter,
    pub inputs: [u8; 32],
    replay: Option<FillRunReplay>,
    mesh_lane: Vec<String>,
    mesh_index: semio_framework_value::retained_clone::ordered_map::RetainedOrderedMap<String, u32>,
    pub(super) events: Vec<FillRunEvent>,
    deferred: Vec<FillRunEvent>,
    live: Option<(u64, ToolRunTraceSubject)>,
    /// 👁️ The one tested candidate whose verdict is still on screen. The run shows ONE candidate at a time: the next
    /// constructed candidate retires it, so a collision is visible only while it is the current attempt and a placement
    /// stays visible as its document instance (the `provisional` style), never as a pile of past trace records.
    pub(super) shown: Option<u64>,
    pub next_key: u64,
    pub placement_keys: Vec<(u64, ToolRunTraceSubject)>,
    tested: u64,
    marked: u64,
    pub collisions: u64,
    pub(super) rejected: u64,
    pub stage: FillRunStage,
    settled: bool,
    capped: bool,
    owed: Option<FillRunOwed>,
    progress_sequence: u64,
    #[cfg(any(test, feature = "testing"))]
    pub verdicts: Vec<FillRunVerdictRecord>,
    #[cfg(any(test, feature = "testing"))]
    live_record: Option<FillRunVerdictRecord>,
}

/// ⏪️ A rebuilt run job's silent way back to its predecessor's checkpoint: every planner transition runs,
/// no tick leaves the job, and on arrival the writer continues from the ledger's provisional length.
#[derive(Clone, Copy, Debug)]
pub(super) struct FillRunReplay {
    pub(super) target: FillRunCheckpoint,
    pub requested: usize,
    provisional: u32,
}

#[cfg(any(test, feature = "testing"))]
/// 🚫️ A checkpoint that does not belong to this resident run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillRunResumeError {
    Malformed,
    Foreign,
}

impl FillRunJob {
    /// 🚦️ The one production construction path of a fill run: begins the cooperative preparation and picks
    /// replay (a `checkpoint` with the same `inputs`), restart (a predecessor left `provisional` ops or was
    /// reconfigured) or a fresh run.
    pub fn start(roots: FillPreparationRoots, operation: Operation, identity: ToolRunIdentity, mesh_lane: Vec<String>, inputs: [u8; 32], requested: usize, checkpoint: Option<FillRunCheckpoint>, provisional: u32) -> Self {
        match checkpoint.filter(|checkpoint| checkpoint.inputs == inputs) {
            Some(checkpoint) => Self::replaying(FillBuilder::begin_preparation(roots, operation, checkpoint.requested as usize), identity, mesh_lane, checkpoint, requested, provisional),
            None if provisional > 0 || identity.generation > 0 => Self::restarting(FillBuilder::begin_preparation(roots, operation, requested), identity, mesh_lane, inputs, provisional),
            None => Self::new(FillBuilder::begin_preparation(roots, operation, requested), identity, mesh_lane, inputs),
        }
    }

    /// 🎬️ Wraps a freshly begun planner; `mesh_lane` is the plugin mesh lane trace subjects index into
    /// (a url missing from it is appended and reported through [`FillRunJob::mesh_lane`]).
    pub fn new(builder: FillBuilder, identity: ToolRunIdentity, mesh_lane: Vec<String>, inputs: [u8; 32]) -> Self {
        Self::with_writer(builder, ToolRunTickWriter::new(identity), mesh_lane, inputs)
    }

    /// 🔁️ A fresh run that replaces a predecessor whose sequence it cannot reproduce (other inputs or no
    /// checkpoint): its first tick clears the trace and retracts all `provisional` ops the ledger holds.
    pub fn restarting(builder: FillBuilder, identity: ToolRunIdentity, mesh_lane: Vec<String>, inputs: [u8; 32], provisional: u32) -> Self {
        let mut job = Self::with_writer(builder, ToolRunTickWriter::with_provisional_base(identity, provisional), mesh_lane, inputs);
        job.writer.clear_trace();
        job.writer.retract_to(0);
        job
    }

    /// ⏪️ A run rebuilt from `checkpoint` (same `inputs`): `builder` must be begun with the checkpoint's own
    /// requested count; it replays silently to the checkpoint, retracts any `provisional` op the ledger holds
    /// past it, and then resumes toward `requested`. A replay that overshoots the checkpoint restarts.
    pub fn replaying(builder: FillBuilder, identity: ToolRunIdentity, mesh_lane: Vec<String>, checkpoint: FillRunCheckpoint, requested: usize, provisional: u32) -> Self {
        let mut job = Self::with_writer(builder, ToolRunTickWriter::new(identity), mesh_lane, checkpoint.inputs);
        job.replay = Some(FillRunReplay { target: checkpoint, requested, provisional });
        job
    }

    fn with_writer(mut builder: FillBuilder, writer: ToolRunTickWriter, mesh_lane: Vec<String>, inputs: [u8; 32]) -> Self {
        builder.observe_run();
        let mesh_index = mesh_lane.iter().enumerate().map(|(index, url)| (url.clone(), index as u32)).collect();
        Self {
            close_started: false,
            close_owners: Default::default(),
            outbox: Default::default(),
            builder,
            writer,
            inputs,
            replay: None,
            mesh_lane,
            mesh_index,
            events: Vec::with_capacity(4),
            deferred: Vec::new(),
            live: None,
            shown: None,
            next_key: 0,
            placement_keys: Vec::new(),
            tested: 0,
            marked: 0,
            collisions: 0,
            rejected: 0,
            stage: FillRunStage::Prepare,
            settled: false,
            capped: false,
            owed: None,
            progress_sequence: 0,
            #[cfg(any(test, feature = "testing"))]
            verdicts: Vec::new(),
            #[cfg(any(test, feature = "testing"))]
            live_record: None,
        }
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn builder(&self) -> &FillBuilder {
        &self.builder
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn operation(&self) -> Operation {
        self.builder.operation
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn mesh_lane(&self) -> &[String] {
        &self.mesh_lane
    }

    /// 📟️ `[tested, locked, collisions, rejected, marked]`, in [`FillRunCounter::ALL`] order.
    pub fn counters(&self) -> [u64; 5] {
        [self.tested, self.placement_keys.len() as u64, self.collisions, self.rejected, self.marked]
    }

    /// 📸️ Where this run stands right now.
    pub fn checkpoint(&self) -> FillRunCheckpoint {
        FillRunCheckpoint { requested: self.builder.requested_count() as u64, placements: self.placement_keys.len() as u64, provisional_ops: self.writer.provisional_len(), tested: self.tested, next_key: self.next_key, inputs: self.inputs }
    }

    #[cfg(any(test, feature = "testing"))]
    /// 🧱️ The provisional placements in op order, for finalize revalidation.
    pub fn provisional_placements(&self) -> Vec<FillRunPlacement> {
        self.placement_keys
            .iter()
            .zip(self.builder.appended_objects.iter().zip(&self.builder.appended_attractions))
            .map(|((key, subject), (object, attraction))| FillRunPlacement { key: *key, subject: *subject, entity: fill_run_entity(&object.id), object: object.clone(), attraction: attraction.clone() })
            .collect()
    }

    #[cfg(any(test, feature = "testing"))]
    /// ⏩️ Resumes this resident run from one of its own checkpoints with a new requested count.
    pub fn resume(&mut self, checkpoint: &[u8], requested: usize) -> Result<(), FillRunResumeError> {
        let checkpoint = FillRunCheckpoint::decode(checkpoint).ok_or(FillRunResumeError::Malformed)?;
        let current = self.checkpoint();
        if checkpoint.inputs != self.inputs || checkpoint.next_key > current.next_key || checkpoint.tested > current.tested || checkpoint.placements > self.builder.sequence.len() as u64 {
            return Err(FillRunResumeError::Foreign);
        }
        self.retarget(self.writer.identity(), requested);
        Ok(())
    }

    /// 🪢️ Every later tick carries `identity` (the framework rebound the run after a base change).
    pub fn rebind(&mut self, identity: ToolRunIdentity) {
        self.writer.rebind(identity);
    }

    /// 🎯️ Retargets this resident run in place (framework `reconfigure: resume`): later ticks carry `identity` and
    /// the plan heads for `requested` — a raise continues the same deterministic sequence, a lower retracts its
    /// tail, and a completed run wakes up again; a run still replaying adopts `requested` on arrival.
    pub fn retarget(&mut self, identity: ToolRunIdentity, requested: usize) {
        self.writer.rebind(identity);
        if let Some(replay) = self.replay.as_mut() {
            replay.requested = requested;
            return;
        }
        self.builder.set_requested_count(requested);
        if !matches!(self.builder.stage, FillJobStage::Complete(_)) {
            self.settled = false;
            if self.owed == Some(FillRunOwed::Complete) {
                self.owed = None;
            }
        }
    }

    pub fn mesh(&mut self, url: &str) -> u32 {
        if let Some(index) = self.mesh_index.get(url) {
            return *index;
        }
        let index = self.mesh_lane.len() as u32;
        self.mesh_lane.push(url.to_string());
        self.mesh_index.cold_insert(url.to_string(), index);
        index
    }

    fn progress_snapshot(&mut self) -> ToolRunProgress {
        self.progress_sequence += 1;
        let counters = self.counters();
        ToolRunProgress {
            identity: self.writer.identity(),
            sequence: self.progress_sequence,
            state: if matches!(self.builder.stage, FillJobStage::Complete(_)) { ToolRunState::Complete } else { ToolRunState::Running },
            stage: self.stage.index(),
            completed: self.placement_keys.len() as u64,
            total: Some(self.builder.requested_count() as u64),
            counters: FillRunCounter::ALL.iter().zip(counters).map(|(counter, value)| ToolRunCounter { counter: counter.index(), value }).collect(),
            units_per_second: 0.0,
            conflicts: 0,
            steps: ToolRunStepRing::default(),
        }
    }

    pub fn verdict(&mut self, context: &mut StepContext<'_>, reason: FillRunReason, verdict: ToolRunVerdict, code: u16) -> Option<(u64, ToolRunTraceSubject)> {
        let (key, subject) = self.live.take()?;
        self.writer.upsert(key, verdict, code, subject);
        self.shown = Some(key);
        if self.replay.is_none() {
            context.consume_fuel(1);
        }
        #[cfg(any(test, feature = "testing"))]
        if let Some(mut record) = self.live_record.take() {
            record.reason = reason;
            self.verdicts.push(record);
        }
        let _ = reason;
        Some((key, subject))
    }

    /// 🔭️ Folds the planner's observations into the tick. A constructed candidate is one visible unit of fuel, like its
    /// verdict: when it exhausts the step's fuel, the observations after it wait in `deferred` for the next step, so a
    /// one-unit step (a paced run, a single step) publishes the candidate under test before the verdict that marks it.
    fn observe(&mut self, context: &mut StepContext<'_>) -> Result<bool, &'static [u8]> {
        if self.deferred.is_empty() {
            self.builder.swap_run_events(&mut self.events);
        } else {
            std::mem::swap(&mut self.events, &mut self.deferred);
            self.deferred.clear();
        }
        let mut events = std::mem::take(&mut self.events);
        let mut accepted = false;
        let mut cursor = 0;
        while let Some(event) = events.get(cursor) {
            cursor += 1;
            match event {
                FillRunEvent::Constructed { mesh_url, origin, orientation, scale } => {
                    if let Some(previous) = self.shown.take() {
                        self.writer.retire(previous);
                    }
                    let key = self.next_key;
                    self.next_key += 1;
                    self.tested += 1;
                    let subject = ToolRunTraceSubject::Instance3d { mesh: self.mesh(mesh_url), position: origin.map(|value| value as f32), rotation: orientation.map(|value| value as f32), scale: *scale };
                    self.writer.upsert(key, ToolRunVerdict::Testing, FillRunReason::Fits.code(), subject);
                    self.live = Some((key, subject));
                    #[cfg(any(test, feature = "testing"))]
                    {
                        self.live_record = Some(FillRunVerdictRecord { key, reason: FillRunReason::Fits, mesh_url: mesh_url.clone(), origin: *origin, orientation: *orientation, host: self.builder.current_target.as_ref().map(|target| target.object_id.clone()), placements_before: self.builder.sequence.len() });
                    }
                    if self.replay.is_none() {
                        context.consume_fuel(1);
                        if context.fuel_exhausted() && cursor < events.len() {
                            self.deferred = events.split_off(cursor);
                            break;
                        }
                    }
                }
                FillRunEvent::Refused(reason) => {
                    if self.verdict(context, *reason, reason.verdict(), reason.code()).is_some() {
                        if *reason == FillRunReason::SolidOverlap {
                            self.collisions += 1;
                        } else {
                            self.rejected += 1;
                        }
                    }
                }
                FillRunEvent::Accepted => {
                    let (Some(object), Some(attraction)) = (self.builder.appended_objects.last(), self.builder.appended_attractions.last()) else {
                        return Err(b"fill-run-placement-missing");
                    };
                    let peers = self.builder.label_peers();
                    let catalog_snapshot = self.builder.label_catalog_snapshot();
                    let Some(ops) = fill_run_ops(object, attraction, &peers, &catalog_snapshot) else {
                        return Err(b"fill-run-op-encode");
                    };
                    let entity = fill_run_entity(&object.id);
                    if self.writer.provisional_len() + FILL_RUN_OPS_PER_PLACEMENT > TOOL_RUN_PROVISIONAL_OPS_MAX {
                        let _ = self.verdict(context, FillRunReason::Rejected, ToolRunVerdict::Warning, TOOL_RUN_REASON_PROVISIONAL_CAP);
                        self.rejected += 1;
                        self.capped = true;
                        let _ = self.writer.step(ToolRunStepKind::Warning, self.stage.index(), TOOL_RUN_REASON_PROVISIONAL_CAP, None, &[ToolRunStepArg::Unsigned(u64::from(TOOL_RUN_PROVISIONAL_OPS_MAX))]);
                        self.builder.set_requested_count(self.builder.sequence.len().saturating_sub(1));
                        continue;
                    }
                    let Some(placement) = self.verdict(context, FillRunReason::Fits, ToolRunVerdict::Success, FillRunReason::Fits.code()) else {
                        return Err(b"fill-run-placement-untested");
                    };
                    let [create, connect] = ops;
                    if self.writer.append_op(create).and_then(|()| self.writer.append_op(connect)).is_err() {
                        return Err(b"fill-run-provisional-ops");
                    }
                    self.writer.append_entity(entity);
                    self.placement_keys.push(placement);
                    accepted = true;
                }
                FillRunEvent::VortexMarked { position } => {
                    let key = self.next_key;
                    self.next_key += 1;
                    let subject = ToolRunTraceSubject::Instance3d { mesh: self.mesh(VORTEX_MARKER_MESH_KIND), position: position.map(|value| value as f32), rotation: [0.0, 0.0, 0.0, 1.0], scale: FILL_RUN_MARKED_VORTEX_SCALE };
                    self.writer.upsert(key, ToolRunVerdict::Danger, FillRunReason::VortexExhausted.code(), subject);
                    self.marked += 1;
                    if self.replay.is_none() {
                        context.consume_fuel(1);
                    }
                }
                FillRunEvent::Abandoned => {
                    if let Some((key, _)) = self.live.take() {
                        self.writer.retire(key);
                    }
                    #[cfg(any(test, feature = "testing"))]
                    {
                        self.live_record = None;
                    }
                }
                FillRunEvent::Discarded => {
                    while self.placement_keys.len() > self.builder.sequence.len() {
                        if let Some((key, _)) = self.placement_keys.pop().filter(|(key, _)| self.shown == Some(*key)) {
                            self.writer.retire(key);
                            self.shown = None;
                        }
                    }
                    self.writer.retract_to(self.placement_keys.len() as u32 * FILL_RUN_OPS_PER_PLACEMENT);
                    let _ = self.writer.step(ToolRunStepKind::Info, FillRunStage::Retract.index(), FillRunReason::Retracted.code(), None, &[]);
                }
            }
        }
        self.events = events;
        if let Some(stage) = fill_run_stage_of(self.builder.stage) {
            self.stage = stage;
        }
        Ok(accepted)
    }

    /// 🏁️ The run's one terminal step, derived from how the plan ended: a declared stall's `warning`, or `success`
    /// `requested-reached` over a request the placements actually meet (a provisional-cap run already published its
    /// `warning`). A completed plan that satisfies neither is refused as `fill-run-end-undeclared` — the run faults
    /// visibly instead of ending without a reason.
    pub(super) fn settle(&mut self) -> Result<(), &'static [u8]> {
        if std::mem::replace(&mut self.settled, true) {
            return Ok(());
        }
        let placements = self.placement_keys.len() as u64;
        let placed = [ToolRunStepArg::Unsigned(placements)];
        let _ = match self.builder.end() {
            Some(FillPlanEnd::Stalled(stall)) => self.writer.step(ToolRunStepKind::Warning, self.stage.index(), stall.reason().code(), None, &placed),
            Some(FillPlanEnd::Reached) if self.capped => Ok(()),
            Some(FillPlanEnd::Reached) if placements >= self.builder.requested_count() as u64 => self.writer.step(ToolRunStepKind::Success, self.stage.index(), FillRunReason::RequestedReached.code(), None, &placed),
            Some(FillPlanEnd::Reached) | None => return Err(b"fill-run-end-undeclared"),
        };
        Ok(())
    }

    pub(super) fn flush(&mut self, _context: &mut StepContext<'_>) -> Option<JobTurn> {
        if self.writer.is_empty() {
            return None;
        }
        let progress = self.progress_snapshot();
        self.writer.progress(progress);
        let bytes = self.writer.finish()?.encode().ok();
        Some(match bytes {
            Some(bytes) if bytes.len() <= semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES => JobTurn::Preview(bytes.to_vec()),
            Some(_) => JobTurn::Fault(b"fill-run-tick-page".to_vec()),
            None => JobTurn::Fault(b"fill-run-tick-encode".to_vec()),
        })
    }

    fn flush_then(&mut self, context: &mut StepContext<'_>, owed: FillRunOwed) -> JobTurn {
        match self.flush(context) {
            Some(preview @ JobTurn::Preview(_)) => {
                self.owed = Some(owed);
                preview
            }
            Some(fault) => fault,
            None => self.settle_owed(context, owed),
        }
    }

    fn settle_owed(&mut self, _context: &mut StepContext<'_>, owed: FillRunOwed) -> JobTurn {
        match owed {
            FillRunOwed::Complete => self.builder.complete(),
            FillRunOwed::Checkpoint => JobTurn::Checkpoint { applied_progress: self.placement_keys.len() as u64, state: self.checkpoint().encode().to_vec() },
        }
    }
}

impl FillRunJob {
    /// ⏪️ Where a replay stands after one observed transition: arrived at its checkpoint (the writer now
    /// continues from the ledger's provisional length and the run resumes toward the requested count),
    /// overshot it (a restart over the ledger's provisional ops), or still on its way (pending bytes are
    /// dropped, never flushed).
    fn settle_replay(&mut self) {
        let Some(replay) = self.replay else { return };
        let target = replay.target;
        let placements = self.placement_keys.len() as u64;
        if placements == target.placements && self.tested == target.tested && self.next_key == target.next_key && self.writer.provisional_len() == target.provisional_ops {
            self.replay = None;
            self.writer = ToolRunTickWriter::with_provisional_base(self.writer.identity(), replay.provisional);
            if replay.provisional > target.provisional_ops {
                self.writer.retract_to(target.provisional_ops);
            }
            self.builder.set_requested_count(replay.requested);
            self.settled = false;
            return;
        }
        if placements > target.placements || self.tested > target.tested || self.next_key > target.next_key || matches!(self.builder.stage, FillJobStage::Complete(_)) {
            self.replay = None;
            self.writer = ToolRunTickWriter::with_provisional_base(self.writer.identity(), replay.provisional);
            self.writer.clear_trace();
            self.writer.retract_to(0);
            return;
        }
        if self.writer.pending_bytes() >= FILL_RUN_TICK_FLUSH_BYTES {
            let _ = self.writer.finish();
        }
    }
}

impl FillRunJob {
    pub fn turn(&mut self, context: &mut StepContext<'_>) -> JobTurn {
        if context.is_cancelled() {
            return JobTurn::Cancelled;
        }
        if let Some(owed) = self.owed.take() {
            return self.settle_owed(context, owed);
        }
        loop {
            if let Some(outcome) = self.builder.capacity_refusal(context, Some(&mut self.writer)) {
                return match outcome {
                    JobTurn::Yield => self.flush(context).unwrap_or(JobTurn::Yield),
                    outcome => outcome,
                };
            }
            if self.builder.stall_on_capacity() {
                if let Err(detail) = self.observe(context) {
                    return JobTurn::Fault(detail.to_vec());
                }
            }
            if self.replay.is_none() && self.deferred.is_empty() && matches!(self.builder.stage, FillJobStage::Complete(_)) {
                if let Err(detail) = self.settle() {
                    return JobTurn::Fault(detail.to_vec());
                }
                return self.flush_then(context, FillRunOwed::Complete);
            }
            if context.deadline_exceeded() || (self.replay.is_none() && (context.fuel_exhausted() || self.writer.pending_bytes() >= FILL_RUN_TICK_FLUSH_BYTES)) {
                return if self.replay.is_some() { JobTurn::Yield } else { self.flush(context).unwrap_or(JobTurn::Yield) };
            }
            let operation = self.builder.operation;
            let outcome = self.deferred.is_empty().then(|| self.builder.advance(&mut FillRunTransitionContext { outer: context, operation }));
            let accepted = match self.observe(context) {
                Ok(accepted) => accepted,
                Err(detail) => {
                    drop(outcome);
                    return JobTurn::Fault(detail.to_vec());
                }
            };
            match outcome {
                Some(JobTurn::Cancelled) => return JobTurn::Cancelled,
                Some(fault @ JobTurn::Fault(_)) => return fault,
                _ => {}
            }
            if self.replay.is_some() {
                self.settle_replay();
                continue;
            }
            if accepted {
                return self.flush_then(context, FillRunOwed::Checkpoint);
            }
        }
    }
}

impl FillRunJob {
    /// 🪜️ The step body; its value refusals are lifted into the original failure at the trait boundary.
    fn step_value<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        match self.outbox.phase(cx)? {
            semio_s_plugin_puzzle_jobs::OutboxPhase::Building => return self.outbox.advance(cx).map_err(semio_framework_job::OriginalTurnFailure::from),
            semio_s_plugin_puzzle_jobs::OutboxPhase::Delivered | semio_s_plugin_puzzle_jobs::OutboxPhase::Retiring => {
                self.outbox.retire_step(cx)?;
                return Ok(None);
            }
            semio_s_plugin_puzzle_jobs::OutboxPhase::Idle => {}
        }
        let turn = self.turn(cx);
        self.outbox.settle(turn, cx).map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

impl InteractiveJob for FillRunJob {
    fn step<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{self.step_value(cx).map_err(semio_framework_job::OriginalTurnFailure::from)}

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outbox.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        self.builder.begin_close();
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> {
        semio_framework_job::close_original_nested_step(cx, |cx, _grant| {

        self.begin_close();
        if !self.outbox.terminal_is_empty() {
            let demand = self.outbox.retirement_demands()?;
            return semio_framework_job::close_original_retained_step(cx, demand, |grant| self.outbox.close_step(grant));
        }
        if !self.builder.terminal_is_empty() {
            let step = self.builder.close_turn(cx)?;
            return Ok(InteractiveJobCloseStep::Pending { progress: step.progress() });
        }
        if !self.close_started {
            let demand = self.close_demands(cx.retained_grant().maximum_copy_bytes)?;
            return semio_framework_job::close_original_retained_step(cx, demand, |_| {
                let identity = self.writer.identity();
                self.close_owners.stage(FillRunClosingOwners {
                    writer: std::mem::replace(&mut self.writer, ToolRunTickWriter::new(identity)),
                    mesh_lane: std::mem::take(&mut self.mesh_lane),
                    mesh_index: std::mem::take(&mut self.mesh_index),
                    events: std::mem::take(&mut self.events),
                    deferred: std::mem::take(&mut self.deferred),
                    placement_keys: std::mem::take(&mut self.placement_keys),
                    #[cfg(any(test, feature = "testing"))]
                    verdicts: std::mem::take(&mut self.verdicts),
                    #[cfg(any(test, feature = "testing"))]
                    live_record: self.live_record.take(),
                });
                self.close_started = true;
                Ok(semio_framework_value::RetainedCloneStep::Progress(semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
            });
        }
        self.close_owners.close_step(cx)
    
        })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.outbox.terminal_is_empty() && self.builder.terminal_is_empty() && self.close_started && self.close_owners.is_empty()
    }
}

impl FillRunJob {
    pub fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if !self.outbox.terminal_is_empty() { return self.outbox.retirement_demands(); }
        if !self.builder.terminal_is_empty() { return self.builder.close_demands(body); }
        if !self.close_started { return Ok(semio_framework_value::RetirementDemand { copy_bytes: std::mem::size_of::<FillRunClosingOwners>(), depth: 1, ..Default::default() }); }
        self.close_owners.demands(body)
    }
}
//#endregion ⏯️FillRunJob

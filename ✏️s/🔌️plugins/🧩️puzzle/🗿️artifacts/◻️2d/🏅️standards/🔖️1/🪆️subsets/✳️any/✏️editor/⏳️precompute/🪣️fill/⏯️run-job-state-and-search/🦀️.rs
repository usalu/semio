//! ⏯️ Run job state and search: the fill search and owed state, the run owners and replay, the fill run job and its search stepping. See [`super`].

use super::*;

//#region ⏯️RunJob
/// 🔎️ How one engine search batch settled, short of a plain yield.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum FillSearch {
    Cancelled,
    Checkpoint,
    Fault,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum FillRunOwed {
    Checkpoint,
    Complete,
}

/// 🎯️ The candidate the search is testing right now.
#[derive(Clone, Copy, Debug)]
struct FillRunLive {
    pub(super) key: u64,
    pub(super) subject: ToolRunTraceSubject,
}

/// ♻️ The owners one `Puzzle2dFillRunJob` still holds once its search and capture are closed, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct Puzzle2dFillRunOwners {
    pub(super) document: Arc<Puzzle2dPlaySnapshot>,
    pub(super) kinds: Vec<Value>,
    pub(super) writer: ToolRunTickWriter,
    pub(super) replay: Option<FillRunReplay>,
    pub(super) placements: Vec<FillRunPlacementKey>,
    pub(super) target_regions: Vec<[f64; 4]>,
}

/// ⏩️ A resumed run silently replays the deterministic search up to the provisional placements it continues.
#[derive(semio_framework_value::RetireOwned)]
struct FillRunReplay {
    expected: Vec<Vec<u8>>,
    retire: Vec<FillRunPlacementKey>,
    pub(super) target: usize,
}

/// ⏯️ The puzzle 2d fill tool run job (`ToolRunDefinition.runJob`). One unit of fuel is one final candidate
/// verdict; ticks travel only in `PreviewReady` pages; `CheckpointReady` follows the tick of every placement
/// and of the completion, `Complete` follows the completion checkpoint. A job built from a checkpoint with
/// a new requested count replays the same search (seeded by the run id, serial after the document's fill
/// ids) without trace or ops, keeps every provisional placement it re-derives byte-identically, retracts
/// the rest and continues live — so a raised count continues and a lowered one retracts the tail.
pub struct Puzzle2dFillRunJob {
    pub(super) close_owners: crate::puzzle_job::WorkClosing<Puzzle2dFillRunOwners>,
    pub(super) owners_staged: bool,
    pub(super) retiring_placement: Option<BoardFillPlacement>,
    pub(super) outbox: crate::puzzle_job::JobOutbox,
    pub(super) document: Arc<Puzzle2dPlaySnapshot>,
    pub(super) kinds: Vec<Value>,
    pub(super) open_handles: u64,
    pub(super) writer: ToolRunTickWriter,
    pub(super) requested: u64,
    pub(super) seed: u64,
    pub(super) capture: Option<FillCapture>,
    pub(super) search: Option<BoardFillJob>,
    pub(super) closing_search: Option<BoardFillJob>,
    pub(super) search_sequence: u64,
    pub(super) replay: Option<FillRunReplay>,
    live: Option<FillRunLive>,
    pub(super) next_key: u64,
    pub(super) tested: u64,
    pub(super) collisions: u64,
    pub(super) rejected: u64,
    decided_since_placement: u64,
    pub(super) placements: Vec<FillRunPlacementKey>,
    pub(super) stage: FillRunStage,
    pub(super) stall: Option<FillRunReason>,
    pub(super) finished: bool,
    settled: bool,
    pub(super) owed: Option<FillRunOwed>,
    progress_sequence: u64,
    pub(super) closing: bool,
    /// 🎯️ Normalized `[min_x, min_y, max_x, max_y]` of every VISIBLE target region the run's base
    /// document declared, read once at construction. Empty means unconstrained — the same rule
    /// puzzle3d states as `world_volumes_contain_aabb`.
    pub(super) target_regions: Vec<[f64; 4]>,
}

impl Puzzle2dFillRunJob {
    /// 🎬️ A run over `document` toward `requested` placements. `checkpoint` and `provisional` are the
    /// ledger's resume state (both empty for a fresh run).
    pub fn new(identity: ToolRunIdentity, document: Arc<Puzzle2dPlaySnapshot>, suggestion_offset: f64, requested: u32, checkpoint: Option<&[u8]>, provisional: &[Puzzle2dMutation]) -> Result<Self, &'static str> {
        let expected = provisional.iter().map(|op| protocol::OpBinary::encode_op(op).map_err(|_| "puzzle2d-fill-run-op-encode")).collect::<Result<Vec<_>, _>>()?;
        let retire = match checkpoint {
            Some(bytes) => FillRunCheckpoint::decode(bytes).ok_or("puzzle2d-fill-run-checkpoint")?.placements,
            None => Vec::new(),
        };
        let provisional_placements = expected.len() / FILL_RUN_OPS_PER_PLACEMENT;
        let kinds = fill_kind_rows(document.value());
        let target_regions = fill_visible_region_bounds(document.value());
        let mut job = Self {
            close_owners: Default::default(),
            owners_staged: false,
            retiring_placement: None,
            outbox: Default::default(),
            kinds,
            open_handles: 0,
            document,
            writer: ToolRunTickWriter::with_provisional_base(identity, expected.len() as u32),
            requested: u64::from(requested),
            seed: identity.id.run.max(1),
            capture: Some(FillCapture::new(suggestion_offset)),
            search: None,
            closing_search: None,
            search_sequence: 0,
            replay: None,
            live: None,
            next_key: 0,
            tested: 0,
            collisions: 0,
            rejected: 0,
            decided_since_placement: 0,
            placements: Vec::new(),
            stage: FillRunStage::Capture,
            stall: None,
            finished: false,
            settled: false,
            owed: None,
            progress_sequence: 0,
            closing: false,
            target_regions,
        };
        let target = provisional_placements.min(requested as usize);
        job.replay = (provisional_placements > 0).then_some(FillRunReplay { expected, retire, target });
        if target == 0 {
            job.end_replay();
        }
        job.finished = job.replay.is_none() && job.requested == 0;
        Ok(job)
    }

    /// 📟️ `[tested, accepted, collisions, rejected]`, in [`FillRunCounter::ALL`] order.
    pub fn counters(&self) -> [u64; 4] {
        [self.tested, self.placements.len() as u64, self.collisions, self.rejected]
    }

    pub fn checkpoint(&self) -> FillRunCheckpoint {
        FillRunCheckpoint { requested: self.requested, tested: self.tested, collisions: self.collisions, rejected: self.rejected, next_key: self.next_key, placements: self.placements.clone() }
    }

    fn replaying(&self) -> bool {
        self.replay.is_some()
    }

    /// 🪚️ Ends a replay: provisional placements past the ones re-derived are retracted and their records retired.
    fn end_replay(&mut self) {
        let Some(replay) = self.replay.take() else { return };
        let kept = self.placements.len();
        if kept * FILL_RUN_OPS_PER_PLACEMENT < replay.expected.len() {
            self.writer.retract_to((kept * FILL_RUN_OPS_PER_PLACEMENT) as u32);
            for placement in replay.retire.iter().skip(kept) {
                self.writer.retire(placement.key);
            }
            self.stage = FillRunStage::Retract;
            let _ = self.writer.step(ToolRunStepKind::Info, FillRunStage::Retract.index(), FillRunReason::Retracted.code(), None, &[ToolRunStepArg::Unsigned(kept as u64)]);
        }
        if kept as u64 >= self.requested {
            self.finished = true;
        }
    }

    pub(super) fn subject(event: &BoardFillCandidateEvent) -> ToolRunTraceSubject {
        ToolRunTraceSubject::Placement2d { shape: event.kind_index as u32, position: [event.position[0] as f32, event.position[1] as f32], rotation: 0.0 }
    }

    /// ⏯️ Records ONE candidate's verdict. It deliberately spends no fuel: this run declares its unit as
    /// `placements` (`🗣️terminology` `fill_unit`) and reports `completed = placements.len()`, so a unit of
    /// fuel has to BE a placement or the tool-run panel's `Step` lies. It used to charge a unit per
    /// decided candidate, and on a crowded board almost every candidate is a `host-collision` rejection —
    /// so `Step` burned its single unit of fuel (`⏯️tool-run` grants exactly 1 while `Paused`) on a
    /// rejection and the census never moved: measured 2026-09-17 as `atPause=29 after=29 waitedMs=30131`.
    /// The tick is still bounded without this charge, by `deadline_exceeded()` and by the trace writer's
    /// `FILL_RUN_TICK_FLUSH_BYTES` — a rejection costs bytes, and the loop yields on them.
    fn decide(&mut self, context: &mut StepContext<'_>, live: FillRunLive, reason: FillRunReason) {
        let _ = context;
        if !self.replaying() {
            self.writer.upsert(live.key, reason.verdict(), reason.code(), live.subject);
        }
    }

    pub(super) fn observe(&mut self, context: &mut StepContext<'_>, event: BoardFillCandidateEvent) {
        let reason = FillRunReason::of_candidate(event.verdict);
        match reason {
            None | Some(FillRunReason::PortIncompatible | FillRunReason::KindIncompatible) => {
                let live = FillRunLive { key: self.next_key, subject: Self::subject(&event) };
                self.next_key += 1;
                self.tested += 1;
                if !self.replaying() {
                    self.writer.upsert(live.key, ToolRunVerdict::Testing, FillRunReason::Fits.code(), live.subject);
                }
                match reason {
                    Some(reason) => {
                        self.rejected += 1;
                        self.decide(context, live, reason);
                    }
                    None => self.live = Some(live),
                }
            }
            Some(FillRunReason::HostCollision | FillRunReason::VirtualCollision) => {
                if let Some(live) = self.live.take() {
                    self.collisions += 1;
                    self.decided_since_placement += 1;
                    self.decide(context, live, reason.expect("collision reason"));
                }
            }
            _ => {}
        }
    }

    /// 🧱️ Moves the engine's pending placement into provisional ops; `true` when a live tick is owed.
    pub(super) fn accept(&mut self, context: &mut StepContext<'_>) -> Result<bool, &'static str> {
        let search = self.search.as_mut().ok_or("puzzle2d-fill-run-search-owner")?;
        let mut checkpoint = search.take_checkpoint().ok_or("puzzle2d-fill-checkpoint-missing")?;
        let placement = checkpoint.take_pending_placement();
        let adopted = search.adopt_checkpoint(checkpoint);
        let placement = placement.ok_or("puzzle2d-fill-placement-missing")?;
        let mutations = fill_placement_mutations(&placement);
        self.retiring_placement = Some(placement);
        if let Err(checkpoint) = adopted {
            self.closing_search = Some(checkpoint.into_closing_job());
            return Err("puzzle2d-fill-checkpoint-stale");
        }
        let [create, connect] = mutations?;
        if let Puzzle2dMutation::CreateNode(payload) = &create {
            let node = &payload.node;
            let rectangle = node.shape.as_ref().is_some_and(|shape|shape.eq_str("rectangle"));
            let bounds = fill_node_bounds(node.x, node.y, node.scale, rectangle, if rectangle { node.width } else { node.radius }, node.height);
            if !fill_regions_admit(&self.target_regions, bounds) {
                if let Some(live) = self.live.take() {
                    self.rejected += 1;
                    self.decided_since_placement += 1;
                    self.decide(context, live, FillRunReason::OutsideTargetRegion);
                }
                return Ok(false);
            }
        }
        let entity = match &create {
            Puzzle2dMutation::CreateNode(payload) => fill_run_entity(&payload.node.id),
            _ => return Err("puzzle2d-fill-run-create-node"),
        };
        let ops = [protocol::OpBinary::encode_op(&create).map_err(|_| "puzzle2d-fill-run-op-encode")?, protocol::OpBinary::encode_op(&connect).map_err(|_| "puzzle2d-fill-run-op-encode")?];
        let live = self.live.take().ok_or("puzzle2d-fill-run-placement-untested")?;
        let shape = match live.subject {
            ToolRunTraceSubject::Placement2d { shape, .. } => shape,
            _ => u32::MAX,
        };
        let index = self.placements.len();
        self.decided_since_placement = 0;
        if let Some(replay) = self.replay.as_ref() {
            let offset = index * FILL_RUN_OPS_PER_PLACEMENT;
            if replay.expected.get(offset..offset + FILL_RUN_OPS_PER_PLACEMENT) == Some(&ops[..]) {
                self.placements.push(FillRunPlacementKey { key: live.key, shape });
                if self.placements.len() >= replay.target {
                    self.end_replay();
                }
                return Ok(false);
            }
            self.end_replay();
        }
        self.writer.upsert(live.key, ToolRunVerdict::Success, FillRunReason::Fits.code(), live.subject);
        let [create, connect] = ops;
        self.writer.append_op(create).and_then(|()| self.writer.append_op(connect)).map_err(|_| "puzzle2d-fill-run-provisional-ops")?;
        self.writer.append_entity(entity);
        context.consume_fuel(1);
        self.placements.push(FillRunPlacementKey { key: live.key, shape });
        if self.placements.len() as u64 >= self.requested {
            self.finished = true;
            self.settle();
        }
        Ok(true)
    }

    pub(super) fn stall_reason(&self) -> FillRunReason {
        match self.decided_since_placement {
            _ if self.open_handles == 0 && self.placements.is_empty() => FillRunReason::NoOpenHandle,
            0 => FillRunReason::NoCompatibleKind,
            _ => FillRunReason::NoFreePlacement,
        }
    }

    pub(super) fn stall(&mut self, context: &mut StepContext<'_>, reason: FillRunReason) {
        if let Some(live) = self.live.take() {
            self.rejected += 1;
            self.decide(context, live, FillRunReason::ArtifactCapacity);
        }
        self.end_replay();
        self.stall = Some(reason);
        self.finished = true;
    }

    pub(super) fn settle(&mut self) {
        if std::mem::replace(&mut self.settled, true) {
            return;
        }
        let placed = [ToolRunStepArg::Unsigned(self.placements.len() as u64)];
        let reason = self.stall.unwrap_or(FillRunReason::RequestedReached);
        let kind = if self.stall.is_some() { ToolRunStepKind::Warning } else { ToolRunStepKind::Success };
        let _ = self.writer.step(kind, self.stage.index(), reason.code(), None, &placed);
    }

    pub(super) fn progress(&mut self) -> ToolRunProgress {
        self.progress_sequence += 1;
        ToolRunProgress {
            identity: self.writer.identity(),
            sequence: self.progress_sequence,
            state: if self.finished { ToolRunState::Complete } else { ToolRunState::Running },
            stage: self.stage.index(),
            completed: self.placements.len() as u64,
            total: Some(self.requested),
            counters: FillRunCounter::ALL.iter().zip(self.counters()).map(|(counter, value)| ToolRunCounter { counter: counter.index(), value }).collect(),
            units_per_second: 0.0,
            conflicts: 0,
            steps: ToolRunStepRing::default(),
        }
    }

    pub(super) fn flush(&mut self, _context: &mut StepContext<'_>) -> Option<JobTurn> {
        if self.writer.is_empty() && !self.finished {
            return None;
        }
        let progress = self.progress();
        self.writer.progress(progress);
        let bytes = self.writer.finish()?.encode().ok();
        Some(match bytes {
            Some(bytes) if bytes.len() <= semio_framework_job::JOB_PAYLOAD_PAGE_BYTES => JobTurn::Preview(bytes),
            Some(_) => fill_run_fault("puzzle2d-fill-run-tick-bytes"),
            None => fill_run_fault("puzzle2d-fill-run-tick-encode"),
        })
    }

    pub(super) fn flush_then(&mut self, context: &mut StepContext<'_>, owed: FillRunOwed) -> JobTurn {
        match self.flush(context) {
            Some(preview @ JobTurn::Preview(_)) => {
                self.owed = Some(owed);
                preview
            }
            Some(fault) => fault,
            None => self.settle_owed(context, owed),
        }
    }

    pub(super) fn settle_owed(&mut self, _context: &mut StepContext<'_>, owed: FillRunOwed) -> JobTurn {
        match owed {
            FillRunOwed::Complete => JobTurn::Complete,
            FillRunOwed::Checkpoint => {
                if self.finished {
                    self.owed = Some(FillRunOwed::Complete);
                }
                JobTurn::Checkpoint { applied_progress: self.placements.len() as u64, state: self.checkpoint().encode() }
            }
        }
    }

    /// 🔎️ Engine transitions until a candidate event, a non-preview outcome, a stall or the wall deadline.
    pub(super) fn search_batch(search: &mut BoardFillJob, sequence: &mut u64, context: &StepContext<'_>) -> (Option<FillSearch>, Option<BoardFillCandidateEvent>) {
        let operation = search.operation();
        let mut progress = semio_framework_value::retained_clone::RetainedCloneProgress::default();
        let unbounded = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX };
        let mut inner = StepContext::new(operation.operation, operation.generation, StepBudget::new(u64::MAX, u64::MAX, unbounded), context.cancel_token(), fill_run_monotonic_zero, sequence, &mut progress);
        for transition in 0..FILL_RUN_SEARCH_BATCH {
            if search.stage() == BoardFillStage::Complete {
                return (None, None);
            }
            let settled = match InteractiveJob::step(search, &mut inner) {
                Ok(None | Some(JobOutcomeBorrow::Yield { .. } | JobOutcomeBorrow::PreviewReady { .. })) => None,
                Ok(Some(JobOutcomeBorrow::Cancelled { .. })) => Some(FillSearch::Cancelled),
                Ok(Some(JobOutcomeBorrow::CheckpointReady { .. })) => Some(FillSearch::Checkpoint),
                Ok(Some(JobOutcomeBorrow::Fault { .. })) | Err(_) => Some(FillSearch::Fault),
                Ok(Some(JobOutcomeBorrow::Complete { .. })) => Some(FillSearch::Complete),
            };
            let _ = search.take_preview();
            let event = search.take_candidate_event();
            match settled {
                None if event.is_some() => return (None, event),
                None => {}
                settled => return (settled, event),
            }
            if transition % FILL_RUN_DEADLINE_CHECK == FILL_RUN_DEADLINE_CHECK - 1 && context.deadline_exceeded() {
                return (None, None);
            }
        }
        (None, None)
    }
}
//#endregion ⏯️RunJob

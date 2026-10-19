//! 🔍️ Revalidate job: the revalidate owners and job that re-test the provisional placements against the head. See [`super`].

use super::*;

//#region 🔍️RevalidateJob
/// 🔍️ The puzzle 2d fill revalidation job (`ToolRunDefinition.revalidateJob`): re-tests every provisional
/// placement against the head document. A placement conflicts when its node id already exists, its source
/// handle is gone (neither in the head nor on a surviving earlier placement) or its footprint overlaps a head
/// node. Each placement is one unit of fuel and ends `success` (`fits`) or `danger` (`TOOL_RUN_REASON_CONFLICT`);
/// the last tick retracts to the first conflict and re-appends every later survivor's ops and entity with one
/// `danger` conflict step, and `Complete` follows on the next call.
/// ♻️ The owners one `Puzzle2dFillRevalidateJob` still holds when it closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle2dFillRevalidateOwners {
    pub(super) writer: ToolRunTickWriter,
    head: Arc<Puzzle2dPlaySnapshot>,
    pub(super) ops: Vec<Puzzle2dMutation>,
    keys: Vec<FillRunPlacementKey>,
    head_bounds: Vec<[f64; 4]>,
    head_ids: HashSet<String>,
    pub(super) handles: HashSet<String>,
    pub(super) conflicts: Vec<bool>,
}

pub struct Puzzle2dFillRevalidateJob {
    pub(super) close_owners: crate::puzzle_job::WorkClosing<Puzzle2dFillRevalidateOwners>,
    pub(super) outbox: crate::puzzle_job::JobOutbox,
    pub(super) writer: ToolRunTickWriter,
    head: Arc<Puzzle2dPlaySnapshot>,
    pub(super) ops: Vec<Puzzle2dMutation>,
    keys: Vec<FillRunPlacementKey>,
    head_cursor: usize,
    head_bounds: Vec<[f64; 4]>,
    head_ids: HashSet<String>,
    pub(super) handles: HashSet<String>,
    pub(super) conflicts: Vec<bool>,
    /// 🚧️ `contactTolerance - brushPlacementOverlapBudget`, the one number the AABB test is widened by.
    placement_slack: f64,
    pub(super) finished: bool,
    pub(super) completed: bool,
    closed: bool,
}

impl Puzzle2dFillRevalidateJob {
    pub fn new(identity: ToolRunIdentity, head: Arc<Puzzle2dPlaySnapshot>, provisional: &[Puzzle2dMutation], checkpoint: Option<&[u8]>, placement_slack: f64) -> Self {
        let keys = checkpoint.and_then(FillRunCheckpoint::decode).map(|checkpoint| checkpoint.placements).unwrap_or_default();
        Self {
            close_owners: Default::default(),
            outbox: Default::default(),
            writer: ToolRunTickWriter::with_provisional_base(identity, provisional.len() as u32),
            head,
            ops: provisional.to_vec(),
            keys,
            head_cursor: 0,
            head_bounds: Vec::new(),
            head_ids: HashSet::new(),
            handles: HashSet::new(),
            conflicts: Vec::new(),
            placement_slack: if placement_slack.is_finite() { placement_slack } else { 0.0 },
            finished: false,
            completed: false,
            closed: false,
        }
    }

    pub(super) fn placement(&self, index: usize) -> Option<(&crate::standards::v1::subsets::any::schema::records::Puzzle2dNode, &semio_framework_value::paged::PagedUtf8<{ usize::MAX }>)> {
        match self.ops.get(index * FILL_RUN_OPS_PER_PLACEMENT..(index + 1) * FILL_RUN_OPS_PER_PLACEMENT)? {
            [Puzzle2dMutation::CreateNode(create), Puzzle2dMutation::ConnectHandles(connect)] => Some((&create.node, &connect.source)),
            _ => None,
        }
    }

    fn prepare_head_one(&mut self) -> bool {
        let Some(node) = self.head.value().get("nodes").and_then(Value::as_array).and_then(|nodes| nodes.get(self.head_cursor)) else { return false };
        self.head_cursor += 1;
        let rectangle = node.get("shape").and_then(Value::as_str) == Some("rectangle");
        let number = |key: &str| node.get(key).and_then(Value::as_f64);
        self.head_bounds.push(fill_node_bounds(number("x").unwrap_or(0.0), number("y").unwrap_or(0.0), number("scale"), rectangle, number(if rectangle { "width" } else { "radius" }), number("height")));
        if let Some(id) = node.get("id").and_then(Value::as_str) {
            self.head_ids.insert(id.to_string());
        }
        for handle in node.get("handles").and_then(Value::as_array).into_iter().flatten() {
            if let Some(id) = handle.get("id").and_then(Value::as_str) {
                self.handles.insert(id.to_string());
            }
        }
        true
    }

    fn test_placement(&mut self, index: usize) -> (bool, ToolRunTraceSubject) {
        let shape = self.keys.get(index).map_or(u32::MAX, |key| key.shape);
        let Some((node, source)) = self.placement(index) else {
            return (true, ToolRunTraceSubject::Placement2d { shape, position: [0.0, 0.0], rotation: 0.0 });
        };
        let subject = ToolRunTraceSubject::Placement2d { shape, position: [node.x as f32, node.y as f32], rotation: 0.0 };
        let rectangle = node.shape.as_ref().is_some_and(|shape|shape.eq_str("rectangle"));
        let bounds = fill_node_bounds(node.x, node.y, node.scale, rectangle, if rectangle { node.width } else { node.radius }, node.height);
        let slack = self.placement_slack;
        let conflict = self.head_ids.iter().any(|id| node.id.eq_str(id)) || !self.handles.iter().any(|id| source.eq_str(id)) || self.head_bounds.iter().any(|head| fill_bounds_overlap_with(bounds, *head, slack));
        let handles: Vec<String> = if conflict { Vec::new() } else { node.handles.iter().map(|handle| handle.id.to_string_owner()).collect() };
        self.handles.extend(handles);
        (conflict, subject)
    }

    pub(super) fn finish(&mut self) -> Result<(), &'static str> {
        self.finished = true;
        let Some(first) = self.conflicts.iter().position(|conflict| *conflict) else { return Ok(()) };
        self.writer.retract_to((first * FILL_RUN_OPS_PER_PLACEMENT) as u32);
        for index in (first..self.conflicts.len()).filter(|index| !self.conflicts[*index]) {
            let offset = index * FILL_RUN_OPS_PER_PLACEMENT;
            for op in &self.ops[offset..offset + FILL_RUN_OPS_PER_PLACEMENT] {
                self.writer.append_op(protocol::OpBinary::encode_op(op).map_err(|_| "puzzle2d-fill-revalidate-op-encode")?).map_err(|_| "puzzle2d-fill-revalidate-provisional-ops")?;
            }
            let entity = self.placement(index).map(|(node, _)| fill_run_entity(&node.id)).ok_or("puzzle2d-fill-revalidate-placement")?;
            self.writer.append_entity(entity);
        }
        let conflicts = self.conflicts.iter().filter(|conflict| **conflict).count() as u64;
        self.writer.step(ToolRunStepKind::Danger, FillRunStage::Place.index(), TOOL_RUN_REASON_CONFLICT, None, &[ToolRunStepArg::Unsigned(conflicts)]).map_err(|_| "puzzle2d-fill-revalidate-step")
    }

    pub(super) fn flush(&mut self) -> JobTurn {
        let total = (self.ops.len() / FILL_RUN_OPS_PER_PLACEMENT) as u64;
        let conflicts = self.conflicts.iter().filter(|conflict| **conflict).count() as u64;
        self.writer.progress(ToolRunProgress {
            identity: self.writer.identity(),
            sequence: self.conflicts.len() as u64,
            state: ToolRunState::Finalizing,
            stage: FillRunStage::Place.index(),
            completed: self.conflicts.len() as u64,
            total: Some(total),
            counters: vec![ToolRunCounter { counter: FillRunCounter::Collisions.index(), value: conflicts }],
            units_per_second: 0.0,
            conflicts: conflicts as u32,
            steps: ToolRunStepRing::default(),
        });
        match self.writer.finish().and_then(|tick| tick.encode().ok()) {
            Some(bytes) if bytes.len() <= semio_framework_job::JOB_PAYLOAD_PAGE_BYTES => JobTurn::Preview(bytes),
            Some(_) => fill_run_fault("puzzle2d-fill-revalidate-tick-bytes"),
            None => fill_run_fault("puzzle2d-fill-revalidate-tick-encode"),
        }
    }
}

impl Puzzle2dFillRevalidateJob {
    pub(super) fn turn(&mut self, context: &mut StepContext<'_>) -> JobTurn {
        if self.closed || context.is_cancelled() {
            return JobTurn::Cancelled;
        }
        loop {
            if self.completed {
                return JobTurn::Complete;
            }
            if self.finished {
                self.completed = true;
                return self.flush();
            }
            if context.fuel_exhausted() || context.deadline_exceeded() || self.writer.pending_bytes() >= FILL_RUN_TICK_FLUSH_BYTES {
                return if self.writer.is_empty() { JobTurn::Yield } else { self.flush() };
            }
            if self.prepare_head_one() {
                continue;
            }
            let index = self.conflicts.len();
            if index * FILL_RUN_OPS_PER_PLACEMENT >= self.ops.len() {
                if let Err(code) = self.finish() {
                    return fill_run_fault(code);
                }
                continue;
            }
            let (conflict, subject) = self.test_placement(index);
            let key = self.keys.get(index).map_or(u64::MAX - index as u64, |key| key.key);
            self.writer.upsert(key, ToolRunVerdict::Testing, FillRunReason::Fits.code(), subject);
            let (verdict, reason) = if conflict { (ToolRunVerdict::Danger, TOOL_RUN_REASON_CONFLICT) } else { (ToolRunVerdict::Success, FillRunReason::Fits.code()) };
            self.writer.upsert(key, verdict, reason, subject);
            self.conflicts.push(conflict);
            context.consume_fuel(1);
        }
    }
}

impl Puzzle2dFillRevalidateJob {
    /// 🪜️ The step body; its value refusals are lifted into the original failure at the trait boundary.
    fn step_value<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        match self.outbox.phase(cx)? {
            crate::puzzle_job::OutboxPhase::Building => return self.outbox.advance(cx).map_err(semio_framework_job::OriginalTurnFailure::from),
            crate::puzzle_job::OutboxPhase::Delivered | crate::puzzle_job::OutboxPhase::Retiring => {
                self.outbox.retire_step(cx)?;
                return Ok(None);
            }
            crate::puzzle_job::OutboxPhase::Idle => {}
        }
        let turn = self.turn(cx);
        self.outbox.settle(turn, cx).map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

impl InteractiveJob for Puzzle2dFillRevalidateJob {
    fn step<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{self.step_value(cx).map_err(semio_framework_job::OriginalTurnFailure::from)}

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outbox.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        if std::mem::replace(&mut self.closed, true) {
            return;
        }
        let identity = self.writer.identity();
        self.close_owners.stage(Puzzle2dFillRevalidateOwners {
            writer: std::mem::replace(&mut self.writer, ToolRunTickWriter::new(identity)),
            head: Arc::clone(&self.head),
            ops: std::mem::take(&mut self.ops),
            keys: std::mem::take(&mut self.keys),
            head_bounds: std::mem::take(&mut self.head_bounds),
            head_ids: std::mem::take(&mut self.head_ids),
            handles: std::mem::take(&mut self.handles),
            conflicts: std::mem::take(&mut self.conflicts),
        });
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> {
        semio_framework_job::close_original_nested_step(cx, |cx, _grant| {

        self.begin_close();
        crate::puzzle_job::job_close_step(&mut self.outbox, &mut self.close_owners, cx)
    
        })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.close_owners, 0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.close_owners, maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.close_owners, 0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.close_owners, 0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closed && self.outbox.terminal_is_empty() && self.close_owners.is_empty()
    }
}
//#endregion 🔍️RevalidateJob

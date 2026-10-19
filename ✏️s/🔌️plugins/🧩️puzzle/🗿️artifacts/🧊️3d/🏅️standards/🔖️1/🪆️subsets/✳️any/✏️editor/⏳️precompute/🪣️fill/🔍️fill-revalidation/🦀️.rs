//! 🔍️ Fill revalidation: the finalize job that re-checks the provisional placements against the committed document and re-appends the survivors.

use super::*;

//#region ⏯️FillRunJob
/// 🔍️ Where a finalize revalidation stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FillRevalidatePhase {
    PrepareHead,
    Placement,
    Finish,
    Done,
}

/// 🔍️ The fill tool run revalidation job (`ToolRunDefinition.revalidateJob`): re-tests every provisional
/// placement against the head document and earlier surviving placements. Its host vortex must still exist,
/// its id must be free, every body's geometry must be available, and penetration must stay within the head's
/// contact tolerance. Preparation and pair checks consume cooperative fuel; each placement ends as a
/// `success` (`fits`) or `danger` (`TOOL_RUN_REASON_CONFLICT`) trace record. The last
/// tick retracts to the first conflict and re-appends every later survivor's ops and entity, with one
/// `danger` conflict step carrying the conflict count; `Complete` follows on the next call.
/// ♻️ The owners one `FillRevalidateJob` still holds when it closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct FillRevalidateOwners {
    pub scene: Arc<SceneConfig>,
    pub meshes: Arc<FillMeshMap>,
    pub(super) placements: Vec<FillRunPlacement>,
    head: Vec<PlacedCollisionEntry>,
    head_ids: semio_framework_value::retained_clone::ordered_map::RetainedOrderedMap<String, ()>,
    vortex_owners: semio_framework_value::retained_clone::ordered_map::RetainedOrderedMap<String, String>,
    pub(super) collision: Option<CollisionPenetrationState>,
    pub(super) conflicts: Vec<bool>,
    pub ops: Vec<ToolRunTraceOp>,
    pub steps: Vec<ToolRunStep>,
}

pub struct FillRevalidateJob {
    close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<FillRevalidateOwners>,
    pub(super) outbox: semio_s_plugin_puzzle_jobs::JobOutbox,
    pub operation: Operation,
    pub identity: ToolRunIdentity,
    pub scene: std::mem::ManuallyDrop<Arc<SceneConfig>>,
    pub meshes: std::mem::ManuallyDrop<Arc<FillMeshMap>>,
    pub(super) placements: Vec<FillRunPlacement>,
    head: Vec<PlacedCollisionEntry>,
    head_ids: semio_framework_value::retained_clone::ordered_map::RetainedOrderedMap<String, ()>,
    vortex_owners: semio_framework_value::retained_clone::ordered_map::RetainedOrderedMap<String, String>,
    head_cursor: usize,
    pair_cursor: usize,
    pub(super) collision: Option<CollisionPenetrationState>,
    pub(super) conflicts: Vec<bool>,
    pub(super) cursor: usize,
    pub(super) phase: FillRevalidatePhase,
    pub ops: Vec<ToolRunTraceOp>,
    /// 👁️ The one placement under revalidation still on screen: the next placement's test retires it and the finish retires
    /// the last, so a finalize never leaves a verdict mesh over every committed object (the one-candidate rule of
    /// [`FillRunJob`]).
    pub(super) shown: Option<u64>,
    pub steps: Vec<ToolRunStep>,
    pub sequence: u64,
    page: u32,
    closed: bool,
    close_started: bool,
}

impl FillRevalidateJob {
    pub fn new(operation: Operation, identity: ToolRunIdentity, head: FillPreparationRoots, placements: Vec<FillRunPlacement>, first_sequence: u64) -> Self {
        Self {
            close_owners: Default::default(),
            outbox: Default::default(),
            operation,
            identity,
            scene: std::mem::ManuallyDrop::new(head.scene),
            meshes: std::mem::ManuallyDrop::new(head.meshes),
            conflicts: Vec::with_capacity(placements.len()),
            placements,
            head: Vec::new(),
            head_ids: semio_framework_value::retained_clone::ordered_map::RetainedOrderedMap::new(),
            vortex_owners: semio_framework_value::retained_clone::ordered_map::RetainedOrderedMap::new(),
            head_cursor: 0,
            pair_cursor: 0,
            collision: None,
            cursor: 0,
            phase: FillRevalidatePhase::PrepareHead,
            ops: Vec::new(),
            shown: None,
            steps: Vec::new(),
            sequence: first_sequence,
            page: 0,
            closed: false,
            close_started: false,
        }
    }

    /// 🪢️ Every later tick carries `identity`.
    pub fn rebind(&mut self, identity: ToolRunIdentity) {
        self.identity = identity;
    }

    #[cfg(any(test, feature = "testing"))]
    /// ⚖️ `true` per placement that conflicts with the head, in op order.
    pub fn conflicts(&self) -> &[bool] {
        &self.conflicts
    }

    fn prepare_head_one(&mut self) {
        let Some(object) = self.scene.scene_snapshot.objects.get(self.head_cursor) else {
            self.phase = FillRevalidatePhase::Placement;
            return;
        };
        self.head_cursor += 1;
        self.head_ids.cold_insert(object.id.clone(), ());
        for vortex in &object.vortices {
            self.vortex_owners.cold_insert(puzzle3d_vortex_full_id(&object.id, &vortex.id), object.id.clone());
        }
        let empty = KindCatalogBundle::default();
        let mesh_url = resolve_placed_object_mesh_url(object, self.scene.kind_catalogs.as_ref().unwrap_or(&empty), &self.scene.scene_snapshot);
        if let Some(mesh_url) = mesh_url.filter(|url| self.meshes.get(url).is_some()) {
            self.head.push(PlacedCollisionEntry { object_id: object.id.clone(), mesh_url, world: pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale) });
        }
    }

    fn host_of(&self, index: usize) -> Option<String> {
        let attracting = &self.placements[index].attraction.attracting;
        self.vortex_owners.get(attracting).cloned().or_else(|| {
            self.placements[..index].iter().zip(&self.conflicts).filter(|(_, conflict)| !**conflict).find(|(placement, _)| placement.object.vortices.iter().any(|vortex| puzzle3d_vortex_full_id(&placement.object.id, &vortex.id) == *attracting)).map(|(placement, _)| placement.object.id.clone())
        })
    }

    /// ⚖️ One bounded unit of the current placement's test: `Some(conflict)` once it is decided.
    fn test_placement_unit(&mut self, context: &mut StepContext<'_>) -> Option<bool> {
        let index = self.cursor;
        let placement = &self.placements[index];
        if self.pair_cursor == 0 && self.collision.is_none() {
            if self.head.len() < self.scene.scene_snapshot.objects.len() {
                return Some(true);
            }
            if self.head_ids.get(&placement.object.id).is_some() {
                return Some(true);
            }
            if self.host_of(index).is_none() {
                return Some(true);
            }
        }
        let placement = &self.placements[index];
        let Some(body) = placement.object.mesh_url.as_ref().and_then(|url| self.meshes.get(url)) else {
            return Some(true);
        };
        let world = pose_isometry(placement.object.origin, placement.object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &placement.object.scale);
        let Some(entry) = self.head.get(self.pair_cursor) else {
            return Some(false);
        };
        let Some(other) = self.meshes.get(&entry.mesh_url) else {
            return Some(true);
        };
        if self.collision.is_none() {
            if !CollisionAabb::from_body(other, &entry.world).intersects(&CollisionAabb::from_body(body, &world)) {
                self.pair_cursor += 1;
                context.consume_fuel(1);
                return None;
            }
        }
        let budget = self.scene.contact_tolerance;
        let collision = self.collision.get_or_insert_with(|| CollisionPenetrationState::new(budget));
        match collision.step(&mut FillRunTransitionContext { outer: context, operation: self.operation }, body, &world, other, &entry.world) {
            CollisionStepResult::Pending | CollisionStepResult::Cancelled => None,
            CollisionStepResult::Complete { depth, .. } => {
                self.collision = None;
                self.pair_cursor += 1;
                (depth > budget).then_some(true)
            }
        }
    }

    pub(super) fn flush(&mut self, _context: &mut StepContext<'_>, final_ops: Option<(u32, Vec<Vec<u8>>, Vec<u64>)>) -> JobTurn {
        let (retract_to, append_ops, append_entities) = match final_ops {
            Some((retract_to, ops, entities)) => (Some(retract_to), ops, entities),
            None => (None, Vec::new(), Vec::new()),
        };
        let trace = if self.ops.is_empty() { Vec::new() } else { vec![ToolRunTracePage { identity: self.identity, page: self.page, ops: std::mem::take(&mut self.ops) }] };
        self.page += u32::from(!trace.is_empty());
        let conflicts = self.conflicts.iter().filter(|conflict| **conflict).count() as u64;
        let progress = ToolRunProgress {
            identity: self.identity,
            sequence: self.sequence,
            state: ToolRunState::Finalizing,
            stage: FillRunStage::Lock.index(),
            completed: self.conflicts.len() as u64,
            total: Some(self.placements.len() as u64),
            counters: vec![ToolRunCounter { counter: FillRunCounter::Collisions.index(), value: conflicts }],
            units_per_second: 0.0,
            conflicts: conflicts as u32,
            steps: ToolRunStepRing::default(),
        };
        let tick = ToolRunTick { identity: self.identity, sequence: self.sequence, progress: Some(progress), steps: std::mem::take(&mut self.steps), trace, append_ops, append_entities, retract_to, payload: None,payload_update:None };
        self.sequence += 1;
        match tick.encode().ok() {
            Some(bytes) if bytes.len() <= semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES => JobTurn::Preview(bytes.to_vec()),
            Some(_) => JobTurn::Fault(b"fill-revalidate-tick-page".to_vec()),
            None => JobTurn::Fault(b"fill-revalidate-tick-encode".to_vec()),
        }
    }

    pub(super) fn finish(&mut self, context: &mut StepContext<'_>) -> JobTurn {
        self.phase = FillRevalidatePhase::Done;
        if let Some(previous) = self.shown.take() {
            self.ops.push(ToolRunTraceOp::Retire { key: previous });
        }
        let Some(first) = self.conflicts.iter().position(|conflict| *conflict) else {
            return self.flush(context, None);
        };
        let mut catalog_snapshot = empty_scene_snapshot();
        if let Some(catalogs) = self.scene.kind_catalogs.as_ref() {
            catalog_snapshot.meta.kind_catalogs = Some(semio_framework_value::ToValue::to_value(catalogs));
        }
        let mut peers: Vec<Puzzle3dObject> = self
            .scene
            .scene_snapshot
            .objects
            .iter()
            .filter_map(|object| semio_framework_value::FromValue::from_value(semio_framework_value::ToValue::to_value(object)).ok())
            .collect();
        let mut ops = Vec::new();
        let mut entities = Vec::new();
        for (placement, _) in self.placements.iter().zip(&self.conflicts).skip(first).filter(|(_, conflict)| !**conflict) {
            let Some([create, connect]) = fill_run_ops(&placement.object, &placement.attraction, &peers, &catalog_snapshot) else {
                return JobTurn::Fault(b"fill-revalidate-op-encode".to_vec());
            };
            if let Ok(mut peer) = <Puzzle3dObject as semio_framework_value::FromValue>::from_value(semio_framework_value::ToValue::to_value(&placement.object)) {
                let kind_id = placement.object.object_kind.as_deref().unwrap_or("object");
                peer.label = Some(puzzle3d_next_object_label(&peers, &catalog_snapshot, kind_id));
                peers.push(peer);
            }
            ops.extend([create, connect]);
            entities.push(placement.entity);
        }
        let conflicts = self.conflicts.iter().filter(|conflict| **conflict).count() as u64;
        self.steps.push(ToolRunStep { sequence: 0, kind: ToolRunStepKind::Danger, stage: FillRunStage::Lock.index(), reason: TOOL_RUN_REASON_CONFLICT, subject: None, repeat: 1, args: vec![ToolRunStepArg::Unsigned(conflicts)] });
        self.flush(context, Some((first as u32 * FILL_RUN_OPS_PER_PLACEMENT, ops, entities)))
    }
}

impl FillRevalidateJob {
    pub fn turn(&mut self, context: &mut StepContext<'_>) -> JobTurn {
        if self.closed { return JobTurn::Complete; }
        if context.is_cancelled() {
            return JobTurn::Cancelled;
        }
        loop {
            match self.phase {
                FillRevalidatePhase::Done => return JobTurn::Complete,
                FillRevalidatePhase::Finish => return self.finish(context),
                _ if context.fuel_exhausted() || context.deadline_exceeded() => {
                    return if self.ops.is_empty() { JobTurn::Yield } else { self.flush(context, None) };
                }
                FillRevalidatePhase::PrepareHead => {
                    self.prepare_head_one();
                    context.consume_fuel(1);
                }
                FillRevalidatePhase::Placement => {
                    let Some(placement) = self.placements.get(self.cursor) else {
                        self.phase = FillRevalidatePhase::Finish;
                        continue;
                    };
                    let (key, subject) = (placement.key, placement.subject);
                    if self.pair_cursor == 0 && self.collision.is_none() {
                        if let Some(previous) = self.shown.take().filter(|previous| *previous != key) {
                            self.ops.push(ToolRunTraceOp::Retire { key: previous });
                        }
                        self.ops.push(ToolRunTraceOp::Upsert { key, verdict: ToolRunVerdict::Testing, reason: FillRunReason::Fits.code(), subject });
                    }
                    if let Some(conflict) = self.test_placement_unit(context) {
                        if !conflict {
                            let object = &self.placements[self.cursor].object;
                            self.head_ids.cold_insert(object.id.clone(), ());
                            self.head.push(PlacedCollisionEntry {
                                object_id: object.id.clone(),
                                mesh_url: object.mesh_url.clone().expect("validated placement mesh"),
                                world: pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale),
                            });
                        }
                        let (verdict, reason) = if conflict { (ToolRunVerdict::Danger, TOOL_RUN_REASON_CONFLICT) } else { (ToolRunVerdict::Success, FillRunReason::Fits.code()) };
                        self.ops.push(ToolRunTraceOp::Upsert { key, verdict, reason, subject });
                        self.shown = Some(key);
                        self.conflicts.push(conflict);
                        self.cursor += 1;
                        self.pair_cursor = 0;
                        self.collision = None;
                        context.consume_fuel(1);
                    }
                }
            }
        }
    }
}

impl FillRevalidateJob {
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

impl InteractiveJob for FillRevalidateJob {
    fn step<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{self.step_value(cx).map_err(semio_framework_job::OriginalTurnFailure::from)}

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outbox.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) { self.closed = true; }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> {
        semio_framework_job::close_original_nested_step(cx, |cx, _grant| {

        self.begin_close();
        if !self.outbox.terminal_is_empty() {
            let demand = self.outbox.retirement_demands()?;
            return semio_framework_job::close_original_retained_step(cx, demand, |grant| self.outbox.close_step(grant));
        }
        if !self.close_started {
            let demand = self.close_demands(cx.retained_grant().maximum_copy_bytes)?;
            return semio_framework_job::close_original_retained_step(cx, demand, |_| {
                self.close_owners.stage(FillRevalidateOwners {
                    scene: unsafe { std::mem::ManuallyDrop::take(&mut self.scene) },
                    meshes: unsafe { std::mem::ManuallyDrop::take(&mut self.meshes) },
                    placements: std::mem::take(&mut self.placements),
                    head: std::mem::take(&mut self.head),
                    head_ids: std::mem::take(&mut self.head_ids),
                    vortex_owners: std::mem::take(&mut self.vortex_owners),
                    collision: self.collision.take(),
                    conflicts: std::mem::take(&mut self.conflicts),
                    ops: std::mem::take(&mut self.ops),
                    steps: std::mem::take(&mut self.steps),
                });
                self.close_started = true;
                Ok(semio_framework_value::RetainedCloneStep::Progress(semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
            });
        }
        self.close_owners.close_step(cx)
    
        })
    }

    

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.copy_bytes) }

    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(body)?.capacity_bytes) }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.release_bytes) }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.depth) }

    fn terminal_is_empty(&self) -> bool { self.closed && self.close_started && self.outbox.terminal_is_empty() && self.close_owners.is_empty() }
}
//#endregion ⏯️FillRunJob

impl FillRevalidateJob {
    pub fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if !self.outbox.terminal_is_empty() { return self.outbox.retirement_demands(); }
        if !self.close_started { return Ok(semio_framework_value::RetirementDemand { copy_bytes: std::mem::size_of::<FillRevalidateOwners>(), depth: 1, ..Default::default() }); }
        self.close_owners.demands(body)
    }
}

impl Drop for FillRevalidateJob {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "fill revalidation retains original roots until paid terminal close"); }
}

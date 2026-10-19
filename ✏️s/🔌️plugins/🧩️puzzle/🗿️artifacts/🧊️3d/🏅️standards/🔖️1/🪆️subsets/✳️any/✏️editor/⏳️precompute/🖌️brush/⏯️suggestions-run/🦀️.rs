//! ⏯️ Brush suggestions run: the interactive job that searches brush suggestions around a drag.

use super::*;

//#region ⏯️BrushSuggestionsRun
/// 🚥️ What the run decided about one compatible candidate of its target vortex.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub enum BrushSuggestionVerdict {
    Pending,
    Free,
    Collision,
    Unavailable,
}

/// 🗂️ One target vortex's compatible candidates as the run resolves them: candidate `key` (the trace key)
/// has pose `previews[key]` and verdict `verdicts[key]`. `writer` is the `(run, generation)` that published
/// it, so a closing job never retires what its successor already published.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct BrushSuggestionsFound {
    pub writer: (u64, u32),
    pub target: String,
    pub previews: Vec<Option<BrushPreviewState>>,
    pub verdicts: Vec<BrushSuggestionVerdict>,
    pub done: bool,
}

impl BrushSuggestionsFound {
    /// 🟢️ The free candidates in candidate order — what the popup lists, a cycle walks and an accept places.
    pub fn free(&self) -> impl Iterator<Item = &BrushPreviewState> {
        self.free_keyed().map(|(_, preview)| preview)
    }

    /// 🔑️ [`Self::free`] paired with each candidate's trace key — the key the run upserted it under in the
    /// `toolRunTrace` lane, so a menu row can focus exactly that record in the viewport.
    pub fn free_keyed(&self) -> impl Iterator<Item = (u64, &BrushPreviewState)> {
        self.previews.iter().zip(&self.verdicts).enumerate().filter(|(_, (_, verdict))| **verdict == BrushSuggestionVerdict::Free).filter_map(|(key, (preview, _))| preview.as_ref().map(|preview| (key as u64, preview)))
    }

    /// 🎯️ The vortex full id this search resolved candidates for.
    pub fn target(&self) -> &str {
        &self.target
    }

    /// 🏁️ Whether every candidate has its verdict — a menu with nothing free stops reading as pending.
    pub fn done(&self) -> bool {
        self.done
    }
}

/// 📨️ A tool run action the link asked the host for and has not seen answered yet.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub enum BrushSuggestionsRequest {
    Start,
    Abort,
}

/// 🔗️ The instance-owned half of the brush suggestions run: the vortex the user points the search at (an
/// open suggestion popup wins over the brush hover), the run's wake port, the one outstanding start or abort
/// request, and the candidates the live run has resolved so far. Commands write the target, the run job
/// follows it and publishes what it found, renders and accepts read it. Ephemeral local-only state.
#[derive(Default, semio_framework_value::RetireOwned)]
pub struct BrushSuggestionsLink {
    menu: Option<String>,
    pub(super) hover: Option<String>,
    pub port: Option<ToolRunJobPort>,
    pub requested: Option<(BrushSuggestionsRequest, Option<u64>)>,
    pub retry: bool,
    pub found: Option<BrushSuggestionsFound>,
}

impl BrushSuggestionsLink {
    /// 🎯️ The vortex the search should be looking at right now.
    pub fn target(&self) -> Option<&str> {
        self.menu.as_deref().or(self.hover.as_deref())
    }

    /// 💡️ A suggestion popup opened on `vortex_full_id`.
    pub fn open_menu(&mut self, vortex_full_id: &str) {
        self.menu = Some(vortex_full_id.to_string()).filter(|id| !id.is_empty());
        self.gesture();
    }

    /// 🔒️ The suggestion popup closed.
    pub fn close_menu(&mut self) {
        if self.menu.take().is_some() {
            self.gesture();
        }
    }

    /// 🖌️ The armed brush now points at `vortex_full_id`, or at nothing.
    pub fn hover(&mut self, vortex_full_id: Option<String>) {
        let hover = vortex_full_id.filter(|id| !id.is_empty());
        if hover != self.hover {
            self.hover = hover;
            self.gesture();
        }
    }

    /// 👆️ A gesture releases the outstanding request and wakes the run to re-read its target.
    fn gesture(&mut self) {
        self.requested = None;
        self.retry = true;
        self.wake();
    }

    pub fn wake(&self) {
        if let Some(port) = &self.port {
            port.wake();
        }
    }

    /// 🗂️ What the run resolved for `target`, if it is the target the run resolved.
    pub fn found(&self, target: &str) -> Option<&BrushSuggestionsFound> {
        self.found.as_ref().filter(|found| found.target == target)
    }

    pub fn is_empty(&self) -> bool {
        self.menu.is_none() && self.hover.is_none() && self.port.is_none() && self.requested.is_none() && self.found.is_none()
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

/// 🧠️ An instance operation owner that carries a [`BrushSuggestionsLink`].
pub trait BrushSuggestionsOwner: std::any::Any {
    fn brush_suggestions(&mut self) -> &mut BrushSuggestionsLink;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BrushSuggestionsPhase {
    Meshes(usize),
    Placed(usize),
    Ready,
}

/// 🧱️ One placed object's collision footprint.
#[derive(semio_framework_value::RetireOwned)]
struct BrushSuggestionsPlaced {
    pub(super) object_id: String,
    pub mesh_url: String,
    pub(super) world: Pose3d,
    bounds: CollisionAabb,
}

/// 🔎️ The search over one target vortex's compatible candidates, resumable between collision units.
#[derive(semio_framework_value::RetireOwned)]
struct BrushSuggestionsSearch {
    pub(super) found: BrushSuggestionsFound,
    cursor: usize,
    pair: usize,
    collision: Option<CollisionPenetrationState>,
    tested: u64,
    pub free: u64,
    pub(super) collisions: u64,
    settled: bool,
}

/// ⏱️ Collision units spend the step's wall clock but never its fuel: one fuel unit is one candidate verdict.
struct BrushSuggestionsCollisionContext<'a, 'b> {
    outer: &'a StepContext<'b>,
}

impl CollisionStepContext for BrushSuggestionsCollisionContext<'_, '_> {
    fn is_cancelled(&self) -> bool {
        self.outer.is_cancelled()
    }

    fn should_yield(&self) -> bool {
        self.outer.deadline_exceeded()
    }

    fn consume_fuel(&mut self, _units: u64) {}
}

/// 📏️ The uniform trace scale of a pose: a number, else the first vector component, else 1.
fn brush_suggestions_scale(scale: &Option<semio_framework_value::DslValue>) -> f32 {
    scale.as_ref().and_then(|value| value.as_f64().or_else(|| value.as_array().and_then(|values| values.first()).and_then(semio_framework_value::DslValue::as_f64))).map_or(1.0, |value| value as f32)
}

/// 🥽️ Where a run reads one mesh identity's geometry: the process-wide derived mesh store in production.
pub type BrushSuggestionsMeshSource = fn(&str) -> Option<(Vec<f32>, Vec<u32>)>;

/// ♻️ The owners one `BrushSuggestionsRunJob` still holds when it closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct BrushSuggestionsRunOwners {
    pub(super) owner: ArtifactInstanceOperationOwnerHandle,
    pub port: ToolRunJobPort,
    pub writer: ToolRunTickWriter,
    pub scene: Arc<SceneConfig>,
    pub(super) catalogs: KindCatalogBundle,
    pub(super) lane: Vec<String>,
    pub(super) meshes: HashMap<String, CollisionBody>,
    fallback: CollisionBody,
    pub(super) placed: Vec<BrushSuggestionsPlaced>,
    search: Option<BrushSuggestionsSearch>,
    released_link: Option<BrushSuggestionsFound>,
}

/// ⏯️ The read-only brush suggestions run job (`📋️tool-run-contract.md` §3.7): bounded preparation of the
/// collision meshes (process-wide derived geometry, else the scaled box fallback) and placed bodies, then a
/// search over the target vortex's compatible candidates. Every candidate is upserted `testing` with its pose,
/// then `success/free` or `danger/collision` — one fuel unit per verdict — and every verdict is published to the
/// instance's [`BrushSuggestionsLink`]. The job follows the link's target: a new target clears the trace and
/// searches again, and a settled search waits on its port until a gesture wakes it.
pub struct BrushSuggestionsRunJob<O: BrushSuggestionsOwner> {
    close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<BrushSuggestionsRunOwners>,
    outbox: semio_s_plugin_puzzle_jobs::JobOutbox,
    pub(super) owner: ArtifactInstanceOperationOwnerHandle,
    pub port: ToolRunJobPort,
    pub writer: ToolRunTickWriter,
    pub scene: Arc<SceneConfig>,
    pub(super) catalogs: KindCatalogBundle,
    pub(super) lane: Vec<String>,
    mesh_source: BrushSuggestionsMeshSource,
    phase: BrushSuggestionsPhase,
    pub(super) meshes: HashMap<String, CollisionBody>,
    fallback: CollisionBody,
    pub(super) placed: Vec<BrushSuggestionsPlaced>,
    search: Option<BrushSuggestionsSearch>,
    pub stage: BrushSuggestionsRunStage,
    published: bool,
    progress_sequence: u64,
    closed: bool,
    owner_kind: PhantomData<fn() -> O>,
}

impl<O: BrushSuggestionsOwner> BrushSuggestionsRunJob<O> {
    /// 🌱️ A run over `scene` whose trace subjects index the published mesh `lane`; a mesh identity `mesh_source`
    /// cannot serve collides as the `fallback` box.
    pub fn new(owner: ArtifactInstanceOperationOwnerHandle, port: ToolRunJobPort, identity: ToolRunIdentity, scene: Arc<SceneConfig>, lane: Vec<String>, mesh_source: BrushSuggestionsMeshSource, fallback: (Vec<f32>, Vec<u32>)) -> Option<Self> {
        let fallback = collision_body_from_buffers(&fallback.0, &fallback.1)?;
        let catalogs = scene.kind_catalogs.clone().unwrap_or_default();
        Some(Self {
            close_owners: Default::default(),
            outbox: Default::default(),
            owner,
            port,
            writer: ToolRunTickWriter::new(identity),
            scene,
            catalogs,
            lane,
            mesh_source,
            phase: BrushSuggestionsPhase::Meshes(0),
            meshes: HashMap::new(),
            fallback,
            placed: Vec::new(),
            search: None,
            stage: BrushSuggestionsRunStage::Prepare,
            published: true,
            progress_sequence: 0,
            closed: false,
            owner_kind: PhantomData,
        })
    }

    fn writer_run(&self) -> (u64, u32) {
        let identity = self.writer.identity();
        (identity.id.run, identity.generation)
    }

    /// 🥽️ One preparation unit; `true` once every mesh and placed body is ready.
    fn prepare_one(&mut self) -> bool {
        match self.phase {
            BrushSuggestionsPhase::Meshes(cursor) => {
                self.phase = match self.lane.get(cursor) {
                    Some(url) => {
                        if let Some(body) = (self.mesh_source)(url).and_then(|(positions, indices)| collision_body_from_buffers(&positions, &indices)) {
                            self.meshes.insert(url.clone(), body);
                        }
                        BrushSuggestionsPhase::Meshes(cursor + 1)
                    }
                    None => BrushSuggestionsPhase::Placed(0),
                };
                false
            }
            BrushSuggestionsPhase::Placed(cursor) => {
                self.phase = match self.scene.scene_snapshot.objects.get(cursor) {
                    Some(object) => {
                        precompute_work(1);
                        if let Some(mesh_url) = resolve_placed_object_mesh_url(object, &self.catalogs, &self.scene.scene_snapshot) {
                            let world = pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale);
                            let bounds = CollisionAabb::from_body(self.meshes.get(&mesh_url).unwrap_or(&self.fallback), &world);
                            self.placed.push(BrushSuggestionsPlaced { object_id: object.id.clone(), mesh_url, world, bounds });
                        }
                        BrushSuggestionsPhase::Placed(cursor + 1)
                    }
                    None => BrushSuggestionsPhase::Ready,
                };
                false
            }
            BrushSuggestionsPhase::Ready => true,
        }
    }

    /// 🎯️ Retargets the search: clears the trace and lists `target`'s compatible candidates with their poses.
    fn begin_search(&mut self, target: Option<String>) {
        self.writer.clear_trace();
        self.published = false;
        let Some(target) = target else {
            self.search = None;
            self.stage = BrushSuggestionsRunStage::Idle;
            return;
        };
        let mut found = BrushSuggestionsFound { writer: self.writer_run(), target: target.clone(), previews: Vec::new(), verdicts: Vec::new(), done: true };
        let scene = Arc::clone(&self.scene);
        let host = scene.scene_snapshot.objects.iter().find_map(|object| object.vortices.iter().position(|vortex| puzzle3d_vortex_full_id(&object.id, &vortex.id) == target).map(|index| (object, index)));
        precompute_work(scene.scene_snapshot.objects.len());
        let refusal = match host {
            None => Some(BrushSuggestionsRunReason::TargetMissing),
            Some((host, index)) if !brush_target_vortex_allows_suggestion(host.vortices[index].vortex_kind.as_deref(), &scene.weights) => Some(BrushSuggestionsRunReason::SuggestionsBlocked),
            Some((host, index)) => match vortex_world_from_object(host, index) {
                None => Some(BrushSuggestionsRunReason::TargetMissing),
                Some((position, direction)) => {
                    let context = AttractionVortexContext { object_kind: host.object_kind.clone(), vortex_kind: host.vortices[index].vortex_kind.clone() };
                    let world = TargetVortexWorld { position, direction, reference_orientation: host.orientation };
                    let candidates = brush_compatible_candidates(&context, &self.catalogs, &scene.kind_compatibility, &scene.host_rules);
                    precompute_work(candidates.len());
                    found.previews = candidates.iter().filter(|candidate| brush_candidate_suggestion_weight(candidate, &scene.weights, &self.catalogs) > 0.0).map(|candidate| brush_preview_from_candidate(&target, candidate, &context, world, &self.catalogs, &scene.scene_snapshot)).collect();
                    found.verdicts = vec![BrushSuggestionVerdict::Pending; found.previews.len()];
                    found.done = false;
                    None
                }
            },
        };
        if let Some(reason) = refusal {
            let _ = self.writer.step(ToolRunStepKind::Warning, BrushSuggestionsRunStage::Target.index(), reason.code(), None, &[]);
        }
        self.stage = if refusal.is_some() { BrushSuggestionsRunStage::Idle } else { BrushSuggestionsRunStage::Test };
        self.search = Some(BrushSuggestionsSearch { found, cursor: 0, pair: 0, collision: None, tested: 0, free: 0, collisions: 0, settled: refusal.is_some() });
    }

    /// 🧪️ One collision unit of the current candidate, or its verdict, or the search's completion step.
    fn test_unit(&mut self, context: &mut StepContext<'_>) {
        let Self { search, writer, meshes, fallback, placed, lane, scene, stage, published, .. } = self;
        let (meshes, fallback, placed, lane) = (&*meshes, &*fallback, &*placed, &*lane);
        let Some(search) = search.as_mut().filter(|search| !search.settled) else { return };
        let key = search.cursor;
        let Some(slot) = search.found.previews.get(key) else {
            search.settled = true;
            search.found.done = true;
            *stage = BrushSuggestionsRunStage::Idle;
            *published = false;
            let _ = writer.step(ToolRunStepKind::Success, BrushSuggestionsRunStage::Test.index(), BrushSuggestionsRunReason::SearchComplete.code(), None, &[ToolRunStepArg::Unsigned(search.free), ToolRunStepArg::Unsigned(search.tested)]);
            return;
        };
        let Some(preview) = slot else {
            writer.upsert(key as u64, ToolRunVerdict::Warning, BrushSuggestionsRunReason::PoseUnavailable.code(), ToolRunTraceSubject::Entity { entity: key as u64 });
            search.found.verdicts[key] = BrushSuggestionVerdict::Unavailable;
            search.tested += 1;
            search.cursor += 1;
            *published = false;
            context.consume_fuel(1);
            return;
        };
        let subject = ToolRunTraceSubject::Instance3d {
            mesh: lane.iter().position(|url| *url == preview.mesh_url).unwrap_or(0) as u32,
            position: preview.origin.map(|value| value as f32),
            rotation: preview.orientation.map(|value| value as f32),
            scale: brush_suggestions_scale(&preview.scale),
        };
        if search.pair == 0 && search.collision.is_none() {
            writer.upsert(key as u64, ToolRunVerdict::Testing, BrushSuggestionsRunReason::Free.code(), subject);
        }
        let body = meshes.get(&preview.mesh_url).unwrap_or(fallback);
        let world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
        let bounds = CollisionAabb::from_body(body, &world);
        let budget = scene.contact_tolerance;
        let collides = loop {
            precompute_work(1);
            let Some(entry) = placed.get(search.pair) else { break Some(false) };
            if search.collision.is_none() && !entry.bounds.intersects(&bounds) {
                search.pair += 1;
                continue;
            }
            let collision = search.collision.get_or_insert_with(|| CollisionPenetrationState::new(budget));
            match collision.step(&mut BrushSuggestionsCollisionContext { outer: context }, body, &world, meshes.get(&entry.mesh_url).unwrap_or(fallback), &entry.world) {
                CollisionStepResult::Pending | CollisionStepResult::Cancelled => break None,
                CollisionStepResult::Complete { depth, .. } => {
                    search.collision = None;
                    search.pair += 1;
                    if depth > budget {
                        break Some(true);
                    }
                }
            }
        };
        let Some(collides) = collides else { return };
        let (verdict, reason, decided) = if collides { (ToolRunVerdict::Danger, BrushSuggestionsRunReason::Collision, BrushSuggestionVerdict::Collision) } else { (ToolRunVerdict::Success, BrushSuggestionsRunReason::Free, BrushSuggestionVerdict::Free) };
        writer.upsert(key as u64, verdict, reason.code(), subject);
        search.found.verdicts[key] = decided;
        search.tested += 1;
        search.free += u64::from(!collides);
        search.collisions += u64::from(collides);
        search.cursor += 1;
        search.pair = 0;
        search.collision = None;
        *published = false;
        context.consume_fuel(1);
    }

    /// 📣️ Publishes an unpublished search to the link, waits on the port once the search it follows is
    /// settled, and flushes the step's tick.
    fn settle_step(&mut self, context: &mut StepContext<'_>, desired: Option<String>) -> JobTurn {
        let following = self.search.as_ref().map(|search| search.found.target.as_str()) == desired.as_deref();
        let idle = self.phase == BrushSuggestionsPhase::Ready && following && self.search.as_ref().is_none_or(|search| search.settled);
        let found = (!self.published).then(|| self.search.as_ref().map(|search| search.found.clone()));
        let port = self.port.clone();
        let exchanged = self.owner.with_mut::<O, _>(|owner| {
            let link = owner.brush_suggestions();
            link.port = Some(port.clone());
            if let Some(found) = found {
                link.found = found;
            }
            if idle && link.target() == desired.as_deref() {
                port.wait();
            }
            Ok(())
        });
        if exchanged.is_ok() {
            self.published = true;
        }
        self.flush(context)
    }

    fn flush(&mut self, _context: &mut StepContext<'_>) -> JobTurn {
        if self.writer.is_empty() {
            return JobTurn::Yield;
        }
        self.progress_sequence += 1;
        let (tested, free, collisions, total) = self.search.as_ref().map_or((0, 0, 0, None), |search| (search.tested, search.free, search.collisions, Some(search.found.previews.len() as u64)));
        let counters = BrushSuggestionsRunCounter::ALL.iter().zip([tested, free, collisions]).map(|(counter, value)| ToolRunCounter { counter: counter.index(), value }).collect();
        self.writer.progress(ToolRunProgress { identity: self.writer.identity(), sequence: self.progress_sequence, state: ToolRunState::Running, stage: self.stage.index(), completed: tested, total, counters, units_per_second: 0.0, conflicts: 0, steps: ToolRunStepRing::default() });
        let Some(Ok(bytes)) = self.writer.finish().map(|tick| tick.encode()) else {
            return JobTurn::Fault(b"brush-suggestions-tick-encode".to_vec());
        };
        if bytes.len() > semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES {
            return JobTurn::Fault(b"brush-suggestions-tick-page".to_vec());
        }
        JobTurn::Preview(bytes.to_vec())
    }
}

impl<O: BrushSuggestionsOwner> BrushSuggestionsRunJob<O> {
    pub fn turn(&mut self, context: &mut StepContext<'_>) -> JobTurn {
        if context.is_cancelled() || self.closed {
            return JobTurn::Cancelled;
        }
        let Ok(desired) = self.owner.with_mut::<O, _>(|owner| Ok(owner.brush_suggestions().target().map(str::to_string))) else {
            return self.flush(context);
        };
        while !context.deadline_exceeded() && !context.fuel_exhausted() {
            precompute_work(1);
            if !self.prepare_one() {
                continue;
            }
            if self.search.as_ref().map(|search| search.found.target.as_str()) != desired.as_deref() {
                self.begin_search(desired.clone());
                continue;
            }
            if self.search.as_ref().is_none_or(|search| search.settled) {
                break;
            }
            self.test_unit(context);
        }
        self.settle_step(context, desired)
    }
}

impl<O: BrushSuggestionsOwner> BrushSuggestionsRunJob<O> {
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

impl<O: BrushSuggestionsOwner> InteractiveJob for BrushSuggestionsRunJob<O> {
    fn step<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{self.step_value(cx).map_err(semio_framework_job::OriginalTurnFailure::from)}

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outbox.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        if std::mem::replace(&mut self.closed, true) {
            return;
        }
        let writer = self.writer_run();
        let released_link = self
            .owner
            .with_mut::<O, _>(|owner| {
                let link = owner.brush_suggestions();
                Ok(link.found.take_if(|found| found.writer == writer))
            })
            .ok()
            .flatten();
        let identity = self.writer.identity();
        self.close_owners.stage(BrushSuggestionsRunOwners {
            owner: self.owner.clone(),
            port: self.port.clone(),
            writer: std::mem::replace(&mut self.writer, ToolRunTickWriter::new(identity)),
            scene: Arc::clone(&self.scene),
            catalogs: std::mem::take(&mut self.catalogs),
            lane: std::mem::take(&mut self.lane),
            meshes: std::mem::take(&mut self.meshes),
            fallback: self.fallback.clone(),
            placed: std::mem::take(&mut self.placed),
            search: self.search.take(),
            released_link,
        });
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> { semio_framework_job::close_original_nested_step(cx, |cx, _grant| {
        self.begin_close();
        semio_s_plugin_puzzle_jobs::job_close_step(&mut self.outbox, &mut self.close_owners, cx)
    }) }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(semio_s_plugin_puzzle_jobs::job_close_demands(&self.outbox, &self.close_owners, 0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(semio_s_plugin_puzzle_jobs::job_close_demands(&self.outbox, &self.close_owners, maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(semio_s_plugin_puzzle_jobs::job_close_demands(&self.outbox, &self.close_owners, 0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(semio_s_plugin_puzzle_jobs::job_close_demands(&self.outbox, &self.close_owners, 0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closed && self.outbox.terminal_is_empty() && self.close_owners.is_empty()
    }
}
//#endregion ⏯️BrushSuggestionsRun

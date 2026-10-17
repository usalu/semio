//! 🗃️ Session registry: the process-wide slots that keep one document instance's session across calls, the instance operation owner and the session itself.

use super::*;

// 🧲️ Gumball drags carry NO app-side session: both hosts paint the drag locally and dispatch exactly
// ONE start→end pose delta (`translateSelection`/`rotateSelection`/`scaleSelection`) on release, which the
// transform tool turns into ONE `ToolTransaction` of relative, parametric selection leaves.

/// 🎟️ Document instances that may hold a live cache slot at once. Sized for real desktop use — a dozen
/// open documents across split panes, with headroom — and deliberately unrelated to
/// any fixed owner page.
pub(super) const PUZZLE3D_SESSION_SLOTS: usize = 64;

/// 🎯️ Slots probed for one instance id before the registry gives up or evicts, mirroring
/// `FillEnvelopeRegistry::begin_measurement`'s bounded candidate walk.
const PUZZLE3D_SESSION_PROBES: usize = 4;

/// ⚖️ Process-wide ceiling on cached session bytes. A check-in whose census would cross it is dropped
/// rather than admitted, so the next call for that instance pays exactly one cold rebuild — the
/// pre-session baseline, never a correctness change.
pub(super) const PUZZLE3D_SESSION_PROCESS_BYTES: usize = 96 * 1024 * 1024;

/// 🧠 Everything one document instance keeps between two dispatches. It is boxed (like the whole
/// state), so the fixed slot row itself never carries the multi-kilobyte payload inline.
#[derive(Default)]
pub(super) struct Puzzle3dSessionState {
    /// 🚚️ Wave B44: the per-object instance residency, so the record text an unchanged object already
    /// has survives a dispatch and a worker hop exactly as the whole-set blob used to.
    pub(super) instances: Option<Box<Puzzle3dInstanceResidency>>,
    pub(super) meshes: Option<(u64, String)>,
    pub(super) collision: Option<Puzzle3dCollisionSession>,
}

impl Puzzle3dSessionState {
    pub(super) fn bytes(&self) -> usize {
        self.instances
            .as_ref()
            .map_or(0, |residency| residency.bytes())
            .saturating_add(self.meshes.as_ref().map_or(0, |(_, meshes)| meshes.len()))
            .saturating_add(self.collision.as_ref().map_or(0, Puzzle3dCollisionSession::bytes))
    }
}

/// 🎟️ One slot of the fixed row. The cached state lives behind a pointer on purpose: the row is a
/// `[Option<Puzzle3dSessionSlot>; PUZZLE3D_SESSION_SLOTS]` built by value, so an inline payload would be
/// multiplied by every slot on the stack of whichever call first touches the registry.
pub(super) struct Puzzle3dSessionSlot {
    app_instance_id: u32,
    artifact_id: Option<String>,
    pub(super) bytes: usize,
    touched: u64,
    state: Option<Box<Puzzle3dSessionState>>,
}

/// 🎫 What a caller holds while a slot's state is checked out. The slot's own generation plus the
/// instance id make it ABA-proof: a slot that was retired, evicted or re-keyed rejects the lease
/// instead of adopting a cache built against a different document.
#[derive(Clone, Copy)]
pub(super) struct Puzzle3dSessionLease {
    slot: usize,
    app_instance_id: u32,
    pub(super) generation: u64,
}

pub(super) struct Puzzle3dSessionRegistry {
    slots: [Option<Puzzle3dSessionSlot>; PUZZLE3D_SESSION_SLOTS],
    pub(super) generations: [u64; PUZZLE3D_SESSION_SLOTS],
    pub(super) aggregate_bytes: usize,
    sequence: u64,
}

impl Default for Puzzle3dSessionRegistry {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| None), generations: [0; PUZZLE3D_SESSION_SLOTS], aggregate_bytes: 0, sequence: 0 }
    }
}

pub(super) fn puzzle3d_session_registry() -> &'static Mutex<Puzzle3dSessionRegistry> {
    static REGISTRY: OnceLock<Mutex<Puzzle3dSessionRegistry>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(Puzzle3dSessionRegistry::default()))
}

impl Puzzle3dSessionRegistry {
    fn candidates(app_instance_id: u32) -> impl Iterator<Item = usize> {
        let base = usize::try_from(app_instance_id).unwrap_or(0) % PUZZLE3D_SESSION_SLOTS;
        (0..PUZZLE3D_SESSION_PROBES).map(move |offset| (base + offset) % PUZZLE3D_SESSION_SLOTS)
    }

    fn owns(&self, slot: usize, app_instance_id: u32) -> bool {
        self.slots[slot].as_ref().is_some_and(|entry| entry.app_instance_id == app_instance_id)
    }

    /// 🚪️ Drops the state a slot held and bumps its generation, so every lease taken against it fails to
    /// check in. Driven in production the moment one instance id is observed carrying a different
    /// parent document — the framework reuses instance ids, and a reused id is a new document.
    fn retire(&mut self, slot: usize) {
        self.vacate(slot);
        semio_s_artifact_puzzle_3d_precompute::retire_abandoned_brush_mesh_uploads();
    }

    /// 🕳️ Empties one slot and bumps its generation without sweeping the process-wide page-run uploads.
    fn vacate(&mut self, slot: usize) {
        if let Some(entry) = self.slots[slot].take() {
            self.aggregate_bytes = self.aggregate_bytes.saturating_sub(entry.bytes);
        }
        self.generations[slot] = self.generations[slot].saturating_add(1);
    }

    fn resolve_slot(&mut self, app_instance_id: u32, artifact_id: Option<&str>) -> Option<usize> {
        if let Some(slot) = Self::candidates(app_instance_id).find(|slot| self.owns(*slot, app_instance_id)) {
            let rekeyed = self.slots[slot].as_ref().is_some_and(|entry| entry.artifact_id.as_deref().zip(artifact_id).is_some_and(|(held, current)| held != current));
            if rekeyed {
                self.retire(slot);
            }
            return Some(slot);
        }
        if let Some(slot) = Self::candidates(app_instance_id).find(|slot| self.slots[*slot].is_none()) {
            return Some(slot);
        }
        let evicted = Self::candidates(app_instance_id).filter(|slot| self.slots[*slot].as_ref().is_some_and(|entry| entry.state.is_some())).min_by_key(|slot| self.slots[*slot].as_ref().map_or(u64::MAX, |entry| entry.touched))?;
        self.retire(evicted);
        Some(evicted)
    }

    pub(super) fn check_out(&mut self, app_instance_id: u32, artifact_id: Option<&str>) -> Option<(Puzzle3dSessionLease, Puzzle3dSessionState)> {
        let slot = self.resolve_slot(app_instance_id, artifact_id)?;
        self.sequence = self.sequence.saturating_add(1);
        let previous = self.slots[slot].take();
        if let Some(entry) = &previous {
            self.aggregate_bytes = self.aggregate_bytes.saturating_sub(entry.bytes);
        }
        let held = previous.and_then(|entry| entry.state);
        let artifact_id = artifact_id.map(str::to_string);
        self.slots[slot] = Some(Puzzle3dSessionSlot { app_instance_id, artifact_id, bytes: 0, touched: self.sequence, state: None });
        Some((Puzzle3dSessionLease { slot, app_instance_id, generation: self.generations[slot] }, held.map_or_else(Puzzle3dSessionState::default, |state| *state)))
    }

    pub(super) fn check_in(&mut self, lease: Puzzle3dSessionLease, state: Puzzle3dSessionState) {
        if self.generations[lease.slot] != lease.generation {
            return;
        }
        let bytes = state.bytes();
        if self.aggregate_bytes.checked_add(bytes).is_none_or(|total| total > PUZZLE3D_SESSION_PROCESS_BYTES) {
            return;
        }
        let Some(entry) = self.slots[lease.slot].as_mut().filter(|entry| entry.app_instance_id == lease.app_instance_id) else {
            return;
        };
        entry.bytes = bytes;
        entry.state = Some(Box::new(state));
        self.aggregate_bytes = self.aggregate_bytes.saturating_add(bytes);
    }
}

/// 🎟️ Adopts one instance's session onto a freshly built app. An exhausted slot row or a first-ever call
/// resolves to "no cached state", which is exactly the pre-session behaviour. The registry lock is WAITED
/// for, never skipped: its critical section is a slot move, and the session carries behaviour, not only
/// speed — the standing mesh re-upload set lives here, so a skipped adoption would answer an already
/// requested identity as new and re-arm the re-announce storm (wave B32).
fn puzzle3d_session_check_out(app_instance_id: u32, artifact_id: Option<&str>, app: &Puzzle3dSession) -> Option<Puzzle3dSessionLease> {
    let (lease, state) = puzzle3d_session_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).check_out(app_instance_id, artifact_id)?;
    *app.instance_residency.lock().expect("instance residency") = state.instances;
    *app.mesh_cache.lock().expect("mesh cache") = state.meshes;
    {
        let mut session = app.precompute.borrow_mut();
        if let Some(collision) = state.collision {
            session.install_collision_session(collision);
        }
    }
    Some(lease)
}

/// 🎟️ Returns the app's caches to their slot, waiting for the registry lock like
/// [`puzzle3d_session_check_out`]. A stale lease (the slot was retired or re-keyed mid-call) is rejected and
/// the state simply dropped.
fn puzzle3d_session_check_in(lease: Puzzle3dSessionLease, app: &Puzzle3dSession) {
    let collision = Some(app.precompute.borrow_mut().take_collision_session());
    let state = Puzzle3dSessionState {
        instances: app.instance_residency.lock().expect("instance residency").take(),
        meshes: app.mesh_cache.lock().expect("mesh cache").take(),
        collision,
    };
    puzzle3d_session_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).check_in(lease, state);
}

/// 🧹️ Vacates every slot `app_instance_id` owns — what a scene_snapshot app does when it is dropped, so one law's
/// session never outlives it into the process-global row the next law probes. The page-run upload sweep is
/// left to the production retirements, so a concurrent law's open upload is never swept by another's drop.
#[cfg(test)]
pub fn puzzle3d_session_release(app_instance_id: u32) {
    let mut registry = puzzle3d_session_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    for slot in Puzzle3dSessionRegistry::candidates(app_instance_id).collect::<Vec<_>>() {
        if registry.owns(slot, app_instance_id) {
            registry.vacate(slot);
        }
    }
}

/// 🪪️ The artifact-instance identity the framework already threads into every view — the session key.
/// A command operation carries the parent artifact id too, which is what lets a reused instance id
/// retire the previous artifact's cache instead of adopting it.
pub fn puzzle3d_view_session_key(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>) -> Option<(u32, Option<String>)> {
    if let Some(operation) = doc.operation_optional() {
        return Some((operation.app_instance_id, Some(operation.parent_document_id.clone())));
    }
    doc.render_operation().map(|operation| (operation.app_instance_id, None))
}

/// 🧠 One document instance's play session. `ArtifactApp` methods are associated fns (no `&self`), so
/// every dispatch still builds a fresh app object — but its caches are now checked out of, and back
/// into, a process-global slot keyed by `app_instance_id`, so `geometry_cache`/`instance_residency`/
/// registered brush meshes/the brush broad-phase index survive a call and a worker hop. No tool run state
/// lives here: a fill run is owned by the framework tool run ledger.
pub fn with_puzzle3d_session_for<R>(session: Option<(u32, Option<String>)>, config: &Puzzle3dRuntime, f: impl FnOnce(&Puzzle3dSession) -> R) -> R {
    let app = Puzzle3dSession::default();
    let lease = session.and_then(|(app_instance_id, artifact_id)| puzzle3d_session_check_out(app_instance_id, artifact_id.as_deref(), &app));
    let _ = config;
    let result = f(&app);
    if let Some(lease) = lease {
        puzzle3d_session_check_in(lease, &app);
    }
    result
}

#[cfg(test)]
pub fn with_puzzle3d_session<R>(f: impl FnOnce(&Puzzle3dSession) -> R) -> R {
    let app = Puzzle3dSession::default();
    f(&app)
}

thread_local! {
    /// 🔬️ How many times `geometry_jsons` genuinely re-serialized a scene_snapshot on THIS thread. The session
    /// registry exists so that a second call on the same document does not, and the tests read this
    /// counter instead of inferring cache behaviour from timings. Thread-local so one test's measurement
    /// cannot be perturbed by another test running concurrently in the same process.
    pub static PUZZLE3D_GEOMETRY_SERIALIZATIONS: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
    /// 🔬️ How many times `ArtifactApp::interaction_topology` decoded the document and rebuilt the whole
    /// object/vortex/attraction forest on THIS thread. One pick used to pay it TWICE — once in the
    /// framework's own `interactionSelect` arm and once in the revalidation behind it — which on the
    /// 180-object flagship is the O(n) term wave B44 §2.2 measured as `interactionSelect`'s 20×.
    pub static PUZZLE3D_INTERACTION_TOPOLOGY_BUILDS: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

/// 🧠️ What one document instance retains across turns for its commands, renders and tool run jobs: the brush
/// suggestions link. Ephemeral local-only; it dies with the instance.
#[derive(Default)]
pub struct Puzzle3dInstanceOperationOwner {
    pub brush_suggestions: BrushSuggestionsLink,
    retirement: semio_s_plugin_puzzle_jobs::WorkClosing<BrushSuggestionsLink>,
}

impl BrushSuggestionsOwner for Puzzle3dInstanceOperationOwner {
    fn brush_suggestions(&mut self) -> &mut BrushSuggestionsLink {
        &mut self.brush_suggestions
    }
}

impl semio_framework_plugin::ArtifactInstanceOperationOwner for Puzzle3dInstanceOperationOwner {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if self.retirement.is_empty() {
            return Ok(semio_framework_value::RetirementDemand { depth: usize::from(!self.brush_suggestions.is_empty()), ..Default::default() });
        }
        self.retirement.demands(body)
    }

    fn maintenance_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,_grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_plugin::PluginLifecycleStep, semio_framework_plugin::PluginLifecycleFailure> {
        Ok(semio_framework_plugin::PluginLifecycleStep::Complete(Default::default()))
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_plugin::PluginLifecycleStep, semio_framework_plugin::PluginLifecycleFailure> {
        if self.retirement.is_empty() && !self.brush_suggestions.is_empty() {
            self.retirement.stage(std::mem::take(&mut self.brush_suggestions));
        }
        Ok(match self.retirement.close_step(grant) {
            InteractiveJobCloseStep::Pending { progress } => semio_framework_plugin::PluginLifecycleStep::Progress(progress),
            InteractiveJobCloseStep::Complete { progress } => semio_framework_plugin::PluginLifecycleStep::Complete(progress),
            InteractiveJobCloseStep::Blocked => semio_framework_plugin::PluginLifecycleStep::Blocked { reason: "puzzle3d instance owner close is blocked" },
            InteractiveJobCloseStep::Refused { kind, .. } => return Err(Fault::from(format!("puzzle3d instance owner close was refused: {kind:?}"))),
        })
    }

    fn terminal_is_empty(&self) -> bool {
        self.brush_suggestions.is_empty() && self.retirement.is_empty()
    }
}

pub struct Puzzle3dSession {
    pub precompute: std::cell::RefCell<Box<Puzzle3dPrecomputeSession>>,
    /// 🚚️ Wave B44: per-object instance residency — the ONE owner of the published instance text, and
    /// the source of the changed/removed id delta the world lane rides with.
    pub instance_residency: Mutex<Option<Box<Puzzle3dInstanceResidency>>>,
    pub(super) mesh_cache: Mutex<Option<(u64, String)>>,
}

impl Default for Puzzle3dSession {
    fn default() -> Self {
        Self {
            precompute: std::cell::RefCell::new(Box::new(Puzzle3dPrecomputeSession::new())),
            instance_residency: Mutex::new(None),
            mesh_cache: Mutex::new(None),
        }
    }
}

impl Puzzle3dSession {
    /// 🚚️ The world lane's three geometry payloads for one scene_snapshot: the authoritative instance set, the
    /// mesh declarations, and the per-object delta that produced the current instance revision.
    ///
    /// ⏱️ Wave B44: invalidation is PER OBJECT. Only the records whose own
    /// `instance_record_fingerprint` moved are re-serialized; the mesh declarations and the outliner
    /// memo stay keyed on the (now allocation-free) whole-scene_snapshot `scene_geometry_fingerprint`, which
    /// is what actually changes when the catalogs or the mesh set do.
    pub fn geometry_jsons(&self, scene_snapshot: &Puzzle3dSceneSnapshot, provisional: &std::collections::BTreeSet<u64>, referenced: &[String]) -> (String, String, Option<String>) {
        let fingerprint = scene_geometry_fingerprint(scene_snapshot);
        let mut residency = self.instance_residency.lock().expect("instance residency");
        let residency = residency.get_or_insert_with(Box::<Puzzle3dInstanceResidency>::default);
        let republished = residency.refresh(scene_snapshot, provisional, referenced);
        let mut meshes = self.mesh_cache.lock().expect("mesh cache");
        let mesh_miss = meshes.as_ref().is_none_or(|(cached, _)| *cached != fingerprint);
        if mesh_miss {
            *meshes = Some((fingerprint, world_meshes_json(scene_snapshot)));
        }
        if republished || mesh_miss {
            PUZZLE3D_GEOMETRY_SERIALIZATIONS.with(|counter| counter.set(counter.get().saturating_add(1)));
        }
        let (_, meshes) = meshes.as_ref().expect("mesh cache populated");
        (residency.instances_json().to_string(), meshes.clone(), residency.delta_json().map(str::to_string))
    }

}

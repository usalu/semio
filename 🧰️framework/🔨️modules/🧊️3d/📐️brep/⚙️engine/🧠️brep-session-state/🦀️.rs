//! 🧠️ B-Rep session state: the live entity enum with its retirement, the native session struct, the boolean job, admission and step records and the session defaults. See [`super`].

use super::*;

// #region 🔖️Types
/// 🧠 One live registry entry. Vertex/Edge/Face/🐚️Shell/Solid wrap the arena id whose own
/// [`semio_framework_3d_brep_representation::topology::history::PersistentLabel`]
/// [`label_of_entity`] resolves through; Wire/Curve/Surface/Compound carry no arena identity of
/// their own (a `Wire` bundles arena ids but isn't itself stored in `Body`; a bare `Curve3`/
/// `Surface` constructed via `register_curve`/`register_surface` never enters `body.curves3`/
/// `body.surfaces` either) so each instance is stamped with its own fresh label at registration
/// time instead.
#[derive(Clone)]
pub(super) enum Entity {
    Vertex(VertexId),
    Edge(EdgeId),
    Wire(Wire, PersistentLabel),
    Face(FaceId),
    Shell(ShellId),
    Solid(SolidId),
    Compound(Vec<SolidId>, PersistentLabel),
    Curve(Curve3, PersistentLabel),
    Surface(Surface, PersistentLabel),
}

impl semio_framework_value::retirement::RetireOwned for Entity {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {
        match self {
            Self::Vertex(id)=>semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(id)]),
            Self::Edge(id)=>semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(id)]),
            Self::Wire(wire,label)=>semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(wire),semio_framework_value::retirement::deferred(label)]),
            Self::Face(id)=>semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(id)]),
            Self::Shell(id)=>semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(id)]),
            Self::Solid(id)=>semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(id)]),
            Self::Compound(solids,label)=>semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(solids),semio_framework_value::retirement::deferred(label)]),
            Self::Curve(curve,label)=>semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(curve),semio_framework_value::retirement::deferred(label)]),
            Self::Surface(surface,label)=>semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(surface),semio_framework_value::retirement::deferred(label)]),
        }
    }
    fn retirement_birth_bytes(&self)->Option<usize> {
        match self {
            Self::Vertex(id)=>semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(id)]),
            Self::Edge(id)=>semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(id)]),
            Self::Wire(wire,label)=>semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(wire),semio_framework_value::retirement::deferred_birth_bytes_for(label)]),
            Self::Face(id)=>semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(id)]),
            Self::Shell(id)=>semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(id)]),
            Self::Solid(id)=>semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(id)]),
            Self::Compound(solids,label)=>semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(solids),semio_framework_value::retirement::deferred_birth_bytes_for(label)]),
            Self::Curve(curve,label)=>semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(curve),semio_framework_value::retirement::deferred_birth_bytes_for(label)]),
            Self::Surface(surface,label)=>semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(surface),semio_framework_value::retirement::deferred_birth_bytes_for(label)]),
        }
    }
    fn controlled_retirement_supported()->bool {true}
}

/// 🧠 Native B-Rep session.
pub struct Brep {
    pub(super) body: Body,
    pub(super) live: semio_framework_mesh_engine::HistoryFoldIndex<String, Entity>,
    pub(super) pending_mutations: usize,
    pub(super) live_revision:u64,
}

semio_framework_value::artifact_retire_struct!(Brep {body,live,pending_mutations,live_revision});

/// ⏱️ A retained resumable boolean plus the operation recorder its whole run accumulates into —
/// the recorder must outlive every step, which is why the job owns it rather than borrowing the
/// caller's (ticket `26/09/09/PROCEDURAL-3D-END-TO-END`).
pub struct BrepBooleanJob {
    pub(super) job: BooleanJob,
    pub(super) rec: OpRecorder,
}

impl BrepBooleanJob {
    /// 📈️ Progress right now — safe to read between steps and after termination.
    pub fn progress(&self) -> BooleanProgress {
        self.job.progress()
    }

    /// 🛑️ Retires this boolean at the next observable boundary.
    pub fn cancel(&mut self) {
        self.job.cancel();
    }

    /// ✅️ True once the job reached a terminal phase.
    pub fn is_terminal(&self) -> bool {
        self.job.is_terminal()
    }
}

/// 🔀️ What [`Brep::boolean_job_sync`] admitted.
pub enum BrepBooleanAdmission {
    /// ⚡️ A fast path answered exactly and in place; there is nothing to resume.
    Answered(GeometryHandle),
    /// ⏱️ The general exact engine, as a budgetable job.
    Job(BrepBooleanJob),
}

/// ⏱️ Outcome of one [`Brep::step_boolean_job_sync`].
pub enum BrepBooleanStep {
    /// 🔁 Budget spent, work remains.
    Working(BooleanProgress),
    /// ✅️ Terminal: the result solid, registered as a live handle.
    Ready(GeometryHandle),
    /// 🛑️ Terminal: the job was cancelled; nothing is produced.
    Cancelled(BooleanProgress),
}

impl Default for Brep {
    fn default() -> Self {
        Self::new()
    }
}

impl Brep {
    /// 📸️ Read-only mathematical topology for an owner-supplied format codec.
    pub fn representation(&self) -> &Body { &self.body }
    /// 🧊️ Resolves a live solid without exposing handle registry mutation.
    pub fn geometry_solid(&self, handle: &GeometryHandle) -> Result<SolidId,BrepError> { self.solid_id(handle) }
    /// 📥️ Merges mathematical topology while preserving every prior live handle.
    pub fn merge_representation(&mut self, imported: Body) -> Vec<GeometryHandle> {
        let solids = imported.solids.ids().collect::<Vec<_>>();
        let map = self.body.merge(&imported);
        solids.into_iter().map(|id| self.register_solid(map.solids[&id])).collect()
    }

    /// 🪪️ Enumerates registered handles for explicit session operation claims.
    pub fn live_handles(&self) -> std::collections::HashSet<String> { self.live.keys().cloned().collect() }

    /// 🏗️ Empty native kernel session.
    pub fn new() -> Self {
        Self { body: Body::new(), live: semio_framework_mesh_engine::HistoryFoldIndex::new(), pending_mutations: 0,live_revision:0 }
    }
}
// #endregion 🔖️Types

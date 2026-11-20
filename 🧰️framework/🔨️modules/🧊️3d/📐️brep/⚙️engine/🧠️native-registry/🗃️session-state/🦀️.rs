//! 🧺️ Native B-Rep arena and registry state.

use super::*;

pub use semio_framework_3d_brep_solid_transaction::{PrimitiveProvenance, SolidProvenance};

/// 🧠 One live registry entry. Vertex/Edge/Face/🐚️Shell/Solid wrap the arena id whose own
/// [`semio_framework_3d_brep_representation::topology::history::PersistentLabel`]
/// [`label_of_entity`] resolves through; Wire/Curve/Surface/Compound carry no arena identity of
/// their own (a `Wire` bundles arena ids but isn't itself stored in `Body`; a bare `Curve3`/
/// `Surface` constructed via `register_curve`/`register_surface` never enters `body.curves3`/
/// `body.surfaces` either) so each instance is stamped with its own fresh label at registration
/// time instead.
#[derive(Clone)]
#[derive(semio_framework_value::RetireOwned)]
pub enum Entity {
    Vertex(VertexId),
    Edge(EdgeId),
    Wire(Wire, PersistentLabel),
    Face(FaceId),
    Shell(ShellId),
    Solid(SolidId, Option<SolidProvenance>),
    Compound(Vec<SolidId>, PersistentLabel),
    Curve(Curve3, PersistentLabel),
    Surface(Surface, PersistentLabel),
}


/// 🧠 Live topology and entity handles of one B-Rep session.
pub struct BrepRegistry {
    pub body: Body,
    pub live: semio_framework_mesh_engine::HistoryFoldIndex<String, Entity>,
    pub pending_mutations: PendingMutations,
    pub unpublished_topology: usize,
    pub live_revision:u64,
    pub shape_mutation_revision:u64,
}

semio_framework_value::artifact_retire_struct!(BrepRegistry {body,live,pending_mutations,unpublished_topology,live_revision,shape_mutation_revision});

impl Default for BrepRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl BrepRegistry {
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
        Self { body: Body::new(), live: semio_framework_mesh_engine::HistoryFoldIndex::new(), pending_mutations: PendingMutations::default(),unpublished_topology:0,live_revision:0,shape_mutation_revision:0 }
    }
}
// #endregion 🔖️Types

impl BrepRegistry {
    pub fn advance_live_revision(&mut self){self.live_revision=self.live_revision.checked_add(1).expect("original live registry revision overflow");}
    pub fn retention_source_revision(&self)->(u64,u64){(self.live_revision,self.body.labels.next())}
    /// 👀️ Borrows the original live registry without constructing a handle owner.
    pub fn contains_live_handle(&self,handle:&str)->bool{self.live.get(handle).is_some()}
    pub fn original_query_entity(&self, handle: &str) -> Result<&Entity, ValueError> {
        if handle.len() != 64 {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "geometry query requires a canonical live handle"));
        }
        self.live.get(handle).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "geometry query requires a live handle"))
    }

    /// 🧊️ Quotes the original borrowed handle index before cold query admission.
    pub fn tessellation_source_depth_demand(&self,source:&str)->Result<usize,semio_framework_value::ValueError>{self.live.next_contains_depth_demand(source)}
}

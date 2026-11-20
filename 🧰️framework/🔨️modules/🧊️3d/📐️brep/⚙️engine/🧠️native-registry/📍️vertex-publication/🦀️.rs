//! 📍️ Retained publication of one native vertex.

use super::analytic_publication::primitive_handle;
use super::solid_publication::{store_demand, store_insert, topology_slot_depth};
use super::*;
use semio_framework_3d_brep_representation::topology::{Vertex, history::LabelSource};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneGrant as Grant, RetainedCloneProgress as Progress, RetainedCloneStep as Step, RetirementDemand};

/// 📍️ Owns a planned vertex until its original session publishes or rolls back its arena slot.
pub struct VertexPublication {
    owner: usize,
    point: EVec3,
    label: PersistentLabel,
    handle: [u8; 64],
    vertex: Option<VertexId>,
    open: bool,
}

impl VertexPublication {
    pub fn handle(&self) -> &str { std::str::from_utf8(&self.handle).expect("vertex handle uses lowercase hexadecimal") }
    pub fn topology_ready(&self) -> bool { self.vertex.is_some() }
    pub fn terminal_is_empty(&self) -> bool { !self.open }

    fn check_owner(&self, kernel: &BrepRegistry) -> Result<(), ValueError> {
        if self.open && self.owner == std::ptr::from_ref(kernel) as usize { Ok(()) }
        else { Err(ValueError::invariant("vertex publication requires its original unpublished session")) }
    }
}

impl BrepRegistry {
    /// 🎟️ Validates the source and reserves a stable label without allocating topology.
    pub fn plan_vertex_publication(&mut self, point: EVec3) -> Result<VertexPublication, ValueError> {
        if !point.iter().all(|value| value.is_finite()) {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "vertex requires a finite point"));
        }
        let label = self.body.labels.next();
        let next = label.checked_add(1).ok_or_else(|| ValueError::invariant("vertex labels are exhausted"))?;
        self.body.labels = LabelSource::from_next(next);
        self.unpublished_topology += 1;
        Ok(VertexPublication {
            owner: std::ptr::from_ref(&*self) as usize,
            point,
            label: PersistentLabel(label),
            handle: primitive_handle(b"Vertex:", label),
            vertex: None,
            open: true,
        })
    }

    /// 📐️ Quotes the next arena or live index turn for the original vertex.
    pub fn vertex_publication_demand(&self, vertex: &VertexPublication, copy: usize) -> Result<RetirementDemand, ValueError> {
        vertex.check_owner(self)?;
        if vertex.topology_ready() {
            let key = vertex.handle();
            return Ok(RetirementDemand { capacity_bytes: self.live.next_insert_capacity_byte_demand(key, copy)?,
                depth: self.live.next_insert_depth_demand(key)?, ..Default::default() });
        }
        let mut demand = store_demand(&self.body.vertices)?;
        if demand.capacity_bytes == 0 { demand.copy_bytes = std::mem::size_of::<EVec3>(); }
        Ok(demand)
    }

    /// 🧱️ Stages one vertex record under the initiating grant.
    pub fn step_vertex_topology(&mut self, vertex: &mut VertexPublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        if vertex.topology_ready() { return Err(ValueError::invariant("vertex topology is already complete")); }
        let grant = effect.grant();
        if grant.maximum_items == 0 || !retained_clone_grant_admits(grant, self.vertex_publication_demand(vertex, grant.maximum_copy_bytes)?) || !effect.performed(1)? {
            return Ok(Progress::default());
        }
        let (id, mut receipt) = store_insert(&mut self.body.vertices, grant, || Vertex {
            position: pnt(vertex.point), tol: Tol::DEFAULT, label: vertex.label,
        })?;
        if id.is_some() {
            vertex.vertex = id;
            receipt.copied_bytes = std::mem::size_of::<EVec3>();
        }
        Ok(receipt)
    }

    /// 🗄️ Reserves live index capacity after the vertex slot is staged.
    pub fn reserve_vertex_publication(&mut self, vertex: &VertexPublication, grant: Grant) -> Result<Progress, ValueError> {
        vertex.check_owner(self)?;
        if !vertex.topology_ready() { return Err(ValueError::invariant("vertex topology is not complete")); }
        self.live.reserve_insert_step(vertex.handle(), grant).map_err(|(error, receipt)| error.with_retained_progress(receipt))
    }

    /// 📤️ Publishes the staged slot under the original live key.
    pub fn publish_vertex(&mut self, vertex: &mut VertexPublication, key: String, grant: Grant) -> Result<Progress, (ValueError, String)> {
        if let Err(error) = vertex.check_owner(self) { return Err((error, key)); }
        let Some(id) = vertex.vertex else { return Err((ValueError::invariant("vertex topology is not complete"), key)); };
        if key != vertex.handle() || self.live.contains_key(&key) {
            return Err((ValueError::invariant("invalid vertex publication key"), key));
        }
        let (previous, receipt) = self.live.insert_reserved(key, Entity::Vertex(id), grant).map_err(|(error, key, _)| (error, key))?;
        assert!(previous.is_none());
        vertex.vertex = None;
        vertex.open = false;
        self.unpublished_topology -= 1;
        self.advance_live_revision();
        Ok(receipt)
    }

    /// 🧹️ Quotes retirement of an unpublished vertex.
    pub fn vertex_close_demand(&self, vertex: &VertexPublication) -> Result<RetirementDemand, ValueError> {
        vertex.check_owner(self)?;
        let depth = vertex.vertex.map_or(Ok(1), |id| topology_slot_depth(&self.body, EntityRef::Vertex(id)))?;
        Ok(RetirementDemand { depth, ..Default::default() })
    }

    /// ♻️ Returns the arena slot when a vertex producer stops before publication.
    pub fn close_vertex(&mut self, vertex: &mut VertexPublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Step, ValueError> {
        let demand = self.vertex_close_demand(vertex)?;
        let grant = effect.grant();
        if grant.maximum_items == 0 || !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? {
            return Ok(Step::Progress(Progress::default()));
        }
        if let Some(id) = vertex.vertex {
            self.body.vertices.remove(id).ok_or_else(|| ValueError::invariant("unpublished vertex is missing"))?;
        }
        vertex.vertex = None;
        vertex.open = false;
        self.unpublished_topology -= 1;
        Ok(Step::Complete(Progress { copied_items: 1, ..Default::default() }))
    }
}

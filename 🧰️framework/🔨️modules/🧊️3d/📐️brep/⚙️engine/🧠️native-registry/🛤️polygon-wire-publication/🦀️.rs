//! 📏️ Original wire publication for retained polygonal and sampled helix recipes.

use super::*;
use semio_framework_3d_brep_representation::arena::Curve3Id;
use semio_framework_3d_brep_representation::topology::{Edge, Vertex};
pub use semio_framework_3d_brep_solid_topology::{HelixWireRecipe, PolygonWireRecipe};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneGrant as Grant, RetainedCloneProgress as Progress, RetainedCloneStep as Step, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use solid_publication::{remove_topology_slot, store_demand, store_insert, topology_slot_depth};
use std::mem::{size_of, take};

/// 🛤️ One private owner for all original polygonal wires. The live index receives it once complete.
pub struct PolygonWirePublication {
    owner: usize,
    recipe: PolygonWireRecipe,
    wire: Wire,
    curve: Option<Curve3Id>,
    vertex_count: usize,
    edge_count: usize,
    first_label: u64,
    label: PersistentLabel,
    handle: [u8; 64],
    open: bool,
    boundary_transferred: bool,
    tolerance: Tol,
}

impl PolygonWirePublication {
    pub fn handle(&self) -> &str { std::str::from_utf8(&self.handle).expect("digest uses lowercase hexadecimal") }
    pub fn topology_ready(&self) -> bool { self.wire.members.len() == self.edge_count }
    pub fn terminal_is_empty(&self) -> bool { !self.open }

    pub(super) fn set_tolerance(&mut self, tolerance: Tol) { self.tolerance = tolerance; }

    /// 🔢️ Points from the original polyline input are borrowed one at a time.
    pub fn next_point_index(&self) -> Option<usize> {
        let index = self.wire.vertices.len();
        (matches!(self.recipe, PolygonWireRecipe::Polyline { .. }) && self.wire.vertices.capacity() != 0
            && self.wire.members.capacity() != 0 && index < self.vertex_count).then_some(index)
    }

    fn check_owner(&self, kernel: &BrepRegistry) -> Result<(), ValueError> {
        if self.open && self.owner == std::ptr::from_ref(kernel) as usize { Ok(()) }
        else { Err(ValueError::invariant("polygonal wire requires its original unpublished session")) }
    }

    pub(super) fn face_boundary<'a>(&'a self, kernel: &BrepRegistry) -> Result<&'a Wire, ValueError> {
        self.check_owner(kernel)?;
        if !self.topology_ready() || !self.wire.closed || self.boundary_transferred {
            return Err(ValueError::invariant("polygon face requires a completed private boundary"));
        }
        Ok(&self.wire)
    }

    fn endpoints(&self) -> (VertexId, VertexId) {
        let index = self.wire.members.len();
        (self.wire.vertices[index], self.wire.vertices[(index + 1) % self.vertex_count])
    }

    fn demand(&self, body: &Body) -> Result<RetirementDemand, ValueError> {
        if self.wire.vertices.capacity() == 0 {
            return Ok(capacity(self.vertex_count * size_of::<VertexId>()));
        }
        if self.wire.members.capacity() == 0 {
            return Ok(capacity(self.edge_count * size_of::<(EdgeId, bool)>()));
        }
        if self.wire.vertices.len() < self.vertex_count {
            let mut demand = store_demand(&body.vertices)?;
            if demand.capacity_bytes == 0 { demand.copy_bytes = size_of::<EVec3>() + size_of::<VertexId>(); }
            return Ok(demand);
        }
        if self.curve.is_some() {
            let mut demand = store_demand(&body.edges)?;
            if demand.capacity_bytes == 0 { demand.copy_bytes = size_of::<(EdgeId, bool)>(); }
            return Ok(demand);
        }
        let mut demand = store_demand(&body.curves3)?;
        if demand.capacity_bytes == 0 {
            let (a, b) = self.endpoints();
            demand.depth = demand.depth.max(body.vertices.next_remove_depth_demand(a)?).max(body.vertices.next_remove_depth_demand(b)?);
            demand.copy_bytes = 2 * size_of::<Pnt3>();
        }
        Ok(demand)
    }

    fn advance(&mut self, body: &mut Body, point: Option<EVec3>, grant: Grant) -> Result<Progress, ValueError> {
        if self.wire.vertices.capacity() == 0 { return reserve(&mut self.wire.vertices, self.vertex_count, grant); }
        if self.wire.members.capacity() == 0 { return reserve(&mut self.wire.members, self.edge_count, grant); }
        if self.wire.vertices.len() < self.vertex_count {
            let index = self.wire.vertices.len();
            if body.vertices.next_insert_capacity_byte_demand(0)? != 0 {
                return body.vertices.reserve_insert_step(grant).map_err(|(error, receipt)| error.with_retained_progress(receipt));
            }
            let position = point.or_else(|| self.recipe.vertex(index)).filter(|p| p.iter().all(|value| value.is_finite()))
                .ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "polyline requires finite ordered points"))?;
            let label = PersistentLabel(self.first_label + index as u64);
            let (id, mut receipt) = body.vertices.insert_reserved(Vertex { position: pnt(position), label, tol: self.tolerance }, grant)
                .map_err(|(error, _)| error)?;
            self.wire.vertices.push(id);
            receipt.copied_bytes = size_of::<EVec3>() + size_of::<VertexId>();
            return Ok(receipt);
        }
        let (a, b) = self.endpoints();
        if let Some(curve) = self.curve {
            let label = PersistentLabel(self.first_label + (self.vertex_count + self.wire.members.len()) as u64);
            let (id, mut receipt) = store_insert(&mut body.edges, grant, || Edge { curve, range: (0., 1.), v0: a, v1: b, tol: self.tolerance, label })?;
            if let Some(id) = id {
                self.wire.members.push((id, true));
                self.curve = None;
                receipt.copied_bytes = size_of::<(EdgeId, bool)>();
            }
            return Ok(receipt);
        }
        if body.curves3.next_insert_capacity_byte_demand(0)? != 0 {
            return body.curves3.reserve_insert_step(grant).map_err(|(error, receipt)| error.with_retained_progress(receipt));
        }
        let origin = body.vertices.get(a).ok_or_else(|| ValueError::invariant("polygon start vertex is missing"))?.position;
        let finish = body.vertices.get(b).ok_or_else(|| ValueError::invariant("polygon end vertex is missing"))?.position;
        let dir = finish - origin;
        let length = dir.x.hypot(dir.y).hypot(dir.z);
        if !length.is_finite() || length <= self.tolerance.value() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "polygonal wire has a collapsed edge"));
        }
        let (id, mut receipt) = body.curves3.insert_reserved(Curve3::Line { origin, dir }, grant).map_err(|(error, _)| error)?;
        self.curve = Some(id);
        receipt.copied_bytes = 2 * size_of::<Pnt3>();
        Ok(receipt)
    }
}

fn capacity(bytes: usize) -> RetirementDemand { RetirementDemand { capacity_bytes: bytes, depth: 1, ..Default::default() } }

fn reserve<T>(target: &mut Vec<T>, count: usize, grant: Grant) -> Result<Progress, ValueError> {
    target.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "polygonal wire storage allocation failed"))?;
    let receipt = Progress { copied_items: 1, retained_capacity_bytes: target.capacity() * size_of::<T>(), ..Default::default() };
    if receipt.fits(grant) { Ok(receipt) }
    else { Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "polygonal wire reservation exceeded its grant").with_retained_progress(receipt)) }
}

impl BrepRegistry {
    /// 🎟️ Admits unallocated polygonal wire topology and reserves only stable labels.
    pub fn plan_polygon_wire(&mut self, recipe: PolygonWireRecipe) -> Result<PolygonWirePublication, ValueError> {
        let (vertex_count, closed, tolerance) = recipe.admit()?;
        let edge_count = vertex_count - usize::from(!closed);
        let first_label = self.body.labels.next();
        let next = first_label.checked_add((vertex_count + edge_count + 1) as u64)
            .ok_or_else(|| ValueError::invariant("polygonal wire labels are exhausted"))?;
        self.body.labels = semio_framework_3d_brep_representation::topology::history::LabelSource::from_next(next);
        self.unpublished_topology += 1;
        let label = PersistentLabel(next - 1);
        Ok(PolygonWirePublication {
            owner: std::ptr::from_ref(&*self) as usize, recipe, wire: Wire { members: Vec::new(), vertices: Vec::new(), closed },
            curve: None, vertex_count, edge_count, first_label, label,
            handle: analytic_publication::primitive_handle(b"Wire:", label.0), open: true, boundary_transferred: false, tolerance,
        })
    }

    pub(super) fn transfer_polygon_wire_boundary(&mut self, wire: &mut PolygonWirePublication) {
        debug_assert!(wire.face_boundary(self).is_ok());
        wire.boundary_transferred = true;
        self.unpublished_topology -= 1;
    }

    /// 📐️ Quotes the next owner turn or permanent live index reservation.
    pub fn polygon_wire_demand(&self, wire: &PolygonWirePublication, copy: usize) -> Result<RetirementDemand, ValueError> {
        wire.check_owner(self)?;
        if !wire.topology_ready() { return wire.demand(&self.body); }
        let key = wire.handle();
        Ok(RetirementDemand { capacity_bytes: self.live.next_insert_capacity_byte_demand(key, copy)?, depth: self.live.next_insert_depth_demand(key)?, ..Default::default() })
    }

    /// 🧱️ Constructs one original wire record under the initiating grant.
    pub fn step_polygon_wire_topology(&mut self, wire: &mut PolygonWirePublication, point: Option<EVec3>, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        wire.check_owner(self)?;
        if wire.topology_ready() { return Err(ValueError::invariant("polygonal wire topology is already complete")); }
        let grant = effect.grant();
        if grant.maximum_items == 0 || !retained_clone_grant_admits(grant, wire.demand(&self.body)?) || !effect.performed(1)? { return Ok(Default::default()); }
        wire.advance(&mut self.body, point, grant)
    }

    /// 🗄️ Reserves the owner index for a completed wire.
    pub fn reserve_polygon_wire(&mut self, wire: &PolygonWirePublication, grant: Grant) -> Result<Progress, ValueError> {
        self.polygon_wire_demand(wire, grant.maximum_copy_bytes)?;
        self.live.reserve_insert_step(wire.handle(), grant).map_err(|(error, receipt)| error.with_retained_progress(receipt))
    }

    /// 📤️ Publishes the complete wire without cloning topology or geometry.
    pub fn publish_polygon_wire(&mut self, plan: &mut PolygonWirePublication, key: String, grant: Grant) -> Result<Progress, (ValueError, String)> {
        if !plan.topology_ready() || key != plan.handle() || self.live.contains_key(&key) { return Err((ValueError::invariant("invalid polygonal wire publication"), key)); }
        if let Err(error) = plan.check_owner(self) { return Err((error, key)); }
        let wire = Wire { members: take(&mut plan.wire.members), vertices: take(&mut plan.wire.vertices), closed: plan.wire.closed };
        let (previous, receipt) = match self.live.insert_reserved(key, Entity::Wire(wire, plan.label), grant) {
            Ok(result) => result,
            Err((error, key, Entity::Wire(wire, _))) => { plan.wire = wire; return Err((error, key)); }
            Err(_) => unreachable!("reserved live index returned the original wire"),
        };
        assert!(previous.is_none());
        plan.open = false;
        self.unpublished_topology -= 1;
        self.advance_live_revision();
        Ok(receipt)
    }

    /// 🧹️ Quotes removal of one unpublished arena record or release of an original ID buffer.
    pub fn polygon_wire_close_demand(&self, plan: &PolygonWirePublication) -> Result<RetirementDemand, ValueError> {
        plan.check_owner(self)?;
        let entity = if plan.boundary_transferred { None }
            else if let Some(id) = plan.curve { Some(EntityRef::Curve3(id)) }
            else if let Some((id, _)) = plan.wire.members.last() { Some(EntityRef::Edge(*id)) }
            else { plan.wire.vertices.last().copied().map(EntityRef::Vertex) };
        let (depth, release_bytes) = match entity {
            Some(entity) => (topology_slot_depth(&self.body, entity)?, 0),
            None if plan.wire.members.capacity() != 0 => (1, plan.wire.members.capacity() * size_of::<(EdgeId, bool)>()),
            None => (1, plan.wire.vertices.capacity() * size_of::<VertexId>()),
        };
        Ok(RetirementDemand { depth, release_bytes, ..Default::default() })
    }

    /// ♻️ Releases one unpublished wire record or backing allocation.
    pub fn close_polygon_wire(&mut self, plan: &mut PolygonWirePublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Step, ValueError> {
        let demand = self.polygon_wire_close_demand(plan)?;
        if effect.grant().maximum_items == 0 || !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? { return Ok(Step::Progress(Default::default())); }
        let receipt = Progress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() };
        if plan.boundary_transferred {
            if plan.wire.members.capacity() != 0 {
                drop(take(&mut plan.wire.members));
            } else if plan.wire.vertices.capacity() != 0 {
                drop(take(&mut plan.wire.vertices));
            } else {
                plan.open = false;
                return Ok(Step::Complete(receipt));
            }
            return Ok(Step::Progress(receipt));
        }
        if let Some(id) = plan.curve.take() {
            remove_topology_slot(&mut self.body, EntityRef::Curve3(id));
        } else if let Some((id, _)) = plan.wire.members.pop() {
            let edge = self.body.edges.remove(id).ok_or_else(|| ValueError::invariant("unpublished wire edge is missing"))?;
            plan.curve = Some(edge.curve);
        } else if let Some(id) = plan.wire.vertices.pop() {
            remove_topology_slot(&mut self.body, EntityRef::Vertex(id));
        } else if plan.wire.members.capacity() != 0 {
            drop(take(&mut plan.wire.members));
        } else if plan.wire.vertices.capacity() != 0 {
            drop(take(&mut plan.wire.vertices));
        } else {
            plan.open = false;
            self.unpublished_topology -= 1;
            return Ok(Step::Complete(receipt));
        }
        Ok(Step::Progress(receipt))
    }
}

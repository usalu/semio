//! 🔁️ Publishes the mapped boundary under the registry's handle and allocation authority.

use super::*;
use semio_framework_3d_brep_solid_topology::mapped_boundary::{FaceMapping, invalid_mapping};

pub(super) fn check_mapped_source(boundary: &MappedBoundary, kernel: &BrepRegistry, source: Option<&str>) -> Result<(), ValueError> {
    let key = source.ok_or_else(invalid_mapping)?;
    if matches!(kernel.original_query_entity(key)?, Entity::Face(id) if *id == boundary.source()) { Ok(()) }
    else { Err(ValueError::invariant("mapped polygon face source changed during construction")) }
}

impl BrepRegistry {
    /// ↔️ Admits a planar line ring offset by its oriented face normal.
    pub fn plan_polygon_face_offset(&mut self, source: &str, distance: f64) -> Result<Option<PolygonFacePublication>, ValueError> {
        if !distance.is_finite() { return Err(invalid_mapping()); }
        self.plan_mapped_polygon_face(source, || Ok(FaceMapping::Offset(distance)))
    }

    /// 🔁️ Admits the exact affine image of an original planar line ring.
    pub fn plan_polygon_face_transform(&mut self, source: &str, transform: AnalyticTransform) -> Result<Option<PolygonFacePublication>, ValueError> {
        self.plan_mapped_polygon_face(source, || transform.map().map(FaceMapping::Affine))
    }

    fn plan_mapped_polygon_face(&mut self, source: &str, mapping: impl FnOnce() -> Result<FaceMapping, ValueError>) -> Result<Option<PolygonFacePublication>, ValueError> {
        let Entity::Face(id) = self.original_query_entity(source)? else { return Ok(None); };
        let id = *id;
        let Some((boundary, inner)) = MappedBoundary::admit(&self.body, id, mapping)? else { return Ok(None); };
        let mut plan = self.start_polygon_face(0, false, None, None);
        plan.mapped = Some(boundary);
        plan.opening=inner.map(|ring|FaceOpening::Circular(CircularOpeningOffset::new(id,ring,[self.body.new_label(),self.body.new_label()])));
        plan.build = Build::MappedBoundary;
        Ok(Some(plan))
    }

    pub(super) fn step_mapped_polygon_boundary(
        &mut self,
        plan: &mut PolygonFacePublication,
        demand: RetirementDemand,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<Progress, ValueError> {
        let grant = effect.grant();
        if grant.maximum_items == 0 || !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? {
            return Ok(Default::default());
        }
        let boundary = plan.mapped.as_mut().ok_or_else(invalid_mapping)?;
        if let Some(plane) = boundary.plane() {
            if boundary.scan(&self.body)? {
                plan.edges = boundary.edge_count();
                let mut wire = self.plan_polygon_wire(PolygonWireRecipe::Polyline { vertices: plan.edges, closed: true })?;
                wire.set_tolerance(plane.tol);
                plan.boundary = Some(wire);
                plan.build = Build::Normal(0);
            }
        } else {
            boundary.prepare(&self.body)?;
        }
        Ok(Progress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })
    }
}

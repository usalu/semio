//! 🔁️ Streams the affine image of a planar line ring into a private polygon face draft.

use super::*;
use semio_framework_3d_brep_representation::topology::Edge;

const MAXIMUM_EDGES: usize = 65_536;

#[derive(Clone, Copy)]
pub(super) struct MappedBoundary {
    source: FaceId,
    ring: LoopId,
    first: CoedgeId,
    current: CoedgeId,
    count: usize,
    first_vertex: Option<VertexId>,
    tail: Option<VertexId>,
    mapping: FaceMapping,
    pub(super) plane: Option<MappedPlane>,
}

#[derive(Clone, Copy)]
enum FaceMapping {
    Offset(f64),
    Affine(Affine3),
}

#[derive(Clone, Copy)]
pub(super) struct MappedPlane {
    pub(super) frame: Frame3,
    pub(super) flipped: bool,
    pub(super) tol: Tol,
}

fn unsupported() -> ValueError {
    ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original face mapping requires one simple planar line ring")
}

fn invalid_mapping() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "face mapping requires finite, nondegenerate geometry and a connected boundary")
}

impl MappedBoundary {
    pub(super) fn check_source(&self, kernel: &BrepRegistry, source: Option<&str>) -> Result<(), ValueError> {
        let key = source.ok_or_else(invalid_mapping)?;
        if matches!(kernel.original_query_entity(key)?, Entity::Face(id) if *id == self.source) { Ok(()) }
        else { Err(ValueError::invariant("mapped polygon face source changed during construction")) }
    }

    fn member<'a>(&self, body: &'a Body) -> Result<(&'a Coedge, &'a Edge), ValueError> {
        let coedge = body.coedges.get(self.current).filter(|row| row.loop_id == self.ring).ok_or_else(invalid_mapping)?;
        let edge = body.edges.get(coedge.edge).ok_or_else(invalid_mapping)?;
        Ok((coedge, edge))
    }

    fn depth(&self, body: &Body, coedge: &Coedge, edge: &Edge) -> Result<usize, ValueError> {
        Ok(body.coedges.next_remove_depth_demand(self.current)?.max(body.edges.next_remove_depth_demand(coedge.edge)?)
            .max(body.curves3.next_remove_depth_demand(edge.curve)?))
    }

    pub(super) fn scan_demand(&self, body: &Body) -> Result<RetirementDemand, ValueError> {
        if self.plane.is_none() {
            let face = body.faces.get(self.source).ok_or_else(invalid_mapping)?;
            let depth = body.faces.next_remove_depth_demand(self.source)?.max(body.surfaces.next_remove_depth_demand(face.surface)?);
            return Ok(RetirementDemand {
                depth, copy_bytes: size_of::<Frame3>() + size_of::<Tol>() + size_of::<bool>(), ..Default::default()
            });
        }
        let (coedge, edge) = self.member(body)?;
        Ok(RetirementDemand {
            depth: self.depth(body, coedge, edge)?,
            copy_bytes: size_of::<CoedgeId>() + 2 * size_of::<VertexId>(),
            ..Default::default()
        })
    }

    fn prepare(&mut self, body: &Body) -> Result<(), ValueError> {
        let face = body.faces.get(self.source).ok_or_else(invalid_mapping)?;
        let Some(Surface::Plane { frame }) = body.surfaces.get(face.surface) else { return Err(unsupported()); };
        let (map, frame, flipped, tol) = match self.mapping {
            FaceMapping::Offset(distance) => {
                let delta = frame.z * if face.flipped { -distance } else { distance };
                let frame = Frame3 { origin: frame.origin + delta, ..*frame };
                (Affine3::translation(delta), frame, face.flipped, face.tol)
            }
            FaceMapping::Affine(map) => {
                let det = map.determinant();
                let scale = map.max_singular_value();
                if !det.is_finite() || det == 0.0 || !scale.is_finite() || scale <= 0.0 { return Err(invalid_mapping()); }
                let x = map.apply_vector(frame.x);
                let y = map.apply_vector(frame.y);
                let frame = Frame3 { origin: map.apply_point(frame.origin), x, y, z: x.cross(y) };
                (map, frame, face.flipped ^ (det < 0.0), face.tol.scaled(scale))
            }
        };
        if !frame.origin.to_array().into_iter().chain(frame.x.to_array()).chain(frame.y.to_array()).chain(frame.z.to_array()).all(f64::is_finite)
            || frame.z.normalized().is_none() || !tol.value().is_finite() { return Err(invalid_mapping()); }
        self.mapping = FaceMapping::Affine(map);
        self.plane = Some(MappedPlane { frame, flipped, tol });
        Ok(())
    }

    fn scan(&mut self, body: &Body) -> Result<bool, ValueError> {
        let (coedge, edge) = self.member(body)?;
        if !matches!(body.curves3.get(edge.curve), Some(Curve3::Line { .. })) { return Err(unsupported()); }
        let (a, b) = if coedge.forward { (edge.v0, edge.v1) } else { (edge.v1, edge.v0) };
        let first_vertex = *self.first_vertex.get_or_insert(a);
        if a == b || self.tail.is_some_and(|tail| tail != a) { return Err(invalid_mapping()); }
        self.count += 1;
        self.tail = Some(b);
        if coedge.next == self.first {
            if self.count < 3 || b != first_vertex { return Err(invalid_mapping()); }
            self.current = self.first;
            Ok(true)
        } else {
            if self.count >= MAXIMUM_EDGES { return Err(unsupported()); }
            self.current = coedge.next;
            Ok(false)
        }
    }

    pub(super) fn point_demand(&self, body: &Body) -> Result<RetirementDemand, ValueError> {
        let (coedge, edge) = self.member(body)?;
        let start = if coedge.forward { edge.v0 } else { edge.v1 };
        let depth = self.depth(body, coedge, edge)?.max(body.vertices.next_remove_depth_demand(start)?);
        Ok(RetirementDemand {
            depth, copy_bytes: size_of::<CoedgeId>() + size_of::<Pnt3>(), ..Default::default()
        })
    }

    pub(super) fn point(&self, body: &Body) -> Result<(EVec3, CoedgeId), ValueError> {
        let (coedge, edge) = self.member(body)?;
        let id = if coedge.forward { edge.v0 } else { edge.v1 };
        let vertex = body.vertices.get(id).ok_or_else(invalid_mapping)?;
        let FaceMapping::Affine(map) = self.mapping else { return Err(ValueError::invariant("mapped polygon plane has not been prepared")); };
        let point = evec(map.apply_point(vertex.position));
        if !point.into_iter().all(f64::is_finite) { return Err(invalid_mapping()); }
        Ok((point, coedge.next))
    }

    pub(super) fn advance(&mut self, next: CoedgeId) { self.current = next; }
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
        let face = self.body.faces.get(id).ok_or_else(invalid_mapping)?;
        if !matches!(self.body.surfaces.get(face.surface), Some(Surface::Plane { .. })) { return Ok(None); }
        let Some(ring_id) = face.outer else { return Err(unsupported()); };
        if !face.inners.is_empty() { return Err(unsupported()); }
        let first = self.body.loops.get(ring_id).filter(|ring| ring.face == id).ok_or_else(invalid_mapping)?.first;
        let coedge = self.body.coedges.get(first).filter(|row| row.loop_id == ring_id).ok_or_else(invalid_mapping)?;
        let edge = self.body.edges.get(coedge.edge).ok_or_else(invalid_mapping)?;
        if !matches!(self.body.curves3.get(edge.curve), Some(Curve3::Line { .. })) { return Ok(None); }
        let boundary = MappedBoundary {
            source: id, ring: ring_id, first, current: first, count: 0, first_vertex: None, tail: None,
            mapping: mapping()?, plane: None,
        };
        let mut plan = self.start_polygon_face(0, false, None, None);
        plan.mapped = Some(boundary);
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
        if let Some(plane) = boundary.plane {
            if boundary.scan(&self.body)? {
                plan.edges = boundary.count;
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

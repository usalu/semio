//! 🔁️ Streams the affine image of a planar line ring into a private polygon face draft.

use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::arena::{CoedgeId, FaceId, LoopId, VertexId};
use semio_framework_3d_brep_representation::contract::Vec3 as EVec3;
use semio_framework_3d_brep_representation::topology::{Body, Coedge, Edge};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::matrix::{Affine3, Frame3};
use semio_framework_3d_brep_vector::{Pnt3, Vec3 as NativeVec3};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::size_of;

const MAXIMUM_EDGES: usize = 65_536;

#[derive(Clone, Copy)]
pub struct MappedBoundary {
    source: FaceId,
    ring: LoopId,
    first: CoedgeId,
    current: CoedgeId,
    count: usize,
    first_vertex: Option<VertexId>,
    tail: Option<VertexId>,
    mapping: FaceMapping,
    plane: Option<MappedPlane>,
}

#[derive(Clone, Copy)]
pub enum FaceMapping {
    Offset(f64),
    Affine(Affine3),
}

#[derive(Clone, Copy)]
pub struct MappedPlane {
    pub frame: Frame3,
    pub flipped: bool,
    pub tol: Tol,
}

fn unsupported() -> ValueError {
    ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original face mapping requires one simple planar line ring")
}

pub fn invalid_mapping() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "face mapping requires finite, nondegenerate geometry and a connected boundary")
}

impl MappedBoundary {
    /// 🎟️ Admits one planar line boundary and its optional offset opening.
    pub fn admit(
        body: &Body, id: FaceId, mapping: impl FnOnce() -> Result<FaceMapping, ValueError>,
    ) -> Result<Option<(Self, Option<LoopId>)>, ValueError> {
        let face = body.faces.get(id).ok_or_else(invalid_mapping)?;
        if !matches!(body.surfaces.get(face.surface), Some(Surface::Plane { .. })) { return Ok(None); }
        let Some(ring_id) = face.outer else { return Err(unsupported()); };
        let mapping=mapping()?;
        let inner=match face.inners.as_slice(){
            []=>None,
            [inner] if matches!(mapping,FaceMapping::Offset(_))=>Some(*inner),
            _=>return Err(unsupported()),
        };
        let first = body.loops.get(ring_id).filter(|ring| ring.face == id).ok_or_else(invalid_mapping)?.first;
        let coedge = body.coedges.get(first).filter(|row| row.loop_id == ring_id).ok_or_else(invalid_mapping)?;
        let edge = body.edges.get(coedge.edge).ok_or_else(invalid_mapping)?;
        if !matches!(body.curves3.get(edge.curve), Some(Curve3::Line { .. })) { return Ok(None); }
        let boundary = Self {
            source: id, ring: ring_id, first, current: first, count: 0, first_vertex: None, tail: None,
            mapping, plane: None,
        };
        Ok(Some((boundary, inner)))
    }

    pub fn source(&self) -> FaceId { self.source }

    pub fn plane(&self) -> Option<MappedPlane> { self.plane }

    pub fn edge_count(&self) -> usize { self.count }

    fn member<'a>(&self, body: &'a Body) -> Result<(&'a Coedge, &'a Edge), ValueError> {
        let coedge = body.coedges.get(self.current).filter(|row| row.loop_id == self.ring).ok_or_else(invalid_mapping)?;
        let edge = body.edges.get(coedge.edge).ok_or_else(invalid_mapping)?;
        Ok((coedge, edge))
    }

    fn depth(&self, body: &Body, coedge: &Coedge, edge: &Edge) -> Result<usize, ValueError> {
        Ok(body.coedges.next_remove_depth_demand(self.current)?.max(body.edges.next_remove_depth_demand(coedge.edge)?)
            .max(body.curves3.next_remove_depth_demand(edge.curve)?))
    }

    pub fn scan_demand(&self, body: &Body) -> Result<RetirementDemand, ValueError> {
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

    pub fn prepare(&mut self, body: &Body) -> Result<(), ValueError> {
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

    pub fn scan(&mut self, body: &Body) -> Result<bool, ValueError> {
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

    pub fn point_demand(&self, body: &Body) -> Result<RetirementDemand, ValueError> {
        let (coedge, edge) = self.member(body)?;
        let start = if coedge.forward { edge.v0 } else { edge.v1 };
        let depth = self.depth(body, coedge, edge)?.max(body.vertices.next_remove_depth_demand(start)?);
        Ok(RetirementDemand {
            depth, copy_bytes: size_of::<CoedgeId>() + size_of::<Pnt3>(), ..Default::default()
        })
    }

    pub fn point(&self, body: &Body) -> Result<(EVec3, CoedgeId), ValueError> {
        let (coedge, edge) = self.member(body)?;
        let id = if coedge.forward { edge.v0 } else { edge.v1 };
        let vertex = body.vertices.get(id).ok_or_else(invalid_mapping)?;
        let FaceMapping::Affine(map) = self.mapping else { return Err(ValueError::invariant("mapped polygon plane has not been prepared")); };
        let point = map.apply_point(vertex.position).to_array();
        if !point.into_iter().all(f64::is_finite) { return Err(invalid_mapping()); }
        Ok((point, coedge.next))
    }

    pub fn translation(&self)->Result<NativeVec3,ValueError>{
        let FaceMapping::Affine(map)=self.mapping else{return Err(invalid_mapping());};
        Ok(map.apply_point(Pnt3::new(0.,0.,0.)).to_vec())
    }

    pub fn advance(&mut self, next: CoedgeId) { self.current = next; }
}

//! 📦️ Incremental exact volume and area integration over planar polygon topology.

use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::arena::{CoedgeId, FaceId, LoopId, ShellId, SolidId};
use semio_framework_3d_brep_representation::topology::{Body, Face};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::Pnt3;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::size_of;

#[derive(Clone, Copy)]
struct Ring {
    id: LoopId,
    first: CoedgeId,
    current: CoedgeId,
    anchor: Option<Pnt3>,
    work: usize,
    area: f64,
}

/// 🔬️ Reads one original coedge per turn and retains only scalar integration state.
pub struct PlanarMeasure {
    source: SolidId,
    measure_area: bool,
    shell: usize,
    face: usize,
    boundary: usize,
    ring: Option<Ring>,
    origin: Option<Pnt3>,
    sum: f64,
    correction: f64,
}

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "original planar measurement encountered an invalid polygonal solid")
}

impl PlanarMeasure {
    /// 🧭️ Starts integration over the named original solid.
    pub fn new(source: SolidId, measure_area: bool) -> Self {
        Self { source, measure_area, shell: 0, face: 0, boundary: 0, ring: None, origin: None, sum: 0.0, correction: 0.0 }
    }

    /// 🪪️ The solid whose topology supplies the integration.
    pub fn source(&self) -> SolidId {
        self.source
    }

    fn face_and_shell(&self, body: &Body) -> Result<(Option<FaceId>, Option<ShellId>), ValueError> {
        let solid = body.solids.get(self.source).ok_or_else(invalid)?;
        let shell = if self.shell == 0 { Some(solid.outer) } else { solid.inners.get(self.shell - 1).copied() };
        let Some(shell) = shell else { return Ok((None, None)); };
        let face = body.shells.get(shell).ok_or_else(invalid)?.faces.get(self.face).copied();
        Ok((face, Some(shell)))
    }

    /// 📏️ Quotes the next original topology read.
    pub fn next_demand(&self, body: &Body) -> Result<RetirementDemand, ValueError> {
        let mut depth = body.solids.next_remove_depth_demand(self.source)?;
        let (face_id, shell_id) = self.face_and_shell(body)?;
        if let Some(shell_id) = shell_id { depth = depth.max(body.shells.next_remove_depth_demand(shell_id)?); }
        let mut copy_bytes = 0;
        if let Some(face_id) = face_id {
            depth = depth.max(body.faces.next_remove_depth_demand(face_id)?);
            let face = body.faces.get(face_id).ok_or_else(invalid)?;
            if let Some(ring) = self.ring {
                depth = depth.max(body.coedges.next_remove_depth_demand(ring.current)?);
                let coedge = body.coedges.get(ring.current).ok_or_else(invalid)?;
                depth = depth.max(body.edges.next_remove_depth_demand(coedge.edge)?);
                let edge = body.edges.get(coedge.edge).ok_or_else(invalid)?;
                depth = depth.max(body.curves3.next_remove_depth_demand(edge.curve)?);
                depth = depth.max(body.vertices.next_remove_depth_demand(edge.v0)?).max(body.vertices.next_remove_depth_demand(edge.v1)?);
                copy_bytes = 2 * size_of::<Pnt3>();
                if self.measure_area { depth = depth.max(body.surfaces.next_remove_depth_demand(face.surface)?); }
            } else {
                depth = depth.max(body.surfaces.next_remove_depth_demand(face.surface)?);
                if let Some(id) = boundary(face, self.boundary) {
                    depth = depth.max(body.loops.next_remove_depth_demand(id)?);
                }
            }
        }
        Ok(RetirementDemand { copy_bytes, depth, ..Default::default() })
    }

    /// ⏱️ Integrates one polygon boundary step against the original topology.
    pub fn step(&mut self, body: &Body) -> Result<Option<f64>, ValueError> {
        let (face_id, shell) = self.face_and_shell(body)?;
        let Some(face_id) = face_id else {
            if shell.is_some() {
                self.shell += 1;
                self.face = 0;
                self.boundary = 0;
                return Ok(None);
            }
            let volume = if self.measure_area { self.sum * 0.5 } else { self.sum.abs() / 6.0 };
            if !volume.is_finite() || self.origin.is_none() { return Err(invalid()); }
            return Ok(Some(volume));
        };
        let face = body.faces.get(face_id).ok_or_else(invalid)?;
        if let Some(ring) = &mut self.ring {
            let coedge = body.coedges.get(ring.current).filter(|item| item.loop_id == ring.id).ok_or_else(invalid)?;
            let edge = body.edges.get(coedge.edge).ok_or_else(invalid)?;
            if !matches!(body.curves3.get(edge.curve), Some(Curve3::Line { .. })) { return Err(unsupported()); }
            let (a, b) = body.coedge_endpoints(ring.current).ok_or_else(invalid)?;
            let a = body.vertices.get(a).ok_or_else(invalid)?.position;
            let b = body.vertices.get(b).ok_or_else(invalid)?.position;
            let origin = *self.origin.get_or_insert(a);
            let anchor = *ring.anchor.get_or_insert(a);
            if self.measure_area {
                let Some(Surface::Plane{frame})=body.surfaces.get(face.surface) else { return Err(unsupported()); };
                let normal = frame.z.normalized().ok_or_else(invalid)?;
                ring.area += (a-anchor).cross(b-anchor).dot(normal);
            } else {
                let signed = (anchor - origin).dot((a - origin).cross(b - origin));
                let term = if face.flipped { -signed } else { signed };
                accumulate(&mut self.sum,&mut self.correction,term);
            }
            ring.work += 1;
            if coedge.next == ring.first {
                if ring.work < 3 { return Err(invalid()); }
                if self.measure_area {
                    let contribution = ring.area.abs() * if self.boundary==0 {1.0} else {-1.0};
                    accumulate(&mut self.sum,&mut self.correction,contribution);
                }
                self.ring = None;
                self.boundary += 1;
            } else {
                if ring.work >= body.coedges.slot_count() { return Err(invalid()); }
                ring.current = coedge.next;
            }
            return Ok(None);
        }
        let Some(id) = boundary(face, self.boundary) else {
            if self.boundary == 0 { return Err(invalid()); }
            self.face += 1;
            self.boundary = 0;
            return Ok(None);
        };
        if !matches!(body.surfaces.get(face.surface), Some(Surface::Plane { .. })) { return Err(unsupported()); }
        let first = body.loops.get(id).filter(|item| item.face == face_id).ok_or_else(invalid)?.first;
        self.ring = Some(Ring { id, first, current: first, anchor: None, work: 0, area: 0.0 });
        Ok(None)
    }
}

fn accumulate(sum: &mut f64, correction: &mut f64, value: f64) {
    let adjusted = value - *correction;
    let next = *sum + adjusted;
    *correction = (next - *sum) - adjusted;
    *sum = next;
}

fn boundary(face: &Face, index: usize) -> Option<LoopId> {
    if index == 0 { face.outer } else { face.inners.get(index - 1).copied() }
}

fn unsupported() -> ValueError {
    ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original measurement requires a certified primitive or planar polygonal solid")
}

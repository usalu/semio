//! 📦️ Incremental volume and area over original planar solid or face topology.

use super::planar_closest::area_correction;
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::arena::{CoedgeId, FaceId, LoopId, ShellId, SolidId, VertexId};
use semio_framework_3d_brep_representation::topology::{Body, Face};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::Pnt3;
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress as Progress, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use std::mem::size_of;

#[path = "🔵️spherical-area/🦀️.rs"]
mod spherical_area;
pub(crate) use spherical_area::SphericalBand;
#[path = "⚖️centroid/🦀️.rs"]
mod centroid;
use centroid::Centroid;

#[derive(Clone, Copy)]
struct Ring {
    id: LoopId,
    first: CoedgeId,
    current: CoedgeId,
    anchor: Option<Pnt3>,
    work: usize,
    area: f64,
    correction: f64,
    has_arc: bool,
    ends: Option<(VertexId, VertexId)>,
    sphere: Option<SphericalBand>,
}

/// 🏷️ Original topology supplying a planar measurement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlanarMeasureSource {
    Solid(SolidId),
    Face(FaceId),
    Compound([Option<SolidId>;4]),
}

impl PlanarMeasureSource {
    /// 🧮️ Captures a bounded collection of distinct original solid roots.
    pub fn compound(roots:&[SolidId])->Option<Self> {
        if !(1..=4).contains(&roots.len()) || roots.iter().enumerate().any(|(i,id)|roots[..i].contains(id)) {return None;}
        Some(Self::Compound(std::array::from_fn(|i|roots.get(i).copied())))
    }
}

/// 🔬️ Reads one original coedge per turn and retains only scalar integration state.
pub struct PlanarMeasure {
    source: PlanarMeasureSource,
    measure_area: bool,
    shell: usize,
    face: usize,
    boundary: usize,
    ring: Option<Ring>,
    origin: Option<Pnt3>,
    sum: f64,
    correction: f64,
    member: usize,
    group_sum:f64,
    group_correction:f64,
    centroid:Option<Centroid>,
    axis_bounds:Option<[Pnt3;2]>,
}

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "original planar measurement encountered invalid or nonfinite topology")
}

impl PlanarMeasure {
    /// 🧭️ Starts integration over the named original solid or area face.
    pub fn new(source: PlanarMeasureSource, measure_area: bool) -> Self {
        Self { source, measure_area, shell: 0, face: 0, boundary: 0, ring: None, origin: None, sum: 0.0, correction: 0.0, member:0, group_sum:0., group_correction:0., centroid:None, axis_bounds:None }
    }

    /// ⚖️ Includes first moments in the original signed volume traversal.
    pub fn with_centroid(mut self)->Self {
        self.measure_area=false;
        self.centroid=Some(Centroid::default());
        self
    }

    /// 🔳️ Accumulates exact world-axis intervals while visiting the original straight edges.
    pub fn with_axis_bounds(mut self)->Self {
        self.axis_bounds=Some([Pnt3::from_array([f64::INFINITY;3]),Pnt3::from_array([f64::NEG_INFINITY;3])]);
        self
    }

    /// 🧭️ Borrows the exact axis extrema accumulated to this point.
    pub fn axis_bounds(&self)->Option<[Pnt3;2]>{self.axis_bounds.filter(|bounds|bounds.iter().flat_map(|point|point.to_array()).all(f64::is_finite))}

    /// 🎯️ Returns the centroid after the measurement completes.
    pub fn centroid(&self)->Option<Pnt3>{self.centroid.as_ref()?.result}

    /// 🪪️ The original entity whose topology supplies the integration.
    pub fn source(&self) -> PlanarMeasureSource {
        self.source
    }

    fn active_source(&self)->Option<PlanarMeasureSource>{
        match self.source {
            PlanarMeasureSource::Compound(roots)=>roots.get(self.member).copied().flatten().map(PlanarMeasureSource::Solid),
            source=>Some(source),
        }
    }

    fn face_and_shell(&self, body: &Body) -> Result<(Option<FaceId>, Option<ShellId>), ValueError> {
        let source = match self.active_source().ok_or_else(invalid)? {
            PlanarMeasureSource::Face(id) if self.measure_area => return Ok(((self.face == 0).then_some(id), None)),
            PlanarMeasureSource::Face(_) => return Err(unsupported()),
            PlanarMeasureSource::Solid(id) => id,
            PlanarMeasureSource::Compound(_) => unreachable!("compound member must supply a solid"),
        };
        let solid = body.solids.get(source).ok_or_else(invalid)?;
        let shell = if self.shell == 0 { Some(solid.outer) } else { solid.inners.get(self.shell - 1).copied() };
        let Some(shell) = shell else { return Ok((None, None)); };
        let face = body.shells.get(shell).ok_or_else(invalid)?.faces.get(self.face).copied();
        Ok((face, Some(shell)))
    }

    /// 📏️ Quotes the next original topology read.
    pub fn next_demand(&self, body: &Body) -> Result<RetirementDemand, ValueError> {
        let mut depth = match self.active_source().ok_or_else(invalid)? {
            PlanarMeasureSource::Solid(id) => body.solids.next_remove_depth_demand(id)?,
            PlanarMeasureSource::Face(id) => body.faces.next_remove_depth_demand(id)?,
            PlanarMeasureSource::Compound(_) => unreachable!("compound member must supply a solid"),
        };
        let (face_id, shell_id) = self.face_and_shell(body)?;
        if let Some(shell_id) = shell_id { depth = depth.max(body.shells.next_remove_depth_demand(shell_id)?); }
        let mut copy_bytes = if shell_id.is_none() && face_id.is_none() && self.axis_bounds.is_some() { size_of::<[Pnt3;2]>() } else { 0 };
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
                if self.measure_area {
                    depth = depth.max(body.surfaces.next_remove_depth_demand(face.surface)?);
                    if let Some(trim) = coedge.pcurve.filter(|_| ring.sphere.is_some() || !matches!(body.curves3.get(edge.curve), Some(Curve3::Line { .. }))) {
                        depth = depth.max(body.curves2.next_remove_depth_demand(trim)?);
                    }
                }
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
            let mut measure = if self.measure_area { self.sum * 0.5 } else { self.sum.abs() / 6.0 };
            let origin=self.origin.ok_or_else(invalid)?;
            if !measure.is_finite() || measure<0. {return Err(invalid());}
            if let Some(centroid)=&mut self.centroid {centroid.finish_member(origin,self.sum,measure)?;}
            if let PlanarMeasureSource::Compound(roots)=self.source {
                accumulate(&mut self.group_sum,&mut self.group_correction,measure);
                self.member+=1;
                if roots.get(self.member).copied().flatten().is_some() {
                    let (member,sum,correction,centroid,bounds)=(self.member,self.group_sum,self.group_correction,self.centroid.take(),self.axis_bounds);
                    *self=Self::new(self.source,self.measure_area);
                    (self.member,self.group_sum,self.group_correction,self.centroid,self.axis_bounds)=(member,sum,correction,centroid,bounds);
                    return Ok(None);
                }
                measure=self.group_sum;
                if !measure.is_finite(){return Err(invalid());}
            }
            if let Some(centroid)=&mut self.centroid {centroid.finish(measure)?;}
            return Ok(Some(measure));
        };
        let face = body.faces.get(face_id).ok_or_else(invalid)?;
        if let Some(ring) = &mut self.ring {
            let coedge = body.coedges.get(ring.current).filter(|item| item.loop_id == ring.id).ok_or_else(invalid)?;
            let edge = body.edges.get(coedge.edge).ok_or_else(invalid)?;
            let curve = body.curves3.get(edge.curve).ok_or_else(invalid)?;
            let linear = matches!(curve, Curve3::Line { .. });
            if !self.measure_area && !linear { return Err(unsupported()); }
            let (start, end) = body.coedge_endpoints(ring.current).ok_or_else(invalid)?;
            let first = match ring.ends {
                Some((first, previous)) if previous == start => first,
                Some(_) => return Err(invalid()),
                None => start,
            };
            ring.ends = Some((first, end));
            let a = body.vertices.get(start).ok_or_else(invalid)?.position;
            let b = body.vertices.get(end).ok_or_else(invalid)?.position;
            if let Some([min,max])=&mut self.axis_bounds {
                let coordinates=a.to_array();
                if !coordinates.into_iter().all(f64::is_finite) { return Err(invalid()); }
                let (low,high)=(min.to_array(),max.to_array());
                *min=Pnt3::from_array(std::array::from_fn(|axis|low[axis].min(coordinates[axis])));
                *max=Pnt3::from_array(std::array::from_fn(|axis|high[axis].max(coordinates[axis])));
            }
            let origin = *self.origin.get_or_insert(a);
            let anchor = *ring.anchor.get_or_insert(a);
            if let Some(sphere) = &mut ring.sphere {
                let surface = body.surfaces.get(face.surface).ok_or_else(invalid)?;
                if let Some(area) = sphere.row(ring.work, body, surface, coedge, edge, curve)? { ring.area = area; }
            } else if self.measure_area {
                let Some(Surface::Plane{frame}) = body.surfaces.get(face.surface) else { return Err(unsupported()); };
                let normal = frame.x.cross(frame.y).normalized().ok_or_else(invalid)?;
                let chord = (a-anchor).cross(b-anchor).dot(normal);
                accumulate(&mut ring.area, &mut ring.correction, chord);
                if !linear {
                    if coedge.prange != edge.range { return Err(unsupported()); }
                    let trim = body.curves2.get(coedge.pcurve.ok_or_else(unsupported)?).ok_or_else(invalid)?;
                    let (vertices, sign) = if coedge.forward { ([a, b], 1.0) } else { ([b, a], -1.0) };
                    let delta = area_correction(curve, trim, edge.range, vertices, frame)?;
                    accumulate(&mut ring.area, &mut ring.correction, sign * delta);
                    ring.has_arc = true;
                }
            } else {
                let (anchor,a,b)=(anchor-origin,a-origin,b-origin);
                let signed=anchor.dot(a.cross(b));
                let term=if face.flipped{-signed}else{signed};
                accumulate(&mut self.sum,&mut self.correction,term);
                if let Some(centroid)=&mut self.centroid {centroid.tetrahedron(anchor,a,b,term);}
            }
            ring.work += 1;
            if coedge.next == ring.first {
                let closed = if ring.sphere.is_some() { ring.work == 4 } else { ring.work >= 3 || (ring.has_arc && ring.work == 2) };
                if end != first || !closed { return Err(invalid()); }
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
        let sphere = match body.surfaces.get(face.surface) {
            Some(Surface::Plane { .. }) => None,
            Some(Surface::Sphere { .. }) if self.measure_area && face.inners.is_empty() => Some(SphericalBand::default()),
            _ => return Err(unsupported()),
        };
        let first = body.loops.get(id).filter(|item| item.face == face_id).ok_or_else(invalid)?.first;
        self.ring = Some(Ring { id, first, current: first, anchor: None, work: 0, area: 0.0, correction: 0.0, has_arc: false, ends: None, sphere });
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
    ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original measurement requires a supported planar or spherical area boundary or planar polygonal volume")
}


/// 🎟️ Advances one funded scalar measurement and preserves its committed read receipt.
pub fn advance_measure<T>(demand: RetirementDemand, effect: &mut RetainedAllocationEffect<'_>,
    step: impl FnOnce() -> Result<Option<T>, ValueError>,
) -> Result<(Option<T>, Progress), ValueError> {
    if effect.grant().maximum_items == 0 || !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? {
        return Ok((None, Progress::default()));
    }
    let receipt = Progress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
    let value = step().map_err(|error| error.with_retained_progress(receipt))?;
    Ok((value, receipt))
}

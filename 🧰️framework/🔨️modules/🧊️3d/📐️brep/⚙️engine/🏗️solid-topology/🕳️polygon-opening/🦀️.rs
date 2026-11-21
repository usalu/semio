//! 🕳️ Publishes ordered planar void rings on an unpublished face.

use super::store_demand;
use semio_framework_3d_brep_curve::{Curve2, Curve3};
use semio_framework_3d_brep_representation::{
    arena::LoopId,
    contract::SectionPolygon,
    topology::history::LabelSource,
};
use semio_framework_3d_brep_vector::{Pnt2, Pnt3, Vec2, matrix::Frame3};
use semio_framework_value::RetainedAllocationEffect;
use super::{Body, Coedge, Edge, EntityRef, FaceId, Loop, Progress, RetirementDemand, Tol, ValueError, ValueRefusalKind, Vertex, dummy_coedge, retained_clone_grant_admits, size_of, take};

#[derive(Clone, Copy)]
enum Build {
    Reserve,
    Vertex(usize),
    Ring,
    Curve(usize),
    Edge(usize),
    Trim(usize),
    Use(usize),
    Link(usize),
    Close,
    Finish,
    Attach,
    Ready,
}

/// 🧵️ Stages native inner loops using the owning face's funded rollback journal.
pub struct PolygonOpening<const N: usize> {
    rings: [Option<SectionPolygon>; N],
    ring: usize,
    base: usize,
    build: Build,
}

impl<const N: usize> PolygonOpening<N> {
    pub fn new(rings: [Option<SectionPolygon>; N]) -> Self {
        Self { rings, ring: 0, base: 0, build: Build::Reserve }
    }

    pub fn ready(&self) -> bool { matches!(self.build, Build::Ready) }

    pub fn journal_entries(&self) -> usize {
        self.rings.iter().flatten().map(|ring| 1 + 5 * ring.count()).sum()
    }

    pub fn demand(&self, body: &Body, target: FaceId, journal: &[EntityRef]) -> Result<RetirementDemand, ValueError> {
        let face_depth = || body.faces.next_remove_depth_demand(target);
        if matches!(self.build, Build::Reserve) {
            let capacity_bytes = self.rings.iter().flatten().count() * size_of::<LoopId>();
            return Ok(RetirementDemand { depth: face_depth()?, capacity_bytes, ..Default::default() });
        }
        let count = self.current()?.count();
        let entries = &journal[self.base..];
        let stored = |depth| Ok(RetirementDemand { depth, ..Default::default() });
        let mut demand = match self.build {
            Build::Vertex(_) => store_demand(&body.vertices)?,
            Build::Ring => store_demand(&body.loops)?,
            Build::Curve(_) => store_demand(&body.curves3)?,
            Build::Edge(_) => store_demand(&body.edges)?,
            Build::Trim(_) => store_demand(&body.curves2)?,
            Build::Use(_) => store_demand(&body.coedges)?,
            Build::Link(0) => return stored(1),
            Build::Link(index) => return stored(body.coedges.next_remove_depth_demand(coedge(entries, count, index - 1))?),
            Build::Close => return stored(body.coedges.next_remove_depth_demand(coedge(entries, count, 0))?),
            Build::Finish => return stored(body.loops.next_remove_depth_demand(loop_id(entries, count))?),
            Build::Attach => return Ok(RetirementDemand { depth: face_depth()?, copy_bytes: size_of::<LoopId>(), ..Default::default() }),
            Build::Reserve | Build::Ready => return Err(invalid()),
        };
        if demand.capacity_bytes == 0 {
            demand.copy_bytes = size_of::<EntityRef>() + match self.build {
                Build::Vertex(_) => 3 * size_of::<f64>(),
                Build::Curve(_) | Build::Edge(_) | Build::Trim(_) | Build::Use(_) => 6 * size_of::<f64>(),
                _ => 0,
            };
        }
        Ok(demand)
    }

    pub fn step(&mut self, body: &mut Body, target: FaceId, plane: Frame3, tol: Tol, journal: &mut Vec<EntityRef>, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        let grant = effect.grant();
        let demand = self.demand(body, target, journal)?;
        if grant.maximum_items == 0 || !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? {
            return Ok(Progress::default());
        }
        let mut progress = Progress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        if matches!(self.build, Build::Reserve) {
            let inners = &mut body.faces.get_mut(target).ok_or_else(invalid)?.inners;
            let count = self.rings.iter().flatten().count();
            if inners.capacity() != 0 || count == 0 || effect.actual().is_some() { return Err(invalid()); }
            let result = inners.try_reserve_exact(count);
            progress.retained_capacity_bytes = inners.capacity() * size_of::<LoopId>();
            *effect.actual() = Some(progress.retained_capacity_bytes);
            result.map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "polygon inner ring allocation failed")
                .with_retained_progress(progress))?;
            if !progress.fits(grant) {
                return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "polygon inner ring capacity exceeded grant").with_retained_progress(progress));
            }
            self.next_ring(0, journal.len());
            return Ok(progress);
        }
        let ring = self.current()?;
        let count = ring.count();
        let entries = &journal[self.base..];
        macro_rules! insert {
            ($store:ident, $kind:ident, $next:expr, $value:expr) => {{
                let (id, mut receipt) = body.$store.insert_reserved($value, grant).map_err(|(error, _)| error)?;
                assert!(journal.len() < journal.capacity(), "opening entries were reserved by the face");
                journal.push(EntityRef::$kind(id));
                self.build = $next;
                receipt.copied_bytes = demand.copy_bytes;
                return Ok(receipt);
            }};
        }
        if demand.capacity_bytes != 0 {
            if effect.actual().is_some() { return Err(invalid()); }
            let receipt = match self.build {
                Build::Vertex(_) => body.vertices.reserve_insert_step(grant),
                Build::Ring => body.loops.reserve_insert_step(grant),
                Build::Curve(_) => body.curves3.reserve_insert_step(grant),
                Build::Edge(_) => body.edges.reserve_insert_step(grant),
                Build::Trim(_) => body.curves2.reserve_insert_step(grant),
                Build::Use(_) => body.coedges.reserve_insert_step(grant),
                _ => return Err(invalid()),
            };
            let allocated = match &receipt {
                Ok(progress) | Err((_, progress)) => progress.retained_capacity_bytes,
            };
            *effect.actual() = Some(allocated);
            return receipt.map_err(|(error, receipt)| error.with_retained_progress(receipt));
        }
        match self.build {
            Build::Vertex(index) => {
                let position = Pnt3::from_array(ring.point(index).ok_or_else(invalid)?);
                let label = body.new_label();
                let next = if index + 1 == count { Build::Ring } else { Build::Vertex(index + 1) };
                insert!(vertices, Vertex, next, Vertex { position, tol, label });
            }
            Build::Ring => insert!(loops, Loop, Build::Curve(0), Loop { face: target, first: dummy_coedge() }),
            Build::Curve(index) => {
                let (curve, _) = segment(ring, index)?;
                insert!(curves3, Curve3, Build::Edge(index), curve);
            }
            Build::Edge(index) => {
                let (_, range) = segment(ring, index)?;
                let EntityRef::Vertex(v0) = entries[index] else { return Err(invalid()); };
                let EntityRef::Vertex(v1) = entries[(index + 1) % count] else { return Err(invalid()); };
                let EntityRef::Curve3(curve) = entries[segment_base(count, index)] else { return Err(invalid()); };
                let label = body.new_label();
                insert!(edges, Edge, Build::Trim(index), Edge { curve, range, v0, v1, tol, label });
            }
            Build::Trim(index) => {
                let (curve, _) = segment(ring, index)?;
                let trim = line_pcurve(&curve, plane).ok_or_else(invalid)?;
                insert!(curves2, Curve2, Build::Use(index), trim);
            }
            Build::Use(index) => {
                let base = segment_base(count, index);
                let EntityRef::Edge(edge) = entries[base + 1] else { return Err(invalid()); };
                let EntityRef::Curve2(pcurve) = entries[base + 2] else { return Err(invalid()); };
                let loop_id = loop_id(entries, count);
                let prev = if index == 0 { dummy_coedge() } else { coedge(entries, count, index - 1) };
                let next = if index + 1 == count { coedge(entries, count, 0) } else { dummy_coedge() };
                let (_, prange) = segment(ring, index)?;
                insert!(coedges, Coedge, Build::Link(index), Coedge { edge, forward: true, pcurve: Some(pcurve), prange, loop_id, next, prev });
            }
            Build::Link(index) => {
                if index != 0 {
                    let id = coedge(entries, count, index - 1);
                    body.coedges.get_mut(id).ok_or_else(invalid)?.next = coedge(entries, count, index);
                }
                self.build = if index + 1 < count { Build::Curve(index + 1) } else { Build::Close };
            }
            Build::Close => {
                let first = coedge(entries, count, 0);
                body.coedges.get_mut(first).ok_or_else(invalid)?.prev = coedge(entries, count, count - 1);
                self.build = Build::Finish;
            }
            Build::Finish => {
                body.loops.get_mut(loop_id(entries, count)).ok_or_else(invalid)?.first = coedge(entries, count, 0);
                self.build = Build::Attach;
            }
            Build::Attach => {
                body.faces.get_mut(target).ok_or_else(invalid)?.inners.push(loop_id(entries, count));
                self.next_ring(self.ring + 1, journal.len());
            }
            Build::Reserve | Build::Ready => return Err(invalid()),
        }
        Ok(progress)
    }

    pub fn close_demand(body: &Body, target: FaceId) -> Result<Option<RetirementDemand>, ValueError> {
        let capacity = body.faces.get(target).ok_or_else(invalid)?.inners.capacity();
        if capacity == 0 { return Ok(None); }
        Ok(Some(RetirementDemand { depth: body.faces.next_remove_depth_demand(target)?, release_bytes: capacity * size_of::<LoopId>(), ..Default::default() }))
    }

    pub fn close(body: &mut Body, target: FaceId, demand: RetirementDemand) -> Result<Progress, ValueError> {
        drop(take(&mut body.faces.get_mut(target).ok_or_else(invalid)?.inners));
        Ok(Progress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() })
    }

    fn current(&self) -> Result<&SectionPolygon, ValueError> {
        self.rings.get(self.ring).and_then(Option::as_ref).ok_or_else(invalid)
    }

    fn next_ring(&mut self, start: usize, base: usize) {
        self.base = base;
        if let Some(offset) = self.rings[start..].iter().position(Option::is_some) {
            self.ring = start + offset;
            self.build = Build::Vertex(0);
        } else {
            self.build = Build::Ready;
        }
    }
}

fn segment_base(count: usize, index: usize) -> usize { count + 1 + 4 * index }

fn loop_id(entries: &[EntityRef], count: usize) -> LoopId {
    let EntityRef::Loop(id) = entries[count] else { unreachable!("opening loop was staged first"); };
    id
}

fn coedge(entries: &[EntityRef], count: usize, index: usize) -> semio_framework_3d_brep_representation::arena::CoedgeId {
    let EntityRef::Coedge(id) = entries[segment_base(count, index) + 3] else { unreachable!("opening coedge was staged first"); };
    id
}

fn segment(ring: &SectionPolygon, index: usize) -> Result<(Curve3, (f64, f64)), ValueError> {
    let origin = Pnt3::from_array(ring.point(index).ok_or_else(invalid)?);
    let next = Pnt3::from_array(ring.point((index + 1) % ring.count()).ok_or_else(invalid)?);
    let direction = next - origin;
    let dir = direction.normalized().ok_or_else(invalid)?;
    Ok((Curve3::Line { origin, dir }, (0., direction.norm())))
}

/// 📏️ Projects an exact 3D line into a planar frame with the same curve parameter.
pub fn line_pcurve(curve: &Curve3, frame: Frame3) -> Option<Curve2> {
    let Curve3::Line { origin, dir } = *curve else { return None; };
    let normal = frame.z.normalized()?;
    let u = frame.y.cross(normal);
    let v = normal.cross(frame.x);
    let u = u * frame.x.dot(u).recip();
    let v = v * frame.y.dot(v).recip();
    let shift = origin - frame.origin;
    let uv = [shift.dot(u), shift.dot(v), dir.dot(u), dir.dot(v)];
    uv.into_iter().all(f64::is_finite).then(|| Curve2::Line {
        origin: Pnt2::new(uv[0], uv[1]),
        dir: Vec2::new(uv[2], uv[3]),
    })
}

/// 🧭️ Maps a world point to its coordinates on the given plane.
pub fn point2(frame: Frame3, point: Pnt3) -> Pnt2 {
    let local = frame.to_local(point);
    Pnt2::new(local.x, local.y)
}

/// 📐️ Tests whether two closed planar segments share any point.
pub fn segments_intersect(a: Pnt2, b: Pnt2, c: Pnt2, d: Pnt2) -> bool {
    use semio_framework_3d_brep_vector::predicates::{Orient, orient2d};
    let overlap = |a: f64, b: f64, c: f64, d: f64| a.min(b).max(c.min(d)) <= a.max(b).min(c.max(d));
    if !overlap(a.x, b.x, c.x, d.x) || !overlap(a.y, b.y, c.y, d.y) { return false; }
    let separates = |first, second| first == second && first != Orient::Zero;
    !separates(orient2d(a, b, c), orient2d(a, b, d)) && !separates(orient2d(c, d, a), orient2d(c, d, b))
}

fn invalid() -> ValueError { ValueError::invariant("polygon opening lacks its certified unpublished ring") }

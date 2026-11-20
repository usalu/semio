//! 🧵️ Certifies exact coincident cuboid boundary faces one native record at a time.

use semio_framework_3d_brep_construction::primitives::SolidPrimitive;
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::{arena::{CoedgeId, FaceId, LoopId}, topology::Body};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::Pnt3;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::size_of;

#[derive(Clone, Copy)]
struct Side {
    lo: [f64; 3],
    hi: [f64; 3],
    axis: usize,
    outward_high: bool,
}

#[derive(Clone, Copy)]
struct Ring {
    current: CoedgeId,
    first: CoedgeId,
    id: LoopId,
    axis: usize,
    positive: bool,
    coordinate: f64,
    flipped: bool,
    corners: [[f64; 3]; 4],
    tail: [f64; 3],
    count: usize,
}

/// 🪡️ Fixed-size original draft for one closed boundary of exactly six rectangular faces.
pub struct CuboidSew {
    revision: Option<u64>,
    faces: [Option<FaceId>; 6],
    sides: [Option<Side>; 6],
    ring: Option<Ring>,
    next: usize,
    tolerance: f64,
}

impl CuboidSew {
    /// 🎟️ Opens cubic-incidence certification before any topology is drafted.
    pub fn new(tolerance: f64) -> Result<Self, ValueError> {
        if !tolerance.is_finite() || tolerance <= 0. {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "sewing requires a positive finite tolerance"));
        }
        Ok(Self { revision: None, faces: [None; 6], sides: [None; 6], ring: None, next: 0, tolerance })
    }

    pub fn next_face(&self) -> Option<usize> { (self.next < 6).then_some(self.next) }

    fn check(&self, id: FaceId, revision: u64) -> Result<(), ValueError> {
        if self.revision.is_some_and(|old| old != revision) {
            return Err(ValueError::invariant("sew source topology changed during certification"));
        }
        match self.faces.get(self.next).ok_or_else(invalid)? {
            Some(old) if *old != id => Err(ValueError::invariant("sew face changed during certification")),
            None if self.faces.contains(&Some(id)) => Err(invalid()),
            _ => Ok(()),
        }
    }

    /// 📏️ Quotes one source face header or directed native edge.
    pub fn demand(&self, body: &Body, id: FaceId, revision: u64, depth: usize) -> Result<RetirementDemand, ValueError> {
        self.check(id, revision)?;
        let (depth, scalars) = if let Some(ring) = self.ring {
            let use_ = body.coedges.get(ring.current).ok_or_else(invalid)?;
            let edge = body.edges.get(use_.edge).ok_or_else(invalid)?;
            let depth = depth.max(body.coedges.next_remove_depth_demand(ring.current)?)
                .max(body.edges.next_remove_depth_demand(use_.edge)?)
                .max(body.curves3.next_remove_depth_demand(edge.curve)?)
                .max(body.vertices.next_remove_depth_demand(edge.v0)?)
                .max(body.vertices.next_remove_depth_demand(edge.v1)?);
            (depth, 14)
        } else {
            let face = body.faces.get(id).ok_or_else(invalid)?;
            let ring = face.outer.ok_or_else(invalid)?;
            (depth.max(body.faces.next_remove_depth_demand(id)?)
                .max(body.surfaces.next_remove_depth_demand(face.surface)?)
                .max(body.loops.next_remove_depth_demand(ring)?), 4)
        };
        Ok(RetirementDemand { depth, copy_bytes: scalars * size_of::<f64>(), ..Default::default() })
    }

    /// 🧭️ Validates one oriented source edge or starts a new source face.
    pub fn step(&mut self, body: &Body, id: FaceId, revision: u64) -> Result<(), ValueError> {
        self.check(id, revision)?;
        if let Some(ring) = &mut self.ring {
            let use_ = body.coedges.get(ring.current).ok_or_else(invalid)?;
            let edge = body.edges.get(use_.edge).ok_or_else(invalid)?;
            let curve = body.curves3.get(edge.curve).ok_or_else(invalid)?;
            if use_.loop_id != ring.id || !matches!(curve, Curve3::Line { .. }) { return Err(invalid()); }
            let a = body.vertices.get(edge.v0).ok_or_else(invalid)?.position;
            let b = body.vertices.get(edge.v1).ok_or_else(invalid)?.position;
            if !(curve.eval(edge.range.0).distance(a) <= self.tolerance
                && curve.eval(edge.range.1).distance(b) <= self.tolerance) { return Err(invalid()); }
            let (a, b) = if use_.forward { (a.to_array(), b.to_array()) } else { (b.to_array(), a.to_array()) };
            if !a.into_iter().chain(b).all(f64::is_finite)
                || a[ring.axis] != ring.coordinate || b[ring.axis] != ring.coordinate
                || (0..3).filter(|&axis| a[axis] != b[axis]).count() != 1
                || (ring.count != 0 && ring.tail != a)
                || (use_.next == ring.first) != (ring.count == 3)
            { return Err(invalid()); }
            ring.corners[ring.count] = a;
            ring.tail = b;
            ring.count += 1;
            ring.current = use_.next;
            if ring.count == 4 {
                if ring.tail != ring.corners[0] { return Err(invalid()); }
                self.sides[self.next] = Some(rectangle(*ring)?);
                self.ring = None;
                self.next += 1;
            }
            return Ok(());
        }
        let face = body.faces.get(id).ok_or_else(invalid)?;
        let Surface::Plane { frame } = body.surfaces.get(face.surface).ok_or_else(invalid)? else { return Err(invalid()); };
        if !face.inners.is_empty() { return Err(invalid()); }
        let normal = frame.z.to_array();
        let axis = normal.iter().position(|&value| value == 1. || value == -1.).ok_or_else(invalid)?;
        if normal.iter().enumerate().any(|(i, &value)| i != axis && value != 0.) { return Err(invalid()); }
        let loop_id = face.outer.ok_or_else(invalid)?;
        let loop_ = body.loops.get(loop_id).ok_or_else(invalid)?;
        if loop_.face != id { return Err(invalid()); }
        self.revision = Some(revision);
        self.faces[self.next] = Some(id);
        self.ring = Some(Ring {
            current: loop_.first, first: loop_.first, id: loop_id, axis, positive: normal[axis] > 0.,
            coordinate: frame.origin.to_array()[axis], flipped: face.flipped, corners: [[0.; 3]; 4], tail: [0.; 3], count: 0,
        });
        Ok(())
    }

    /// 📦️ Certifies exactly one outward full side at every cuboid extremum.
    pub fn cuboid(&self, revision: u64) -> Result<SolidPrimitive, ValueError> {
        if self.revision != Some(revision) { return Err(ValueError::invariant("sew source topology changed before publication")); }
        if self.next != 6 { return Err(invalid()); }
        let sides = self.sides.map(|side| side.expect("all six sides were certified"));
        let lo: [f64; 3] = std::array::from_fn(|i| sides.iter().map(|side| side.lo[i]).fold(f64::INFINITY, f64::min));
        let hi: [f64; 3] = std::array::from_fn(|i| sides.iter().map(|side| side.hi[i]).fold(f64::NEG_INFINITY, f64::max));
        let size: [f64; 3] = std::array::from_fn(|i| hi[i] - lo[i]);
        if !(0..3).all(|i| size[i].is_finite() && size[i] > 0. && lo[i] + size[i] == hi[i]) { return Err(invalid()); }
        let mut mask = 0u8;
        for side in sides {
            let mut expected_lo = lo;
            let mut expected_hi = hi;
            if side.outward_high { expected_lo[side.axis] = hi[side.axis]; }
            else { expected_hi[side.axis] = lo[side.axis]; }
            let bit = 1u8 << (2 * side.axis + usize::from(side.outward_high));
            if side.lo != expected_lo || side.hi != expected_hi || mask & bit != 0 { return Err(invalid()); }
            mask |= bit;
        }
        if mask != 0b11_1111 { return Err(invalid()); }
        Ok(SolidPrimitive::Box { origin: Pnt3::from_array(lo), width: size[0], depth: size[1], height: size[2] })
    }
}

fn rectangle(ring: Ring) -> Result<Side, ValueError> {
    let lo = std::array::from_fn(|i| ring.corners.iter().map(|p| p[i]).fold(f64::INFINITY, f64::min));
    let hi = std::array::from_fn(|i| ring.corners.iter().map(|p| p[i]).fold(f64::NEG_INFINITY, f64::max));
    let (u, v) = ((ring.axis + 1) % 3, (ring.axis + 2) % 3);
    let mut seen = 0u8;
    for p in ring.corners {
        if lo[u] == hi[u] || lo[v] == hi[v] || (p[u] != lo[u] && p[u] != hi[u]) || (p[v] != lo[v] && p[v] != hi[v]) { return Err(invalid()); }
        let bit = 1u8 << (usize::from(p[u] == hi[u]) + 2 * usize::from(p[v] == hi[v]));
        if seen & bit != 0 { return Err(invalid()); }
        seen |= bit;
    }
    let [a, b, c, _] = ring.corners;
    let positive = if a[u] != b[u] { (b[u] > a[u]) == (c[v] > b[v]) } else { (b[v] > a[v]) != (c[u] > b[u]) };
    if ring.positive != positive { return Err(invalid()); }
    Ok(Side { lo, hi, axis: ring.axis, outward_high: positive != ring.flipped })
}

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::UnsupportedOwner, "retained sew requires six exact outward cuboid faces")
}

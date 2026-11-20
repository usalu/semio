//! 🧱 Analytic solid primitives: box/sphere/cylinder/cone/torus + wires/planar faces/convex hull.
//!
//! Builds closed [`Body`](semio_framework_3d_brep_representation::topology::Body) solids exclusively through
//! [`crate::euler`] editors, attaching shared [`Curve3`](semio_framework_3d_brep_curve::Curve3) /
//! [`Surface`](semio_framework_3d_brep_surface::Surface) geometry from the body's pools, with a stored [`Curve2`] p-curve on
//! every coedge. Topology: box (V=8 E=12 F=6), sphere/cylinder/cone/torus as exact analytic
//! surfaces with seam/degenerate edges (no faceting, no `segments`), convex hull as
//! coplanar-merged polygon faces (Quickhull + boundary-walk merge).
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/🧱️primitives` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL3. Rewritten to
//! exact analytic primitives (no `segments`, no triangle-soup topology) in ticket
//! 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME wave W1-E.
//!

use std::collections::HashMap;
use std::f64::consts::{FRAC_PI_2, TAU};

use {crate::euler::{add_face, add_shell, add_solid, make_edge, make_vertex}, semio_framework_3d_brep_representation::topology::make_loop};
use crate::staged::{drive_solid, Plan, StageOutput, StageProgress, StageStep, StagedOperation};
use semio_framework_3d_brep_representation::arena::{ArenaId, Curve2Id, EdgeId, FaceId, SolidId, VertexId};
use semio_framework_3d_brep_curve::{Curve2, Curve3};
use semio_framework_3d_brep_representation::error::KernelError;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_representation::topology::history::OpRecorder;
use semio_framework_3d_brep_representation::topology::{Body, Wire};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_3d_brep_representation::topology::Edge;
use semio_framework_3d_brep_vector::matrix::Frame3;
use semio_framework_3d_brep_vector::{Pnt2, Pnt3, Vec2, Vec3};

/// 🔺 Coplanar triangles grouped by quantized plane coordinates.
pub type PlaneClusters = HashMap<(i64, i64, i64, i64), (Vec3, Vec<[usize; 3]>)>;

// #region 🔖️Helpers

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn placeholder_face() -> FaceId {
    ArenaId::from_raw(0, 0)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn require_positive(name: &str, value: f64) -> Result<(), KernelError> {
    if value <= Tol::DEFAULT.value() {
        Err(KernelError::InvalidInput(format!("{name} must be positive, got {value}")))
    } else {
        Ok(())
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn attach_face(body: &mut Body, surface_id: semio_framework_3d_brep_representation::arena::SurfaceId, members: &[(EdgeId, bool)], flipped: bool, tol: Tol, rec: &mut OpRecorder) -> FaceId {
    let outer = make_loop(body, placeholder_face(), members);
    let face = add_face(body, surface_id, Some(outer), vec![], flipped, tol, rec);
    body.loops.get_mut(outer).unwrap().face = face;
    face
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn line_edge(body: &mut Body, a: Pnt3, b: Pnt3, va: VertexId, vb: VertexId, tol: Tol, rec: &mut OpRecorder) -> EdgeId {
    let curve = body.curves3.insert(Curve3::Line { origin: a, dir: b - a });
    make_edge(body, curve, (0.0, 1.0), va, vb, tol, rec)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn plane_at(origin: Pnt3, normal: Vec3) -> Surface {
    Surface::Plane { frame: Frame3::from_normal(origin, normal).expect("plane frame") }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn finish_solid(body: &mut Body, faces: Vec<FaceId>, rec: &mut OpRecorder) -> SolidId {
    let shell = add_shell(body, faces, rec);
    add_solid(body, shell, vec![], rec)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn newell_normal(points: &[Pnt3]) -> Option<Vec3> {
    if points.len() < 3 {
        return None;
    }
    let mut n = Vec3::ZERO;
    for i in 0..points.len() {
        let p = points[i];
        let q = points[(i + 1) % points.len()];
        n.x += (p.y - q.y) * (p.z + q.z);
        n.y += (p.z - q.z) * (p.x + q.x);
        n.z += (p.x - q.x) * (p.y + q.y);
    }
    n.normalized()
}

/// 🧱 Inserts a `Curve2::Line` p-curve (`origin + dir·t`) — the common shape for every straight

/// 🧱 Inserts a `Curve2::Circle` p-curve — used where a 3D circle edge lands on a *planar* cap

/// 🧭️ Projects a planar edge curve into face coordinates while preserving its parameters.
pub fn planar_pcurve(curve: &Curve3, plane: Frame3) -> Curve2 {
    let point = |p| {
        let local = plane.to_local(p);
        Pnt2::new(local.x, local.y)
    };
    let vector = |v| {
        let local = plane.to_local_vector(v);
        Vec2::new(local.x, local.y)
    };
    let ellipse = |frame: Frame3, major_radius, minor_radius: f64| Curve2::Ellipse {
        center: point(frame.origin),
        x_axis: vector(frame.x),
        major_radius,
        minor_radius: minor_radius.copysign(plane.z.dot(frame.z)),
    };
    match curve {
        Curve3::Line { origin, dir } => Curve2::Line { origin: point(*origin), dir: vector(*dir) },
        Curve3::Circle { frame, radius } => ellipse(*frame, *radius, *radius),
        Curve3::Ellipse { frame, major_radius, minor_radius } => ellipse(*frame, *major_radius, *minor_radius),
        Curve3::Nurbs { knots, controls, weights } => Curve2::Nurbs {
            knots: knots.clone(),
            controls: controls.iter().copied().map(point).collect(),
            weights: weights.clone(),
        },
    }
}

/// 📎️ Gives each planar coedge its exact trim in the edge's original parameter range.
fn attach_planar_face_pcurves(body: &mut Body, face: FaceId, frame: Frame3, members: &[(EdgeId, bool)]) {
    let pcurves: Vec<(Curve2Id, (f64, f64))> = members
        .iter()
        .map(|&(id, _)| {
            let edge = body.edges.get(id).expect("member edge exists");
            let curve = body.curves3.get(edge.curve).expect("edge curve exists");
            (body.curves2.insert(planar_pcurve(curve, frame)), edge.range)
        })
        .collect();
    set_outer_pcurves(body, face, &pcurves);
}

/// 🧱 Stamps `pcurves` (one `(curve2, prange)` pair per coedge, in `attach_face`'s own `members`
/// order) onto `face`'s outer loop. `prange` always shares its interpolating parameter `s` with
/// the coedge's underlying [`Edge::range`] — i.e. it is *never* reversed to account for
/// `Coedge::forward` (see [`semio_framework_3d_brep_validation::check_same_parameter`], which samples both ranges with
/// the same `s`) — only reparametrized (phase/sign/offset) when the p-curve targets a different
/// surface frame than the edge's own curve.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn set_outer_pcurves(body: &mut Body, face: FaceId, pcurves: &[(Curve2Id, (f64, f64))]) {
    let outer = body.faces.get(face).and_then(|f| f.outer).expect("face has an outer loop");
    let coedges = body.loop_coedges(outer);
    debug_assert_eq!(coedges.len(), pcurves.len(), "one p-curve per coedge, in member order");
    for (&coedge_id, &(pcurve, prange)) in coedges.iter().zip(pcurves) {
        let coedge = body.coedges.get_mut(coedge_id).expect("just-created coedge");
        coedge.pcurve = Some(pcurve);
        coedge.prange = prange;
    }
}

/// 🧱 A "degenerate" edge (OCCT convention): both endpoints are the same vertex *and* the 3D
/// curve is a zero-length line — the standard closing device for a pole where the whole `u`
/// range of a periodic surface collapses to one point. Used by the Euler-characteristic tests
/// below to exclude poles from the edge count (see `w1e-primitives.md` §sphere for why the naive
/// `V−E+F` would otherwise read 0, not 2, for the sphere).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
#[cfg(any(test, feature = "testkit"))]
pub fn is_degenerate_edge(edge: &Edge, curve: &Curve3) -> bool {
    edge.v0 == edge.v1 && matches!(curve, Curve3::Line { dir, .. } if dir.norm() < 1e-12)
}

// #endregion 🔖️Helpers

#[path = "🧊️analytic-solids/🦀️.rs"]
mod analytic_solids;
pub use analytic_solids::{BoxSectionDraft, BoxSectionPolygon, PrimitiveEdge, PrimitiveTrim, SolidPrimitive};
use analytic_solids::make_analytic_solid;

// #region 🔖️Solids

/// 📐 The oriented analytic boundary shared by the synchronous and retained Box constructors.
pub struct BoxGeometry;

impl BoxGeometry {
    pub const EDGES: [[usize; 2]; 12] = [
        [0, 1], [1, 2], [2, 3], [3, 0], [4, 5], [5, 6], [6, 7], [7, 4], [0, 4], [1, 5], [2, 6], [3, 7],
    ];
    pub const FACES: [[(usize, bool); 4]; 6] = [
        [(0, false), (3, false), (2, false), (1, false)],
        [(4, true), (5, true), (6, true), (7, true)],
        [(0, true), (9, true), (4, false), (8, false)],
        [(2, true), (11, true), (6, false), (10, false)],
        [(3, true), (8, true), (7, false), (11, false)],
        [(1, true), (10, true), (5, false), (9, false)],
    ];

    pub fn corners(w: f64, d: f64, h: f64) -> [Pnt3; 8] {
        [
            Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(w, 0.0, 0.0), Pnt3::new(w, d, 0.0), Pnt3::new(0.0, d, 0.0),
            Pnt3::new(0.0, 0.0, h), Pnt3::new(w, 0.0, h), Pnt3::new(w, d, h), Pnt3::new(0.0, d, h),
        ]
    }

    pub fn frame(corners: &[Pnt3; 8], face: usize) -> Frame3 {
        let (origin, normal) = [(0, -Vec3::Z), (4, Vec3::Z), (0, -Vec3::Y), (3, Vec3::Y), (0, -Vec3::X), (1, Vec3::X)][face];
        Frame3::from_normal(corners[origin], normal).expect("box face normal is an axis")
    }
}

/// 🧱 Axis-aligned box from the origin to `(w, d, h)` with six planar faces (V=8, E=12, F=6).
/// Threads the caller-owned `rec` through every euler call so the whole box's [`semio_framework_3d_brep_representation::topology::history::OpDelta`]
/// is observable after this call returns, instead of being discarded at the function boundary.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_box(body: &mut Body, w: f64, d: f64, h: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    make_analytic_solid(body, SolidPrimitive::axis_box(w, d, h), rec)
}

/// 🧱 Sphere centered at the origin: ONE analytic [`Surface::Sphere`] face, bounded by a single
/// seam edge (the u=0 half-meridian great circle from south to north pole, curve domain
/// `[-π/2, π/2]`) used twice — once at u=0, once at u=2π — plus two degenerate edges (3D curve
/// collapsed to the pole point, equal endpoint ids and a zero-length line) that close the (u,v) parameter
/// rectangle along `v=±π/2`. This is the standard OCCT-style sphere topology (chosen over the old
/// two-hemisphere-plus-equator split so there is exactly one seam, matching cylinder/cone/torus'
/// own single-seam convention) — see `w1e-primitives.md` for the coordinate-with-W1-F note.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_sphere(body: &mut Body, radius: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    make_analytic_solid(body, SolidPrimitive::sphere(radius), rec)
}

/// 🧱 Cylinder along +Z from `z=0` to `z=height`: one analytic [`Surface::Cylinder`] lateral face
/// (single seam at u=0) plus two planar caps, all four faces' coedges carrying exact p-curves.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_cylinder(body: &mut Body, radius: f64, height: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    make_analytic_solid(body, SolidPrimitive::cylinder(radius, height), rec)
}

/// 🧱 Pointed cone with base radius at `z=0` and apex at `(0,0,height)`: one analytic
/// [`Surface::Cone`] lateral face plus a planar base cap. No separate degenerate apex edge — the
/// single seam edge (base→apex) is used TWICE in the lateral loop (up, then down), both
/// traversals terminating at the shared apex vertex. This is the standard representation for a
/// full (untrimmed) cone: since the whole `u` range already collapses to the apex point at v=0
/// (radius `v·tan(half_angle)` → 0), an extra explicit degenerate edge there would just duplicate
/// what the seam's own two endpoints already express — unlike the sphere, whose TWO poles are not
/// both endpoints of the same seam edge and so each needs its own degenerate closer.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_cone(body: &mut Body, radius: f64, height: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    make_analytic_solid(body, SolidPrimitive::cone(radius, height), rec)
}

/// 🧱 Torus in the XY plane: ONE analytic [`Surface::Torus`] face, the classic "fundamental
/// polygon" identification of a torus as a square with opposite sides glued — two full-circle
/// seam edges (the u=0 meridian and the v=0 equatorial circle), each used TWICE (once per glued
/// side pair), all four coedges meeting at the single vertex `(u,v)=(0,0)`. No degenerate edges:
/// genus 1 ⇒ χ = V−E+F = 1−2+1 = 0 (the existing test's own expectation, unlike the χ=2 solids).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_torus(body: &mut Body, major: f64, minor: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    make_analytic_solid(body, SolidPrimitive::torus(major, minor), rec)
}

/// 🧱 Builds a (possibly non-convex) solid from a triangle soup — used ONLY for mesh import and
/// as [`make_convex_hull`]'s last-resort fallback (see [`merge_coplanar_triangles`], which is
/// preferred whenever the hull's planar clusters walk to a clean boundary).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn solid_from_triangle_soup(body: &mut Body, triangles: &[[Pnt3; 3]], rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    if triangles.is_empty() {
        return Err(KernelError::InvalidInput("triangle soup is empty".into()));
    }
    let tol = Tol::DEFAULT;
    let quant = |v: f64| -> i64 { (v * 1e6).round() as i64 };
    let mut key_to_idx: HashMap<(i64, i64, i64), usize> = HashMap::new();
    let mut positions: Vec<Pnt3> = Vec::new();
    let mut verts: Vec<VertexId> = Vec::new();
    for tri in triangles {
        for &p in tri {
            let key = (quant(p.x), quant(p.y), quant(p.z));
            if key_to_idx.contains_key(&key) {
                continue;
            }
            key_to_idx.insert(key, positions.len());
            positions.push(p);
            verts.push(make_vertex(body, p, tol, rec));
        }
    }
    let mut edge_map: HashMap<(usize, usize), EdgeId> = HashMap::new();
    let mut faces = Vec::with_capacity(triangles.len());
    for tri in triangles {
        let idxs = [
            *key_to_idx.get(&(quant(tri[0].x), quant(tri[0].y), quant(tri[0].z))).unwrap(),
            *key_to_idx.get(&(quant(tri[1].x), quant(tri[1].y), quant(tri[1].z))).unwrap(),
            *key_to_idx.get(&(quant(tri[2].x), quant(tri[2].y), quant(tri[2].z))).unwrap(),
        ];
        let mut members = Vec::with_capacity(3);
        for (ia, ib) in [(idxs[0], idxs[1]), (idxs[1], idxs[2]), (idxs[2], idxs[0])] {
            let key = (ia.min(ib), ia.max(ib));
            let (eid, forward) = if let Some(&existing) = edge_map.get(&key) {
                let edge = body.edges.get(existing).unwrap();
                (existing, edge.v0 == verts[ia])
            } else {
                let eid = line_edge(body, positions[ia], positions[ib], verts[ia], verts[ib], tol, rec);
                edge_map.insert(key, eid);
                (eid, true)
            };
            members.push((eid, forward));
        }
        let normal = (tri[1] - tri[0]).cross(tri[2] - tri[0]).normalized().unwrap_or(Vec3::Z);
        let surface = body.surfaces.insert(plane_at(tri[0], normal));
        faces.push(attach_face(body, surface, &members, false, tol, rec));
    }
    Ok(finish_solid(body, faces, rec))
}

/// 🧱 Convex hull of a point cloud as a closed solid whose faces are the coplanar-MERGED polygons
/// of the underlying Quickhull triangulation (e.g. 8 box corners → 6 quad faces, not 12
/// triangles) — see [`merge_coplanar_triangles`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_convex_hull(body: &mut Body, points: &[Pnt3], rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    drive_solid(&mut ConvexHullJob::new(points)?, body, rec)
}

// #endregion 🔖️Solids

// #region 🔖️WiresFaces

/// 🧱 Open or closed polyline wire through `points` (closed requires ≥ 3 points).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_polyline_wire(body: &mut Body, points: &[Pnt3], closed: bool, rec: &mut OpRecorder) -> Result<Wire, KernelError> {
    if points.len() < 2 {
        return Err(KernelError::InvalidInput("polyline needs at least 2 points".into()));
    }
    if closed && points.len() < 3 {
        return Err(KernelError::InvalidInput("closed polyline needs at least 3 points".into()));
    }
    let tol = Tol::DEFAULT;
    let vertices: Vec<VertexId> = points.iter().map(|&p| make_vertex(body, p, tol, rec)).collect();
    let mut members = Vec::new();
    let n_edges = if closed { points.len() } else { points.len() - 1 };
    for i in 0..n_edges {
        let j = (i + 1) % points.len();
        let eid = line_edge(body, points[i], points[j], vertices[i], vertices[j], tol, rec);
        members.push((eid, true));
    }
    Ok(Wire { members, vertices, closed })
}

/// 🧱 Axis-aligned rectangle wire in the XY plane from the origin to `(width, height)`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_rectangle_wire(body: &mut Body, width: f64, height: f64, rec: &mut OpRecorder) -> Result<Wire, KernelError> {
    require_positive("rectangle width", width)?;
    require_positive("rectangle height", height)?;
    make_polyline_wire(body, &[Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(width, 0.0, 0.0), Pnt3::new(width, height, 0.0), Pnt3::new(0.0, height, 0.0)], true, rec)
}

/// 🧱 Regular `sides`-gon wire of given `radius` in the XY plane, centered at the origin.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_regular_polygon_wire(body: &mut Body, radius: f64, sides: usize, rec: &mut OpRecorder) -> Result<Wire, KernelError> {
    require_positive("polygon radius", radius)?;
    if sides < 3 {
        return Err(KernelError::InvalidInput(format!("polygon needs at least 3 sides, got {sides}")));
    }
    let points: Vec<Pnt3> = (0..sides)
        .map(|i| {
            let a = TAU * i as f64 / sides as f64;
            Pnt3::new(radius * a.cos(), radius * a.sin(), 0.0)
        })
        .collect();
    make_polyline_wire(body, &points, true, rec)
}

/// 🧱 Planar face from a closed point loop (Newell normal); points must be non-collinear.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_planar_face_from_points(body: &mut Body, points: &[Pnt3], rec: &mut OpRecorder) -> Result<FaceId, KernelError> {
    if points.len() < 3 {
        return Err(KernelError::InvalidInput("planar face needs at least 3 points".into()));
    }
    let normal = newell_normal(points).ok_or_else(|| KernelError::InvalidInput("points are collinear".into()))?;
    let wire = make_polyline_wire(body, points, true, rec)?;
    make_planar_face_from_wire(body, &wire, points[0], normal, rec)
}

/// 🧵️ Builds a one-edge wire over a curve's natural domain, with a single seam vertex when closed.
pub fn make_curve_wire(body: &mut Body, curve: Curve3, rec: &mut OpRecorder) -> Result<Wire, KernelError> {
    let range = match &curve {
        Curve3::Line { .. } => (0.0, 1.0),
        Curve3::Circle { .. } | Curve3::Ellipse { .. } => (0.0, TAU),
        Curve3::Nurbs { knots, .. } => knots.domain(),
    };
    let (first, last) = (curve.eval(range.0), curve.eval(range.1));
    if ![first.x, first.y, first.z, last.x, last.y, last.z].iter().all(|coordinate| coordinate.is_finite()) {
        return Err(KernelError::InvalidInput("curve wire requires finite endpoints".into()));
    }
    let closed = matches!(curve, Curve3::Circle { .. } | Curve3::Ellipse { .. }) || first.distance(last) <= 1e-9;
    let curve = body.curves3.insert(curve);
    let start = make_vertex(body, first, Tol::DEFAULT, rec);
    let end = if closed { start } else { make_vertex(body, last, Tol::DEFAULT, rec) };
    let edge = make_edge(body, curve, range, start, end, Tol::DEFAULT, rec);
    Ok(Wire { members: vec![(edge, true)], vertices: if closed { vec![start] } else { vec![start, end] }, closed })
}

/// 📐️ Infers a closed wire's winding plane from its oriented curve samples.
pub fn planar_wire_support(body: &Body, wire: &Wire) -> Result<(Pnt3, Vec3), KernelError> {
    if !wire.closed {
        return Err(KernelError::InvalidInput("profile boundary must be closed".into()));
    }
    let mut points = Vec::with_capacity(wire.members.len());
    for &(id, forward) in &wire.members {
        let edge = body.edges.get(id).ok_or_else(|| KernelError::MissingEntity(format!("edge {id:?}")))?;
        let curve = body.curves3.get(edge.curve).ok_or_else(|| KernelError::MissingEntity(format!("curve {:?}", edge.curve)))?;
        let (from, to) = if forward { edge.range } else { (edge.range.1, edge.range.0) };
        let samples = if matches!(curve, Curve3::Line { .. }) { 1 } else { 32 };
        points.extend((0..samples).map(|step| curve.eval(from + (to - from) * step as f64 / samples as f64)));
    }
    let normal = newell_normal(&points)
        .filter(|normal| normal.norm().is_finite() && normal.norm() > 0.0)
        .ok_or_else(|| KernelError::InvalidInput("profile boundary encloses no planar area".into()))?;
    let origin = points[0];
    let extent = points.iter().map(|&point| point.distance(origin)).fold(1.0_f64, f64::max);
    if points.iter().any(|&point| (point - origin).dot(normal).abs() > Tol::DEFAULT.value() * extent) {
        return Err(KernelError::InvalidInput("profile boundary must be planar".into()));
    }
    Ok((origin, normal))
}

/// 🧱 Planar face whose outer loop is an existing closed [`Wire`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_planar_face_from_wire(body: &mut Body, wire: &Wire, origin: Pnt3, normal: Vec3, rec: &mut OpRecorder) -> Result<FaceId, KernelError> {
    if !wire.closed {
        return Err(KernelError::InvalidInput("planar face requires a closed wire".into()));
    }
    if wire.members.is_empty() {
        return Err(KernelError::InvalidInput("planar face wire is empty".into()));
    }
    let frame = Frame3::from_normal(origin, normal).expect("plane frame");
    let surface = body.surfaces.insert(Surface::Plane { frame });
    let face = attach_face(body, surface, &wire.members, false, Tol::DEFAULT, rec);
    attach_planar_face_pcurves(body, face, frame, &wire.members);
    Ok(face)
}

// #endregion 🔖️WiresFaces

// #region 🔖️ConvexHull

#[derive(Clone)]
struct HullFace {
    verts: [usize; 3],
    normal: Vec3,
    d: f64,
    alive: bool,
}

struct ConvexHull {
    vertices: Vec<Pnt3>,
    faces: Vec<[usize; 3]>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn face_normal(pts: &[Pnt3], a: usize, b: usize, c: usize) -> Vec3 {
    (pts[b] - pts[a]).cross(pts[c] - pts[a]).normalized().unwrap_or(Vec3::Z)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn signed_distance(face: &HullFace, p: Pnt3) -> f64 {
    face.normal.dot(p.to_vec()) + face.d
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn find_initial_tetrahedron(pts: &[Pnt3]) -> Option<[usize; 4]> {
    let mut i0 = 0usize;
    for (i, p) in pts.iter().enumerate() {
        if p.x < pts[i0].x {
            i0 = i;
        }
    }
    let mut i1 = None;
    let mut best = 0.0;
    for (i, p) in pts.iter().enumerate() {
        if i == i0 {
            continue;
        }
        let dist = p.distance(pts[i0]);
        if dist > best {
            best = dist;
            i1 = Some(i);
        }
    }
    let i1 = i1?;
    let mut i2 = None;
    best = 0.0;
    let edge = pts[i1] - pts[i0];
    for (i, p) in pts.iter().enumerate() {
        if i == i0 || i == i1 {
            continue;
        }
        let area = edge.cross(*p - pts[i0]).norm();
        if area > best {
            best = area;
            i2 = Some(i);
        }
    }
    let i2 = i2?;
    if best <= 1e-12 {
        return None;
    }
    let n = face_normal(pts, i0, i1, i2);
    let mut i3 = None;
    best = 0.0;
    for (i, p) in pts.iter().enumerate() {
        if i == i0 || i == i1 || i == i2 {
            continue;
        }
        let dist = n.dot(*p - pts[i0]).abs();
        if dist > best {
            best = dist;
            i3 = Some(i);
        }
    }
    let i3 = i3?;
    if best <= 1e-12 {
        return None;
    }
    Some([i0, i1, i2, i3])
}

/// 🧱 Phase indices of a [`ConvexHullJob`] plan.
const DEDUPE: usize = 0;
const SEED: usize = 1;
const INSERT: usize = 2;
const VERTICES: usize = 3;
const MERGE: usize = 4;
const FACES: usize = 5;

/// 🧱 Resumable [`make_convex_hull`]: Quickhull one point per unit. One unit per input point
/// de-duplicates it, one seeds the initial tetrahedron, one per remaining point inserts it (finding
/// its visible faces and re-capping the horizon), one per hull point mints its vertex, one merges
/// coplanar triangles into polygons, one per polygon builds its face, one closes the solid. The
/// insertion, vertex and face phases are sized once de-duplication has fixed the point count.
pub struct ConvexHullJob {
    input: Vec<Pnt3>,
    pts: Vec<Pnt3>,
    faces: Vec<HullFace>,
    centroid: Pnt3,
    order: Vec<usize>,
    vertex_ids: Vec<VertexId>,
    groups: Vec<FaceGroup>,
    edge_map: HashMap<(usize, usize), EdgeId>,
    solid_faces: Vec<FaceId>,
    plan: Plan,
}

impl ConvexHullJob {
    const TOL: f64 = 1e-10;

    /// 🧱 Plans the convex hull of `points` (at least four, not all coplanar).
    pub fn new(points: &[Pnt3]) -> Result<Self, KernelError> {
        if points.len() < 4 {
            return Err(KernelError::InvalidInput("points are coplanar or degenerate — cannot form a 3D convex hull".into()));
        }
        let plan = Plan::new(&[("dedupe", points.len()), ("seed", 1), ("insert", 0), ("vertices", 0), ("merge", 1), ("faces", 0), ("close", 1)]);
        Ok(Self { input: points.to_vec(), pts: Vec::new(), faces: Vec::new(), centroid: Pnt3::new(0.0, 0.0, 0.0), order: Vec::new(), vertex_ids: Vec::new(), groups: Vec::new(), edge_map: HashMap::new(), solid_faces: Vec::new(), plan })
    }

    /// 🧱 Seeds the four faces of the initial tetrahedron, wound outward.
    fn seed(&mut self) -> Result<(), KernelError> {
        let degenerate = || KernelError::InvalidInput("points are coplanar or degenerate — cannot form a 3D convex hull".into());
        if self.pts.len() < 4 {
            return Err(degenerate());
        }
        let tet = find_initial_tetrahedron(&self.pts).ok_or_else(degenerate)?;
        let pts = &self.pts;
        let tet_faces = [[tet[0], tet[1], tet[2]], [tet[0], tet[2], tet[3]], [tet[0], tet[3], tet[1]], [tet[1], tet[3], tet[2]]];
        for &[a, b, c] in &tet_faces {
            let normal = face_normal(pts, a, b, c);
            let d = -normal.dot(pts[a].to_vec());
            self.faces.push(HullFace { verts: [a, b, c], normal, d, alive: true });
        }
        self.centroid = Pnt3::new((pts[tet[0]].x + pts[tet[1]].x + pts[tet[2]].x + pts[tet[3]].x) / 4.0, (pts[tet[0]].y + pts[tet[1]].y + pts[tet[2]].y + pts[tet[3]].y) / 4.0, (pts[tet[0]].z + pts[tet[1]].z + pts[tet[2]].z + pts[tet[3]].z) / 4.0);
        let centroid = self.centroid;
        for face in &mut self.faces {
            if signed_distance(face, centroid) > 0.0 {
                face.normal = -face.normal;
                face.d = -face.d;
                face.verts.swap(1, 2);
            }
        }
        self.order = (0..pts.len()).filter(|index| !tet.contains(index)).collect();
        self.plan.grow(INSERT, self.order.len());
        self.plan.grow(VERTICES, pts.len());
        Ok(())
    }

    /// 🧱 Inserts point `pi`: removes the faces it sees and caps the horizon with new faces.
    fn insert(&mut self, pi: usize) {
        let point = self.pts[pi];
        let faces = &mut self.faces;
        let visible: Vec<usize> = faces.iter().enumerate().filter(|(_, f)| f.alive && signed_distance(f, point) > Self::TOL).map(|(i, _)| i).collect();
        if visible.is_empty() {
            return;
        }
        let mut horizon: Vec<[usize; 2]> = Vec::new();
        for &fi in &visible {
            let verts = faces[fi].verts;
            for edge_idx in 0..3 {
                let e = [verts[edge_idx], verts[(edge_idx + 1) % 3]];
                let twin_visible = visible.iter().any(|&fj| {
                    fj != fi && {
                        let w = faces[fj].verts;
                        (0..3).any(|k| w[k] == e[1] && w[(k + 1) % 3] == e[0])
                    }
                });
                if !twin_visible {
                    horizon.push(e);
                }
            }
        }
        for &fi in &visible {
            faces[fi].alive = false;
        }
        for edge in horizon {
            let a = edge[0];
            let b = edge[1];
            let normal = face_normal(&self.pts, a, b, pi);
            let d = -normal.dot(self.pts[a].to_vec());
            let mut face = HullFace { verts: [a, b, pi], normal, d, alive: true };
            if signed_distance(&face, self.centroid) > 0.0 {
                face.normal = -face.normal;
                face.d = -face.d;
                face.verts.swap(1, 2);
            }
            faces.push(face);
        }
    }
}

impl StagedOperation for ConvexHullJob {
    fn progress(&self) -> StageProgress {
        self.plan.progress()
    }

    fn advance(&mut self, body: &mut Body, rec: &mut OpRecorder) -> Result<StageStep, KernelError> {
        let unit = self.plan.take().ok_or_else(|| KernelError::Operation("convex hull job already finished".into()))?;
        match unit.phase {
            DEDUPE => {
                let p = self.input[unit.index];
                if self.pts.iter().all(|q| q.distance(p) >= Self::TOL) {
                    self.pts.push(p);
                }
            }
            SEED => self.seed()?,
            INSERT => self.insert(self.order[unit.index]),
            VERTICES => {
                if unit.index == 0 && self.faces.iter().filter(|f| f.alive).count() < 4 {
                    return Err(KernelError::InvalidInput("points are coplanar or degenerate — cannot form a 3D convex hull".into()));
                }
                self.vertex_ids.push(make_vertex(body, self.pts[unit.index], Tol::DEFAULT, rec));
            }
            MERGE => {
                let hull = ConvexHull { vertices: std::mem::take(&mut self.pts), faces: self.faces.iter().filter(|f| f.alive).map(|f| f.verts).collect() };
                self.groups = merge_coplanar_triangles(&hull);
                self.pts = hull.vertices;
                self.plan.grow(FACES, self.groups.len());
            }
            FACES => {
                let tol = Tol::DEFAULT;
                let boundary = &self.groups[unit.index].boundary;
                let mut members = Vec::with_capacity(boundary.len());
                for i in 0..boundary.len() {
                    let ia = boundary[i];
                    let ib = boundary[(i + 1) % boundary.len()];
                    let key = (ia.min(ib), ia.max(ib));
                    let (eid, forward) = if let Some(&existing) = self.edge_map.get(&key) {
                        let edge = body.edges.get(existing).unwrap();
                        (existing, edge.v0 == self.vertex_ids[ia])
                    } else {
                        let eid = line_edge(body, self.pts[ia], self.pts[ib], self.vertex_ids[ia], self.vertex_ids[ib], tol, rec);
                        self.edge_map.insert(key, eid);
                        (eid, true)
                    };
                    members.push((eid, forward));
                }
                let frame = Frame3::from_normal(self.pts[boundary[0]], self.groups[unit.index].normal).expect("plane frame");
                let surface = body.surfaces.insert(Surface::Plane { frame });
                let face = attach_face(body, surface, &members, false, tol, rec);
                attach_planar_face_pcurves(body, face, frame, &members);
                self.solid_faces.push(face);
            }
            _ => return Ok(StageStep::Done(StageOutput::Solid(finish_solid(body, std::mem::take(&mut self.solid_faces), rec)))),
        }
        Ok(StageStep::Working)
    }
}

// #endregion 🔖️ConvexHull

// #region 🔖️CoplanarMerge

/// 🧱 One merged planar face of a convex hull: all Quickhull triangles sharing (within tolerance)
/// the same supporting plane, reduced to their outer boundary loop.
struct FaceGroup {
    normal: Vec3,
    boundary: Vec<usize>,
}

/// 🧱 Quantizes a plane `(normal, d)` into a hashable bucket key — Quickhull triangles that are
/// genuinely coplanar (same supporting plane of the convex body) compute bit-close normal/offset
/// values from their own 3 vertices, so a `1e-7`-scale round is enough to bucket them together
/// without merging two triangles that merely happen to be nearly-but-not-exactly coplanar.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn plane_key(normal: Vec3, d: f64) -> (i64, i64, i64, i64) {
    let q = |x: f64| (x * 1e7).round() as i64;
    (q(normal.x), q(normal.y), q(normal.z), q(d))
}

/// 🧱 Groups `hull`'s triangles by supporting plane, then reduces each group to its outer
/// boundary loop by cancelling every directed edge against its opposite-direction twin (present
/// exactly when two triangles in the group share that edge — the standard consistent-winding
/// invariant a convex-hull triangulation maintains) and walking what remains. A singleton group
/// (no coplanar neighbor) degenerates to its one triangle's own three edges, so non-merged faces
/// (e.g. every face of a tetrahedron) round-trip unchanged.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn merge_coplanar_triangles(hull: &ConvexHull) -> Vec<FaceGroup> {
    let mut clusters: PlaneClusters = HashMap::new();
    for &[a, b, c] in &hull.faces {
        let normal = face_normal(&hull.vertices, a, b, c);
        let d = -normal.dot(hull.vertices[a].to_vec());
        let key = plane_key(normal, d);
        clusters.entry(key).or_insert_with(|| (normal, Vec::new())).1.push([a, b, c]);
    }
    let mut ordered: Vec<_> = clusters.into_iter().collect();
    ordered.sort_unstable_by_key(|(key, _)| *key);
    let mut groups = Vec::with_capacity(ordered.len());
    for (normal, tris) in ordered.into_iter().map(|(_, cluster)| cluster) {
        let mut directed: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
        for tri in &tris {
            for i in 0..3 {
                directed.insert((tri[i], tri[(i + 1) % 3]));
            }
        }
        let mut next: HashMap<usize, usize> = HashMap::new();
        for &(a, b) in &directed {
            if !directed.contains(&(b, a)) {
                next.insert(a, b);
            }
        }
        let Some(&start) = next.keys().min() else { continue };
        let mut boundary = vec![start];
        let mut current = start;
        while let Some(&n) = next.get(&current) {
            if n == start {
                break;
            }
            boundary.push(n);
            current = n;
            if boundary.len() > next.len() {
                break;
            }
        }
        groups.push(FaceGroup { normal, boundary });
    }
    groups
}

// #endregion 🔖️CoplanarMerge


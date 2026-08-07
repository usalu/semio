//! 🧱 Analytic solid primitives: box/sphere/cylinder/cone/torus + wires/planar faces/convex hull.
//!
//! Builds closed [`Body`](crate::brep::topo::Body) solids exclusively through
//! [`crate::brep::euler`] editors, attaching shared [`Curve3`](crate::brep::curve::Curve3) /
//! [`Surface`](crate::brep::surface::Surface) geometry from the body's pools.
//! Topology layouts follow the reference shapes (box V−E+F=2, sphere hemispheres,
//! cylinder/cone seam wires, torus fundamental polygon, Quickhull convex hull).

use std::collections::HashMap;
use std::f64::consts::{FRAC_PI_2, TAU};

use crate::brep::arena::{ArenaId, EdgeId, FaceId, SolidId, VertexId};
use crate::brep::curve::Curve3;
use crate::brep::error::KernelError;
use crate::brep::euler::{add_face, add_shell, add_solid, make_edge, make_loop, make_vertex};
use crate::brep::history::OpRecorder;
use crate::brep::mat::Frame3;
use crate::brep::surface::Surface;
use crate::brep::tolerance::Tol;
use crate::brep::topo::Body;
use crate::brep::vec::{Pnt3, Vec3};

// #region 🔖️Wire

/// 🧱 An ordered chain of oriented edges produced by a wire constructor (not yet bound to a face).
#[derive(Clone, Debug, PartialEq)]
pub struct Wire {
    pub members: Vec<(EdgeId, bool)>,
    pub vertices: Vec<VertexId>,
    pub closed: bool,
}

// #endregion 🔖️Wire

// #region 🔖️Helpers

fn placeholder_face() -> FaceId {
    ArenaId::from_raw(0, 0)
}

fn require_positive(name: &str, value: f64) -> Result<(), KernelError> {
    if value <= Tol::DEFAULT.value() {
        Err(KernelError::InvalidInput(format!("{name} must be positive, got {value}")))
    } else {
        Ok(())
    }
}

fn attach_face(
    body: &mut Body,
    surface_id: crate::brep::arena::SurfaceId,
    members: &[(EdgeId, bool)],
    flipped: bool,
    tol: Tol,
    rec: &mut OpRecorder,
) -> FaceId {
    let outer = make_loop(body, placeholder_face(), members);
    let face = add_face(body, surface_id, Some(outer), vec![], flipped, tol, rec);
    body.loops.get_mut(outer).unwrap().face = face;
    face
}

fn line_edge(body: &mut Body, a: Pnt3, b: Pnt3, va: VertexId, vb: VertexId, tol: Tol, rec: &mut OpRecorder) -> EdgeId {
    let curve = body.curves3.insert(Curve3::Line { origin: a, dir: b - a });
    make_edge(body, curve, (0.0, 1.0), va, vb, tol, rec)
}

fn circle_edge(
    body: &mut Body,
    center: Pnt3,
    normal: Vec3,
    radius: f64,
    vertex: VertexId,
    tol: Tol,
    rec: &mut OpRecorder,
) -> EdgeId {
    let frame = Frame3::from_normal(center, normal).expect("circle frame");
    let curve = body.curves3.insert(Curve3::Circle { frame, radius });
    make_edge(body, curve, (0.0, TAU), vertex, vertex, tol, rec)
}

fn plane_at(origin: Pnt3, normal: Vec3) -> Surface {
    Surface::Plane {
        frame: Frame3::from_normal(origin, normal).expect("plane frame"),
    }
}

fn finish_solid(body: &mut Body, faces: Vec<FaceId>, rec: &mut OpRecorder) -> SolidId {
    let shell = add_shell(body, faces, rec);
    add_solid(body, shell, vec![], rec)
}

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

// #endregion 🔖️Helpers

// #region 🔖️Solids

/// 🧱 Axis-aligned box from the origin to `(w, d, h)` with six planar faces (V=8, E=12, F=6).
pub fn make_box(body: &mut Body, w: f64, d: f64, h: f64) -> Result<SolidId, KernelError> {
    require_positive("box width", w)?;
    require_positive("box depth", d)?;
    require_positive("box height", h)?;
    let mut rec = OpRecorder::new();
    let tol = Tol::DEFAULT;
    let corners = [
        Pnt3::new(0.0, 0.0, 0.0),
        Pnt3::new(w, 0.0, 0.0),
        Pnt3::new(w, d, 0.0),
        Pnt3::new(0.0, d, 0.0),
        Pnt3::new(0.0, 0.0, h),
        Pnt3::new(w, 0.0, h),
        Pnt3::new(w, d, h),
        Pnt3::new(0.0, d, h),
    ];
    let v: Vec<VertexId> = corners.iter().map(|&p| make_vertex(body, p, tol, &mut rec)).collect();
    let eb0 = line_edge(body, corners[0], corners[1], v[0], v[1], tol, &mut rec);
    let eb1 = line_edge(body, corners[1], corners[2], v[1], v[2], tol, &mut rec);
    let eb2 = line_edge(body, corners[2], corners[3], v[2], v[3], tol, &mut rec);
    let eb3 = line_edge(body, corners[3], corners[0], v[3], v[0], tol, &mut rec);
    let et0 = line_edge(body, corners[4], corners[5], v[4], v[5], tol, &mut rec);
    let et1 = line_edge(body, corners[5], corners[6], v[5], v[6], tol, &mut rec);
    let et2 = line_edge(body, corners[6], corners[7], v[6], v[7], tol, &mut rec);
    let et3 = line_edge(body, corners[7], corners[4], v[7], v[4], tol, &mut rec);
    let ev0 = line_edge(body, corners[0], corners[4], v[0], v[4], tol, &mut rec);
    let ev1 = line_edge(body, corners[1], corners[5], v[1], v[5], tol, &mut rec);
    let ev2 = line_edge(body, corners[2], corners[6], v[2], v[6], tol, &mut rec);
    let ev3 = line_edge(body, corners[3], corners[7], v[3], v[7], tol, &mut rec);

    let s_bottom = body.surfaces.insert(plane_at(corners[0], -Vec3::Z));
    let s_top = body.surfaces.insert(plane_at(corners[4], Vec3::Z));
    let s_front = body.surfaces.insert(plane_at(corners[0], -Vec3::Y));
    let s_back = body.surfaces.insert(plane_at(corners[3], Vec3::Y));
    let s_left = body.surfaces.insert(plane_at(corners[0], -Vec3::X));
    let s_right = body.surfaces.insert(plane_at(corners[1], Vec3::X));
    let bottom = attach_face(body, s_bottom, &[(eb0, false), (eb3, false), (eb2, false), (eb1, false)], false, tol, &mut rec);
    let top = attach_face(body, s_top, &[(et0, true), (et1, true), (et2, true), (et3, true)], false, tol, &mut rec);
    let front = attach_face(body, s_front, &[(eb0, true), (ev1, true), (et0, false), (ev0, false)], false, tol, &mut rec);
    let back = attach_face(body, s_back, &[(eb2, true), (ev3, true), (et2, false), (ev2, false)], false, tol, &mut rec);
    let left = attach_face(body, s_left, &[(eb3, true), (ev0, true), (et3, false), (ev3, false)], false, tol, &mut rec);
    let right = attach_face(body, s_right, &[(eb1, true), (ev2, true), (et1, false), (ev1, false)], false, tol, &mut rec);
    Ok(finish_solid(body, vec![bottom, top, front, back, left, right], &mut rec))
}

/// 🧱 Sphere centered at the origin as two hemispherical faces sharing an `segments`-gon equator.
pub fn make_sphere(body: &mut Body, radius: f64, segments: usize) -> Result<SolidId, KernelError> {
    require_positive("sphere radius", radius)?;
    if segments < 4 {
        return Err(KernelError::InvalidInput(format!("sphere needs at least 4 segments, got {segments}")));
    }
    let mut rec = OpRecorder::new();
    let tol = Tol::DEFAULT;
    let frame = Frame3::WORLD;
    let surface_n = body.surfaces.insert(Surface::Sphere { frame, radius });
    let surface_s = body.surfaces.insert(Surface::Sphere { frame, radius });
    let mut verts = Vec::with_capacity(segments);
    let mut positions = Vec::with_capacity(segments);
    for i in 0..segments {
        let theta = TAU * i as f64 / segments as f64;
        let p = Pnt3::new(radius * theta.cos(), radius * theta.sin(), 0.0);
        positions.push(p);
        verts.push(make_vertex(body, p, tol, &mut rec));
    }
    let mut edges = Vec::with_capacity(segments);
    for i in 0..segments {
        let j = (i + 1) % segments;
        edges.push(line_edge(body, positions[i], positions[j], verts[i], verts[j], tol, &mut rec));
    }
    let north_members: Vec<(EdgeId, bool)> = edges.iter().map(|&e| (e, true)).collect();
    let south_members: Vec<(EdgeId, bool)> = edges.iter().rev().map(|&e| (e, false)).collect();
    let north = attach_face(body, surface_n, &north_members, false, tol, &mut rec);
    let south = attach_face(body, surface_s, &south_members, true, tol, &mut rec);
    Ok(finish_solid(body, vec![north, south], &mut rec))
}

/// 🧱 Cylinder along +Z from `z=0` to `z=height` with analytic lateral surface and planar caps.
///
/// `segments` is retained for tessellation hints and must be ≥ 3; topology uses a single seam.
pub fn make_cylinder(body: &mut Body, radius: f64, height: f64, segments: usize) -> Result<SolidId, KernelError> {
    require_positive("cylinder radius", radius)?;
    require_positive("cylinder height", height)?;
    if segments < 3 {
        return Err(KernelError::InvalidInput(format!("cylinder needs at least 3 segments, got {segments}")));
    }
    let _ = segments;
    let mut rec = OpRecorder::new();
    let tol = Tol::DEFAULT;
    let bot_pt = Pnt3::new(radius, 0.0, 0.0);
    let top_pt = Pnt3::new(radius, 0.0, height);
    let v_bot = make_vertex(body, bot_pt, tol, &mut rec);
    let v_top = make_vertex(body, top_pt, tol, &mut rec);
    let e_bot = circle_edge(body, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z, radius, v_bot, tol, &mut rec);
    let e_top = circle_edge(body, Pnt3::new(0.0, 0.0, height), Vec3::Z, radius, v_top, tol, &mut rec);
    let e_seam = line_edge(body, bot_pt, top_pt, v_bot, v_top, tol, &mut rec);
    let cyl = body.surfaces.insert(Surface::Cylinder { frame: Frame3::WORLD, radius });
    let lateral = attach_face(
        body,
        cyl,
        &[(e_bot, true), (e_seam, true), (e_top, false), (e_seam, false)],
        false,
        tol,
        &mut rec,
    );
    let s_bottom = body.surfaces.insert(plane_at(Pnt3::new(0.0, 0.0, 0.0), -Vec3::Z));
    let s_top = body.surfaces.insert(plane_at(Pnt3::new(0.0, 0.0, height), Vec3::Z));
    let bottom = attach_face(body, s_bottom, &[(e_bot, false)], false, tol, &mut rec);
    let top = attach_face(body, s_top, &[(e_top, true)], false, tol, &mut rec);
    Ok(finish_solid(body, vec![lateral, bottom, top], &mut rec))
}

/// 🧱 Pointed cone with base radius at `z=0` and apex at `(0,0,height)`.
///
/// `segments` is a tessellation hint (≥ 3); topology uses a single generator seam.
pub fn make_cone(body: &mut Body, radius: f64, height: f64, segments: usize) -> Result<SolidId, KernelError> {
    require_positive("cone radius", radius)?;
    require_positive("cone height", height)?;
    if segments < 3 {
        return Err(KernelError::InvalidInput(format!("cone needs at least 3 segments, got {segments}")));
    }
    let _ = segments;
    let mut rec = OpRecorder::new();
    let tol = Tol::DEFAULT;
    let half_angle = radius.atan2(height);
    if half_angle <= Tol::DEFAULT.value() || half_angle >= FRAC_PI_2 {
        return Err(KernelError::InvalidInput(format!("cone half-angle out of range: {half_angle}")));
    }
    let apex = Pnt3::new(0.0, 0.0, height);
    let base_pt = Pnt3::new(radius, 0.0, 0.0);
    let v_apex = make_vertex(body, apex, tol, &mut rec);
    let v_base = make_vertex(body, base_pt, tol, &mut rec);
    let e_circle = circle_edge(body, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z, radius, v_base, tol, &mut rec);
    let e_seam = line_edge(body, base_pt, apex, v_base, v_apex, tol, &mut rec);
    let cone_frame = Frame3 {
        origin: apex,
        x: Vec3::X,
        y: Vec3::Y,
        z: -Vec3::Z,
    };
    let cone_surf = body.surfaces.insert(Surface::Cone { frame: cone_frame, half_angle });
    let lateral = attach_face(
        body,
        cone_surf,
        &[(e_circle, true), (e_seam, true), (e_seam, false)],
        false,
        tol,
        &mut rec,
    );
    let s_base = body.surfaces.insert(plane_at(Pnt3::new(0.0, 0.0, 0.0), -Vec3::Z));
    let base = attach_face(body, s_base, &[(e_circle, false)], false, tol, &mut rec);
    Ok(finish_solid(body, vec![lateral, base], &mut rec))
}

/// 🧱 Torus in the XY plane as one toroidal face with the fundamental-polygon seam wire (genus 1).
pub fn make_torus(body: &mut Body, major: f64, minor: f64, segments: usize) -> Result<SolidId, KernelError> {
    require_positive("torus major radius", major)?;
    require_positive("torus minor radius", minor)?;
    if minor >= major {
        return Err(KernelError::InvalidInput(format!(
            "torus minor radius ({minor}) must be less than major radius ({major})"
        )));
    }
    if segments < 4 {
        return Err(KernelError::InvalidInput(format!("torus needs at least 4 segments, got {segments}")));
    }
    let _ = segments;
    let mut rec = OpRecorder::new();
    let tol = Tol::DEFAULT;
    let seam_pt = Pnt3::new(major + minor, 0.0, 0.0);
    let v0 = make_vertex(body, seam_pt, tol, &mut rec);
    let long_frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
    let e_long = {
        let curve = body.curves3.insert(Curve3::Circle { frame: long_frame, radius: major + minor });
        make_edge(body, curve, (0.0, TAU), v0, v0, tol, &mut rec)
    };
    let mer_frame = Frame3::from_x_z(Pnt3::new(major, 0.0, 0.0), Vec3::X, Vec3::Y).unwrap();
    let e_mer = {
        let curve = body.curves3.insert(Curve3::Circle { frame: mer_frame, radius: minor });
        make_edge(body, curve, (0.0, TAU), v0, v0, tol, &mut rec)
    };
    let surface = body.surfaces.insert(Surface::Torus {
        frame: Frame3::WORLD,
        major_radius: major,
        minor_radius: minor,
    });
    let face = attach_face(
        body,
        surface,
        &[(e_long, true), (e_mer, true), (e_long, false), (e_mer, false)],
        false,
        tol,
        &mut rec,
    );
    Ok(finish_solid(body, vec![face], &mut rec))
}

/// 🧱 Convex hull of a point cloud as a closed solid of planar triangles (Quickhull).
pub fn make_convex_hull(body: &mut Body, points: &[Pnt3]) -> Result<SolidId, KernelError> {
    let hull = convex_hull_3d(points).ok_or_else(|| {
        KernelError::InvalidInput("points are coplanar or degenerate — cannot form a 3D convex hull".into())
    })?;
    let mut rec = OpRecorder::new();
    let tol = Tol::DEFAULT;
    let vertex_ids: Vec<VertexId> = hull.vertices.iter().map(|&p| make_vertex(body, p, tol, &mut rec)).collect();
    let mut edge_map: HashMap<(usize, usize), EdgeId> = HashMap::new();
    let mut faces = Vec::with_capacity(hull.faces.len());
    for &[a, b, c] in &hull.faces {
        let pairs = [(a, b), (b, c), (c, a)];
        let mut members = Vec::with_capacity(3);
        for (ia, ib) in pairs {
            let key = (ia.min(ib), ia.max(ib));
            let (eid, forward) = if let Some(&existing) = edge_map.get(&key) {
                let edge = body.edges.get(existing).unwrap();
                let forward = edge.v0 == vertex_ids[ia];
                (existing, forward)
            } else {
                let eid = line_edge(
                    body,
                    hull.vertices[ia],
                    hull.vertices[ib],
                    vertex_ids[ia],
                    vertex_ids[ib],
                    tol,
                    &mut rec,
                );
                edge_map.insert(key, eid);
                (eid, true)
            };
            members.push((eid, forward));
        }
        let pa = hull.vertices[a];
        let pb = hull.vertices[b];
        let pc = hull.vertices[c];
        let normal = (pb - pa).cross(pc - pa).normalized().unwrap_or(Vec3::Z);
        let surface = body.surfaces.insert(plane_at(pa, normal));
        faces.push(attach_face(body, surface, &members, false, tol, &mut rec));
    }
    Ok(finish_solid(body, faces, &mut rec))
}

// #endregion 🔖️Solids

// #region 🔖️WiresFaces

/// 🧱 Open or closed polyline wire through `points` (closed requires ≥ 3 points).
pub fn make_polyline_wire(body: &mut Body, points: &[Pnt3], closed: bool) -> Result<Wire, KernelError> {
    if points.len() < 2 {
        return Err(KernelError::InvalidInput("polyline needs at least 2 points".into()));
    }
    if closed && points.len() < 3 {
        return Err(KernelError::InvalidInput("closed polyline needs at least 3 points".into()));
    }
    let mut rec = OpRecorder::new();
    let tol = Tol::DEFAULT;
    let vertices: Vec<VertexId> = points.iter().map(|&p| make_vertex(body, p, tol, &mut rec)).collect();
    let mut members = Vec::new();
    let n_edges = if closed { points.len() } else { points.len() - 1 };
    for i in 0..n_edges {
        let j = (i + 1) % points.len();
        let eid = line_edge(body, points[i], points[j], vertices[i], vertices[j], tol, &mut rec);
        members.push((eid, true));
    }
    Ok(Wire { members, vertices, closed })
}

/// 🧱 Axis-aligned rectangle wire in the XY plane from the origin to `(width, height)`.
pub fn make_rectangle_wire(body: &mut Body, width: f64, height: f64) -> Result<Wire, KernelError> {
    require_positive("rectangle width", width)?;
    require_positive("rectangle height", height)?;
    make_polyline_wire(
        body,
        &[
            Pnt3::new(0.0, 0.0, 0.0),
            Pnt3::new(width, 0.0, 0.0),
            Pnt3::new(width, height, 0.0),
            Pnt3::new(0.0, height, 0.0),
        ],
        true,
    )
}

/// 🧱 Regular `sides`-gon wire of given `radius` in the XY plane, centered at the origin.
pub fn make_regular_polygon_wire(body: &mut Body, radius: f64, sides: usize) -> Result<Wire, KernelError> {
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
    make_polyline_wire(body, &points, true)
}

/// 🧱 Planar face from a closed point loop (Newell normal); points must be non-collinear.
pub fn make_planar_face_from_points(body: &mut Body, points: &[Pnt3]) -> Result<FaceId, KernelError> {
    if points.len() < 3 {
        return Err(KernelError::InvalidInput("planar face needs at least 3 points".into()));
    }
    let normal = newell_normal(points).ok_or_else(|| KernelError::InvalidInput("points are collinear".into()))?;
    let wire = make_polyline_wire(body, points, true)?;
    make_planar_face_from_wire(body, &wire, points[0], normal)
}

/// 🧱 Planar face whose outer loop is an existing closed [`Wire`].
pub fn make_planar_face_from_wire(
    body: &mut Body,
    wire: &Wire,
    origin: Pnt3,
    normal: Vec3,
) -> Result<FaceId, KernelError> {
    if !wire.closed {
        return Err(KernelError::InvalidInput("planar face requires a closed wire".into()));
    }
    if wire.members.is_empty() {
        return Err(KernelError::InvalidInput("planar face wire is empty".into()));
    }
    let mut rec = OpRecorder::new();
    let surface = body.surfaces.insert(plane_at(origin, normal));
    Ok(attach_face(body, surface, &wire.members, false, Tol::DEFAULT, &mut rec))
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

fn face_normal(pts: &[Pnt3], a: usize, b: usize, c: usize) -> Vec3 {
    (pts[b] - pts[a]).cross(pts[c] - pts[a]).normalized().unwrap_or(Vec3::Z)
}

fn signed_distance(face: &HullFace, p: Pnt3) -> f64 {
    face.normal.dot(p.to_vec()) + face.d
}

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

fn convex_hull_3d(points: &[Pnt3]) -> Option<ConvexHull> {
    if points.len() < 4 {
        return None;
    }
    let tol = 1e-10;
    let mut pts: Vec<Pnt3> = Vec::with_capacity(points.len());
    for &p in points {
        if pts.iter().all(|q| q.distance(p) >= tol) {
            pts.push(p);
        }
    }
    if pts.len() < 4 {
        return None;
    }
    let tet = find_initial_tetrahedron(&pts)?;
    let mut faces: Vec<HullFace> = Vec::new();
    let tet_faces = [
        [tet[0], tet[1], tet[2]],
        [tet[0], tet[2], tet[3]],
        [tet[0], tet[3], tet[1]],
        [tet[1], tet[3], tet[2]],
    ];
    for &[a, b, c] in &tet_faces {
        let normal = face_normal(&pts, a, b, c);
        let d = -normal.dot(pts[a].to_vec());
        faces.push(HullFace { verts: [a, b, c], normal, d, alive: true });
    }
    let centroid = Pnt3::new(
        (pts[tet[0]].x + pts[tet[1]].x + pts[tet[2]].x + pts[tet[3]].x) / 4.0,
        (pts[tet[0]].y + pts[tet[1]].y + pts[tet[2]].y + pts[tet[3]].y) / 4.0,
        (pts[tet[0]].z + pts[tet[1]].z + pts[tet[2]].z + pts[tet[3]].z) / 4.0,
    );
    for face in &mut faces {
        if signed_distance(face, centroid) > 0.0 {
            face.normal = -face.normal;
            face.d = -face.d;
            face.verts.swap(1, 2);
        }
    }
    let tet_set: std::collections::HashSet<usize> = tet.iter().copied().collect();
    for (pi, &point) in pts.iter().enumerate() {
        if tet_set.contains(&pi) {
            continue;
        }
        let visible: Vec<usize> = faces
            .iter()
            .enumerate()
            .filter(|(_, f)| f.alive && signed_distance(f, point) > tol)
            .map(|(i, _)| i)
            .collect();
        if visible.is_empty() {
            continue;
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
            let normal = face_normal(&pts, a, b, pi);
            let d = -normal.dot(pts[a].to_vec());
            let mut face = HullFace { verts: [a, b, pi], normal, d, alive: true };
            if signed_distance(&face, centroid) > 0.0 {
                face.normal = -face.normal;
                face.d = -face.d;
                face.verts.swap(1, 2);
            }
            faces.push(face);
        }
    }
    let out_faces: Vec<[usize; 3]> = faces.into_iter().filter(|f| f.alive).map(|f| f.verts).collect();
    if out_faces.len() < 4 {
        return None;
    }
    Some(ConvexHull { vertices: pts, faces: out_faces })
}

// #endregion 🔖️ConvexHull

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::brep::validate::validate_body;

    fn solid_counts(body: &Body, solid: SolidId) -> (usize, usize, usize) {
        let faces = body.solid_faces(solid);
        let mut edge_ids = std::collections::HashSet::new();
        let mut vertex_ids = std::collections::HashSet::new();
        for face in &faces {
            for coedge in body.face_coedges(*face) {
                let edge = body.coedges.get(coedge).unwrap().edge;
                edge_ids.insert(edge);
                let e = body.edges.get(edge).unwrap();
                vertex_ids.insert(e.v0);
                vertex_ids.insert(e.v1);
            }
        }
        (vertex_ids.len(), edge_ids.len(), faces.len())
    }

    fn assert_rings_ok(body: &Body) {
        let issues = validate_body(body);
        let ring_issues: Vec<_> = issues
            .iter()
            .filter(|i| matches!(i.code, "empty-loop" | "broken-ring" | "loop-not-closed" | "next-prev-mismatch"))
            .collect();
        assert!(ring_issues.is_empty(), "ring integrity failed: {ring_issues:?}");
    }

    #[test]
    fn make_box_euler_and_validate() {
        let mut body = Body::new();
        let solid = make_box(&mut body, 2.0, 3.0, 4.0).unwrap();
        let (v, e, f) = solid_counts(&body, solid);
        assert_eq!((v, e, f), (8, 12, 6));
        assert_eq!(v as i64 - e as i64 + f as i64, 2);
        assert_rings_ok(&body);
        let issues = validate_body(&body);
        assert!(issues.is_empty(), "box should validate clean: {issues:?}");
    }

    #[test]
    fn make_box_rejects_non_positive() {
        let mut body = Body::new();
        assert!(make_box(&mut body, 0.0, 1.0, 1.0).is_err());
        assert!(make_box(&mut body, 1.0, -1.0, 1.0).is_err());
    }

    #[test]
    fn make_sphere_two_hemispheres_euler() {
        let mut body = Body::new();
        let solid = make_sphere(&mut body, 1.0, 8).unwrap();
        let (v, e, f) = solid_counts(&body, solid);
        assert_eq!(f, 2);
        assert_eq!(v, 8);
        assert_eq!(e, 8);
        assert_eq!(v as i64 - e as i64 + f as i64, 2);
        assert_rings_ok(&body);
        assert!(make_sphere(&mut body, 1.0, 3).is_err());
    }

    #[test]
    fn make_cylinder_three_faces_and_rings() {
        let mut body = Body::new();
        let solid = make_cylinder(&mut body, 1.0, 2.0, 16).unwrap();
        let (_, _, f) = solid_counts(&body, solid);
        assert_eq!(f, 3);
        assert_rings_ok(&body);
        let issues = validate_body(&body);
        assert!(
            issues.iter().all(|i| i.code != "broken-ring" && i.code != "loop-not-closed"),
            "{issues:?}"
        );
    }

    #[test]
    fn make_cone_pointed_two_faces() {
        let mut body = Body::new();
        let solid = make_cone(&mut body, 1.0, 2.0, 12).unwrap();
        let (_, _, f) = solid_counts(&body, solid);
        assert_eq!(f, 2);
        assert_rings_ok(&body);
    }

    #[test]
    fn make_torus_genus_one_euler() {
        let mut body = Body::new();
        let solid = make_torus(&mut body, 3.0, 1.0, 8).unwrap();
        let (v, e, f) = solid_counts(&body, solid);
        assert_eq!(f, 1);
        assert_eq!(v, 1);
        assert_eq!(e, 2);
        assert_eq!(v as i64 - e as i64 + f as i64, 0, "torus χ must be 0");
        assert_rings_ok(&body);
        assert!(make_torus(&mut body, 1.0, 1.0, 8).is_err());
    }

    #[test]
    fn make_convex_hull_tetrahedron() {
        let mut body = Body::new();
        let pts = [
            Pnt3::new(0.0, 0.0, 0.0),
            Pnt3::new(1.0, 0.0, 0.0),
            Pnt3::new(0.0, 1.0, 0.0),
            Pnt3::new(0.0, 0.0, 1.0),
        ];
        let solid = make_convex_hull(&mut body, &pts).unwrap();
        let (v, e, f) = solid_counts(&body, solid);
        assert_eq!((v, e, f), (4, 6, 4));
        assert_eq!(v as i64 - e as i64 + f as i64, 2);
        assert_rings_ok(&body);
        let issues = validate_body(&body);
        assert!(issues.is_empty(), "{issues:?}");
    }

    #[test]
    fn make_convex_hull_rejects_coplanar() {
        let mut body = Body::new();
        let pts = [
            Pnt3::new(0.0, 0.0, 0.0),
            Pnt3::new(1.0, 0.0, 0.0),
            Pnt3::new(0.0, 1.0, 0.0),
            Pnt3::new(1.0, 1.0, 0.0),
        ];
        assert!(make_convex_hull(&mut body, &pts).is_err());
    }

    #[test]
    fn wires_and_planar_faces() {
        let mut body = Body::new();
        let rect = make_rectangle_wire(&mut body, 2.0, 3.0).unwrap();
        assert!(rect.closed);
        assert_eq!(rect.members.len(), 4);
        let face = make_planar_face_from_wire(&mut body, &rect, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        assert_eq!(body.loop_coedges(body.faces.get(face).unwrap().outer.unwrap()).len(), 4);
        let poly = make_regular_polygon_wire(&mut body, 1.0, 6).unwrap();
        assert_eq!(poly.members.len(), 6);
        let face2 = make_planar_face_from_points(
            &mut body,
            &[
                Pnt3::new(0.0, 0.0, 1.0),
                Pnt3::new(1.0, 0.0, 1.0),
                Pnt3::new(0.0, 1.0, 1.0),
            ],
        )
        .unwrap();
        assert!(body.faces.get(face2).unwrap().outer.is_some());
        assert_rings_ok(&body);
    }

    #[test]
    fn open_polyline_wire() {
        let mut body = Body::new();
        let wire = make_polyline_wire(
            &mut body,
            &[Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(1.0, 1.0, 0.0)],
            false,
        )
        .unwrap();
        assert!(!wire.closed);
        assert_eq!(wire.members.len(), 2);
        assert!(make_planar_face_from_wire(&mut body, &wire, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).is_err());
    }
}
// #endregion 🔖️Tests

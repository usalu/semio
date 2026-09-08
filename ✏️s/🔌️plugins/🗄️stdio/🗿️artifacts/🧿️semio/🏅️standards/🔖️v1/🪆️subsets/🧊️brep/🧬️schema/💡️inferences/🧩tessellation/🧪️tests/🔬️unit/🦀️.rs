
use super::*;
use crate::standards::v1::subsets::brep::schema::diff::euler::{add_face, add_shell, add_solid, make_edge, make_loop, make_vertex};
use crate::standards::v1::subsets::brep::schema::diff::primitives::{make_cylinder, make_rectangle_wire, make_sphere};
use crate::standards::v1::subsets::brep::schema::snapshot::arena::ArenaId;
use crate::standards::v1::subsets::brep::schema::snapshot::curve::{Curve2, Curve3};
use crate::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt2, Vec2};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn build_unit_box(body: &mut Body, rec: &mut OpRecorder) -> SolidId {
    let positions = [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), Pnt3::new(0.0, 0.0, 1.0), Pnt3::new(1.0, 0.0, 1.0), Pnt3::new(1.0, 1.0, 1.0), Pnt3::new(0.0, 1.0, 1.0)];
    let vertices: Vec<_> = positions.iter().map(|&p| make_vertex(body, p, Tol::DEFAULT, rec)).collect();
    let edge_pairs = [(0, 1), (1, 2), (2, 3), (3, 0), (4, 5), (5, 6), (6, 7), (7, 4), (0, 4), (1, 5), (2, 6), (3, 7)];
    let mut edges = HashMap::new();
    for &(a, b) in &edge_pairs {
        let curve = body.curves3.insert(Curve3::Line { origin: positions[a], dir: positions[b] - positions[a] });
        let edge = make_edge(body, curve, (0.0, 1.0), vertices[a], vertices[b], Tol::DEFAULT, rec);
        edges.insert((a, b), edge);
        edges.insert((b, a), edge);
    }
    let face_defs: [([usize; 4], Vec3); 6] = [([0, 3, 2, 1], -Vec3::Z), ([4, 5, 6, 7], Vec3::Z), ([0, 1, 5, 4], -Vec3::Y), ([3, 7, 6, 2], Vec3::Y), ([0, 4, 7, 3], -Vec3::X), ([1, 2, 6, 5], Vec3::X)];
    let mut faces = Vec::new();
    for (corners, normal) in face_defs {
        let frame = Frame3::from_normal(positions[corners[0]], normal).unwrap();
        let surface = body.surfaces.insert(Surface::Plane { frame });
        let members: Vec<(EdgeId, bool)> = (0..4)
            .map(|i| {
                let a = corners[i];
                let b = corners[(i + 1) % 4];
                let edge = edges[&(a, b)];
                let forward = body.edges.get(edge).unwrap().v0 == vertices[a];
                (edge, forward)
            })
            .collect();
        let outer = make_loop(body, FaceId::from_raw(0, 0), &members);
        let face = add_face(body, surface, Some(outer), vec![], false, Tol::DEFAULT, rec);
        body.loops.get_mut(outer).unwrap().face = face;
        faces.push(face);
    }
    let shell = add_shell(body, faces, rec);
    add_solid(body, shell, vec![], rec)
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn build_plane_face_with_hole(body: &mut Body, rec: &mut OpRecorder) -> FaceId {
    let tol = Tol::DEFAULT;
    let corners = [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(4.0, 0.0, 0.0), Pnt3::new(4.0, 3.0, 0.0), Pnt3::new(0.0, 3.0, 0.0)];
    let v: Vec<_> = corners.iter().map(|&p| make_vertex(body, p, tol, rec)).collect();
    let mut edges = Vec::new();
    for i in 0..4 {
        let a = i;
        let b = (i + 1) % 4;
        let curve = body.curves3.insert(Curve3::Line { origin: corners[a], dir: corners[b] - corners[a] });
        edges.push(make_edge(body, curve, (0.0, 1.0), v[a], v[b], tol, rec));
    }
    let outer_members: Vec<(EdgeId, bool)> = (0..4).map(|i| (edges[i], true)).collect();
    let outer = make_loop(body, FaceId::from_raw(0, 0), &outer_members);
    let hole_center = Pnt3::new(2.0, 1.5, 0.0);
    let hole_frame = Frame3::from_normal(hole_center, Vec3::Z).unwrap();
    // `Frame3::from_normal`'s x/y axes are derived from `Vec3::any_orthogonal` (deterministic
    // but NOT necessarily world X/Y) — the hole vertex must sit at the circle's own t=0 point
    // (`frame.to_world(radius, 0, 0)`), matching `make_cylinder`/`make_cone`'s own convention
    // (`w1e-primitives.md`), otherwise the edge's topological endpoint disagrees with its
    // curve's actual evaluation, which fractures the closed ring at that one sample.
    let hole_v = make_vertex(body, hole_frame.to_world(Pnt3::new(0.5, 0.0, 0.0)), tol, rec);
    let circle_curve = body.curves3.insert(Curve3::Circle { frame: hole_frame, radius: 0.5 });
    let hole_edge = make_edge(body, circle_curve, (0.0, std::f64::consts::TAU), hole_v, hole_v, tol, rec);
    let inner = make_loop(body, FaceId::from_raw(0, 0), &[(hole_edge, false)]);
    let surface = body.surfaces.insert(Surface::Plane { frame: Frame3::from_normal(corners[0], Vec3::Z).unwrap() });
    let face = add_face(body, surface, Some(outer), vec![inner], false, tol, rec);
    body.loops.get_mut(outer).unwrap().face = face;
    body.loops.get_mut(inner).unwrap().face = face;
    face
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn triangle_area_sum(mesh: &MeshTransfer) -> f64 {
    let mut area = 0.0;
    for tri in mesh.index.chunks_exact(3) {
        let p = |i: u32| -> Pnt3 {
            let b = (i as usize) * 3;
            Pnt3::new(mesh.position[b] as f64, mesh.position[b + 1] as f64, mesh.position[b + 2] as f64)
        };
        let (a, b, c) = (p(tri[0]), p(tri[1]), p(tri[2]));
        area += (b - a).cross(c - a).norm() * 0.5;
    }
    area
}

#[semio_framework_async_macros::async_test]
async fn unit_box_has_six_face_groups_and_unit_normals() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_unit_box(&mut body, &mut rec);
    let (mesh, _report) = tessellate_solid_with_report(&body, solid, 0.1).expect("tessellate unit box");
    assert_eq!(mesh.face_groups.len(), 6, "unit box must yield 6 face groups");
    assert_eq!(mesh.index.len() / 3, 12, "unit box must yield 12 triangles (2 per planar quad)");
    assert_eq!(mesh.edge_groups.len(), 12, "unit box must yield 12 edge groups");
    assert!(!mesh.position.is_empty(), "positions must be nonempty");
    assert!(!mesh.index.is_empty(), "indices must be nonempty");
    assert!(!mesh.normal.is_empty(), "normals must be nonempty");
    assert_eq!(mesh.position.len(), mesh.normal.len());
    assert_eq!(mesh.position.len() % 3, 0);
    assert_eq!(mesh.index.len() % 3, 0);
    for n in mesh.normal.chunks_exact(3) {
        let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        assert!((len - 1.0).abs() < 1e-3, "normal length {len} should be ~1");
    }
    let total_group = mesh.face_groups.iter().map(|g| g.count as usize).sum::<usize>();
    assert_eq!(total_group, mesh.index.len());
    for label in mesh.face_groups.iter().map(|g| &g.entity_id) {
        assert!(label.parse::<u64>().is_ok(), "face entity_id {label} must be a decimal PersistentLabel");
    }
    assert_eq!(mesh.face_infos.len(), 6);
    for info in &mesh.face_infos {
        assert_eq!(info.surface_kind, SurfaceKind::Plane);
        assert!((info.area - 1.0).abs() < 1e-6, "unit box face area should be 1, got {}", info.area);
    }
}

#[semio_framework_async_macros::async_test]
async fn tessellate_face_matches_one_box_face() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_unit_box(&mut body, &mut rec);
    let face = body.solid_faces(solid)[0];
    let mesh = tessellate_face(&body, face, 0.1).expect("tessellate face");
    assert_eq!(mesh.face_groups.len(), 1);
    assert_eq!(mesh.index.len(), 6);
    assert_eq!(mesh.position.len() / 3, 4);
}

#[semio_framework_async_macros::async_test]
async fn sample_edge_polyline_returns_line_endpoints() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let v0 = make_vertex(&mut body, Pnt3::new(0.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
    let v1 = make_vertex(&mut body, Pnt3::new(2.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
    let curve = body.curves3.insert(Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::X * 2.0 });
    let edge = make_edge(&mut body, curve, (0.0, 1.0), v0, v1, Tol::DEFAULT, &mut rec);
    let poly = sample_edge_polyline(&body, edge, 0.1);
    assert_eq!(poly.len(), 6);
    assert!((poly[0] - 0.0).abs() < 1e-6);
    assert!((poly[3] - 2.0).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn shared_edge_samples_are_identical_across_adjacent_faces() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_unit_box(&mut body, &mut rec);
    let faces = body.solid_faces(solid);
    let edge = body.face_coedges(faces[0]).into_iter().map(|c| body.coedges.get(c).unwrap().edge).next().unwrap();
    let a = sample_edge_polyline(&body, edge, 0.05);
    let b = sample_edge_polyline(&body, edge, 0.05);
    assert_eq!(a, b);
}

#[semio_framework_async_macros::async_test]
async fn circle_edge_samples_respect_deflection() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let frame = Frame3::WORLD;
    let radius = 1.0;
    let curve = body.curves3.insert(Curve3::Circle { frame, radius });
    let v = make_vertex(&mut body, Pnt3::new(1.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
    let edge = make_edge(&mut body, curve, (0.0, std::f64::consts::TAU), v, v, Tol::DEFAULT, &mut rec);
    let coarse = sample_edge_polyline(&body, edge, 0.05);
    let fine = sample_edge_polyline(&body, edge, 0.005);
    assert!(fine.len() > coarse.len(), "tighter deflection must densify circle samples ({} vs {})", fine.len(), coarse.len());
    assert!(coarse.len() >= 6);
}

#[semio_framework_async_macros::async_test]
async fn missing_solid_returns_missing_entity() {
    let body = Body::new();
    let err = tessellate_solid(&body, SolidId::from_raw(9, 0), 0.1).unwrap_err();
    assert!(matches!(err, KernelError::MissingEntity(_)));
}

#[semio_framework_async_macros::async_test]
async fn tessellate_rectangle_wire_emits_edge_segments() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let wire = make_rectangle_wire(&mut body, 2.0, 1.5, &mut rec).expect("wire");
    let mesh = tessellate_wire(&body, &wire, 0.1).expect("tessellate wire");
    assert!(mesh.edges.len() >= 24, "expected closed rectangle edge polylines, got {}", mesh.edges.len());
    assert!(mesh.position.is_empty());
    assert!(mesh.index.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn cylinder_shared_edge_vertices_are_reused_exactly() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_cylinder(&mut body, 1.0, 2.0, &mut rec).expect("cylinder");
    let deflection = 0.1;
    let mesh = tessellate_solid(&body, solid, deflection).expect("tessellate cylinder");
    let mut circle_edge = None;
    'search: for face in body.solid_faces(solid) {
        for coedge_id in body.face_coedges(face) {
            let edge_id = body.coedges.get(coedge_id).unwrap().edge;
            if let Some(Curve3::Circle { .. }) = body.curves3.get(body.edges.get(edge_id).unwrap().curve) {
                circle_edge = Some(edge_id);
                break 'search;
            }
        }
    }
    let edge_id = circle_edge.expect("cylinder must have a circle edge");
    let poly = sample_edge_polyline(&body, edge_id, deflection);
    for chunk in poly.chunks_exact(3) {
        let p = Pnt3::new(chunk[0] as f64, chunk[1] as f64, chunk[2] as f64);
        let found = mesh.position.chunks_exact(3).any(|q| Pnt3::new(q[0] as f64, q[1] as f64, q[2] as f64).distance(p) < 1e-6);
        assert!(found, "edge sample point {p:?} must be reused verbatim as a mesh vertex (crack-free)");
    }
}

#[semio_framework_async_macros::async_test]
async fn cylinder_lateral_face_area_matches_analytic_across_the_seam() {
    let (radius, height) = (1.0, 2.0);
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_cylinder(&mut body, radius, height, &mut rec).expect("cylinder");
    let lateral = body.solid_faces(solid)[0];
    let (mesh, _report) = tessellate_face_with_report(&body, lateral, 0.02).expect("tessellate lateral face");
    let area = triangle_area_sum(&mesh);
    let analytic = 2.0 * std::f64::consts::PI * radius * height;
    assert!((area - analytic).abs() / analytic < 0.02, "lateral area {area} should match analytic {analytic} across the seam");
    assert_eq!(mesh.face_infos.len(), 1);
    assert_eq!(mesh.face_infos[0].surface_kind, SurfaceKind::Cylinder);
    for n in mesh.normal.chunks_exact(3) {
        let len = ((n[0] * n[0] + n[1] * n[1] + n[2] * n[2]) as f64).sqrt();
        assert!((len - 1.0).abs() < 1e-3, "normal length {len} should be ~1 across the seam");
    }
}

#[semio_framework_async_macros::async_test]
async fn tighter_deflection_yields_more_triangles() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_cylinder(&mut body, 1.0, 2.0, &mut rec).expect("cylinder");
    let coarse = tessellate_solid(&body, solid, 0.2).expect("coarse");
    let fine = tessellate_solid(&body, solid, 0.02).expect("fine");
    assert!(fine.index.len() > coarse.index.len(), "finer deflection must yield more triangles ({} vs {})", fine.index.len(), coarse.index.len());
}

#[semio_framework_async_macros::async_test]
async fn report_max_chordal_respects_deflection() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_cylinder(&mut body, 1.0, 2.0, &mut rec).expect("cylinder");
    let deflection = 0.05;
    let (_mesh, report) = tessellate_solid_with_report(&body, solid, deflection).expect("tessellate with report");
    assert!(report.max_chordal <= deflection * 1.05, "max_chordal {} should respect deflection {}", report.max_chordal, deflection);
}

#[semio_framework_async_macros::async_test]
async fn face_with_circular_hole_triangulates_inside_the_trim() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let face = build_plane_face_with_hole(&mut body, &mut rec);
    let mesh = tessellate_face(&body, face, 0.1).expect("tessellate hole face");
    assert!(!mesh.index.is_empty());
    let hole_center = (2.0_f64, 1.5_f64);
    let hole_radius = 0.5_f64;
    for tri in mesh.index.chunks_exact(3) {
        let p = |i: u32| -> Pnt3 {
            let b = (i as usize) * 3;
            Pnt3::new(mesh.position[b] as f64, mesh.position[b + 1] as f64, mesh.position[b + 2] as f64)
        };
        let (a, b, c) = (p(tri[0]), p(tri[1]), p(tri[2]));
        let cx = (a.x + b.x + c.x) / 3.0;
        let cy = (a.y + b.y + c.y) / 3.0;
        assert!((-1e-6..=4.0 + 1e-6).contains(&cx) && (-1e-6..=3.0 + 1e-6).contains(&cy), "triangle centroid ({cx}, {cy}) outside outer rectangle");
        let dist = ((cx - hole_center.0).powi(2) + (cy - hole_center.1).powi(2)).sqrt();
        assert!(dist >= hole_radius - 1e-3, "triangle centroid ({cx}, {cy}) falls inside the hole (dist {dist})");
    }
}

#[semio_framework_async_macros::async_test]
async fn sphere_caps_collapse_pole_to_single_vertex() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let radius = 1.0;
    let solid = make_sphere(&mut body, radius, &mut rec).expect("sphere");
    for face in body.solid_faces(solid) {
        let mesh = tessellate_face(&body, face, 0.1).expect("tessellate cap");
        let mut extreme = Pnt3::new(0.0, 0.0, 0.0);
        let mut extreme_abs = 0.0_f64;
        for chunk in mesh.position.chunks_exact(3) {
            let p = Pnt3::new(chunk[0] as f64, chunk[1] as f64, chunk[2] as f64);
            if p.z.abs() > extreme_abs {
                extreme_abs = p.z.abs();
                extreme = p;
            }
        }
        assert!((extreme_abs - radius).abs() < 0.05, "cap should approach the pole radius, got {extreme_abs}");
        let count = mesh.position.chunks_exact(3).filter(|chunk| Pnt3::new(chunk[0] as f64, chunk[1] as f64, chunk[2] as f64).distance(extreme) < 1e-4).count();
        assert_eq!(count, 1, "pole must collapse to exactly one vertex, found {count}");
    }
}

#[semio_framework_async_macros::async_test]
async fn coedge_uv_prefers_stored_pcurve_when_present() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let tol = Tol::DEFAULT;
    let p0 = Pnt3::new(0.0, 0.0, 0.0);
    let p1 = Pnt3::new(1.0, 0.0, 0.0);
    let v0 = make_vertex(&mut body, p0, tol, &mut rec);
    let v1 = make_vertex(&mut body, p1, tol, &mut rec);
    let curve = body.curves3.insert(Curve3::Line { origin: p0, dir: p1 - p0 });
    let edge = make_edge(&mut body, curve, (0.0, 1.0), v0, v1, tol, &mut rec);
    let pcurve = body.curves2.insert(Curve2::Line { origin: Pnt2::new(5.0, 5.0), dir: Vec2::new(1.0, 0.0) });
    let surface = body.surfaces.insert(Surface::Plane { frame: Frame3::WORLD });
    let outer = make_loop(&mut body, FaceId::from_raw(0, 0), &[(edge, true)]);
    let face = add_face(&mut body, surface, Some(outer), vec![], false, tol, &mut rec);
    body.loops.get_mut(outer).unwrap().face = face;
    let coedge_id = body.loop_coedges(outer)[0];
    {
        let coedge = body.coedges.get_mut(coedge_id).unwrap();
        coedge.pcurve = Some(pcurve);
        coedge.prange = (0.0, 1.0);
    }
    let mut cache = HashMap::new();
    cache.insert(edge, sample_edge_points(&body, edge, 0.1).unwrap());
    let (_positions, uvs, _poles) = collect_loop_uv(&body, outer, body.surfaces.get(surface).unwrap(), &cache).unwrap();
    assert!((uvs[0].0 - 5.0).abs() < 1e-9 && (uvs[0].1 - 5.0).abs() < 1e-9, "first sample should come from the stored pcurve, got {:?}", uvs[0]);
}

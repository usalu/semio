use super::*;
use crate::brep::operations::euler::{add_face, add_shell, add_solid, make_edge, make_loop, make_vertex};
use crate::brep::operations::primitives::{make_cylinder, make_rectangle_wire, make_sphere};
use crate::brep::representation::arena::ArenaId;
use crate::brep::representation::curve::{Curve2, Curve3};
use crate::brep::representation::surface::Surface;
use crate::brep::representation::tolerance::Tol;
use crate::brep::representation::topology::history::OpRecorder;
use crate::brep::representation::topology::Body;
use crate::brep::representation::vector::matrix::Frame3;
use crate::brep::representation::vector::{Pnt2, Vec2};

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
    let mut cache = EdgeSampleCache::new();
    cache.insert(edge, sample_edge_points(&body, edge, 0.1).unwrap());
    let (_positions, uvs, _poles) = collect_loop_uv(&body, outer, body.surfaces.get(surface).unwrap(), &cache).unwrap();
    assert!((uvs[0].0 - 5.0).abs() < 1e-9 && (uvs[0].1 - 5.0).abs() < 1e-9, "first sample should come from the stored pcurve, got {:?}", uvs[0]);
}


struct TessellationAllocationObserver;
std::thread_local! {
    static SYSTEM_OBSERVING: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static SYSTEM_CAPACITY: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static SYSTEM_RELEASE: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}
pub(crate) fn observe_tessellation_system<T>(operation: impl FnOnce() -> T) -> (T, (usize, usize)) {
    SYSTEM_CAPACITY.with(|value| value.set(0));
    SYSTEM_RELEASE.with(|value| value.set(0));
    SYSTEM_OBSERVING.with(|value| value.set(true));
    let result = operation();
    SYSTEM_OBSERVING.with(|value| value.set(false));
    (result, (SYSTEM_CAPACITY.with(std::cell::Cell::get), SYSTEM_RELEASE.with(std::cell::Cell::get)))
}
static OBSERVED_SAMPLE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static OBSERVED_MESH: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static SAMPLE_RELEASES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static MESH_RELEASES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static OBSERVED_RELEASE_BYTES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
#[global_allocator]
static TESSELLATION_ALLOCATOR: TessellationAllocationObserver = TessellationAllocationObserver;
unsafe impl std::alloc::GlobalAlloc for TessellationAllocationObserver {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        let pointer = unsafe { std::alloc::GlobalAlloc::alloc(&std::alloc::System, layout) };
        if !pointer.is_null() && SYSTEM_OBSERVING.with(std::cell::Cell::get) { SYSTEM_CAPACITY.with(|value| value.set(value.get() + layout.size())); }
        pointer
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: std::alloc::Layout) {
        use std::sync::atomic::Ordering::SeqCst;
        if SYSTEM_OBSERVING.with(std::cell::Cell::get) { SYSTEM_RELEASE.with(|value| value.set(value.get() + layout.size())); }
        if OBSERVED_SAMPLE.compare_exchange(pointer as usize,0,SeqCst,SeqCst).is_ok() { SAMPLE_RELEASES.fetch_add(1,SeqCst);OBSERVED_RELEASE_BYTES.fetch_add(layout.size(),SeqCst); }
        if OBSERVED_MESH.compare_exchange(pointer as usize,0,SeqCst,SeqCst).is_ok() { MESH_RELEASES.fetch_add(1,SeqCst);OBSERVED_RELEASE_BYTES.fetch_add(layout.size(),SeqCst); }
        unsafe { std::alloc::GlobalAlloc::dealloc(&std::alloc::System,pointer,layout); }
    }
}

fn exact_retirement_grant<T:semio_framework_value::retirement::RetireOwned>(owner:&semio_framework_value::retirement::controlled::ControlledRetirement<T>,items:usize)->semio_framework_value::retained_clone::RetainedCloneGrant {
    let copy=owner.next_copy_byte_demand().unwrap();semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:items,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()}
}
fn small_retirement_grant<T:semio_framework_value::retirement::RetireOwned>(owner:&semio_framework_value::retirement::controlled::ControlledRetirement<T>,items:usize,release:usize)->semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:items,maximum_copy_bytes:1,maximum_capacity_bytes:0,maximum_release_bytes:release,maximum_depth:owner.next_depth_demand().unwrap()}
}

#[test]
fn tessellation_cancel_handoff_and_small_grants_cover_every_actual_system_allocation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎟️ownership/🔣️.json")).unwrap();
    let mut body = Body::new();
    let solid = build_unit_box(&mut body, &mut OpRecorder::new());
    let (mut job, constructor) = observe_tessellation_system(|| TessellationJob::new(0.1));
    while job.transfer.position.is_empty() { assert!(matches!(job.step(&body, TessellationInput::Solid(solid), 1).unwrap(), TessellationStep::Working(_))); }
    let (_, signal) = observe_tessellation_system(|| job.cancel());
    let (mut retirement, handoff) = observe_tessellation_system(|| job.detach_retirement());
    let items = fixture["grants"]["items"].as_u64().unwrap() as usize;
    let bytes = fixture["grants"]["bytes"].as_u64().unwrap() as usize;
    let mut maximum_small_release = 0;
    let mut maximum_unfunded_capacity = 0;
    let mut turns = 0;
    while !retirement.terminal_is_empty() {
        let (step, heap) = observe_tessellation_system(|| retirement.step(small_retirement_grant(&retirement,items,bytes)).unwrap());
        maximum_small_release = maximum_small_release.max(heap.1);
        maximum_unfunded_capacity = maximum_unfunded_capacity.max(heap.0);
        assert!(step.progress().fits(small_retirement_grant(&retirement,items,bytes)));
        if step.progress()==Default::default() { let grant=exact_retirement_grant(&retirement,items);let (receipt,heap)=observe_tessellation_system(||retirement.step(grant).unwrap());assert!(receipt.progress().fits(grant));assert_eq!(heap,(receipt.progress().retained_capacity_bytes,receipt.progress().released_bytes)); }
        turns += 1;
        assert!(turns < 10000);
    }
    eprintln!("[DEBUG] whole original Tessellation System constructor={constructor:?} cancel={signal:?} handoff={handoff:?} maximumEightByteRelease={maximum_small_release} maximumUnfundedCapacity={maximum_unfunded_capacity} turns={turns}");
    assert_eq!(constructor, (fixture["system"]["constructorCapacityBytes"].as_u64().unwrap() as usize, 0));
    assert_eq!(signal, (fixture["system"]["cancelCapacityBytes"].as_u64().unwrap() as usize, fixture["system"]["cancelReleaseBytes"].as_u64().unwrap() as usize));
    assert_eq!(handoff, (fixture["system"]["handoffCapacityBytes"].as_u64().unwrap() as usize, fixture["system"]["handoffReleaseBytes"].as_u64().unwrap() as usize), "existing owner handoff requires no unfunded allocation or release");
    assert!(maximum_small_release <= bytes, "actual whole System deallocation exceeded the eight-byte grant: {maximum_small_release}");
    assert_eq!(maximum_unfunded_capacity, fixture["system"]["turnCapacityBytes"].as_u64().unwrap() as usize);
}

#[test]
fn original_native_family_handoff_retains_all_system_owners_until_grants() {
    use crate::brep::engine::Brep;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎟️ownership/🔣️.json")).unwrap();
    let mut family=Brep::new();
    family.box_prim_sync(1.0,1.0,1.0).unwrap();
    let (mut retirement,handoff)=observe_tessellation_system(||family.detach_retirement());
    let small=fixture["grants"]["bytes"].as_u64().unwrap() as usize;
    let mut maximum_release=0;
    let mut maximum_birth=0;
    let mut turns=0;
    while !retirement.terminal_is_empty() {
        let (step,heap)=observe_tessellation_system(||retirement.step(small_retirement_grant(&retirement,1,small)).unwrap());
        maximum_birth=maximum_birth.max(heap.0);maximum_release=maximum_release.max(heap.1);
        if step.progress()==Default::default() {let grant=exact_retirement_grant(&retirement,1);let (receipt,heap)=observe_tessellation_system(||retirement.step(grant).unwrap());assert!(receipt.progress().fits(grant));assert_eq!(heap,(receipt.progress().retained_capacity_bytes,receipt.progress().released_bytes));}
        turns+=1;assert!(turns<100000);
    }
    eprintln!("[DEBUG] original native family handoff={handoff:?} maximumEightByteRelease={maximum_release} maximumUnfundedBirth={maximum_birth} turns={turns}");
    assert_eq!(handoff,(fixture["nativeFamily"]["handoffCapacityBytes"].as_u64().unwrap() as usize,fixture["nativeFamily"]["handoffReleaseBytes"].as_u64().unwrap() as usize));
    assert!(maximum_release<=small,"original native family physical release exceeded the actual eight-byte grant");
    assert_eq!(maximum_birth,0);
}

#[test]
fn original_native_family_typed_owner_covers_nurbs_wire_aliases_and_compound() {
    use crate::brep::engine::Brep;
    use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎟️ownership/🔣️.json")).unwrap();let cases=&fixture["nativeFamily"];
    for converted in cases["convertToNurbs"].as_array().unwrap() {
        let converted=converted.as_bool().unwrap();
        let (family,source)=observe_tessellation_system(|| {
            let mut family=Brep::new();let first=family.box_prim_sync(1.0,1.0,1.0).unwrap();let second=family.box_prim_sync(1.0,1.0,1.0).unwrap();
            if converted {family.convert_to_nurbs_sync(&first).unwrap();}
            family.deconstruct_sync(&first).unwrap();
            let compound=family.compound_sync(&[first,second]).unwrap();assert!((family.volume_sync(&compound).unwrap()-cases["compoundVolume"].as_f64().unwrap()).abs()<1e-8);
            let wire=family.rectangle_wire_sync(cases["wire"]["width"].as_f64().unwrap(),cases["wire"]["height"].as_f64().unwrap()).unwrap();assert!((family.length_sync(&wire).unwrap()-cases["wire"]["length"].as_f64().unwrap()).abs()<1e-8);
            family.arc_curve_sync([0.0;3],[0.0,0.0,1.0],1.0,0.0,1.0).unwrap();family.plane_surface_sync([0.0;3],[0.0,0.0,1.0]).unwrap();
            family
        });
        let vertex=family.representation().vertices.ids().next().unwrap();let pointer=family.representation().vertices.get(vertex).unwrap() as *const _;
        let (mut owner,handoff)=observe_tessellation_system(||ControlledRetirement::new(family).unwrap_or_else(|_|panic!("original Brep controlled family")));assert_eq!(handoff,(0,0));assert_eq!(owner.original().unwrap().representation().vertices.get(vertex).unwrap() as *const _,pointer);
        let (mut born,mut released,mut refusals,mut turns)=(0,0,0,0);
        while !owner.terminal_is_empty() {
            let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
            if grant.maximum_release_bytes>8 {for _ in 0..2 {let (step,heap)=observe_tessellation_system(||owner.step(RetainedCloneGrant{maximum_release_bytes:8,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.next_release_byte_demand().unwrap(),grant.maximum_release_bytes);refusals+=1;}}
            let (step,heap)=observe_tessellation_system(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;turns+=1;assert!(turns<100000);
        }
        assert!(refusals>0);assert_eq!(released,source.0-source.1+born);
        eprintln!("[DEBUG] original typed Brep nurbs={converted} wire+aliases+compound samePointer=true retainedSource={} born={born} physicallyReleased={released} repeatedEightByteRefusals={refusals} turns={turns}",source.0-source.1);
    }
}

#[test]
fn original_live_index_extraction_preserves_row_pointers_without_heap_work() {
    use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎟️ownership/🔣️.json")).unwrap();let rows=&fixture["liveRows"];
    let ((mut index,mut extracted),source)=observe_tessellation_system(|| {
        let index:HistoryFoldIndex<String,Vec<u8>>=rows["keys"].as_array().unwrap().iter().map(|key|(key.as_str().unwrap().to_owned(),vec![0;rows["payloadBytes"].as_u64().unwrap() as usize])).collect();
        (index,Vec::with_capacity(rows["removedKeys"].as_array().unwrap().len()))
    });
    let pointers=index.iter().map(|(key,value)|(key.clone(),(key.as_ptr(),value.as_ptr()))).collect::<std::collections::BTreeMap<_,_>>();
    let removed=rows["removedKeys"].as_array().unwrap().iter().map(|key|key.as_str().unwrap()).collect::<Vec<_>>();
    for slot in 0..index.slot_count() {
        let mut predicates=0;
        let (_,heap)=observe_tessellation_system(||if let Some(row)=index.extract_slot_if(slot,|key,_|{predicates+=1;!removed.contains(&key.as_str())}) {assert_eq!((row.0.as_ptr(),row.1.as_ptr()),pointers[&row.0]);extracted.push(row);});
        assert!(predicates<=rows["maximumPredicatesPerTurn"].as_u64().unwrap() as usize);assert_eq!(heap,(rows["extractionCapacityBytes"].as_u64().unwrap() as usize,rows["extractionReleaseBytes"].as_u64().unwrap() as usize));
    }
    assert_eq!(extracted.iter().map(|(key,_)|key.as_str()).collect::<Vec<_>>(),removed);
    let (mut owner,handoff)=observe_tessellation_system(||ControlledRetirement::new((index,extracted)).unwrap_or_else(|_|panic!("original live index row ownership")));assert_eq!(handoff,(0,0));
    let (mut born,mut released)=(0,0);
    while !owner.terminal_is_empty() {
        let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
        let (step,heap)=observe_tessellation_system(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;
    }
    assert_eq!(released,source.0-source.1+born);eprintln!("[DEBUG] original live index extract sameKey=true sameValue=true slotMaximum=1 heap=(0,0) retainedSource={} born={born} released={released}",source.0-source.1);
}

#[test]
fn original_body_arenas_retire_by_exact_physical_full_grants() {
    use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎟️ownership/🔣️.json")).unwrap();
    let ((body,vertex),source)=observe_tessellation_system(|| {
        let mut body=Body::new();build_unit_box(&mut body,&mut OpRecorder::new());
        let vertex=body.vertices.ids().next().unwrap();
        let spare=body.vertices.get(vertex).unwrap().clone();let hole=body.vertices.insert(spare);drop(body.vertices.remove(hole));
        (body,vertex)
    });
    let pointer=body.vertices.get(vertex).unwrap() as *const _;
    let (mut owner,handoff)=observe_tessellation_system(||ControlledRetirement::new(body).unwrap_or_else(|_|panic!("original Body controlled authority")));
    assert_eq!(handoff,(0,0));assert_eq!(owner.original().unwrap().vertices.get(vertex).unwrap() as *const _,pointer);
    let (mut born,mut freed,mut refusals,mut turns)=(0,0,0,0);
    while !owner.terminal_is_empty() {
        let copy=owner.next_copy_byte_demand().unwrap();
        let grant=RetainedCloneGrant{maximum_items:fixture["grants"]["items"].as_u64().unwrap() as usize,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
        let small=fixture["grants"]["bytes"].as_u64().unwrap() as usize;
        if grant.maximum_release_bytes>small {
            for _ in 0..2 {
                let (step,heap)=observe_tessellation_system(||owner.step(RetainedCloneGrant{maximum_release_bytes:small,..grant}).unwrap());
                assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.next_release_byte_demand().unwrap(),grant.maximum_release_bytes);refusals+=1;
            }
        }
        let (step,heap)=observe_tessellation_system(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        born+=heap.0;freed+=heap.1;turns+=1;assert!(turns<100000);
    }
    assert!(refusals>0);assert_eq!(freed,source.0-source.1+born);
    eprintln!("[DEBUG] original Body slots+holes+freeLists retainedSource={} born={born} physicallyReleased={freed} repeatedEightByteRefusals={refusals} turns={turns}",source.0-source.1);
}

#[test]
fn original_whole_tessellation_typed_owner_matches_each_physical_full_grant() {
    use semio_framework_value::{retained_clone::RetainedCloneGrant, retirement::controlled::ControlledRetirement};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎟️ownership/🔣️.json")).unwrap();
    let mut body = Body::new();
    let solid = build_unit_box(&mut body, &mut OpRecorder::new());
    let (mut job, source) = observe_tessellation_system(|| {
        let mut job = TessellationJob::new(0.1);
        while job.transfer.position.is_empty() { job.step(&body, TessellationInput::Solid(solid), 1).unwrap(); }
        job
    });
    let sample_pointer = job.edge_cache.values().next().unwrap().as_ptr();
    let mesh_pointer = job.transfer.position.as_ptr();
    let (_, cancel) = observe_tessellation_system(|| job.cancel());
    let (mut owner, handoff) = observe_tessellation_system(|| ControlledRetirement::new(job).unwrap_or_else(|_| panic!("original whole Tessellation authority")));
    assert_eq!(owner.original().unwrap().edge_cache.values().next().unwrap().as_ptr(), sample_pointer);
    assert_eq!(owner.original().unwrap().transfer.position.as_ptr(), mesh_pointer);
    assert_eq!(cancel, (0, 0));
    assert_eq!(handoff, (0, 0));
    let mut born = 0;
    let mut freed = 0;
    let mut refusals = 0;
    let mut turns = 0;
    while !owner.terminal_is_empty() {
        let copy = owner.next_copy_byte_demand().unwrap();
        let grant = RetainedCloneGrant { maximum_items: fixture["grants"]["items"].as_u64().unwrap() as usize, maximum_copy_bytes: copy, maximum_capacity_bytes: owner.next_capacity_byte_demand(copy).unwrap(), maximum_release_bytes: owner.next_release_byte_demand().unwrap(), maximum_depth: owner.next_depth_demand().unwrap() };
        let small_release = fixture["grants"]["bytes"].as_u64().unwrap() as usize;
        if grant.maximum_release_bytes > small_release {
            for _ in 0..2 {
                let small = RetainedCloneGrant { maximum_release_bytes: small_release, ..grant };
                let (step, heap) = observe_tessellation_system(|| owner.step(small).unwrap());
                assert!(step.progress().fits(small));
                assert_eq!(heap, (0, 0), "actual whole allocation must stay owned under the eight-byte grant");
                assert_eq!(step.progress(), Default::default());
                assert_eq!(owner.next_release_byte_demand().unwrap(), grant.maximum_release_bytes);
                refusals += 1;
            }
        }
        let (step, heap) = observe_tessellation_system(|| owner.step(grant).unwrap());
        let progress = step.progress();
        assert!(progress.fits(grant));
        assert_eq!(heap, (progress.retained_capacity_bytes, progress.released_bytes), "each whole original typed receipt equals System allocation/deallocation");
        born += heap.0;
        freed += heap.1;
        turns += 1;
        assert!(turns < 100000);
    }
    assert!(refusals > 0);
    assert_eq!(freed, source.0 - source.1 + born);
    eprintln!("[DEBUG] original whole Tessellation typed custody retainedSource={} born={born} physicallyReleased={freed} repeatedEightByteRefusals={refusals} turns={turns}", source.0 - source.1);
}

#[test]
fn tessellation_cancel_preserves_original_allocations_until_retirement_grants() {
    use std::sync::atomic::Ordering::SeqCst;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎟️ownership/🔣️.json")).unwrap();
    let mut body=Body::new();let mut rec=OpRecorder::new();let solid=build_unit_box(&mut body,&mut rec);
    let mut job=TessellationJob::new(0.1);
    while job.transfer.position.is_empty() { assert!(matches!(job.step(&body,TessellationInput::Solid(solid),1).unwrap(),TessellationStep::Working(_))); }
    let sample=job.edge_cache.values().next().unwrap();
    OBSERVED_SAMPLE.store(sample.as_ptr() as usize,SeqCst);OBSERVED_MESH.store(job.transfer.position.as_ptr() as usize,SeqCst);
    SAMPLE_RELEASES.store(0,SeqCst);MESH_RELEASES.store(0,SeqCst);OBSERVED_RELEASE_BYTES.store(0,SeqCst);
    job.cancel();
    assert_eq!(SAMPLE_RELEASES.load(SeqCst),fixture["cancel"]["releasesAtSignal"].as_u64().unwrap() as usize,"cancel must retain the original sample allocation");
    assert_eq!(MESH_RELEASES.load(SeqCst),0,"cancel must retain the original transfer allocation");
    assert!(matches!(job.step(&body,TessellationInput::Solid(solid),1).unwrap(),TessellationStep::Cancelled(_)));
    let mut retirement=job.detach_retirement();
    assert_eq!(retirement.step(small_retirement_grant(&retirement,0,8)).unwrap().progress(),Default::default());assert_eq!(retirement.step(small_retirement_grant(&retirement,1,0)).unwrap().progress(),Default::default());
    assert_eq!(SAMPLE_RELEASES.load(SeqCst),0);assert_eq!(MESH_RELEASES.load(SeqCst),0);
    let mut turns=0;let mut refused=0;let mut physical=0;
    while !retirement.terminal_is_empty() {
        OBSERVED_RELEASE_BYTES.store(0,SeqCst);let grant=small_retirement_grant(&retirement,1,8);let small=retirement.step(grant).unwrap();
        let released=OBSERVED_RELEASE_BYTES.load(SeqCst);assert!(released<=8,"physical sample/mesh release exceeded the actual eight-byte grant");physical+=released;assert!(small.progress().fits(grant));
        if small.progress()==Default::default() {
            refused+=1;let sample_before=SAMPLE_RELEASES.load(SeqCst);let mesh_before=MESH_RELEASES.load(SeqCst);
            assert_eq!(retirement.step(grant).unwrap().progress(),Default::default());assert_eq!(SAMPLE_RELEASES.load(SeqCst),sample_before);assert_eq!(MESH_RELEASES.load(SeqCst),mesh_before);
            let demand=exact_retirement_grant(&retirement,1);OBSERVED_RELEASE_BYTES.store(0,SeqCst);let receipt=retirement.step(demand).unwrap();let released=OBSERVED_RELEASE_BYTES.load(SeqCst);assert!(released<=demand.maximum_release_bytes,"physical sample/mesh release exceeded admitted original demand");physical+=released;assert!(receipt.progress().fits(demand));
        }
        turns+=1;assert!(turns<10000);
    }
    assert!(refused>0);assert!(physical>8);
    assert_eq!(SAMPLE_RELEASES.load(SeqCst),fixture["cancel"]["sampleReleases"].as_u64().unwrap() as usize);assert_eq!(MESH_RELEASES.load(SeqCst),fixture["cancel"]["meshReleases"].as_u64().unwrap() as usize);
    OBSERVED_SAMPLE.store(0,SeqCst);OBSERVED_MESH.store(0,SeqCst);
    eprintln!("[DEBUG] Tessellation ownership: original sample/mesh allocations survived cancel and zero grants; actual allocator released each once after {} one-item turns, {} repeated eight-byte refusals, {} observed physical bytes",turns,refused,physical);
}

#[test]
fn tessellation_cold_admission_does_not_collect_original_topology_before_a_grant() {
    let fixture: serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎟️ownership/🔣️.json")).unwrap();
    let mut body=Body::new();let mut rec=OpRecorder::new();let solid=build_unit_box(&mut body,&mut rec);
    let mut job=TessellationJob::new(0.1);
    assert_eq!(job.edge_order.len(),fixture["cold"]["initialEdges"].as_u64().unwrap() as usize);
    assert_eq!(job.faces.len(),fixture["cold"]["initialFaces"].as_u64().unwrap() as usize);
    assert_eq!(job.progress().units_done,fixture["cold"]["initialUnits"].as_u64().unwrap() as usize);
    assert!(matches!(job.step(&body,TessellationInput::Solid(solid),0).unwrap(),TessellationStep::Working(_)));
    assert!(job.edge_order.is_empty() && job.faces.is_empty());
    let before=job.progress().units_done;assert!(matches!(job.step(&body,TessellationInput::Solid(solid),1).unwrap(),TessellationStep::Working(_)));assert_eq!(job.progress().units_done,before+1);
    let (mesh,_)=job.run_to_completion(&body,TessellationInput::Solid(solid)).unwrap();assert_eq!(mesh.face_groups.len(),fixture["box"]["faces"].as_u64().unwrap() as usize);assert_eq!(mesh.edge_groups.len(),12);assert_eq!(mesh.index.len()/3,12);assert!((triangle_area_sum(&mesh)-6.0).abs()<1e-6);
    let volume=mesh.index.chunks_exact(3).map(|tri| {let p=|id:u32|Vec3::new(mesh.position[id as usize*3] as f64,mesh.position[id as usize*3+1] as f64,mesh.position[id as usize*3+2] as f64);p(tri[0]).dot(p(tri[1]).cross(p(tri[2])))/6.0}).sum::<f64>().abs();assert!((volume-fixture["box"]["volume"].as_f64().unwrap()).abs()<1e-6);
    use crate::brep::operations::staged::{StageStep,StagedOperation};
    let mut caller=crate::brep::operations::boolean::SplitJob::new(&body,solid,Pnt3::new(0.5,0.0,0.0),Vec3::new(1.0,0.0,0.0),0.1).unwrap();
    for turn in 0..fixture["cold"]["callerProbeTurns"].as_u64().unwrap() as usize {
        assert_eq!(caller.advance(&mut body,&mut rec).unwrap(),StageStep::Working);
        assert_eq!(caller.progress().phase,"tessellate","original Split Plan advanced before its cold child produced a mesh");assert_eq!(caller.progress().done,turn+1);assert!(caller.progress().total>=caller.progress().done);
    }
    assert!((crate::brep::queries::mass_properties::solid_volume(&body,solid,0.1).unwrap()-fixture["box"]["volume"].as_f64().unwrap()).abs()<1e-6);
    eprintln!("[DEBUG] Tessellation cold ownership: no original topology collection before grants; one grant advanced one frontier; original box parity area=6 volume=1 triangles=12; original Split Plan remains on cold tessellation through eight granted caller turns");
}

#[test]
fn original_validation_scratch_retains_all_physical_buffers_until_full_grants() {
    use crate::brep::queries::validation::BodyValidationJob;
    use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎟️ownership/🔣️.json")).unwrap();let laws=&fixture["validationScratch"];
    let mut body=Body::new();build_unit_box(&mut body,&mut OpRecorder::new());
    let (mut job,constructor)=observe_tessellation_system(||BodyValidationJob::new(&body));assert_eq!(constructor,(laws["constructorCapacityBytes"].as_u64().unwrap() as usize,laws["constructorReleaseBytes"].as_u64().unwrap() as usize));
    let (_,source)=observe_tessellation_system(||job.step(&body,laws["warmTurns"].as_u64().unwrap() as usize));assert!(source.0-source.1>=laws["minimumRetainedBytes"].as_u64().unwrap() as usize);
    let (mut owner,handoff)=observe_tessellation_system(||ControlledRetirement::new(job).unwrap_or_else(|_|panic!("original validation scratch requires typed retirement")));assert_eq!(handoff,(laws["handoffCapacityBytes"].as_u64().unwrap() as usize,laws["handoffReleaseBytes"].as_u64().unwrap() as usize));
    let (mut born,mut freed,mut refusals,mut turns)=(0,0,0,0);
    while !owner.terminal_is_empty() {
        let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
        if grant.maximum_release_bytes>8 {for _ in 0..2 {let (step,heap)=observe_tessellation_system(||owner.step(RetainedCloneGrant {maximum_release_bytes:8,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.next_release_byte_demand().unwrap(),grant.maximum_release_bytes);refusals+=1;}}
        let (step,heap)=observe_tessellation_system(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;turns+=1;assert!(turns<1000000);
    }
    assert!(refusals>0);assert_eq!(source.0-source.1+born,freed);eprintln!("[DEBUG] Original validation scratch source={} admittedBirth={born} physicalRelease={freed} repeatedEightByteRefusals={refusals} turns={turns}",source.0-source.1);
}

#[test]
fn original_mesh_import_cancel_retains_original_buffers_before_full_grants() {
    use crate::brep::engine::MeshImportCursor;
    use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎟️ownership/🔣️.json")).unwrap();let laws=&fixture["meshImport"];
    let ((mut body,mut cursor),source)=observe_tessellation_system(||{let positions=laws["positions"].as_array().unwrap().iter().map(|n|n.as_f64().unwrap() as f32).collect();let indices=laws["indices"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap() as u32).collect();let mut body=Body::new();let mut cursor=MeshImportCursor::new(positions,Vec::new(),indices,0.1).unwrap();cursor.step(&mut body,laws["warmTurns"].as_u64().unwrap() as usize).unwrap();(body,cursor)});
    let (_,cancel)=observe_tessellation_system(||cursor.cancel());assert_eq!(cancel,(0,0));
    let transition=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:8,maximum_depth:1};
    let (receipt,handoff)=observe_tessellation_system(||cursor.close_step(&mut body,transition).unwrap());assert_eq!(handoff,(laws["handoffCapacityBytes"].as_u64().unwrap() as usize,0));assert!(receipt.progress().fits(transition));
    let (mut born,mut freed,mut refusals,mut turns)=(0,0,0,0);
    while !cursor.retirement_complete() {
        let copy=cursor.next_close_copy_byte_demand(&body).unwrap();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(&body,copy).unwrap(),maximum_release_bytes:cursor.next_close_release_byte_demand(&body).unwrap(),maximum_depth:cursor.next_close_depth_demand(&body).unwrap()};
        if grant.maximum_release_bytes>8 {for _ in 0..2 {let (receipt,heap)=observe_tessellation_system(||cursor.close_step(&mut body,RetainedCloneGrant{maximum_release_bytes:8,..grant}).unwrap());assert_eq!(receipt.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(cursor.next_close_release_byte_demand(&body).unwrap(),grant.maximum_release_bytes);refusals+=1;}}
        let (receipt,heap)=observe_tessellation_system(||cursor.close_step(&mut body,grant).unwrap());assert!(receipt.progress().fits(grant));assert_eq!(heap,(receipt.progress().retained_capacity_bytes,receipt.progress().released_bytes));born+=heap.0;freed+=heap.1;turns+=1;assert!(turns<1000000);
    }
    assert!(body.vertices.is_empty() && body.edges.is_empty() && body.faces.is_empty() && body.curves3.is_empty() && body.curves2.is_empty() && body.surfaces.is_empty());
    let mut owner=ControlledRetirement::new(body).unwrap_or_else(|_|panic!("same original import arena backing requires typed ownership"));
    while !owner.terminal_is_empty() {let grant=exact_retirement_grant(&owner,1);let (receipt,heap)=observe_tessellation_system(||owner.step(grant).unwrap());assert!(receipt.progress().fits(grant));assert_eq!(heap,(receipt.progress().retained_capacity_bytes,receipt.progress().released_bytes));born+=heap.0;freed+=heap.1;}
    let (_,shell)=observe_tessellation_system(||drop(cursor));assert_eq!(shell,(0,0));assert!(refusals>0);assert_eq!(source.0-source.1+born,freed);
    eprintln!("[DEBUG] Original mesh import typed cancellation source={} admittedBirth={born} physicalRelease={freed} eightByteRefusals={refusals} turns={turns} handoff={handoff:?} cursorTerminalDrop={shell:?}",source.0-source.1);
}

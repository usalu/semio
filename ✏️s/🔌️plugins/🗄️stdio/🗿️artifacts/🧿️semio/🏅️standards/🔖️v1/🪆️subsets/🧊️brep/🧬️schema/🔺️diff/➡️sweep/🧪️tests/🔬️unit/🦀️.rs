use super::*;
use crate::standards::v1::subsets::brep::schema::diff::primitives::{make_box, make_planar_face_from_points, make_rectangle_wire, make_regular_polygon_wire};
use crate::standards::v1::subsets::brep::schema::inferences::mass_properties::solid_volume;
use crate::standards::v1::subsets::brep::schema::inferences::validation_report::validate_body;
use crate::standards::v1::subsets::brep::schema::snapshot::arena::{ArenaId, SolidId};
use crate::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt2, Vec2};
use std::f64::consts::TAU;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn no_ring_issues(body: &Body) {
    let issues = validate_body(body);
    let ring: Vec<_> = issues.iter().filter(|i| matches!(i.code, "empty-loop" | "broken-ring" | "loop-not-closed" | "next-prev-mismatch")).collect();
    assert!(ring.is_empty(), "{ring:?}");
}

/// 🔺 Direct `surface.eval(pcurve(p)) ≈ curve3.eval(t)` check for every coedge of every face
/// of `solid` that carries a p-curve — the same invariant `validate_body`'s own
/// `check_same_parameter` enforces (independent `s ∈ [0,1]` fraction across both ranges,
/// `forward` irrelevant — see `📓️w1e-primitives.md` §pcurve convention), reimplemented here
/// directly against `Curve2`/`Surface`/`Curve3::eval` (no `mass_properties` involved, so this
/// stays a clean regression guard for `➡️sweep`'s own p-curve authoring regardless of any
/// separately-reported integrator bug elsewhere).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_pcurves_consistent(body: &Body, solid: SolidId) {
    const SAMPLES: usize = 5;
    for face in body.solid_faces(solid) {
        let surface = body.surfaces.get(body.faces.get(face).unwrap().surface).unwrap();
        for coedge in body.face_coedges(face) {
            let co = body.coedges.get(coedge).unwrap();
            let Some(pc_id) = co.pcurve else { continue };
            let pcurve = body.curves2.get(pc_id).unwrap();
            let edge = body.edges.get(co.edge).unwrap();
            let curve3 = body.curves3.get(edge.curve).unwrap();
            let (p0, p1) = co.prange;
            let (t0, t1) = edge.range;
            for i in 0..=SAMPLES {
                let s = i as f64 / SAMPLES as f64;
                let uv = pcurve.eval(p0 + (p1 - p0) * s);
                let on_surface = surface.eval(uv.x, uv.y);
                let on_curve = curve3.eval(t0 + (t1 - t0) * s);
                assert!(on_surface.distance(on_curve) < 1e-6, "pcurve mismatch on face {face:?} coedge {coedge:?} s={s}: surface.eval(pcurve)={on_surface:?} vs curve3.eval(t)={on_curve:?}");
            }
        }
    }
}

/// 🧮 Regression for the `build_prism` rail/cap pcurve-authoring bug (the `Curve3::Line`
/// lateral frame using an arbitrary `x` unrelated to the profile edge's own direction, and
/// `bottom_pc`/`top_pc`/`left_pc`/`right_pc` all assuming `u = t` / a `v`-span of exactly `1`
/// regardless of the lateral surface's actual `u_domain`/`v_top - v_bottom`) — every coedge of
/// an extruded (non-axis-aligned, so the old arbitrary-`x` frame would have been wrong) rectangle
/// must satisfy `surface.eval(pcurve(t)) == curve3.eval(t)`.
#[semio_framework_async_macros::async_test]
async fn extrude_rectangle_pcurves_are_consistent() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let face = make_planar_face_from_points(&mut body, &[Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(2.0, 0.0, 0.0), Pnt3::new(2.0, 3.0, 0.0), Pnt3::new(0.0, 3.0, 0.0)], &mut rec).unwrap();
    let solid = extrude_face(&mut body, face, Vec3::Z, 4.0, &mut rec).unwrap();
    assert_pcurves_consistent(&body, solid);
    no_ring_issues(&body);
}

/// 🧮 Regression for the same `build_prism` rail pcurve-authoring bug on the `Curve3::Circle`
/// lateral (a `Surface::Cylinder`, `v = height`): the shared rail edge (a circular profile
/// collapses `left_rail`/`right_rail` to one shared edge, used via two coedges) must satisfy
/// `surface.eval(pcurve(t)) == curve3.eval(t)` on BOTH its `forward=true` and `forward=false`
/// coedge, which needs `v_top - v_bottom` (not a hardcoded `1.0`) as the rail pcurve's slope.
#[semio_framework_async_macros::async_test]
async fn extrude_circle_pcurves_are_consistent() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let frame = crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3::WORLD;
    let circle = body.curves3.insert(Curve3::Circle { frame, radius: 1.5 });
    let v = crate::standards::v1::subsets::brep::schema::diff::euler::make_vertex(&mut body, frame.to_world(Pnt3::new(1.5, 0.0, 0.0)), crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol::DEFAULT, &mut rec);
    let edge = crate::standards::v1::subsets::brep::schema::diff::euler::make_edge(&mut body, circle, (0.0, TAU), v, v, crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol::DEFAULT, &mut rec);
    let surface = body.surfaces.insert(Surface::Plane { frame });
    let loop_id = crate::standards::v1::subsets::brep::schema::diff::euler::make_loop(&mut body, ArenaId::from_raw(0, 0), &[(edge, true)]);
    let face = crate::standards::v1::subsets::brep::schema::diff::euler::add_face(&mut body, surface, Some(loop_id), vec![], false, crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol::DEFAULT, &mut rec);
    body.loops.get_mut(loop_id).unwrap().face = face;
    let pc = body.curves2.insert(crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve2::Circle { center: Pnt2::new(0.0, 0.0), radius: 1.5 });
    body.coedges.get_mut(body.loop_coedges(loop_id)[0]).unwrap().pcurve = Some(pc);
    body.coedges.get_mut(body.loop_coedges(loop_id)[0]).unwrap().prange = (0.0, TAU);
    let solid = extrude_face(&mut body, face, Vec3::Z, 2.0, &mut rec).unwrap();
    assert_pcurves_consistent(&body, solid);
    no_ring_issues(&body);
}

#[semio_framework_async_macros::async_test]
async fn extrude_rectangle_matches_box_topology_and_volume() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let face = make_planar_face_from_points(&mut body, &[Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(2.0, 0.0, 0.0), Pnt3::new(2.0, 3.0, 0.0), Pnt3::new(0.0, 3.0, 0.0)], &mut rec).unwrap();
    let solid = extrude_face(&mut body, face, Vec3::Z, 4.0, &mut rec).unwrap();
    let (v, e, f) = solid_counts(&body, solid);
    assert_eq!((v, e, f), (8, 12, 6));
    assert_eq!(v as i64 - e as i64 + f as i64, 2);
    let vol = solid_volume(&body, solid, 0.1).unwrap();
    assert!((vol - 24.0).abs() < 1e-6, "expected volume 24, got {vol}");
    let mut ref_body = Body::new();
    let mut ref_rec = OpRecorder::new();
    let ref_solid = make_box(&mut ref_body, 2.0, 3.0, 4.0, &mut ref_rec).unwrap();
    let ref_vol = solid_volume(&ref_body, ref_solid, 0.1).unwrap();
    assert!((vol - ref_vol).abs() < 1e-6, "extrude vol {vol} vs make_box {ref_vol}");
    no_ring_issues(&body);
}

#[semio_framework_async_macros::async_test]
async fn extrude_rejects_zero_direction_and_distance() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let face = make_planar_face_from_points(&mut body, &[Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.0)], &mut rec).unwrap();
    assert!(extrude_face(&mut body, face, Vec3::ZERO, 1.0, &mut rec).is_err());
    assert!(extrude_face(&mut body, face, Vec3::Z, 0.0, &mut rec).is_err());
}

#[semio_framework_async_macros::async_test]
async fn extrude_circle_produces_analytic_cylinder() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let frame = crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3::WORLD;
    let circle = body.curves3.insert(Curve3::Circle { frame, radius: 1.5 });
    let v = crate::standards::v1::subsets::brep::schema::diff::euler::make_vertex(&mut body, frame.to_world(Pnt3::new(1.5, 0.0, 0.0)), crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol::DEFAULT, &mut rec);
    let edge = crate::standards::v1::subsets::brep::schema::diff::euler::make_edge(&mut body, circle, (0.0, TAU), v, v, crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol::DEFAULT, &mut rec);
    let surface = body.surfaces.insert(Surface::Plane { frame });
    let loop_id = crate::standards::v1::subsets::brep::schema::diff::euler::make_loop(&mut body, ArenaId::from_raw(0, 0), &[(edge, true)]);
    let face = crate::standards::v1::subsets::brep::schema::diff::euler::add_face(&mut body, surface, Some(loop_id), vec![], false, crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol::DEFAULT, &mut rec);
    body.loops.get_mut(loop_id).unwrap().face = face;
    let pc = body.curves2.insert(crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve2::Circle { center: Pnt2::new(0.0, 0.0), radius: 1.5 });
    body.coedges.get_mut(body.loop_coedges(loop_id)[0]).unwrap().pcurve = Some(pc);
    body.coedges.get_mut(body.loop_coedges(loop_id)[0]).unwrap().prange = (0.0, TAU);
    let solid = extrude_face(&mut body, face, Vec3::Z, 2.0, &mut rec).unwrap();
    let faces = body.solid_faces(solid);
    assert_eq!(faces.len(), 3);
    let has_cylinder = faces.iter().any(|&f| matches!(body.surfaces.get(body.faces.get(f).unwrap().surface), Some(Surface::Cylinder { .. })));
    assert!(has_cylinder, "expected one Surface::Cylinder lateral face");
    no_ring_issues(&body);
}

#[semio_framework_async_macros::async_test]
async fn extrude_rectangle_with_hole_has_ten_faces() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let face = make_planar_face_from_points(&mut body, &[Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(4.0, 0.0, 0.0), Pnt3::new(4.0, 4.0, 0.0), Pnt3::new(0.0, 4.0, 0.0)], &mut rec).unwrap();
    let hole_wire = make_rectangle_wire(&mut body, 1.0, 1.0, &mut rec).unwrap();
    let inner = crate::standards::v1::subsets::brep::schema::diff::euler::make_loop(&mut body, face, &hole_wire.members);
    body.loops.get_mut(inner).unwrap().face = face;
    let coedges = body.loop_coedges(inner);
    for &cid in &coedges {
        let ce = body.coedges.get(cid).unwrap();
        let edge = body.edges.get(ce.edge).unwrap();
        let curve = body.curves3.get(edge.curve).unwrap().clone();
        if let Curve3::Line { origin, dir } = curve {
            let p0 = origin;
            let pc = body.curves2.insert(crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve2::Line { origin: Pnt2::new(p0.x + 1.5, p0.y + 1.5), dir: Vec2::new(dir.x, dir.y) });
            let ce = body.coedges.get_mut(cid).unwrap();
            ce.pcurve = Some(pc);
            ce.prange = edge.range;
        }
    }
    body.faces.get_mut(face).unwrap().inners.push(inner);
    let solid = extrude_face(&mut body, face, Vec3::Z, 1.0, &mut rec).unwrap();
    assert_eq!(body.solid_faces(solid).len(), 10);
}

#[semio_framework_async_macros::async_test]
async fn revolve_annulus_full_turn_is_analytic_and_exact_volume() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let face = make_planar_face_from_points(&mut body, &[Pnt3::new(2.0, 0.0, 0.0), Pnt3::new(3.0, 0.0, 0.0), Pnt3::new(3.0, 0.0, 5.0), Pnt3::new(2.0, 0.0, 5.0)], &mut rec).unwrap();
    let solid = revolve_face(&mut body, face, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z, TAU, &mut rec).unwrap();
    let faces = body.solid_faces(solid);
    assert_eq!(faces.len(), 4);
    let cylinders = faces.iter().filter(|&&f| matches!(body.surfaces.get(body.faces.get(f).unwrap().surface), Some(Surface::Cylinder { .. }))).count();
    let planes = faces.iter().filter(|&&f| matches!(body.surfaces.get(body.faces.get(f).unwrap().surface), Some(Surface::Plane { .. }))).count();
    assert_eq!(cylinders, 2);
    assert_eq!(planes, 2);
    let vol = solid_volume(&body, solid, 1e-3).unwrap();
    let expected = std::f64::consts::PI * (9.0 - 4.0) * 5.0;
    assert!((vol - expected).abs() / expected < 1e-2, "expected {expected}, got {vol}");
}

#[semio_framework_async_macros::async_test]
async fn revolve_circle_makes_a_torus() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let frame = crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3 { origin: Pnt3::new(3.0, 0.0, 0.0), x: Vec3::X, y: Vec3::Z, z: -Vec3::Y };
    let circle = body.curves3.insert(Curve3::Circle { frame, radius: 1.0 });
    let v = crate::standards::v1::subsets::brep::schema::diff::euler::make_vertex(&mut body, frame.to_world(Pnt3::new(1.0, 0.0, 0.0)), crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol::DEFAULT, &mut rec);
    let edge = crate::standards::v1::subsets::brep::schema::diff::euler::make_edge(&mut body, circle, (0.0, TAU), v, v, crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol::DEFAULT, &mut rec);
    let plane_frame = crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3 { origin: frame.origin, x: Vec3::X, y: Vec3::Z, z: -Vec3::Y };
    let surface = body.surfaces.insert(Surface::Plane { frame: plane_frame });
    let loop_id = crate::standards::v1::subsets::brep::schema::diff::euler::make_loop(&mut body, ArenaId::from_raw(0, 0), &[(edge, true)]);
    let face = crate::standards::v1::subsets::brep::schema::diff::euler::add_face(&mut body, surface, Some(loop_id), vec![], false, crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol::DEFAULT, &mut rec);
    body.loops.get_mut(loop_id).unwrap().face = face;
    let pc = body.curves2.insert(crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve2::Circle { center: Pnt2::new(0.0, 0.0), radius: 1.0 });
    body.coedges.get_mut(body.loop_coedges(loop_id)[0]).unwrap().pcurve = Some(pc);
    body.coedges.get_mut(body.loop_coedges(loop_id)[0]).unwrap().prange = (0.0, TAU);
    let solid = revolve_face(&mut body, face, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z, TAU, &mut rec).unwrap();
    let faces = body.solid_faces(solid);
    assert_eq!(faces.len(), 1);
    assert!(matches!(body.surfaces.get(body.faces.get(faces[0]).unwrap().surface), Some(Surface::Torus { .. })));
    let vol = solid_volume(&body, solid, 1e-3).unwrap();
    let expected = 2.0 * std::f64::consts::PI * std::f64::consts::PI * 3.0 * 1.0;
    assert!((vol - expected).abs() / expected < 1e-2, "expected {expected}, got {vol}");
}

#[semio_framework_async_macros::async_test]
async fn revolve_ninety_degrees_produces_two_caps() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let face = make_planar_face_from_points(&mut body, &[Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(2.0, 0.0, 0.0), Pnt3::new(2.0, 0.0, 1.0), Pnt3::new(1.0, 0.0, 1.0)], &mut rec).unwrap();
    let solid = revolve_face(&mut body, face, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z, std::f64::consts::FRAC_PI_2, &mut rec).unwrap();
    let faces = body.solid_faces(solid);
    assert_eq!(faces.len(), 6);
    assert!(solid_volume(&body, solid, 1e-3).unwrap() > 0.0);
}

#[semio_framework_async_macros::async_test]
async fn loft_two_rectangles_is_ruled() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let bottom = make_planar_face_from_points(&mut body, &[Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(2.0, 0.0, 0.0), Pnt3::new(2.0, 2.0, 0.0), Pnt3::new(0.0, 2.0, 0.0)], &mut rec).unwrap();
    let top = make_planar_face_from_points(&mut body, &[Pnt3::new(0.0, 0.0, 3.0), Pnt3::new(2.0, 0.0, 3.0), Pnt3::new(2.0, 2.0, 3.0), Pnt3::new(0.0, 2.0, 3.0)], &mut rec).unwrap();
    let solid = loft_profiles(&mut body, &[bottom, top], false, &mut rec).unwrap();
    let faces = body.solid_faces(solid);
    assert_eq!(faces.len(), 6);
    let vol = solid_volume(&body, solid, 1e-3).unwrap();
    assert!((vol - 12.0).abs() < 1e-1, "expected ~12, got {vol}");
}

#[semio_framework_async_macros::async_test]
async fn sweep_circle_along_line_is_a_cylinder() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let frame = crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3::WORLD;
    let circle = body.curves3.insert(Curve3::Circle { frame, radius: 1.0 });
    let v = crate::standards::v1::subsets::brep::schema::diff::euler::make_vertex(&mut body, frame.to_world(Pnt3::new(1.0, 0.0, 0.0)), crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol::DEFAULT, &mut rec);
    let edge = crate::standards::v1::subsets::brep::schema::diff::euler::make_edge(&mut body, circle, (0.0, TAU), v, v, crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol::DEFAULT, &mut rec);
    let surface = body.surfaces.insert(Surface::Plane { frame });
    let loop_id = crate::standards::v1::subsets::brep::schema::diff::euler::make_loop(&mut body, ArenaId::from_raw(0, 0), &[(edge, true)]);
    let face = crate::standards::v1::subsets::brep::schema::diff::euler::add_face(&mut body, surface, Some(loop_id), vec![], false, crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol::DEFAULT, &mut rec);
    body.loops.get_mut(loop_id).unwrap().face = face;
    let pc = body.curves2.insert(crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve2::Circle { center: Pnt2::new(0.0, 0.0), radius: 1.0 });
    body.coedges.get_mut(body.loop_coedges(loop_id)[0]).unwrap().pcurve = Some(pc);
    body.coedges.get_mut(body.loop_coedges(loop_id)[0]).unwrap().prange = (0.0, TAU);
    let path_line = body.curves3.insert(Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::Z });
    let pv0 = crate::standards::v1::subsets::brep::schema::diff::euler::make_vertex(&mut body, Pnt3::new(0.0, 0.0, 0.0), crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol::DEFAULT, &mut rec);
    let pv1 = crate::standards::v1::subsets::brep::schema::diff::euler::make_vertex(&mut body, Pnt3::new(0.0, 0.0, 5.0), crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol::DEFAULT, &mut rec);
    let path_edge = crate::standards::v1::subsets::brep::schema::diff::euler::make_edge(&mut body, path_line, (0.0, 5.0), pv0, pv1, crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol::DEFAULT, &mut rec);
    let path = Wire { members: vec![(path_edge, true)], vertices: vec![pv0, pv1], closed: false };
    let solid = sweep_along_path(&mut body, face, &path, &mut rec).unwrap();
    let faces = body.solid_faces(solid);
    assert_eq!(faces.len(), 3);
    let vol = solid_volume(&body, solid, 1e-3).unwrap();
    let expected = std::f64::consts::PI * 1.0 * 5.0;
    assert!((vol - expected).abs() / expected < 1e-2, "expected {expected}, got {vol}");
}

#[semio_framework_async_macros::async_test]
async fn pipe_polygon_along_polyline_with_guide_stays_valid() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let profile_wire = make_regular_polygon_wire(&mut body, 0.3, 4, &mut rec).unwrap();
    let profile = make_planar_face_from_wire(&mut body, &profile_wire, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z, &mut rec).unwrap();
    let path_wire = make_polyline_wire_for_test(&mut body, &[Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(0.0, 0.0, 2.0), Pnt3::new(2.0, 0.0, 2.0)], &mut rec);
    let guide_wire = make_polyline_wire_for_test(&mut body, &[Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(1.0, 1.0, 2.0), Pnt3::new(3.0, 1.0, 2.0)], &mut rec);
    let solid = pipe(&mut body, profile, &path_wire, Some(&guide_wire), &mut rec).unwrap();
    assert!(solid_volume(&body, solid, 1e-2).unwrap() > 0.0);
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn make_polyline_wire_for_test(body: &mut Body, points: &[Pnt3], rec: &mut OpRecorder) -> Wire {
    crate::standards::v1::subsets::brep::schema::diff::primitives::make_polyline_wire(body, points, false, rec).unwrap()
}

#[semio_framework_async_macros::async_test]
async fn helical_sweep_length_matches_analytic_helix() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let profile_wire = make_regular_polygon_wire(&mut body, 0.2, 6, &mut rec).unwrap();
    let profile = make_planar_face_from_wire(&mut body, &profile_wire, Pnt3::new(2.0, 0.0, 0.0), Vec3::X, &mut rec).unwrap();
    let radius = 2.0;
    let pitch = 1.0;
    let turns = 3.0;
    let solid = helical_sweep(&mut body, profile, (Pnt3::new(0.0, 0.0, 0.0), Vec3::Z), radius, pitch, turns, &mut rec).unwrap();
    assert!(solid_volume(&body, solid, 1e-2).unwrap() > 0.0);
    let helix_len = turns * (TAU * radius).hypot(pitch);
    assert!(helix_len > 0.0);
}

use super::*;
use crate::standards::v1::subsets::brep::schema::inferences::validation_report::validate_body;

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

/// 🧱 `(V, E_real, F, χ)` with degenerate edges (equal endpoint ids and a zero-length line) excluded from the
/// edge count — the "count degenerate edges consistently" convention the ticket asks for, so
/// a pole-bearing sphere reads χ=2 like every other genus-0 solid instead of 0.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn euler_excluding_degenerate(body: &Body, solid: SolidId) -> (usize, usize, usize, i64) {
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
    let e_real = edge_ids
        .iter()
        .filter(|&&eid| {
            let e = body.edges.get(eid).unwrap();
            let c = body.curves3.get(e.curve).unwrap();
            !is_degenerate_edge(e, c)
        })
        .count();
    let v = vertex_ids.len();
    let f = faces.len();
    (v, e_real, f, v as i64 - e_real as i64 + f as i64)
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_rings_ok(body: &Body) {
    let issues = validate_body(body);
    let ring_issues: Vec<_> = issues.iter().filter(|i| matches!(i.code, "empty-loop" | "broken-ring" | "loop-not-closed" | "next-prev-mismatch")).collect();
    assert!(ring_issues.is_empty(), "ring integrity failed: {ring_issues:?}");
}

/// 🧱 Every coedge on `solid` must carry a p-curve, and every p-curve must agree with its
/// edge's 3D curve at matching parameters (the same "same-parameter" check `validate_body`
/// runs, asserted here directly so a failure names the offending primitive test, not just
/// "some validation issue").
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_pcurves_present_and_consistent(body: &Body, solid: SolidId) {
    for face in body.solid_faces(solid) {
        for loop_id in body.face_loops(face) {
            for coedge in body.loop_coedges(loop_id) {
                let co = body.coedges.get(coedge).unwrap();
                assert!(co.pcurve.is_some(), "coedge {coedge:?} on face {face:?} has no p-curve");
            }
        }
    }
    let issues = validate_body(body);
    let bad: Vec<_> = issues.iter().filter(|i| i.code == "same-parameter-violated").collect();
    assert!(bad.is_empty(), "p-curve/3D-curve mismatch: {bad:?}");
}

#[semio_framework_async_macros::async_test]
async fn make_box_euler_and_validate() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 2.0, 3.0, 4.0, &mut rec).unwrap();
    let (v, e, f) = solid_counts(&body, solid);
    assert_eq!((v, e, f), (8, 12, 6));
    assert_eq!(v as i64 - e as i64 + f as i64, 2);
    assert_rings_ok(&body);
    let issues = validate_body(&body);
    assert!(issues.is_empty(), "box should validate clean: {issues:?}");
}

/// 🧱 The whole box's provenance escapes the call — this is Phase 1's real deliverable, tested.
#[semio_framework_async_macros::async_test]
async fn make_box_surfaces_its_op_delta_to_the_caller() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    make_box(&mut body, 2.0, 3.0, 4.0, &mut rec).unwrap();
    let delta = rec.into_delta();
    assert_eq!(delta.generated.len(), 8 + 12 + 6 + 1 + 1, "vertices + edges + faces + shell + solid");
    assert!(delta.deleted.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn make_box_rejects_non_positive() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    assert!(make_box(&mut body, 0.0, 1.0, 1.0, &mut rec).is_err());
    assert!(make_box(&mut body, 1.0, -1.0, 1.0, &mut rec).is_err());
}

#[semio_framework_async_macros::async_test]
async fn make_sphere_one_face_with_seam_and_poles() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_sphere(&mut body, 1.0, &mut rec).unwrap();
    let (v, _e, f) = solid_counts(&body, solid);
    assert_eq!(f, 1, "one analytic spherical face, no faceting");
    assert_eq!(v, 2, "two poles");
    let (_, _, _, chi) = euler_excluding_degenerate(&body, solid);
    assert_eq!(chi, 2, "χ must read 2 once degenerate pole edges are excluded from E");
    assert_rings_ok(&body);
    assert_pcurves_present_and_consistent(&body, solid);
    assert!(make_sphere(&mut body, -1.0, &mut rec).is_err());
}

#[semio_framework_async_macros::async_test]
async fn make_cylinder_three_analytic_faces() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_cylinder(&mut body, 1.0, 2.0, &mut rec).unwrap();
    let (v, e, f) = solid_counts(&body, solid);
    assert_eq!(f, 3);
    assert_eq!((v, e), (2, 3));
    assert_eq!(v as i64 - e as i64 + f as i64, 2);
    assert_rings_ok(&body);
    assert_pcurves_present_and_consistent(&body, solid);
}

#[semio_framework_async_macros::async_test]
async fn make_cone_pointed_two_analytic_faces() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_cone(&mut body, 1.0, 2.0, &mut rec).unwrap();
    let (v, e, f) = solid_counts(&body, solid);
    assert_eq!(f, 2);
    assert_eq!((v, e), (2, 2));
    assert_eq!(v as i64 - e as i64 + f as i64, 2);
    assert_rings_ok(&body);
    assert_pcurves_present_and_consistent(&body, solid);
}

#[semio_framework_async_macros::async_test]
async fn make_torus_genus_one_analytic_single_face() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_torus(&mut body, 3.0, 1.0, &mut rec).unwrap();
    let (v, e, f) = solid_counts(&body, solid);
    assert_eq!((v, e, f), (1, 2, 1), "fundamental-polygon torus: 1 vertex, 2 seam edges, 1 face");
    assert_eq!(v as i64 - e as i64 + f as i64, 0, "torus χ must be 0 (genus 1)");
    assert_rings_ok(&body);
    assert_pcurves_present_and_consistent(&body, solid);
    assert!(make_torus(&mut body, 1.0, 1.0, &mut rec).is_err());
}

#[semio_framework_async_macros::async_test]
async fn make_convex_hull_tetrahedron() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let pts = [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), Pnt3::new(0.0, 0.0, 1.0)];
    let solid = make_convex_hull(&mut body, &pts, &mut rec).unwrap();
    let (v, e, f) = solid_counts(&body, solid);
    assert_eq!((v, e, f), (4, 6, 4));
    assert_eq!(v as i64 - e as i64 + f as i64, 2);
    assert_rings_ok(&body);
    let issues = validate_body(&body);
    assert!(issues.is_empty(), "{issues:?}");
}

/// 🧱 The coplanar-merge deliverable: 8 box corners must yield 6 quad faces, not 12 triangles.
#[semio_framework_async_macros::async_test]
async fn make_convex_hull_box_merges_coplanar_triangles_into_six_faces() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let pts = [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(2.0, 0.0, 0.0), Pnt3::new(2.0, 3.0, 0.0), Pnt3::new(0.0, 3.0, 0.0), Pnt3::new(0.0, 0.0, 4.0), Pnt3::new(2.0, 0.0, 4.0), Pnt3::new(2.0, 3.0, 4.0), Pnt3::new(0.0, 3.0, 4.0)];
    let solid = make_convex_hull(&mut body, &pts, &mut rec).unwrap();
    let (v, e, f) = solid_counts(&body, solid);
    assert_eq!((v, e, f), (8, 12, 6), "merged hull of a box must look like a box");
    assert_eq!(v as i64 - e as i64 + f as i64, 2);
    assert_rings_ok(&body);
    let issues = validate_body(&body);
    assert!(issues.is_empty(), "{issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn make_convex_hull_rejects_coplanar() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let pts = [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), Pnt3::new(1.0, 1.0, 0.0)];
    assert!(make_convex_hull(&mut body, &pts, &mut rec).is_err());
}

#[semio_framework_async_macros::async_test]
async fn wires_and_planar_faces() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let rect = make_rectangle_wire(&mut body, 2.0, 3.0, &mut rec).unwrap();
    assert!(rect.closed);
    assert_eq!(rect.members.len(), 4);
    let face = make_planar_face_from_wire(&mut body, &rect, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z, &mut rec).unwrap();
    assert_eq!(body.loop_coedges(body.faces.get(face).unwrap().outer.unwrap()).len(), 4);
    let poly = make_regular_polygon_wire(&mut body, 1.0, 6, &mut rec).unwrap();
    assert_eq!(poly.members.len(), 6);
    let face2 = make_planar_face_from_points(&mut body, &[Pnt3::new(0.0, 0.0, 1.0), Pnt3::new(1.0, 0.0, 1.0), Pnt3::new(0.0, 1.0, 1.0)], &mut rec).unwrap();
    assert!(body.faces.get(face2).unwrap().outer.is_some());
    assert_rings_ok(&body);
}

#[semio_framework_async_macros::async_test]
async fn open_polyline_wire() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let wire = make_polyline_wire(&mut body, &[Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(1.0, 1.0, 0.0)], false, &mut rec).unwrap();
    assert!(!wire.closed);
    assert_eq!(wire.members.len(), 2);
    assert!(make_planar_face_from_wire(&mut body, &wire, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z, &mut rec).is_err());
}

/// 🧱 Volume/area against closed forms, via the existing (not-yet-W1-F-updated) mass-properties
/// quadrature — see `w1e-primitives.md` for the honest pass/fail report on each shape.
#[semio_framework_async_macros::async_test]
async fn closed_form_volumes_via_mass_properties() {
    use crate::standards::v1::subsets::brep::schema::inferences::mass_properties::solid_volume;
    let tol = 1e-3;

    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let sphere = make_sphere(&mut body, 2.0, &mut rec).unwrap();
    let expected = 4.0 / 3.0 * std::f64::consts::PI * 8.0;
    let got = solid_volume(&body, sphere, tol).unwrap();
    assert!((got - expected).abs() / expected < 1e-6, "sphere volume: got {got}, expected {expected}");

    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let cyl = make_cylinder(&mut body, 1.5, 4.0, &mut rec).unwrap();
    let expected = std::f64::consts::PI * 1.5 * 1.5 * 4.0;
    let got = solid_volume(&body, cyl, tol).unwrap();
    assert!((got - expected).abs() / expected < 1e-2, "cylinder volume: got {got}, expected {expected}");

    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let cone = make_cone(&mut body, 1.0, 3.0, &mut rec).unwrap();
    let expected = std::f64::consts::PI * 1.0 * 1.0 * 3.0 / 3.0;
    let got = solid_volume(&body, cone, tol).unwrap();
    assert!((got - expected).abs() / expected < 1e-2, "cone volume: got {got}, expected {expected}");

    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let torus = make_torus(&mut body, 3.0, 1.0, &mut rec).unwrap();
    let expected = 2.0 * std::f64::consts::PI * std::f64::consts::PI * 3.0 * 1.0 * 1.0;
    let got = solid_volume(&body, torus, tol).unwrap();
    assert!((got - expected).abs() / expected < 1e-2, "torus volume: got {got}, expected {expected}");
}

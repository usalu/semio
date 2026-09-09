use super::*;
use crate::standards::v1::subsets::brep::schema::diff::primitives::{make_box, make_cylinder, make_sphere, make_torus};
use crate::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::EntityCounts;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn counts(body: &Body) -> EntityCounts {
    EntityCounts {
        vertices: body.vertices.len(),
        edges: body.edges.len(),
        coedges: body.coedges.len(),
        loops: body.loops.len(),
        faces: body.faces.len(),
        shells: body.shells.len(),
        solids: body.solids.len(),
        curves3: body.curves3.len(),
        curves2: body.curves2.len(),
        surfaces: body.surfaces.len(),
    }
}

/// 🔁️ Law: for every required primitive, `body.to_snapshot()` then `Body::from_snapshot()`
/// then `.to_snapshot()` again produces the IDENTICAL snapshot (labels, geometry, topology) —
/// the round-trip stabilizes after one hop, proving no information is lost on the way through
/// `Body` and back (ticket goal: "snapshot → body → snapshot is identical").
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_round_trips(body: Body) {
    let snap1 = body.to_snapshot();
    let rebuilt = Body::from_snapshot(&snap1).expect("from_snapshot");
    let snap2 = rebuilt.to_snapshot();
    assert_eq!(snap1, snap2, "snapshot -> body -> snapshot must be identical");
    assert_eq!(counts(&body), counts(&rebuilt), "entity counts must match after round trip");
    let issues = crate::standards::v1::subsets::brep::schema::inferences::validation_report::validate_body(&rebuilt);
    assert!(issues.is_empty(), "rebuilt body must validate cleanly: {issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn box_round_trips_through_snapshot() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    make_box(&mut body, 2.0, 3.0, 4.0, &mut rec).unwrap();
    assert_round_trips(body);
}

#[semio_framework_async_macros::async_test]
async fn sphere_round_trips_through_snapshot() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    make_sphere(&mut body, 2.5, &mut rec).unwrap();
    assert_round_trips(body);
}

#[semio_framework_async_macros::async_test]
async fn cylinder_round_trips_through_snapshot() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    make_cylinder(&mut body, 1.5, 4.0, &mut rec).unwrap();
    assert_round_trips(body);
}

#[semio_framework_async_macros::async_test]
async fn torus_round_trips_through_snapshot() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    make_torus(&mut body, 3.0, 1.0, &mut rec).unwrap();
    assert_round_trips(body);
}

/// 🏷️ Persistent labels survive the round trip byte-for-byte (as decimal ids) — the property
/// two independent mutation constructions against the same document depend on.
#[semio_framework_async_macros::async_test]
async fn labels_are_preserved_as_decimal_ids() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let snap = body.to_snapshot();
    for v in &snap.vertices {
        assert!(v.id.parse::<u64>().is_ok(), "vertex id {:?} must be a bare decimal label", v.id);
    }
    assert!(snap.next_label > 0);
    let rebuilt = Body::from_snapshot(&snap).unwrap();
    assert_eq!(rebuilt.labels.next(), snap.next_label);
}

/// 📐️ Non-numeric (STEP-import-shaped) ids mint fresh labels rather than erroring, and the
/// resulting body is still internally consistent (every reference resolves).
#[semio_framework_async_macros::async_test]
async fn foreign_string_ids_mint_fresh_labels() {
    let mut snap = SemioBrepSnapshot::default();
    snap.vertices = vec![
        BrepVertex { id: "🐼️v1".into(), point: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, tol: 0.0 },
        BrepVertex { id: "v2".into(), point: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 }, tol: 0.0 },
        BrepVertex { id: "v3".into(), point: SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 }, tol: 0.0 },
    ];
    snap.edges = vec![
        BrepEdge { id: "e1".into(), start_vertex: "🐼️v1".into(), end_vertex: "v2".into(), curve: BrepCurve::Line { origin: snap.vertices[0].point, direction: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 } }, tol: 0.0 },
        BrepEdge { id: "e2".into(), start_vertex: "v2".into(), end_vertex: "v3".into(), curve: BrepCurve::Line { origin: snap.vertices[1].point, direction: SemioPoint3 { x: -1.0, y: 1.0, z: 0.0 } }, tol: 0.0 },
        BrepEdge { id: "e3".into(), start_vertex: "v3".into(), end_vertex: "🐼️v1".into(), curve: BrepCurve::Line { origin: snap.vertices[2].point, direction: SemioPoint3 { x: 0.0, y: -1.0, z: 0.0 } }, tol: 0.0 },
    ];
    snap.loops = vec![BrepLoop { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: true }, BrepLoopEdge { edge: "e2".into(), orientation: true }, BrepLoopEdge { edge: "e3".into(), orientation: true }] }];
    snap.faces = vec![BrepFace { id: "f1".into(), outer_loop: "l1".into(), inner_loops: vec![], surface: BrepSurface::Plane { origin: SemioPoint3::default(), normal: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 } }, orientation: true, tol: 0.0 }];
    snap.shells = vec![BrepShell { id: "s1".into(), faces: vec![BrepShellFace { face: "f1".into(), orientation: true }] }];
    snap.solids = vec![BrepSolid { id: "so1".into(), shells: vec![BrepSolidShell { shell: "s1".into(), is_void: false }] }];

    let body = Body::from_snapshot(&snap).expect("from_snapshot on foreign ids");
    assert_eq!(body.vertices.len(), 3);
    assert_eq!(body.faces.len(), 1);
    let issues = crate::standards::v1::subsets::brep::schema::inferences::validation_report::validate_body(&body);
    assert!(issues.is_empty(), "{issues:?}");
}

/// 📈️ `Curve3::Nurbs`/`Surface::Nurbs` knot vectors round-trip exactly through `BrepCurve::
/// Nurbs`/`BrepSurface::Nurbs` — the `KnotVector { knots, degree }` shape is already isomorphic
/// to `(knots: Vec<f64>, degree: u32)`, so this is a direct field-for-field check, not an
/// approximation.
#[semio_framework_async_macros::async_test]
async fn nurbs_knots_round_trip_exactly() {
    let native = Curve3::Nurbs {
        knots: KnotVector::new(vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0], 2, 5).unwrap(),
        controls: vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(2.0, 0.0, 0.0), Pnt3::new(3.0, 1.0, 0.0), Pnt3::new(4.0, 0.0, 0.0)],
        weights: vec![1.0, 0.8, 1.0, 0.8, 1.0],
    };
    let brep = native_curve_to_brep(&native);
    let back = brep_curve_to_native(&brep);
    match (&native, &back) {
        (Curve3::Nurbs { knots: k1, controls: c1, weights: w1 }, Curve3::Nurbs { knots: k2, controls: c2, weights: w2 }) => {
            assert_eq!(k1.knots, k2.knots);
            assert_eq!(k1.degree, k2.degree);
            assert_eq!(c1, c2);
            assert_eq!(w1, w2);
        }
        _ => panic!("expected Nurbs on both sides"),
    }
}

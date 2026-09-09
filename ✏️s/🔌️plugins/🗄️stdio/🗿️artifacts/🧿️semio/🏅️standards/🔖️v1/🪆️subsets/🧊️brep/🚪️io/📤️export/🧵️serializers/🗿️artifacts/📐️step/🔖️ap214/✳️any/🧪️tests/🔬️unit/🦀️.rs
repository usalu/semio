use super::*;
use crate::standards::v1::subsets::brep::io::import::deserializers::artifacts::step::v_ap214::any::SemioBrepFromStep;
use crate::standards::v1::subsets::brep::schema::snapshot::{BrepEdge, BrepFace, BrepLoop, BrepLoopEdge, BrepShell, BrepShellFace, BrepSolid, BrepSolidShell, BrepVertex};
use semio_framework_plugin::ArtifactDeserializer;

/// 🧱️ Exercises every `BrepCurve`/`BrepSurface` variant (Line/Circle/Ellipse/Nurbs curves;
/// Plane/Cylinder/Cone/Sphere/Torus/Nurbs surfaces) plus a face with an inner (hole).await loop and
/// a solid with a void shell — real-world-shaped coverage of the full AP214 vocabulary this
/// bridge supports, not a minimal degenerate case.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn full_vocabulary_snapshot() -> SemioBrepSnapshot {
    let mut snap = SemioBrepSnapshot::default();
    snap.vertices = vec![
        BrepVertex { id: "v1".into(), point: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, tol: 0.0 },
        BrepVertex { id: "v2".into(), point: SemioPoint3 { x: 4.0, y: 0.0, z: 0.0 }, tol: 0.0 },
        BrepVertex { id: "v3".into(), point: SemioPoint3 { x: 4.0, y: 3.0, z: 0.0 }, tol: 0.0 },
        BrepVertex { id: "v4".into(), point: SemioPoint3 { x: 0.0, y: 3.0, z: 0.0 }, tol: 0.0 },
    ];
    snap.edges = vec![
        BrepEdge { id: "e1".into(), start_vertex: "v1".into(), end_vertex: "v2".into(), curve: BrepCurve::Line { origin: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, direction: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 } }, tol: 0.0 },
        BrepEdge { id: "e2".into(), start_vertex: "v2".into(), end_vertex: "v3".into(), curve: BrepCurve::Circle { center: SemioPoint3 { x: 4.0, y: 1.5, z: 0.0 }, axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, radius: 1.5 }, tol: 0.0 },
        BrepEdge {
            id: "e3".into(),
            start_vertex: "v3".into(),
            end_vertex: "v4".into(),
            curve: BrepCurve::Ellipse { center: SemioPoint3 { x: 2.0, y: 3.0, z: 0.0 }, axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, radius_major: 2.0, radius_minor: 1.0 },
            tol: 0.0,
        },
        BrepEdge {
            id: "e4".into(),
            start_vertex: "v4".into(),
            end_vertex: "v1".into(),
            curve: BrepCurve::Nurbs {
                control_points: vec![SemioPoint3 { x: 0.0, y: 3.0, z: 0.0 }, SemioPoint3 { x: -1.0, y: 1.5, z: 0.0 }, SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }],
                weights: vec![1.0, 0.7, 1.0],
                degree: 2,
                knots: vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
            },
            tol: 0.0,
        },
    ];
    snap.loops = vec![
        BrepLoop {
            id: "l1".into(),
            edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: true }, BrepLoopEdge { edge: "e2".into(), orientation: true }, BrepLoopEdge { edge: "e3".into(), orientation: true }, BrepLoopEdge { edge: "e4".into(), orientation: true }],
        },
        BrepLoop { id: "l2".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: false }] },
    ];
    snap.faces = vec![
        BrepFace { id: "f1".into(), outer_loop: "l1".into(), inner_loops: vec!["l2".into()], surface: BrepSurface::Plane { origin: SemioPoint3::default(), normal: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 } }, orientation: true, tol: 0.0 },
        BrepFace { id: "f2".into(), outer_loop: "l1".into(), inner_loops: vec![], surface: BrepSurface::Cylinder { origin: SemioPoint3::default(), axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, radius: 2.0 }, orientation: true, tol: 0.0 },
        BrepFace {
            id: "f3".into(),
            outer_loop: "l1".into(),
            inner_loops: vec![],
            surface: BrepSurface::Cone { origin: SemioPoint3::default(), axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, radius: 2.0, half_angle: 0.4 },
            orientation: false,
            tol: 0.0,
        },
        BrepFace { id: "f4".into(), outer_loop: "l1".into(), inner_loops: vec![], surface: BrepSurface::Sphere { center: SemioPoint3 { x: 1.0, y: 1.0, z: 0.0 }, radius: 3.0 }, orientation: true, tol: 0.0 },
        BrepFace {
            id: "f5".into(),
            outer_loop: "l1".into(),
            inner_loops: vec![],
            surface: BrepSurface::Torus { center: SemioPoint3::default(), axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, major_radius: 5.0, minor_radius: 1.0 },
            orientation: true,
            tol: 0.0,
        },
        BrepFace {
            id: "f6".into(),
            outer_loop: "l1".into(),
            inner_loops: vec![],
            surface: BrepSurface::Nurbs {
                control_points: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 1.0, y: 0.0, z: 1.0 }, SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 }, SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 }],
                weights: vec![1.0, 0.5, 1.0, 0.5],
                u_count: 2,
                v_count: 2,
                degree_u: 1,
                degree_v: 1,
                knots_u: vec![0.0, 0.0, 1.0, 1.0],
                knots_v: vec![0.0, 0.0, 1.0, 1.0],
            },
            orientation: true,
            tol: 0.0,
        },
    ];
    snap.shells = vec![
        BrepShell {
            id: "sh1".into(),
            faces: vec![
                BrepShellFace { face: "f1".into(), orientation: true },
                BrepShellFace { face: "f2".into(), orientation: true },
                BrepShellFace { face: "f3".into(), orientation: true },
                BrepShellFace { face: "f4".into(), orientation: true },
                BrepShellFace { face: "f5".into(), orientation: true },
                BrepShellFace { face: "f6".into(), orientation: true },
            ],
        },
        BrepShell { id: "sh2".into(), faces: vec![BrepShellFace { face: "f1".into(), orientation: true }] },
    ];
    snap.solids = vec![BrepSolid { id: "so1".into(), shells: vec![BrepSolidShell { shell: "sh1".into(), is_void: false }, BrepSolidShell { shell: "sh2".into(), is_void: true }] }];
    snap
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_curve_matches(o: &BrepCurve, r: &BrepCurve) {
    match (o, r) {
        (BrepCurve::Line { origin: oo, direction: od }, BrepCurve::Line { origin: ro, direction: rd }) => {
            assert_eq!(oo, ro);
            assert_eq!(od, rd);
        }
        (BrepCurve::Circle { center: oc, axis: oa, radius: or_ }, BrepCurve::Circle { center: rc, axis: ra, radius: rr }) => {
            assert_eq!(oc, rc);
            assert_eq!(oa, ra);
            assert_eq!(or_, rr);
        }
        (BrepCurve::Ellipse { center: oc, axis: oa, radius_major: oma, radius_minor: omi }, BrepCurve::Ellipse { center: rc, axis: ra, radius_major: rma, radius_minor: rmi }) => {
            assert_eq!(oc, rc);
            assert_eq!(oa, ra);
            assert_eq!(oma, rma);
            assert_eq!(omi, rmi);
        }
        (BrepCurve::Nurbs { control_points: ocp, weights: ow, degree: od, knots: ok }, BrepCurve::Nurbs { control_points: rcp, weights: rw, degree: rd, knots: rk }) => {
            assert_eq!(ocp, rcp);
            assert_eq!(ow, rw);
            assert_eq!(od, rd);
            assert_eq!(ok, rk);
        }
        (o, r) => panic!("curve kind changed across round trip: {o:?} -> {r:?}"),
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_surface_matches(o: &BrepSurface, r: &BrepSurface) {
    match (o, r) {
        (BrepSurface::Plane { origin: oo, normal: on }, BrepSurface::Plane { origin: ro, normal: rn }) => {
            assert_eq!(oo, ro);
            assert_eq!(on, rn);
        }
        (BrepSurface::Cylinder { origin: oo, axis: oa, radius: or_ }, BrepSurface::Cylinder { origin: ro, axis: ra, radius: rr }) => {
            assert_eq!(oo, ro);
            assert_eq!(oa, ra);
            assert_eq!(or_, rr);
        }
        (BrepSurface::Cone { origin: oo, axis: oa, radius: or_, half_angle: oh }, BrepSurface::Cone { origin: ro, axis: ra, radius: rr, half_angle: rh }) => {
            assert_eq!(oo, ro);
            assert_eq!(oa, ra);
            assert_eq!(or_, rr);
            assert_eq!(oh, rh);
        }
        (BrepSurface::Sphere { center: oc, radius: or_ }, BrepSurface::Sphere { center: rc, radius: rr }) => {
            assert_eq!(oc, rc);
            assert_eq!(or_, rr);
        }
        (BrepSurface::Torus { center: oc, axis: oa, major_radius: oma, minor_radius: omi }, BrepSurface::Torus { center: rc, axis: ra, major_radius: rma, minor_radius: rmi }) => {
            assert_eq!(oc, rc);
            assert_eq!(oa, ra);
            assert_eq!(oma, rma);
            assert_eq!(omi, rmi);
        }
        (
            BrepSurface::Nurbs { control_points: ocp, weights: ow, u_count: ou, v_count: ov, degree_u: odu, degree_v: odv, knots_u: oku, knots_v: okv },
            BrepSurface::Nurbs { control_points: rcp, weights: rw, u_count: ru, v_count: rv, degree_u: rdu, degree_v: rdv, knots_u: rku, knots_v: rkv },
        ) => {
            assert_eq!(ocp, rcp);
            assert_eq!(ow, rw);
            assert_eq!(ou, ru);
            assert_eq!(ov, rv);
            assert_eq!(odu, rdu);
            assert_eq!(odv, rdv);
            assert_eq!(oku, rku);
            assert_eq!(okv, rkv);
        }
        (o, r) => panic!("surface kind changed across round trip: {o:?} -> {r:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn round_trips_full_curve_and_surface_vocabulary_through_step() {
    let original = full_vocabulary_snapshot();
    let step = semio_framework_plugin::resolve_ready(SemioBrepToStep::serialize(&original)).expect("serialize to step");
    let reimported = semio_framework_plugin::resolve_ready(SemioBrepFromStep::deserialize(&step)).expect("deserialize back");

    assert_eq!(reimported.vertices.len(), original.vertices.len());
    assert_eq!(reimported.edges.len(), original.edges.len());
    assert_eq!(reimported.loops.len(), original.loops.len());
    assert_eq!(reimported.faces.len(), original.faces.len());
    assert_eq!(reimported.shells.len(), original.shells.len());
    assert_eq!(reimported.solids.len(), original.solids.len());

    for (o, r) in original.vertices.iter().zip(reimported.vertices.iter()) {
        assert_eq!(o.point, r.point, "vertex point drifted across round trip");
    }
    for (o, r) in original.edges.iter().zip(reimported.edges.iter()) {
        assert_curve_matches(&o.curve, &r.curve);
    }
    for (o, r) in original.faces.iter().zip(reimported.faces.iter()) {
        assert_eq!(o.orientation, r.orientation, "face orientation drifted");
        assert_eq!(o.inner_loops.len(), r.inner_loops.len(), "inner loop count drifted");
        assert_surface_matches(&o.surface, &r.surface);
    }

    let void_count = |shells: &[BrepSolidShell]| shells.iter().filter(|m| m.is_void).count();
    assert_eq!(void_count(&original.solids[0].shells), void_count(&reimported.solids[0].shells));
    assert!(reimported.solids[0].shells.iter().any(|m| m.is_void), "void shell must survive the round trip");
}

#[semio_framework_async_macros::async_test]
async fn dangling_reference_errors_rather_than_fabricating() {
    let mut snap = SemioBrepSnapshot::default();
    snap.edges = vec![BrepEdge { id: "e1".into(), start_vertex: "nonexistent".into(), end_vertex: "also-nonexistent".into(), curve: BrepCurve::Line { origin: SemioPoint3::default(), direction: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 } }, tol: 0.0 }];
    let result = semio_framework_plugin::resolve_ready(SemioBrepToStep::serialize(&snap));
    assert!(result.is_err(), "an edge referencing a nonexistent vertex must error, not silently drop the edge");
}

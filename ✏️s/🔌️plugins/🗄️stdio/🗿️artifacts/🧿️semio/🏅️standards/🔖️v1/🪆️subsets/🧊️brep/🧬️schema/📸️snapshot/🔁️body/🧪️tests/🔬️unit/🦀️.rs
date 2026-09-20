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
/// resulting body is still internally consistent (every reference resolves, every ring closes,
/// the rebuilt solid validates exactly as its numeric-id original does — and no minted label
/// collides with the document's own high-water mark).
///
/// The document under test is a real solid re-keyed the way a STEP import (or a hand-authored
/// fixture) addresses its entities: `to_snapshot` emits bare decimal labels, so re-keying every
/// id to a non-numeric string is precisely the "no persistent-label history to preserve" case
/// this file's module doc describes. It has to be a *valid* solid, because
/// [`crate::standards::v1::subsets::brep::schema::inferences::validation_report::validate_body`]
/// checks p-curve presence, same-parameter agreement and shell closure — a single open face
/// carrying no p-curves could never come back clean, whatever its ids looked like.
#[semio_framework_async_macros::async_test]
async fn foreign_string_ids_mint_fresh_labels() {
    let mut original = Body::new();
    let mut rec = OpRecorder::new();
    make_box(&mut original, 2.0, 3.0, 4.0, &mut rec).unwrap();
    let numeric = original.to_snapshot();

    // 🐼️ Re-key EVERY id and every reference to it — nothing in the document parses as `u64` any more.
    let foreign = |id: &str| if id.is_empty() { String::new() } else { format!("🐼️{id}") };
    let mut snap = numeric.clone();
    for v in &mut snap.vertices {
        v.id = foreign(&v.id);
    }
    for e in &mut snap.edges {
        e.id = foreign(&e.id);
        e.start_vertex = foreign(&e.start_vertex);
        e.end_vertex = foreign(&e.end_vertex);
    }
    for l in &mut snap.loops {
        l.id = foreign(&l.id);
        for le in &mut l.edges {
            le.edge = foreign(&le.edge);
        }
    }
    for f in &mut snap.faces {
        f.id = foreign(&f.id);
        f.outer_loop = foreign(&f.outer_loop);
        for inner in &mut f.inner_loops {
            let renamed = foreign(inner);
            *inner = renamed;
        }
    }
    for s in &mut snap.shells {
        s.id = foreign(&s.id);
        for sf in &mut s.faces {
            sf.face = foreign(&sf.face);
        }
    }
    for s in &mut snap.solids {
        s.id = foreign(&s.id);
        for ss in &mut s.shells {
            ss.shell = foreign(&ss.shell);
        }
    }
    for c in &mut snap.coedges {
        c.id = foreign(&c.id);
        c.edge = foreign(&c.edge);
        c.loop_id = foreign(&c.loop_id);
        c.next = foreign(&c.next);
        c.prev = foreign(&c.prev);
    }

    let rebuilt = Body::from_snapshot(&snap).expect("from_snapshot on foreign ids");
    assert_eq!(counts(&rebuilt), counts(&original), "foreign ids must rebuild the very same topology and geometry");
    for (_, v) in rebuilt.vertices.iter() {
        assert!(v.label.0 >= numeric.next_label, "vertex label {:?} must be minted fresh above the document's high-water mark {}", v.label, numeric.next_label);
    }
    let issues = crate::standards::v1::subsets::brep::schema::inferences::validation_report::validate_body(&rebuilt);
    assert!(issues.is_empty(), "{issues:?}");

    // 🧱️ The pre-`coedges` shape of the same document (STEP import, older fixtures) still rebuilds
    // its whole topology from `BrepLoop.edges` alone — carrying no p-curves, exactly as
    // `SemioBrepSnapshot::coedges`' own doc comment states.
    let mut without_coedges = snap.clone();
    without_coedges.coedges.clear();
    let fallback = Body::from_snapshot(&without_coedges).expect("from_snapshot on the pre-coedges fallback path");
    assert_eq!(fallback.vertices.len(), original.vertices.len());
    assert_eq!(fallback.edges.len(), original.edges.len());
    assert_eq!(fallback.coedges.len(), original.coedges.len());
    assert_eq!(fallback.faces.len(), original.faces.len());
    assert_eq!(fallback.shells.len(), original.shells.len());
    assert_eq!(fallback.solids.len(), original.solids.len());
    assert!(fallback.coedges.iter().all(|(_, coedge)| coedge.pcurve.is_none()), "the fallback path attaches no p-curves");
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

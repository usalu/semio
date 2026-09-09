use super::*;
use std::f64::consts::PI;

use crate::standards::v1::subsets::brep::schema::diff::primitives::{make_box, make_cylinder};
use crate::standards::v1::subsets::brep::schema::inferences::mass_properties::{solid_signed_volume, solid_surface_area, solid_volume};
use crate::standards::v1::subsets::brep::schema::inferences::validation_report::validate_body;

/// 🧪 Chord tolerance for a solid whose boundary is a FULL circle: the mass-properties integrator
/// polygonises such a boundary, so its own bias dominates every measurement there and refining it
/// only costs time (a bare cylinder measures `−2.7e-3` at `1e-3`, `−2.8e-5` at `1e-5` — exactly
/// linear in the tolerance, and 130× the wall time). The blend's own exactness is asserted against
/// that measured bias instead. Solids whose faces are bounded by straight `(u, v)` lines — every
/// filleted or chamfered BOX — have no such bias and are measured at `1e-5`.
const CIRCULAR_TOL: f64 = 1e-3;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn box_edges(body: &Body, solid: SolidId) -> Vec<EdgeId> {
    solid_edge_set(body, solid).into_iter().collect()
}

/// 🧪 `V − E + F` over the solid's own entities — 2 for every sphere-like closed shell.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn euler_characteristic(body: &Body, solid: SolidId) -> i64 {
    let faces = body.solid_faces(solid);
    let mut edges = BTreeSet::new();
    let mut vertices = BTreeSet::new();
    for &face in &faces {
        for cid in body.face_coedges(face) {
            let coedge = body.coedges.get(cid).unwrap();
            edges.insert(coedge.edge);
            let edge = body.edges.get(coedge.edge).unwrap();
            vertices.insert(edge.v0);
            vertices.insert(edge.v1);
        }
    }
    vertices.len() as i64 - edges.len() as i64 + faces.len() as i64
}

/// 🧪 Every structural law a finished blend must satisfy at once: `validate_body` silent over the
/// WHOLE body (the input solid is left intact, so this also proves the surgery is non-destructive),
/// χ = 2, and an outward-oriented shell.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_well_formed(body: &Body, solid: SolidId) {
    // 🩺️ Advisory `warning-` codes are excluded by the validator's own contract, and a rounded
    // solid produces one legitimately: a corner patch meets each of its three planar neighbours at
    // exactly one point, their shared VERTEX, which the self-intersection probe's edge-adjacency
    // test does not recognise as adjacency.
    let issues: Vec<_> = validate_body(body).into_iter().filter(|issue| !issue.code.starts_with("warning-")).collect();
    assert!(issues.is_empty(), "validate_body reported {} error(s), first: {}:{}:{}", issues.len(), issues[0].entity, issues[0].code, issues[0].message);
    assert_eq!(euler_characteristic(body, solid), 2, "a blended solid must stay a topological sphere");
    let signed = solid_signed_volume(body, solid, 1e-2).unwrap();
    assert!(signed > 0.0, "the blended shell must be outward-oriented, got signed volume {signed}");
}

#[semio_framework_async_macros::async_test]
async fn fillet_one_box_edge_matches_closed_form() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let (w, d, h, r) = (2.0, 2.0, 2.0, 0.3);
    let solid = make_box(&mut body, w, d, h, &mut rec).unwrap();
    let v0 = solid_volume(&body, solid, 1e-6).unwrap();
    let edge = box_edges(&body, solid)[0];
    let len = edge_length(&body, edge).unwrap();
    let out = fillet_edges(&mut body, solid, &[edge], r, &mut rec).unwrap();
    assert_well_formed(&body, out);
    let v1 = solid_volume(&body, out, 1e-5).unwrap();
    let closed_form = v0 - len * r * r * (1.0 - PI / 4.0);
    assert!((v1 - closed_form).abs() < 1e-6 * closed_form, "v1={v1} expected={closed_form}");
}

#[semio_framework_async_macros::async_test]
async fn fillet_all_box_edges_matches_rounded_box_closed_form() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let (bw, bd, bh, r) = (2.6, 3.6, 2.1, 0.3);
    let solid = make_box(&mut body, bw, bd, bh, &mut rec).unwrap();
    let edges = box_edges(&body, solid);
    assert_eq!(edges.len(), 12);
    let out = fillet_edges(&mut body, solid, &edges, r, &mut rec).unwrap();
    assert_well_formed(&body, out);
    // 🧊 Minkowski sum of the inner box `a × b × c` with a ball of radius `r` (Steiner): the inner
    // box, six slabs, twelve quarter-cylinders and eight sphere octants.
    let (a, b, c) = (bw - 2.0 * r, bd - 2.0 * r, bh - 2.0 * r);
    let volume = a * b * c + 2.0 * r * (a * b + b * c + c * a) + PI * r * r * (a + b + c) + (4.0 / 3.0) * PI * r * r * r;
    let area = 2.0 * (a * b + b * c + c * a) + 2.0 * PI * r * (a + b + c) + 4.0 * PI * r * r;
    assert_eq!(body.solid_faces(out).len(), 26, "6 planes + 12 cylinders + 8 spheres");
    let v = solid_volume(&body, out, 1e-5).unwrap();
    assert!((v - volume).abs() < 1e-6 * volume, "v={v} expected={volume}");
    let s = solid_surface_area(&body, out, 1e-5).unwrap();
    assert!((s - area).abs() < 1e-6 * area, "area={s} expected={area}");
}

#[semio_framework_async_macros::async_test]
async fn chamfer_asymmetric_matches_closed_form() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let (w, d, h, d1, d2) = (2.0, 2.0, 2.0, 0.2, 0.35);
    let solid = make_box(&mut body, w, d, h, &mut rec).unwrap();
    let v0 = solid_volume(&body, solid, 1e-6).unwrap();
    let edge = box_edges(&body, solid)[0];
    let len = edge_length(&body, edge).unwrap();
    let out = chamfer_edges(&mut body, solid, &[edge], d1, d2, &mut rec).unwrap();
    assert_well_formed(&body, out);
    let v1 = solid_volume(&body, out, 1e-5).unwrap();
    let closed_form = v0 - 0.5 * d1 * d2 * len;
    assert!((v1 - closed_form).abs() < 1e-6 * closed_form, "v1={v1} expected={closed_form}");
}

#[semio_framework_async_macros::async_test]
async fn chamfer_all_box_edges_matches_closed_form() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let (a, b, c, d) = (2.0, 3.0, 1.5, 0.25);
    let solid = make_box(&mut body, a, b, c, &mut rec).unwrap();
    let edges = box_edges(&body, solid);
    let out = chamfer_edges(&mut body, solid, &edges, d, d, &mut rec).unwrap();
    assert_well_formed(&body, out);
    assert_eq!(body.solid_faces(out).len(), 26, "6 planes + 12 cut planes + 8 corner triangles");
    // 🧊 Twelve triangular prisms of section `d²/2` over the full edges, minus the `3d³/4` each
    // corner triples-counts, plus the `d³/12` its own corner plane takes beyond them (verified
    // independently by Monte-Carlo integration of the half-space intersection).
    let v = solid_volume(&body, out, 1e-5).unwrap();
    let closed_form = a * b * c - 2.0 * d * d * (a + b + c) + (16.0 / 3.0) * d * d * d;
    assert!((v - closed_form).abs() < 1e-6 * closed_form, "v={v} expected={closed_form}");
}

#[semio_framework_async_macros::async_test]
async fn fillet_cylinder_cap_matches_pappus_closed_form() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let (big_r, h, r) = (1.0, 2.0, 0.2);
    let solid = make_cylinder(&mut body, big_r, h, &mut rec).unwrap();
    let top = *body
        .solid_faces(solid)
        .iter()
        .find(|&&f| matches!(body.surfaces.get(body.faces.get(f).unwrap().surface).unwrap(), Surface::Plane { frame } if (frame.origin.z - h).abs() < 1e-9))
        .unwrap();
    let edge = body.face_coedges(top).into_iter().map(|c| body.coedges.get(c).unwrap().edge).next().unwrap();
    // 📏 The integrator polygonises a full circular boundary, so it under-measures ANY cylinder by
    // a chord-tolerance-proportional bias; the untouched input solid (which this surgery leaves
    // intact) measures that bias directly and the blended result must not exceed it.
    let bare = solid_volume(&body, solid, CIRCULAR_TOL).unwrap();
    let bias = (bare - PI * big_r * big_r * h).abs();
    let out = fillet_edges(&mut body, solid, &[edge], r, &mut rec).unwrap();
    assert_well_formed(&body, out);
    assert_eq!(body.solid_faces(out).len(), 4, "lateral + torus band + both caps");
    // 🍩 Pappus on the removed meridian section: the `r × r` corner square minus its quarter disc,
    // each revolved about the cylinder's own axis at its own centroid radius.
    let removed = 2.0 * PI * (r * r * (big_r - 0.5 * r) - (PI * r * r / 4.0) * (big_r - r + 4.0 * r / (3.0 * PI)));
    let closed_form = PI * big_r * big_r * h - removed;
    let v = solid_volume(&body, out, CIRCULAR_TOL).unwrap();
    assert!((v - closed_form).abs() < 2.0 * bias, "v={v} expected={closed_form} bias={bias}");
    assert!((v - closed_form).abs() < 1e-3 * closed_form, "v={v} expected={closed_form}");
}

#[semio_framework_async_macros::async_test]
async fn chamfer_cylinder_cap_matches_pappus_closed_form() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let (big_r, h, d) = (1.0, 2.0, 0.2);
    let solid = make_cylinder(&mut body, big_r, h, &mut rec).unwrap();
    let top = *body
        .solid_faces(solid)
        .iter()
        .find(|&&f| matches!(body.surfaces.get(body.faces.get(f).unwrap().surface).unwrap(), Surface::Plane { frame } if (frame.origin.z - h).abs() < 1e-9))
        .unwrap();
    let edge = body.face_coedges(top).into_iter().map(|c| body.coedges.get(c).unwrap().edge).next().unwrap();
    let bare = solid_volume(&body, solid, CIRCULAR_TOL).unwrap();
    let bias = (bare - PI * big_r * big_r * h).abs();
    let out = chamfer_edges(&mut body, solid, &[edge], d, d, &mut rec).unwrap();
    assert_well_formed(&body, out);
    assert_eq!(body.solid_faces(out).len(), 4, "lateral + cone frustum + both caps");
    // 🔻 Pappus again, now on the removed right triangle of legs `d` — the cut face is an exact
    // cone frustum, so the section is straight-sided.
    let removed = 2.0 * PI * (0.5 * d * d) * (big_r - d / 3.0);
    let closed_form = PI * big_r * big_r * h - removed;
    let v = solid_volume(&body, out, CIRCULAR_TOL).unwrap();
    assert!((v - closed_form).abs() < 2.0 * bias, "v={v} expected={closed_form} bias={bias}");
    assert!((v - closed_form).abs() < 1e-3 * closed_form, "v={v} expected={closed_form}");
}

#[semio_framework_async_macros::async_test]
async fn variable_fillet_is_monotone_in_radius() {
    let mut volumes = Vec::new();
    for far in [0.2, 0.4] {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 2.0, 2.0, 2.0, &mut rec).unwrap();
        let edge = box_edges(&body, solid)[0];
        let out = fillet_variable(&mut body, solid, edge, 0.1, far, &mut rec).unwrap();
        assert_well_formed(&body, out);
        volumes.push(solid_volume(&body, out, 1e-5).unwrap());
    }
    assert!(volumes[1] < volumes[0], "growing the far-end radius must remove strictly more material: {volumes:?}");
}

#[semio_framework_async_macros::async_test]
async fn fillet_determinism() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let edge = box_edges(&body, solid)[0];
    let a = fillet_edges(&mut body, solid, &[edge], 0.2, &mut rec).unwrap();
    let solid2 = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let edge2 = box_edges(&body, solid2).into_iter().find(|&e| (edge_length(&body, e).unwrap() - edge_length(&body, edge).unwrap()).abs() < 1e-9).unwrap();
    let b = fillet_edges(&mut body, solid2, &[edge2], 0.2, &mut rec).unwrap();
    assert_eq!(body.solid_faces(a).len(), body.solid_faces(b).len());
    let va = solid_volume(&body, a, 1e-6).unwrap();
    let vb = solid_volume(&body, b, 1e-6).unwrap();
    assert!((va - vb).abs() < 1e-12, "va={va} vb={vb}");
}

#[semio_framework_async_macros::async_test]
async fn reject_zero_radius_and_empty_edges() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let edge = box_edges(&body, solid)[0];
    assert!(fillet_edges(&mut body, solid, &[edge], 0.0, &mut rec).is_err());
    assert!(chamfer_edges(&mut body, solid, &[edge], 0.0, 0.1, &mut rec).is_err());
    assert!(fillet_variable(&mut body, solid, edge, 0.0, 0.1, &mut rec).is_err());
    assert!(fillet_edges(&mut body, solid, &[], 0.1, &mut rec).is_err());
    assert!(chamfer_edges(&mut body, solid, &[], 0.1, 0.1, &mut rec).is_err());
}

#[semio_framework_async_macros::async_test]
async fn reject_partially_blended_corner() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 2.0, 2.0, 2.0, &mut rec).unwrap();
    let solid_faces: BTreeSet<FaceId> = body.solid_faces(solid).into_iter().collect();
    let all = box_edges(&body, solid);
    let first = all[0];
    let ends = body.edges.get(first).map(|e| [e.v0, e.v1]).unwrap();
    let neighbour = all
        .iter()
        .copied()
        .find(|&e| {
            e != first
                && body.edges.get(e).map(|x| ends.contains(&x.v0) || ends.contains(&x.v1)).unwrap_or(false)
                && edge_two_faces(&body, &solid_faces, e).map(|(a, b)| edge_two_faces(&body, &solid_faces, first).map(|(c, d)| [a, b].contains(&c) || [a, b].contains(&d)).unwrap_or(false)).unwrap_or(false)
        })
        .expect("a box corner has two more edges");
    let error = fillet_edges(&mut body, solid, &[first, neighbour], 0.2, &mut rec).unwrap_err();
    assert!(format!("{error:?}").contains("either one or all"), "expected an explicit partial-corner refusal, got {error:?}");
}


use super::*;
use crate::standards::v1::subsets::brep::schema::diff::euler::{add_face, add_shell, add_solid, make_edge, make_loop, make_vertex};
use crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
use crate::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn attach_planar_pcurves(body: &mut Body, face: FaceId, frame: &Frame3) {
    for coedge_id in body.face_coedges(face) {
        let co = body.coedges.get(coedge_id).unwrap();
        let edge = body.edges.get(co.edge).unwrap();
        let Curve3::Line { origin, dir } = *body.curves3.get(edge.curve).unwrap() else { continue };
        let local_o = frame.to_local(origin);
        let local_d = frame.to_local_vector(dir);
        let pcurve = body.curves2.insert(crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve2::Line {
            origin: crate::standards::v1::subsets::brep::schema::snapshot::vector::Pnt2::new(local_o.x, local_o.y),
            dir: crate::standards::v1::subsets::brep::schema::snapshot::vector::Vec2::new(local_d.x, local_d.y),
        });
        // 🩺 A p-curve is always parametrized in the EDGE's own curve order — never reversed
        // to match a particular coedge's traversal direction (W1-E's binding convention, see
        // 📓️w1e-primitives.md §"p-curve convention"): both coedges sharing an edge get the
        // IDENTICAL `(pcurve, prange)`. `pcurve`/`local_o`/`local_d` above are built to trace
        // A→B exactly as `curve3` does over `edge.range = (0.0, 1.0)`, so `prange` must match
        // that same order regardless of `co.forward` — reversing it here (the previous bug)
        // desynchronized `check_same_parameter`'s per-`s` correspondence for every backward
        // coedge, by a full edge length at each sample's far end.
        let prange = (0.0, 1.0);
        let co = body.coedges.get_mut(coedge_id).unwrap();
        co.pcurve = Some(pcurve);
        co.prange = prange;
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn build_tetrahedron_with_windings(body: &mut Body, rec: &mut OpRecorder, face_defs: [[usize; 3]; 4]) -> crate::standards::v1::subsets::brep::schema::snapshot::arena::SolidId {
    let positions = [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), Pnt3::new(0.0, 0.0, 1.0)];
    let vertices: Vec<_> = positions.iter().map(|&p| make_vertex(body, p, Tol::DEFAULT, rec)).collect();
    let edge_pairs = [(0, 1), (1, 2), (2, 0), (0, 3), (1, 3), (2, 3)];
    let mut edges = std::collections::HashMap::new();
    for &(a, b) in &edge_pairs {
        let curve = body.curves3.insert(Curve3::Line { origin: positions[a], dir: positions[b] - positions[a] });
        let edge = make_edge(body, curve, (0.0, 1.0), vertices[a], vertices[b], Tol::DEFAULT, rec);
        edges.insert((a, b), edge);
        edges.insert((b, a), edge);
    }
    let mut faces = Vec::new();
    for tri in face_defs {
        let normal = (positions[tri[1]] - positions[tri[0]]).cross(positions[tri[2]] - positions[tri[0]]);
        let frame = Frame3::from_normal(positions[tri[0]], normal).unwrap();
        let surface = body.surfaces.insert(Surface::Plane { frame });
        let members: Vec<(EdgeId, bool)> = (0..3)
            .map(|i| {
                let a = tri[i];
                let b = tri[(i + 1) % 3];
                let edge = edges[&(a, b)];
                let forward = body.edges.get(edge).unwrap().v0 == vertices[a];
                (edge, forward)
            })
            .collect();
        let outer = make_loop(body, ArenaId::from_raw(0, 0), &members);
        let face = add_face(body, surface, Some(outer), vec![], false, Tol::DEFAULT, rec);
        body.loops.get_mut(outer).unwrap().face = face;
        attach_planar_pcurves(body, face, &frame);
        faces.push(face);
    }
    let shell = add_shell(body, faces, rec);
    add_solid(body, shell, vec![], rec)
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
// 🐛 FIX (ticket `26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME` FX-5): these two windings were
// swapped. For vertices `0=(0,0,0), 1=(1,0,0), 2=(0,1,0), 3=(0,0,1)`, `[[0,1,2],[0,3,1],[1,3,2],
// [2,3,0]]` — the OLD `build_tetrahedron` — has every face's `(v1-v0)×(v2-v0)` normal pointing
// TOWARD the face's own excluded (4th) vertex, i.e. INWARD (verified algebraically per face,
// not just by the validator's own say-so): e.g. face `[0,1,2]` lies in `z=0` with normal
// `(0,0,1)`, the SAME side as the excluded vertex `3=(0,0,1)`. `[[0,2,1],[0,1,3],[1,2,3],
// [2,0,3]]` — the OLD `build_tetrahedron_globally_reversed` — is each of those faces with its
// last two vertices swapped, i.e. the genuinely OUTWARD-facing tetrahedron. The fixtures were
// simply mislabeled; the validator's `shell-orientation-inward` check was correct both times.
fn build_tetrahedron(body: &mut Body, rec: &mut OpRecorder) -> crate::standards::v1::subsets::brep::schema::snapshot::arena::SolidId {
    build_tetrahedron_with_windings(body, rec, [[0, 2, 1], [0, 1, 3], [1, 2, 3], [2, 0, 3]])
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn build_tetrahedron_globally_reversed(body: &mut Body, rec: &mut OpRecorder) -> crate::standards::v1::subsets::brep::schema::snapshot::arena::SolidId {
    build_tetrahedron_with_windings(body, rec, [[0, 1, 2], [0, 3, 1], [1, 3, 2], [2, 3, 0]])
}

#[semio_framework_async_macros::async_test]
async fn a_cleanly_built_tetrahedron_validates_with_no_issues() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    build_tetrahedron(&mut body, &mut rec);
    let issues = validate_body(&body);
    assert!(issues.is_empty(), "unexpected issues on a clean solid: {issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn a_broken_ring_pointer_is_detected() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_tetrahedron(&mut body, &mut rec);
    let face = body.solid_faces(solid)[0];
    let outer = body.faces.get(face).unwrap().outer.unwrap();
    let coedges = body.loop_coedges(outer);
    // Corrupt the ring: point the first coedge's `next` at itself instead of its real neighbor.
    let first = coedges[0];
    body.coedges.get_mut(first).unwrap().next = first;
    let issues = validate_body(&body);
    assert!(issues.iter().any(|i| i.code == "broken-ring" || i.code == "next-prev-mismatch"), "expected a ring issue, got {issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn a_vertex_tolerance_exceeding_its_edge_tolerance_is_detected() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    build_tetrahedron(&mut body, &mut rec);
    let (vertex_id, _) = body.vertices.iter().next().unwrap();
    body.vertices.get_mut(vertex_id).unwrap().tol = Tol::new(10.0);
    let issues = validate_body(&body);
    assert!(issues.iter().any(|i| i.code == "tolerance-containment-violated"), "expected a tolerance issue, got {issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn a_non_manifold_edge_is_flagged() {
    // Build a free-standing edge with three coedges referencing it (impossible in a clean
    // 2-manifold build, so constructed directly).
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let v0 = make_vertex(&mut body, Pnt3::new(0.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
    let v1 = make_vertex(&mut body, Pnt3::new(1.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
    let curve = body.curves3.insert(Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::X });
    let edge = make_edge(&mut body, curve, (0.0, 1.0), v0, v1, Tol::DEFAULT, &mut rec);
    for _ in 0..3 {
        body.coedges.insert(crate::standards::v1::subsets::brep::schema::snapshot::topology::Coedge {
            edge,
            forward: true,
            pcurve: None,
            prange: (0.0, 1.0),
            loop_id: ArenaId::from_raw(0, 0),
            next: ArenaId::from_raw(0, 0),
            prev: ArenaId::from_raw(0, 0),
        });
    }
    let issues = validate_body(&body);
    assert!(issues.iter().any(|i| i.code == "non-manifold-edge"), "expected a non-manifold-edge issue, got {issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn same_parameter_violation_is_detected_when_pcurve_disagrees_with_3d_curve() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_tetrahedron(&mut body, &mut rec);
    let face = body.solid_faces(solid)[0];
    let outer = body.faces.get(face).unwrap().outer.unwrap();
    let coedge_id = body.loop_coedges(outer)[0];
    // Attach a pcurve that does NOT correspond to the face's surface at all — a constant,
    // clearly-wrong 2D point far from where the 3D edge actually projects.
    let bad_pcurve = body.curves2.insert(crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve2::Line {
        origin: crate::standards::v1::subsets::brep::schema::snapshot::vector::Pnt2::new(500.0, 500.0),
        dir: crate::standards::v1::subsets::brep::schema::snapshot::vector::Vec2::new(0.0, 0.0),
    });
    let coedge = body.coedges.get_mut(coedge_id).unwrap();
    coedge.pcurve = Some(bad_pcurve);
    coedge.prange = (0.0, 1.0);
    let issues = validate_body(&body);
    assert!(issues.iter().any(|i| i.code == "same-parameter-violated"), "expected a same-parameter issue, got {issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn missing_pcurve_is_an_error_not_a_skip() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_tetrahedron(&mut body, &mut rec);
    let face = body.solid_faces(solid)[0];
    let outer = body.faces.get(face).unwrap().outer.unwrap();
    let coedge_id = body.loop_coedges(outer)[0];
    body.coedges.get_mut(coedge_id).unwrap().pcurve = None;
    let issues = validate_body(&body);
    assert!(issues.iter().any(|i| i.code == "missing-pcurve"), "expected a missing-pcurve issue, got {issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn shell_not_closed_is_detected_when_a_face_is_removed() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_tetrahedron(&mut body, &mut rec);
    let shell_id = body.solid_shells(solid)[0];
    body.shells.get_mut(shell_id).unwrap().faces.pop();
    let issues = validate_body(&body);
    assert!(issues.iter().any(|i| i.code == "shell-not-closed"), "expected a shell-not-closed issue, got {issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn orientation_inconsistent_is_detected_when_a_shared_edge_sense_is_flipped() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_tetrahedron(&mut body, &mut rec);
    let face = body.solid_faces(solid)[0];
    let outer = body.faces.get(face).unwrap().outer.unwrap();
    let coedge_id = body.loop_coedges(outer)[0];
    let co = body.coedges.get_mut(coedge_id).unwrap();
    co.forward = !co.forward;
    let issues = validate_body(&body);
    assert!(issues.iter().any(|i| i.code == "orientation-inconsistent"), "expected an orientation-inconsistent issue, got {issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn shell_orientation_inward_is_detected_on_a_globally_reversed_tetrahedron() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    build_tetrahedron_globally_reversed(&mut body, &mut rec);
    let issues = validate_body(&body);
    assert!(issues.iter().any(|i| i.code == "shell-orientation-inward"), "expected a shell-orientation-inward issue, got {issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn degenerate_edge_is_detected_when_tolerance_exceeds_edge_length() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    build_tetrahedron(&mut body, &mut rec);
    let (edge_id, _) = body.edges.iter().next().unwrap();
    body.edges.get_mut(edge_id).unwrap().tol = Tol::new(10.0);
    let issues = validate_body(&body);
    assert!(issues.iter().any(|i| i.code == "degenerate-edge"), "expected a degenerate-edge issue, got {issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn sliver_face_is_detected_when_tolerance_exceeds_face_area() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    build_tetrahedron(&mut body, &mut rec);
    let (face_id, _) = body.faces.iter().next().unwrap();
    body.faces.get_mut(face_id).unwrap().tol = Tol::new(10.0);
    let issues = validate_body(&body);
    assert!(issues.iter().any(|i| i.code == "sliver-face"), "expected a sliver-face issue, got {issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn self_intersection_probe_warns_on_overlapping_non_adjacent_faces() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let a = crate::standards::v1::subsets::brep::schema::diff::primitives::make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let b = crate::standards::v1::subsets::brep::schema::diff::primitives::make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let mut faces = body.solid_faces(a);
    faces.extend(body.solid_faces(b));
    let shell = add_shell(&mut body, faces, &mut rec);
    add_solid(&mut body, shell, vec![], &mut rec);
    let issues = validate_body(&body);
    assert!(issues.iter().any(|i| i.code == "warning-possible-self-intersection"), "expected a self-intersection warning, got {issues:?}");
}

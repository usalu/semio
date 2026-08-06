//! 🩺️ Shape validation: everything the kernel's "never wrong, fail loud" invariant needs a way to
//! check for. Every check returns [`ValidationIssue`]s rather than a bare bool so a caller (or a
//! human) can see exactly which entity failed and why; nothing here mutates the body.

use crate::brep::arena::ArenaId;
use crate::brep::error::ValidationIssue;
use crate::brep::topo::Body;

// #region 🔖️Topology

fn check_loop_rings(body: &Body, issues: &mut Vec<ValidationIssue>) {
    for (loop_id, lp) in body.loops.iter() {
        let coedges = body.loop_coedges(loop_id);
        if coedges.is_empty() {
            issues.push(ValidationIssue { entity: format!("loop-{}", loop_id.raw_index()), code: "empty-loop", message: "loop has no coedges".to_string() });
            continue;
        }
        if coedges[0] != lp.first {
            issues.push(ValidationIssue { entity: format!("loop-{}", loop_id.raw_index()), code: "broken-ring", message: "walking next from Loop::first did not return to itself — the ring is broken or too long".to_string() });
            continue;
        }
        let n = coedges.len();
        for i in 0..n {
            let Some((_, end_a)) = body.coedge_endpoints(coedges[i]) else { continue };
            let Some((start_b, _)) = body.coedge_endpoints(coedges[(i + 1) % n]) else { continue };
            if end_a != start_b {
                issues.push(ValidationIssue { entity: format!("loop-{}", loop_id.raw_index()), code: "loop-not-closed", message: format!("coedge {i} ends at a different vertex than coedge {} starts at", (i + 1) % n) });
            }
            let coedge_a = body.coedges.get(coedges[i]).unwrap();
            let coedge_b = body.coedges.get(coedges[(i + 1) % n]).unwrap();
            if coedge_a.next != coedges[(i + 1) % n] || coedge_b.prev != coedges[i] {
                issues.push(ValidationIssue { entity: format!("loop-{}", loop_id.raw_index()), code: "next-prev-mismatch", message: format!("coedge {i}'s next/prev pointers are not symmetric with its ring neighbor") });
            }
        }
    }
}

/// 🩺️ Flags edges used by more than 2 coedges — valid for future non-manifold support but worth
/// surfacing explicitly (the boolean/sewing pipeline in later phases assumes 2-manifold input
/// unless a caller has opted into non-manifold handling).
fn check_edge_valence(body: &Body, issues: &mut Vec<ValidationIssue>) {
    for (edge_id, _) in body.edges.iter() {
        let valence = body.edge_coedges(edge_id).len();
        if valence > 2 {
            issues.push(ValidationIssue { entity: format!("edge-{}", edge_id.raw_index()), code: "non-manifold-edge", message: format!("edge is used by {valence} coedges (2-manifold shapes use at most 2)") });
        }
    }
}

// #endregion 🔖️Topology

// #region 🔖️Geometry

/// 🩺️ Every vertex's tolerance must fit inside every incident edge's tolerance, and every edge's
/// inside every face whose loop uses it — the containment hierarchy from the plan's tolerance model.
fn check_tolerance_containment(body: &Body, issues: &mut Vec<ValidationIssue>) {
    for (edge_id, edge) in body.edges.iter() {
        for v in [edge.v0, edge.v1] {
            let Some(vertex) = body.vertices.get(v) else { continue };
            if let Some((finer, coarser)) = crate::brep::tolerance::check_containment(&format!("vertex-{}", v.raw_index()), vertex.tol, &format!("edge-{}", edge_id.raw_index()), edge.tol) {
                issues.push(ValidationIssue { entity: finer.clone(), code: "tolerance-containment-violated", message: format!("{finer}'s tolerance exceeds its containing {coarser}'s") });
            }
        }
    }
    for (face_id, face) in body.faces.iter() {
        for coedge_id in body.face_coedges(face_id) {
            let Some(coedge) = body.coedges.get(coedge_id) else { continue };
            let Some(edge) = body.edges.get(coedge.edge) else { continue };
            if let Some((finer, coarser)) = crate::brep::tolerance::check_containment(&format!("edge-{}", coedge.edge.raw_index()), edge.tol, &format!("face-{}", face_id.raw_index()), face.tol) {
                issues.push(ValidationIssue { entity: finer.clone(), code: "tolerance-containment-violated", message: format!("{finer}'s tolerance exceeds its containing {coarser}'s") });
            }
        }
    }
}

/// 🩺️ Same-parameter check: samples a coedge's pcurve against its 3D edge curve at corresponding
/// parameters (mapped linearly from the pcurve's `prange` onto the edge's `range`) and confirms
/// the face's surface, evaluated at the pcurve point, agrees with the 3D curve within the edge's
/// tolerance. Skips coedges with no pcurve (only an issue on non-planar faces, which nothing
/// before Phase 4 produces yet, so this check is dormant until surfaces with pcurves exist).
fn check_same_parameter(body: &Body, issues: &mut Vec<ValidationIssue>) {
    const SAMPLES: usize = 5;
    for (face_id, face) in body.faces.iter() {
        let Some(surface) = body.surfaces.get(face.surface) else { continue };
        for coedge_id in body.face_coedges(face_id) {
            let Some(coedge) = body.coedges.get(coedge_id) else { continue };
            let Some(pcurve_id) = coedge.pcurve else { continue };
            let Some(pcurve) = body.curves2.get(pcurve_id) else { continue };
            let Some(edge) = body.edges.get(coedge.edge) else { continue };
            let Some(curve3) = body.curves3.get(edge.curve) else { continue };
            for i in 0..=SAMPLES {
                let s = i as f64 / SAMPLES as f64;
                let p = coedge.prange.0 + (coedge.prange.1 - coedge.prange.0) * s;
                let t = edge.range.0 + (edge.range.1 - edge.range.0) * s;
                let uv = pcurve.eval(p);
                let via_surface = surface.eval(uv.x, uv.y);
                let via_curve = curve3.eval(t);
                if via_surface.distance(via_curve) > edge.tol.value() {
                    issues.push(ValidationIssue {
                        entity: format!("coedge-{}", coedge_id.raw_index()),
                        code: "same-parameter-violated",
                        message: format!("pcurve and 3D curve disagree by {} at s={s} (tol {})", via_surface.distance(via_curve), edge.tol.value()),
                    });
                }
            }
        }
    }
}

// #endregion 🔖️Geometry

// #region 🔖️Report

/// 🩺️ Runs every structural and geometric check and returns every finding.
pub fn validate_body(body: &Body) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    check_loop_rings(body, &mut issues);
    check_edge_valence(body, &mut issues);
    check_tolerance_containment(body, &mut issues);
    check_same_parameter(body, &mut issues);
    issues
}

// #endregion 🔖️Report

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::brep::curve::Curve3;
    use crate::brep::euler::{add_face, add_shell, add_solid, make_edge, make_loop, make_vertex};
    use crate::brep::history::OpRecorder;
    use crate::brep::mat::Frame3;
    use crate::brep::surface::Surface;
    use crate::brep::tolerance::Tol;
    use crate::brep::vec::{Pnt3, Vec3};

    fn build_tetrahedron(body: &mut Body, rec: &mut OpRecorder) -> crate::brep::arena::SolidId {
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
        let face_defs = [[0, 1, 2], [0, 3, 1], [1, 3, 2], [2, 3, 0]];
        let mut faces = Vec::new();
        for tri in face_defs {
            let normal = (positions[tri[1]] - positions[tri[0]]).cross(positions[tri[2]] - positions[tri[0]]);
            let frame = Frame3::from_normal(positions[tri[0]], normal).unwrap();
            let surface = body.surfaces.insert(Surface::Plane { frame });
            let members: Vec<(crate::brep::arena::EdgeId, bool)> = (0..3)
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
            faces.push(face);
        }
        let shell = add_shell(body, faces, rec);
        add_solid(body, shell, vec![], rec)
    }

    #[test]
    fn a_cleanly_built_tetrahedron_validates_with_no_issues() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        build_tetrahedron(&mut body, &mut rec);
        let issues = validate_body(&body);
        assert!(issues.is_empty(), "unexpected issues on a clean solid: {issues:?}");
    }

    #[test]
    fn a_broken_ring_pointer_is_detected() {
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

    #[test]
    fn a_vertex_tolerance_exceeding_its_edge_tolerance_is_detected() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        build_tetrahedron(&mut body, &mut rec);
        let (vertex_id, _) = body.vertices.iter().next().unwrap();
        body.vertices.get_mut(vertex_id).unwrap().tol = Tol::new(10.0);
        let issues = validate_body(&body);
        assert!(issues.iter().any(|i| i.code == "tolerance-containment-violated"), "expected a tolerance issue, got {issues:?}");
    }

    #[test]
    fn a_non_manifold_edge_is_flagged() {
        // Build a free-standing edge with three coedges referencing it (impossible in a clean
        // 2-manifold build, so constructed directly).
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let v0 = make_vertex(&mut body, Pnt3::new(0.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
        let v1 = make_vertex(&mut body, Pnt3::new(1.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
        let curve = body.curves3.insert(Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::X });
        let edge = make_edge(&mut body, curve, (0.0, 1.0), v0, v1, Tol::DEFAULT, &mut rec);
        for _ in 0..3 {
            body.coedges.insert(crate::brep::topo::Coedge { edge, forward: true, pcurve: None, prange: (0.0, 1.0), loop_id: ArenaId::from_raw(0, 0), next: ArenaId::from_raw(0, 0), prev: ArenaId::from_raw(0, 0) });
        }
        let issues = validate_body(&body);
        assert!(issues.iter().any(|i| i.code == "non-manifold-edge"), "expected a non-manifold-edge issue, got {issues:?}");
    }

    #[test]
    fn same_parameter_violation_is_detected_when_pcurve_disagrees_with_3d_curve() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = build_tetrahedron(&mut body, &mut rec);
        let face = body.solid_faces(solid)[0];
        let outer = body.faces.get(face).unwrap().outer.unwrap();
        let coedge_id = body.loop_coedges(outer)[0];
        // Attach a pcurve that does NOT correspond to the face's surface at all — a constant,
        // clearly-wrong 2D point far from where the 3D edge actually projects.
        let bad_pcurve = body.curves2.insert(crate::brep::curve::Curve2::Line { origin: crate::brep::vec::Pnt2::new(500.0, 500.0), dir: crate::brep::vec::Vec2::new(0.0, 0.0) });
        let coedge = body.coedges.get_mut(coedge_id).unwrap();
        coedge.pcurve = Some(bad_pcurve);
        coedge.prange = (0.0, 1.0);
        let issues = validate_body(&body);
        assert!(issues.iter().any(|i| i.code == "same-parameter-violated"), "expected a same-parameter issue, got {issues:?}");
    }
}
// #endregion 🔖️Tests

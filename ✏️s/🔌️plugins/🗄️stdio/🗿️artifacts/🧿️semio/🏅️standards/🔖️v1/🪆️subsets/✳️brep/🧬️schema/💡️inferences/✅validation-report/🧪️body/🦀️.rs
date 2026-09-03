//! 🧪 `validate_body` — pure kernel-layer structural/geometric invariant checker for an ephemeral
//! `Body` mid-construction (topology ring/valence/tolerance/same-parameter/orientation/degenerate/
//! self-intersection checks). Split out of the parent `✅validation-report/🦀️.rs` file (ticket
//! `26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME`) so this file depends ONLY on kernel modules
//! (`snapshot`/`🔺️diff`/`inferences::{bounding_volume,mass_properties}`) — no `SemioBrepSnapshot`,
//! no `store::InferredField`, no artifact-layer/STEP/plugin chain — which lets the standalone
//! kernel test harness (`TICKET/🔬️harness`) mount it directly. The parent file's
//! `BrepValidationReport` (a real `InferredField<SemioBrepSnapshot>`, whole-document referential
//! integrity via `check_brep_referential_integrity`) is a DIFFERENT, complementary check and stays
//! there — this file's `validate_body` is called directly by diff constructors on their own
//! ephemeral rep, never on a persisted snapshot.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::engine::Aabb;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::bounding_volume::face_aabb;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::mass_properties;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::{ArenaId, EdgeId, FaceId};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::curve_ops;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::ValidationIssue;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;

// #region 🔖️Topology

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn check_tolerance_containment(body: &Body, issues: &mut Vec<ValidationIssue>) {
    for (edge_id, edge) in body.edges.iter() {
        for v in [edge.v0, edge.v1] {
            let Some(vertex) = body.vertices.get(v) else { continue };
            if let Some((finer, coarser)) = crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::tolerance::check_containment(&format!("vertex-{}", v.raw_index()), vertex.tol, &format!("edge-{}", edge_id.raw_index()), edge.tol) {
                issues.push(ValidationIssue { entity: finer.clone(), code: "tolerance-containment-violated", message: format!("{finer}'s tolerance exceeds its containing {coarser}'s") });
            }
        }
    }
    for (face_id, face) in body.faces.iter() {
        for coedge_id in body.face_coedges(face_id) {
            let Some(coedge) = body.coedges.get(coedge_id) else { continue };
            let Some(edge) = body.edges.get(coedge.edge) else { continue };
            if let Some((finer, coarser)) =
                crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::tolerance::check_containment(&format!("edge-{}", coedge.edge.raw_index()), edge.tol, &format!("face-{}", face_id.raw_index()), face.tol)
            {
                issues.push(ValidationIssue { entity: finer.clone(), code: "tolerance-containment-violated", message: format!("{finer}'s tolerance exceeds its containing {coarser}'s") });
            }
        }
    }
}

/// 🩺️ Every coedge must carry a p-curve — trims and same-parameter checks below silently could
/// not verify a coedge without one, so a missing p-curve is an ERROR here, not a skip (audit
/// §6.12: "missing p-curves are skipped rather than rejected or repaired").
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn check_missing_pcurves(body: &Body, issues: &mut Vec<ValidationIssue>) {
    for (coedge_id, coedge) in body.coedges.iter() {
        if coedge.pcurve.is_none() {
            issues.push(ValidationIssue { entity: format!("coedge-{}", coedge_id.raw_index()), code: "missing-pcurve", message: "coedge has no p-curve — every coedge must carry one for trim/same-parameter validation".to_string() });
        }
    }
}

/// 🩺️ Same-parameter check: samples a coedge's p-curve against its 3D edge curve at corresponding
/// parameters (mapped linearly from the p-curve's `prange` onto the edge's `range`) and confirms
/// the face's surface, evaluated at the p-curve point, agrees with the 3D curve within the edge's
/// tolerance. Starts at 16 base samples and adaptively bisects any interval whose deviation grows
/// sharply relative to its neighbor, up to 3 refinement passes, so a localized divergence between
/// two coarse samples can't hide (audit §6.12: "same-parameter sampling is sparse").
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn check_same_parameter(body: &Body, issues: &mut Vec<ValidationIssue>) {
    const BASE_SAMPLES: usize = 16;
    for (face_id, face) in body.faces.iter() {
        let Some(surface) = body.surfaces.get(face.surface) else { continue };
        for coedge_id in body.face_coedges(face_id) {
            let Some(coedge) = body.coedges.get(coedge_id) else { continue };
            let Some(pcurve_id) = coedge.pcurve else { continue };
            let Some(pcurve) = body.curves2.get(pcurve_id) else { continue };
            let Some(edge) = body.edges.get(coedge.edge) else { continue };
            let Some(curve3) = body.curves3.get(edge.curve) else { continue };
            let samples = same_parameter_deviations(surface, pcurve, curve3, coedge.prange, edge.range, BASE_SAMPLES);
            let Some(&(worst_s, worst_dev)) = samples.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)) else { continue };
            if worst_dev > edge.tol.value() {
                issues.push(ValidationIssue {
                    entity: format!("coedge-{}", coedge_id.raw_index()),
                    code: "same-parameter-violated",
                    message: format!("pcurve and 3D curve disagree by {worst_dev} at s={worst_s} (tol {})", edge.tol.value()),
                });
            }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn same_parameter_deviation_at(surface: &Surface, pcurve: &crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve2, curve3: &crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3, prange: (f64, f64), range: (f64, f64), s: f64) -> f64 {
    let p = prange.0 + (prange.1 - prange.0) * s;
    let t = range.0 + (range.1 - range.0) * s;
    let uv = pcurve.eval(p);
    let via_surface = surface.eval(uv.x, uv.y);
    let via_curve = curve3.eval(t);
    via_surface.distance(via_curve)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn same_parameter_deviations(
    surface: &Surface,
    pcurve: &crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve2,
    curve3: &crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3,
    prange: (f64, f64),
    range: (f64, f64),
    base_samples: usize,
) -> Vec<(f64, f64)> {
    let mut samples: Vec<(f64, f64)> = (0..=base_samples).map(|i| i as f64 / base_samples as f64).map(|s| (s, same_parameter_deviation_at(surface, pcurve, curve3, prange, range, s))).collect();
    for _ in 0..3 {
        let mut midpoints = Vec::new();
        for w in samples.windows(2) {
            let (s0, d0) = w[0];
            let (s1, d1) = w[1];
            if (d1 - d0).abs() > d0.max(d1).max(1e-12) * 0.5 {
                midpoints.push(0.5 * (s0 + s1));
            }
        }
        if midpoints.is_empty() {
            break;
        }
        for s in midpoints {
            samples.push((s, same_parameter_deviation_at(surface, pcurve, curve3, prange, range, s)));
        }
        samples.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    }
    samples
}

/// 🩺️ Every edge within one shell must be used by exactly 2 coedges with OPPOSITE `forward` sense
/// — a closed, consistently-oriented shell where adjacent faces agree on traversal direction
/// (audit §6.12: "manifold orientation ... shell closure ... incomplete"). Fewer than 2 is an open
/// boundary; more than 2 is non-manifold within this shell; exactly 2 with the SAME sense means
/// the two faces sharing the edge disagree on orientation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn check_shell_closure_and_orientation(body: &Body, issues: &mut Vec<ValidationIssue>) {
    for (shell_id, shell) in body.shells.iter() {
        let mut edge_uses: std::collections::HashMap<EdgeId, Vec<bool>> = std::collections::HashMap::new();
        for &face in &shell.faces {
            for coedge_id in body.face_coedges(face) {
                if let Some(co) = body.coedges.get(coedge_id) {
                    edge_uses.entry(co.edge).or_default().push(co.forward);
                }
            }
        }
        for (edge_id, uses) in edge_uses {
            if uses.len() != 2 {
                issues.push(ValidationIssue {
                    entity: format!("shell-{}-edge-{}", shell_id.raw_index(), edge_id.raw_index()),
                    code: "shell-not-closed",
                    message: format!("edge is used {} time(s) within this shell (a closed shell needs exactly 2)", uses.len()),
                });
                continue;
            }
            if uses[0] == uses[1] {
                issues.push(ValidationIssue {
                    entity: format!("shell-{}-edge-{}", shell_id.raw_index(), edge_id.raw_index()),
                    code: "orientation-inconsistent",
                    message: "both faces sharing this edge traverse it in the same direction — adjacent face orientations disagree".to_string(),
                });
            }
        }
    }
}

/// 🩺️ A solid's outer shell must have a positive signed volume (face normals net outward); void
/// (inner) shells must be inverted relative to it — same sign as the outer shell means the void
/// was not correctly flipped (audit §6.12: "manifold orientation ... incomplete").
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn check_solid_orientation(body: &Body, issues: &mut Vec<ValidationIssue>) {
    const PROBE_TOL: f64 = 1e-3;
    for (solid_id, solid) in body.solids.iter() {
        let Ok(outer_v) = mass_properties::shell_signed_volume(body, solid.outer, PROBE_TOL) else { continue };
        if outer_v < 0.0 {
            issues.push(ValidationIssue { entity: format!("solid-{}", solid_id.raw_index()), code: "shell-orientation-inward", message: format!("outer shell's signed volume is negative ({outer_v}); face normals appear to point inward") });
        }
        for &void_shell in &solid.inners {
            if let Ok(void_v) = mass_properties::shell_signed_volume(body, void_shell, PROBE_TOL) {
                if outer_v.signum() == void_v.signum() {
                    issues.push(ValidationIssue {
                        entity: format!("solid-{}-void-shell-{}", solid_id.raw_index(), void_shell.raw_index()),
                        code: "void-shell-not-inverted",
                        message: "void (inner) shell's signed volume has the same sign as the outer shell — it should be inverted relative to the solid's exterior".to_string(),
                    });
                }
            }
        }
    }
}

/// 🩺️ Flags edges shorter than their own tolerance and faces smaller than their tolerance squared
/// — degenerate/sliver topology a downstream Boolean or sew pass would choke on (audit §6.12:
/// "tiny/sliver topology, degenerate edges ... incomplete").
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn check_degenerate_geometry(body: &Body, issues: &mut Vec<ValidationIssue>) {
    for (edge_id, edge) in body.edges.iter() {
        let Some(curve) = body.curves3.get(edge.curve) else { continue };
        let len = curve_ops::arc_length(curve, edge.range.0, edge.range.1, 1e-9);
        if len < edge.tol.value() {
            issues.push(ValidationIssue { entity: format!("edge-{}", edge_id.raw_index()), code: "degenerate-edge", message: format!("edge length {len} is below its own tolerance {}", edge.tol.value()) });
        }
    }
    for (face_id, face) in body.faces.iter() {
        let Ok(area) = mass_properties::face_area(body, face_id, 1e-3) else { continue };
        let tol2 = face.tol.value() * face.tol.value();
        if area < tol2 {
            issues.push(ValidationIssue { entity: format!("face-{}", face_id.raw_index()), code: "sliver-face", message: format!("face area {area} is below tol² ({tol2})") });
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn aabb_overlaps(a: &Aabb, b: &Aabb) -> bool {
    a.min[0] <= b.max[0] && a.max[0] >= b.min[0] && a.min[1] <= b.max[1] && a.max[1] >= b.min[1] && a.min[2] <= b.max[2] && a.max[2] >= b.min[2]
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn faces_share_edge(body: &Body, a: FaceId, b: FaceId) -> bool {
    let edges_a: std::collections::HashSet<EdgeId> = body.face_coedges(a).into_iter().filter_map(|c| body.coedges.get(c).map(|co| co.edge)).collect();
    body.face_coedges(b).into_iter().filter_map(|c| body.coedges.get(c).map(|co| co.edge)).any(|e| edges_a.contains(&e))
}

/// 🩺️ Self-intersection PROBE (not a certified global check): for every pair of non-adjacent
/// faces on the same solid whose AABBs overlap, samples each face's boundary/interior points
/// (`mass_properties::face_sample_points`) and flags a Warning when the closest pair comes within
/// tolerance — cheap enough to run always, catches the common case (audit §6.12: "general
/// self-intersection is not fully checked").
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn check_self_intersection_probe(body: &Body, issues: &mut Vec<ValidationIssue>) {
    const PROBE_TOL: f64 = 1e-6;
    for (solid_id, _) in body.solids.iter() {
        let faces = body.solid_faces(solid_id);
        for i in 0..faces.len() {
            for j in (i + 1)..faces.len() {
                let (fa, fb) = (faces[i], faces[j]);
                if faces_share_edge(body, fa, fb) {
                    continue;
                }
                let (Ok(aabb_a), Ok(aabb_b)) = (face_aabb(body, fa), face_aabb(body, fb)) else { continue };
                if !aabb_overlaps(&aabb_a, &aabb_b) {
                    continue;
                }
                let (Ok(pa), Ok(pb)) = (mass_properties::face_sample_points(body, fa), mass_properties::face_sample_points(body, fb)) else { continue };
                let mut best = f64::INFINITY;
                for p in &pa {
                    for q in &pb {
                        best = best.min(p.distance(*q));
                    }
                }
                if best < PROBE_TOL {
                    issues.push(ValidationIssue {
                        entity: format!("face-{}-face-{}", fa.raw_index(), fb.raw_index()),
                        code: "warning-possible-self-intersection",
                        message: format!("non-adjacent faces {fa} and {fb} come within {best} of each other away from any shared edge"),
                    });
                }
            }
        }
    }
}

// #endregion 🔖️Geometry

// #region 🔖️Report

/// 🩺️ Runs every structural and geometric check and returns every finding. Codes prefixed
/// `warning-` are advisory (self-intersection probe); every other code is an ERROR — strong
/// enough to reject a broken solid outright, not merely note it (ticket goal: "a validator strong
/// enough to reject broken solids").
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate_body(body: &Body) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    check_loop_rings(body, &mut issues);
    check_edge_valence(body, &mut issues);
    check_tolerance_containment(body, &mut issues);
    check_missing_pcurves(body, &mut issues);
    check_same_parameter(body, &mut issues);
    check_shell_closure_and_orientation(body, &mut issues);
    check_solid_orientation(body, &mut issues);
    check_degenerate_geometry(body, &mut issues);
    check_self_intersection_probe(body, &mut issues);
    issues
}

// #region 🔖️Tests

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::euler::{add_face, add_shell, add_solid, make_edge, make_loop, make_vertex};
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn attach_planar_pcurves(body: &mut Body, face: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::FaceId, frame: &Frame3) {
        for coedge_id in body.face_coedges(face) {
            let co = body.coedges.get(coedge_id).unwrap();
            let edge = body.edges.get(co.edge).unwrap();
            let Curve3::Line { origin, dir } = *body.curves3.get(edge.curve).unwrap() else { continue };
            let local_o = frame.to_local(origin);
            let local_d = frame.to_local_vector(dir);
            let pcurve = body.curves2.insert(crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve2::Line {
                origin: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::Pnt2::new(local_o.x, local_o.y),
                dir: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::Vec2::new(local_d.x, local_d.y),
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
    fn build_tetrahedron_with_windings(body: &mut Body, rec: &mut OpRecorder, face_defs: [[usize; 3]; 4]) -> crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::SolidId {
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
            let members: Vec<(crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::EdgeId, bool)> = (0..3)
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
    fn build_tetrahedron(body: &mut Body, rec: &mut OpRecorder) -> crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::SolidId {
        build_tetrahedron_with_windings(body, rec, [[0, 2, 1], [0, 1, 3], [1, 2, 3], [2, 0, 3]])
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn build_tetrahedron_globally_reversed(body: &mut Body, rec: &mut OpRecorder) -> crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::SolidId {
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
            body.coedges.insert(crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Coedge {
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
        let bad_pcurve = body.curves2.insert(crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve2::Line {
            origin: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::Pnt2::new(500.0, 500.0),
            dir: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::Vec2::new(0.0, 0.0),
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
        let a = crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let b = crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let mut faces = body.solid_faces(a);
        faces.extend(body.solid_faces(b));
        let shell = add_shell(&mut body, faces, &mut rec);
        add_solid(&mut body, shell, vec![], &mut rec);
        let issues = validate_body(&body);
        assert!(issues.iter().any(|i| i.code == "warning-possible-self-intersection"), "expected a self-intersection warning, got {issues:?}");
    }
}
//#endregion 🧪️Tests

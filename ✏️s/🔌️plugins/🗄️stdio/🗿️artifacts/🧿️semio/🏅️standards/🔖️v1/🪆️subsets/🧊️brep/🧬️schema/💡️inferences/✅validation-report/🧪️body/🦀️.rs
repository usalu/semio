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

use crate::standards::v1::subsets::brep::schema::engine::Aabb;
use crate::standards::v1::subsets::brep::schema::inferences::bounding_volume::face_aabb;
use crate::standards::v1::subsets::brep::schema::inferences::mass_properties;
use crate::standards::v1::subsets::brep::schema::snapshot::arena::{ArenaId, EdgeId, FaceId};
use crate::standards::v1::subsets::brep::schema::snapshot::curve::curve_ops;
use crate::standards::v1::subsets::brep::schema::snapshot::error::ValidationIssue;
use crate::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::Body;

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
            if let Some((finer, coarser)) = crate::standards::v1::subsets::brep::schema::snapshot::tolerance::check_containment(&format!("vertex-{}", v.raw_index()), vertex.tol, &format!("edge-{}", edge_id.raw_index()), edge.tol) {
                issues.push(ValidationIssue { entity: finer.clone(), code: "tolerance-containment-violated", message: format!("{finer}'s tolerance exceeds its containing {coarser}'s") });
            }
        }
    }
    for (face_id, face) in body.faces.iter() {
        for coedge_id in body.face_coedges(face_id) {
            let Some(coedge) = body.coedges.get(coedge_id) else { continue };
            let Some(edge) = body.edges.get(coedge.edge) else { continue };
            if let Some((finer, coarser)) =
                crate::standards::v1::subsets::brep::schema::snapshot::tolerance::check_containment(&format!("edge-{}", coedge.edge.raw_index()), edge.tol, &format!("face-{}", face_id.raw_index()), face.tol)
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
fn same_parameter_deviation_at(surface: &Surface, pcurve: &crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve2, curve3: &crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve3, prange: (f64, f64), range: (f64, f64), s: f64) -> f64 {
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
    pcurve: &crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve2,
    curve3: &crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve3,
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

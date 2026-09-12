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
            if let Some((finer, coarser)) = crate::standards::v1::subsets::brep::schema::snapshot::tolerance::check_containment(&format!("edge-{}", coedge.edge.raw_index()), edge.tol, &format!("face-{}", face_id.raw_index()), face.tol) {
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
                    message: format!("pcurve and 3D curve disagree by {worst_dev} at s={worst_s} (tol {}; face-{} edge-{} prange {:?} range {:?})", edge.tol.value(), face_id.raw_index(), coedge.edge.raw_index(), coedge.prange, edge.range),
                });
            }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn same_parameter_deviation_at(
    surface: &Surface,
    pcurve: &crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve2,
    curve3: &crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve3,
    prange: (f64, f64),
    range: (f64, f64),
    s: f64,
) -> f64 {
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
/// 🩺️ `true` when `edge_id` is a POINT edge — the standard BREP idiom that closes a parametric
/// rectangle along a collapsed iso-line (a sphere's two poles, `Curve3::Line { dir: ZERO }` with
/// `v0 == v1`). Such an edge carries no traversal direction and bounds no surface strip, so it
/// legitimately appears ONCE in its shell (OCCT's `BRep_Builder::Degenerated` flag, here derived
/// intrinsically from the geometry rather than persisted) and is not a sliver. A full-period seam
/// edge also has `v0 == v1` but a non-zero length, so it stays subject to the ordinary two-use
/// rule.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_point_edge(body: &Body, edge_id: EdgeId) -> bool {
    let Some(edge) = body.edges.get(edge_id) else { return false };
    if edge.v0 != edge.v1 {
        return false;
    }
    let Some(curve) = body.curves3.get(edge.curve) else { return false };
    curve_ops::arc_length(curve, edge.range.0, edge.range.1, 1e-9) <= edge.tol.value()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn check_shell_closure_and_orientation(body: &Body, issues: &mut Vec<ValidationIssue>) {
    for (shell_id, shell) in body.shells.iter() {
        let mut edge_uses: std::collections::HashMap<EdgeId, Vec<bool>> = std::collections::HashMap::new();
        for &face in &shell.faces {
            // A coedge's own `forward` is stated in its SURFACE's natural sense; the face's
            // outward normal is that sense only when `flipped` is false. Two faces of a coherently
            // oriented shell traverse their shared edge oppositely as SEEN FROM OUTSIDE, so the
            // flag to compare is `forward XOR flipped` — comparing the raw flags reported every
            // boolean `Cut` as `orientation-inconsistent`, since a cut flips the tool's faces to
            // face into the cavity and their rings quite correctly stay put.
            let flipped = body.faces.get(face).is_some_and(|f| f.flipped);
            for coedge_id in body.face_coedges(face) {
                if let Some(co) = body.coedges.get(coedge_id) {
                    edge_uses.entry(co.edge).or_default().push(co.forward != flipped);
                }
            }
        }
        for (edge_id, uses) in edge_uses {
            if is_point_edge(body, edge_id) {
                continue;
            }
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

/// 🩺️ Every face's OUTER loop must be counter-clockwise in that face's own `(u, v)`, and every
/// hole loop clockwise — the sense that decides which side of the boundary the trimmed region is
/// on, independently of `flipped` (which only decides the normal). A face written with the
/// backwards circuit is still a closed ring, still passes `check_shell_closure_and_orientation`
/// whenever its `flipped` happens to compensate, and still integrates to the right MAGNITUDE, so
/// this is the only check that sees it: it is exactly what `➡️sweep`'s prism builder produced for
/// every profile edge its cap traversed forward (ticket `26/09/09/PROCEDURAL-3D-END-TO-END`,
/// `📓️sweep-kernel-2026-09-09.md`), and what let a watertight extrusion tessellate inside out.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn check_face_loop_winding(body: &Body, issues: &mut Vec<ValidationIssue>) {
    const PROBE_TOL: f64 = 1e-3;
    for (face_id, face) in body.faces.iter() {
        let Some(surface) = body.surfaces.get(face.surface) else { continue };
        if let Some(outer) = face.outer {
            if let Ok(area) = mass_properties::loop_uv_signed_area(body, outer, surface, PROBE_TOL) {
                if area < 0.0 {
                    issues.push(ValidationIssue { entity: format!("face-{}", face_id.raw_index()), code: "outer-loop-winding-inverted", message: format!("outer loop's (u, v) signed area is negative ({area}) — the loop is clockwise in its own surface, so the face trims to the COMPLEMENT of its region") });
                }
            }
        }
        for &inner in &face.inners {
            if let Ok(area) = mass_properties::loop_uv_signed_area(body, inner, surface, PROBE_TOL) {
                if area > 0.0 {
                    issues.push(ValidationIssue { entity: format!("face-{}-loop-{}", face_id.raw_index(), inner.raw_index()), code: "hole-loop-winding-inverted", message: format!("hole loop's (u, v) signed area is positive ({area}) — a hole must run counter to its outer loop") });
                }
            }
        }
    }
}

/// ⚖️ The shell-volume probe tolerance [`BodyValidationJob`]'s orientation phase integrates at.
const ORIENTATION_PROBE_TOL: f64 = 1e-3;

/// ⚖️ The chord tolerance the sliver-face probe measures area at.
const SLIVER_PROBE_TOL: f64 = 1e-3;

/// 🩺️ ONE edge's degeneracy verdict — the atomic unit of the former `check_degenerate_geometry`'s
/// first loop. A point edge (a pole closing a periodic patch) is legitimate topology and is never
/// flagged.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn degenerate_edge_issue(body: &Body, edge_id: EdgeId) -> Option<ValidationIssue> {
    if is_point_edge(body, edge_id) {
        return None;
    }
    let edge = body.edges.get(edge_id)?;
    let curve = body.curves3.get(edge.curve)?;
    let len = curve_ops::arc_length(curve, edge.range.0, edge.range.1, 1e-9);
    (len < edge.tol.value()).then(|| ValidationIssue { entity: format!("edge-{}", edge_id.raw_index()), code: "degenerate-edge", message: format!("edge length {len} is below its own tolerance {}", edge.tol.value()) })
}

/// 🩺️ ONE face's sliver verdict — the atomic unit of the former `check_degenerate_geometry`'s
/// second loop, and the dearest single call in the whole validator (`face_area` tessellates the
/// trimmed patch): measured at 1.6 s for the three faces of `🍩️sphere-cut-with-torus` on a native
/// debug build, which is precisely why it is a step boundary and not part of a whole-body pass
/// (ticket `26/09/09/PROCEDURAL-3D-END-TO-END`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sliver_face_issue(body: &Body, face_id: FaceId) -> Option<ValidationIssue> {
    let face = body.faces.get(face_id)?;
    let area = mass_properties::face_area(body, face_id, SLIVER_PROBE_TOL).ok()?;
    let tol2 = face.tol.value() * face.tol.value();
    (area < tol2).then(|| ValidationIssue { entity: format!("face-{}", face_id.raw_index()), code: "sliver-face", message: format!("face area {area} is below tol² ({tol2})") })
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
///
/// ⏱️ Unbudgeted façade over [`BodyValidationJob`], the ONE implementation — exactly the relation
/// `tessellate_solid` has to `TessellationJob` (see that type's own doc): there is no second pass
/// to drift from, so a caller with an interactive ceiling and a caller with none agree by
/// construction.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate_body(body: &Body) -> Vec<ValidationIssue> {
    BodyValidationJob::new(body).run_to_completion(body)
}

// #endregion 🔖️Report

// #region ⏱️ResumableValidation

/// ⏱️ What a [`BodyValidationJob`] is currently checking. Phases run in declaration order and
/// reproduce [`validate_body`]'s historical check order exactly; `Complete` is terminal.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BodyValidationPhase {
    /// 🔗 Every loop's coedge ring, in one whole-body unit (measured in microseconds).
    #[default]
    LoopRings,
    /// 🪢 Edge valence, one whole-body unit.
    EdgeValence,
    /// 📏 Tolerance containment, one whole-body unit.
    ToleranceContainment,
    /// 🗺️ Missing pcurves, one whole-body unit.
    MissingPcurves,
    /// 📐 Same-parameter agreement, one whole-body unit.
    SameParameter,
    /// 🐚 Shell closure and orientation, one whole-body unit.
    ShellClosure,
    /// 🔄 Face loop winding, one whole-body unit.
    FaceLoopWinding,
    /// ⚖️ Shell signed volumes, ONE FACE per unit — the divergence-theorem sum is accumulated face
    /// by face so a solid whose shell costs seconds never costs them inside one step.
    SolidOrientation,
    /// 🖇️ Degenerate edges, ONE EDGE per unit.
    DegenerateEdges,
    /// 🔺 Sliver faces, ONE FACE per unit.
    DegenerateFaces,
    /// 🪞 The self-intersection probe, one whole-body unit.
    SelfIntersection,
    Complete,
}

impl BodyValidationPhase {
    /// 🏷️ The stable wire tag every host/extension/UI layer names this phase by.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn tag(self) -> &'static str {
        match self {
            Self::LoopRings => "loopRings",
            Self::EdgeValence => "edgeValence",
            Self::ToleranceContainment => "toleranceContainment",
            Self::MissingPcurves => "missingPcurves",
            Self::SameParameter => "sameParameter",
            Self::ShellClosure => "shellClosure",
            Self::FaceLoopWinding => "faceLoopWinding",
            Self::SolidOrientation => "solidOrientation",
            Self::DegenerateEdges => "degenerateEdges",
            Self::DegenerateFaces => "degenerateFaces",
            Self::SelfIntersection => "selfIntersection",
            Self::Complete => "complete",
        }
    }
}

/// 📈 Monotone progress of one resumable validation. `units_total` is fixed at construction, so
/// the ratio a surface paints never moves backwards.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BodyValidationProgress {
    pub units_done: usize,
    pub units_total: usize,
    pub phase: BodyValidationPhase,
}

/// ⚖️ One shell whose signed volume the orientation phase accumulates face by face.
struct ShellVolumeUnit {
    solid: crate::standards::v1::subsets::brep::schema::snapshot::arena::SolidId,
    shell: crate::standards::v1::subsets::brep::schema::snapshot::arena::ShellId,
    outer: bool,
    faces: Vec<FaceId>,
    /// ⚖️ `None` once any face refused to integrate — the same "skip this shell entirely" verdict
    /// the whole-shell `shell_signed_volume(..)?` produced before.
    total: Option<f64>,
}

/// ⏱️ [`validate_body`] split into budgetable units so a host can run it inside an interactive
/// step ceiling across many turns, report progress, and never block a worker's event loop long
/// enough for a liveness watchdog to read the silence as death.
///
/// The unit is one whole cheap check, or ONE face / ONE edge inside the two checks that dominate
/// the cost (`solidOrientation`'s per-face volume integral and `degenerateFaces`' per-face area).
/// A pathological single face still costs one whole unit — the same bound
/// `TessellationJob`'s own doc states, for the same reason: abandoning a face mid-quadrature would
/// throw its work away.
pub struct BodyValidationJob {
    phase: BodyValidationPhase,
    issues: Vec<ValidationIssue>,
    shells: Vec<ShellVolumeUnit>,
    shell_cursor: usize,
    shell_face_cursor: usize,
    edges: Vec<EdgeId>,
    edge_cursor: usize,
    faces: Vec<FaceId>,
    face_cursor: usize,
    cheap_done: usize,
    units_total: usize,
}

impl BodyValidationJob {
    /// 🧪 Plans one validation of `body`. The plan (which shells, edges and faces exist) is read
    /// ONCE here, so `units_total` is fixed and the caller may re-present the same body on every
    /// step without the total moving.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn new(body: &Body) -> Self {
        let mut shells = Vec::new();
        for (solid_id, solid) in body.solids.iter() {
            shells.push(ShellVolumeUnit { solid: solid_id, shell: solid.outer, outer: true, faces: body.shell_faces(solid.outer), total: Some(0.0) });
            for &void_shell in &solid.inners {
                shells.push(ShellVolumeUnit { solid: solid_id, shell: void_shell, outer: false, faces: body.shell_faces(void_shell), total: Some(0.0) });
            }
        }
        let edges: Vec<EdgeId> = body.edges.iter().map(|(id, _)| id).collect();
        let faces: Vec<FaceId> = body.faces.iter().map(|(id, _)| id).collect();
        let shell_faces: usize = shells.iter().map(|unit| unit.faces.len()).sum();
        let units_total = CHEAP_CHECK_UNITS + shell_faces + edges.len() + faces.len();
        Self { phase: BodyValidationPhase::LoopRings, issues: Vec::new(), shells, shell_cursor: 0, shell_face_cursor: 0, edges, edge_cursor: 0, faces, face_cursor: 0, cheap_done: 0, units_total }
    }

    /// 📈 This job's progress right now — safe to read between steps and after termination.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn progress(&self) -> BodyValidationProgress {
        let shell_faces_done: usize = self.shells.iter().take(self.shell_cursor).map(|unit| unit.faces.len()).sum::<usize>() + self.shell_face_cursor;
        BodyValidationProgress { units_done: (self.cheap_done + shell_faces_done + self.edge_cursor + self.face_cursor).min(self.units_total), units_total: self.units_total, phase: self.phase }
    }

    /// ✅ True once every unit has run and [`Self::into_issues`] is final.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn is_complete(&self) -> bool {
        matches!(self.phase, BodyValidationPhase::Complete)
    }

    /// 🩺️ Every finding so far. Only final once [`Self::is_complete`] answers true.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn into_issues(self) -> Vec<ValidationIssue> {
        self.issues
    }

    /// ♾️ Runs every remaining unit in one call — the unbudgeted façade [`validate_body`] is.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn run_to_completion(mut self, body: &Body) -> Vec<ValidationIssue> {
        while !self.is_complete() {
            self.step(body, usize::MAX);
        }
        self.into_issues()
    }

    /// ⏱️ Advances by at most `budget` units. A `budget` of zero is a legal progress probe that
    /// performs no work.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn step(&mut self, body: &Body, budget: usize) -> BodyValidationProgress {
        let mut spent = 0usize;
        while spent < budget && !self.is_complete() {
            match self.phase {
                BodyValidationPhase::LoopRings => self.cheap(body, check_loop_rings, BodyValidationPhase::EdgeValence),
                BodyValidationPhase::EdgeValence => self.cheap(body, check_edge_valence, BodyValidationPhase::ToleranceContainment),
                BodyValidationPhase::ToleranceContainment => self.cheap(body, check_tolerance_containment, BodyValidationPhase::MissingPcurves),
                BodyValidationPhase::MissingPcurves => self.cheap(body, check_missing_pcurves, BodyValidationPhase::SameParameter),
                BodyValidationPhase::SameParameter => self.cheap(body, check_same_parameter, BodyValidationPhase::ShellClosure),
                BodyValidationPhase::ShellClosure => self.cheap(body, check_shell_closure_and_orientation, BodyValidationPhase::FaceLoopWinding),
                BodyValidationPhase::FaceLoopWinding => self.cheap(body, check_face_loop_winding, BodyValidationPhase::SolidOrientation),
                BodyValidationPhase::SolidOrientation => {
                    if self.shell_cursor >= self.shells.len() {
                        self.emit_orientation_issues();
                        self.phase = BodyValidationPhase::DegenerateEdges;
                        continue;
                    }
                    let unit = &mut self.shells[self.shell_cursor];
                    if self.shell_face_cursor >= unit.faces.len() {
                        if unit.faces.is_empty() {
                            unit.total = None;
                        }
                        self.shell_cursor += 1;
                        self.shell_face_cursor = 0;
                        continue;
                    }
                    let face = unit.faces[self.shell_face_cursor];
                    match mass_properties::face_volume_contribution(body, face, ORIENTATION_PROBE_TOL) {
                        Ok(contribution) => {
                            if let Some(total) = unit.total.as_mut() {
                                *total += contribution;
                            }
                        }
                        Err(_) => unit.total = None,
                    }
                    self.shell_face_cursor += 1;
                }
                BodyValidationPhase::DegenerateEdges => {
                    if self.edge_cursor >= self.edges.len() {
                        self.phase = BodyValidationPhase::DegenerateFaces;
                        continue;
                    }
                    if let Some(issue) = degenerate_edge_issue(body, self.edges[self.edge_cursor]) {
                        self.issues.push(issue);
                    }
                    self.edge_cursor += 1;
                }
                BodyValidationPhase::DegenerateFaces => {
                    if self.face_cursor >= self.faces.len() {
                        self.phase = BodyValidationPhase::SelfIntersection;
                        continue;
                    }
                    if let Some(issue) = sliver_face_issue(body, self.faces[self.face_cursor]) {
                        self.issues.push(issue);
                    }
                    self.face_cursor += 1;
                }
                BodyValidationPhase::SelfIntersection => self.cheap(body, check_self_intersection_probe, BodyValidationPhase::Complete),
                BodyValidationPhase::Complete => break,
            }
            spent += 1;
        }
        self.progress()
    }

    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    fn cheap(&mut self, body: &Body, check: fn(&Body, &mut Vec<ValidationIssue>), next: BodyValidationPhase) {
        check(body, &mut self.issues);
        self.cheap_done += 1;
        self.phase = next;
    }

    /// ⚖️ Emits the orientation verdicts once every shell's per-face sum is complete, in the exact
    /// solid-then-void order the whole-body check produced.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    fn emit_orientation_issues(&mut self) {
        let mut index = 0usize;
        while index < self.shells.len() {
            let Some(outer_v) = self.shells[index].total.filter(|_| self.shells[index].outer) else {
                index += 1;
                continue;
            };
            let solid = self.shells[index].solid;
            if outer_v < 0.0 {
                self.issues.push(ValidationIssue { entity: format!("solid-{}", solid.raw_index()), code: "shell-orientation-inward", message: format!("outer shell's signed volume is negative ({outer_v}); face normals appear to point inward") });
            }
            index += 1;
            while index < self.shells.len() && !self.shells[index].outer {
                if let Some(void_v) = self.shells[index].total {
                    if outer_v.signum() == void_v.signum() {
                        let void_shell = self.shells[index].shell;
                        self.issues.push(ValidationIssue {
                            entity: format!("solid-{}-void-shell-{}", solid.raw_index(), void_shell.raw_index()),
                            code: "void-shell-not-inverted",
                            message: "void (inner) shell's signed volume has the same sign as the outer shell — it should be inverted relative to the solid's exterior".to_string(),
                        });
                    }
                }
                index += 1;
            }
        }
    }
}

/// 🔢 How many whole-body units the cheap checks contribute to `units_total` — the phases that
/// measured in microseconds and are therefore not worth a per-entity cursor.
const CHEAP_CHECK_UNITS: usize = 8;

// #endregion ⏱️ResumableValidation

// #region 🔖️Tests

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

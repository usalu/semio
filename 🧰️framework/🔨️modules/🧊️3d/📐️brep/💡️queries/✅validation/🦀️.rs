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

use crate::brep::engine::Aabb;
use crate::brep::queries::bounding_volume::face_aabb;
use crate::brep::queries::mass_properties;
use crate::brep::representation::arena::{ArenaId, CoedgeId, EdgeId, FaceId, LoopId, VertexId};
use crate::brep::representation::curve::curve_ops;
use crate::brep::representation::error::ValidationIssue;
use crate::brep::representation::surface::Surface;
use crate::brep::representation::topology::{Body, ReachSet};
use crate::brep::representation::vector::Pnt3;

#[cfg(test)]
std::thread_local! { static LOOP_PROBES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }
#[cfg(test)]
std::thread_local! { static PCURVE_PROBES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }
#[cfg(test)]
std::thread_local! { static TOLERANCE_PROBES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }
#[cfg(test)]
std::thread_local! { static SAME_PARAMETER_SAMPLES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }

// #region 🔖️Topology

fn check_loop_pair(body: &Body, loop_id: LoopId, ring: &[CoedgeId], index: usize, issues: &mut Vec<ValidationIssue>) {
    #[cfg(test)]
    LOOP_PROBES.with(|probes| probes.set(probes.get() + 1));
    let next = (index + 1) % ring.len();
    let Some((_, end_a)) = body.coedge_endpoints(ring[index]) else { return; };
    let Some((start_b, _)) = body.coedge_endpoints(ring[next]) else { return; };
    if end_a != start_b {
        issues.push(ValidationIssue { entity: format!("loop-{}", loop_id.raw_index()), code: "loop-not-closed", message: format!("coedge {index} ends at a different vertex than coedge {next} starts at") });
    }
    let coedge_a = body.coedges.get(ring[index]).unwrap();
    let coedge_b = body.coedges.get(ring[next]).unwrap();
    if coedge_a.next != ring[next] || coedge_b.prev != ring[index] {
        issues.push(ValidationIssue { entity: format!("loop-{}", loop_id.raw_index()), code: "next-prev-mismatch", message: format!("coedge {index}'s next/prev pointers are not symmetric with its ring neighbor") });
    }
}

// #endregion 🔖️Topology

// #region 🔖️Geometry

fn check_tolerance_pair(finer: String, finer_tol: crate::brep::representation::tolerance::Tol, coarser: String, coarser_tol: crate::brep::representation::tolerance::Tol, issues: &mut Vec<ValidationIssue>) {
    #[cfg(test)]
    TOLERANCE_PROBES.with(|probes| probes.set(probes.get() + 1));
    if let Some((finer, coarser)) = crate::brep::representation::tolerance::check_containment(&finer, finer_tol, &coarser, coarser_tol) {
        issues.push(ValidationIssue { entity: finer.clone(), code: "tolerance-containment-violated", message: format!("{finer}'s tolerance exceeds its containing {coarser}'s") });
    }
}

/// 🩺️ Every coedge must carry a p-curve — trims and same-parameter checks below silently could
/// not verify a coedge without one, so a missing p-curve is an ERROR here, not a skip (audit
/// §6.12: "missing p-curves are skipped rather than rejected or repaired").
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn check_missing_pcurve(coedge_id: CoedgeId, coedge: &crate::brep::representation::topology::Coedge, issues: &mut Vec<ValidationIssue>) {
    #[cfg(test)]
    PCURVE_PROBES.with(|probes| probes.set(probes.get() + 1));
    if coedge.pcurve.is_none() {
        issues.push(ValidationIssue { entity: format!("coedge-{}", coedge_id.raw_index()), code: "missing-pcurve", message: "coedge has no p-curve — every coedge must carry one for trim/same-parameter validation".to_string() });
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn same_parameter_deviation_at(
    surface: &Surface,
    pcurve: &crate::brep::representation::curve::Curve2,
    curve3: &crate::brep::representation::curve::Curve3,
    prange: (f64, f64),
    range: (f64, f64),
    s: f64,
) -> f64 {
    #[cfg(test)]
    SAME_PARAMETER_SAMPLES.with(|samples| samples.set(samples.get() + 1));
    let p = prange.0 + (prange.1 - prange.0) * s;
    let t = range.0 + (range.1 - range.0) * s;
    let uv = pcurve.eval(p);
    let via_surface = surface.eval(uv.x, uv.y);
    let via_curve = curve3.eval(t);
    via_surface.distance(via_curve)
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
pub(crate) fn is_point_edge(body: &Body, edge_id: EdgeId) -> bool {
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

/// ⚖️ The chord tolerance the sliver-face probe measures area at — deliberately coarse: the verdict
/// is `area < tol²` (`1e-14 m²`), so a chordal polygon a few percent short of the true area answers
/// it exactly as well, while the `1e-3` it used to be drove the adaptive quadrature to depth 6 on
/// every cylindrical face (seconds per bore on a debug build, the whole validator's cost).
const SLIVER_PROBE_TOL: f64 = 1e-2;

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

/// 🩺️ The positions of the vertices two faces share TOPOLOGICALLY (the same `VertexId` on both
/// boundaries) — legitimate contact the self-intersection probe must not read as a collision: any
/// vertex of valence four or more (a notch corner, a hexagonal column meeting a beam) joins faces
/// that touch there without sharing an edge.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn shared_vertex_positions(body: &Body, a: FaceId, b: FaceId) -> Vec<Pnt3> {
    let vertices_of = |face: FaceId| -> std::collections::HashSet<VertexId> { body.face_coedges(face).into_iter().filter_map(|c| body.coedges.get(c).and_then(|co| body.edges.get(co.edge))).flat_map(|edge| [edge.v0, edge.v1]).collect() };
    let vertices_a = vertices_of(a);
    vertices_of(b).into_iter().filter(|vertex| vertices_a.contains(vertex)).filter_map(|vertex| body.vertices.get(vertex).map(|v| v.position)).collect()
}

/// 🩺️ Self-intersection PROBE (not a certified global check): for every pair of non-adjacent
/// faces on the same solid whose AABBs overlap, samples each face's boundary/interior points
/// (`mass_properties::face_sample_points`) and flags a Warning when the closest pair comes within
/// tolerance — cheap enough to run always, catches the common case (audit §6.12: "general
/// self-intersection is not fully checked"). Samples sitting on a vertex both faces share are
/// exempt: that contact is the shared topology itself, not an intersection.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn check_self_intersection_probe(body: &Body, issues: &mut Vec<ValidationIssue>) {
    const PROBE_TOL: f64 = 1e-6;
    for (solid_id, _) in body.solids.iter() {
        let faces = body.solid_faces(solid_id);
        let aabbs: Vec<Option<Aabb>> = faces.iter().map(|&face| face_aabb(body, face).ok()).collect();
        let samples: Vec<Option<Vec<Pnt3>>> = faces.iter().map(|&face| mass_properties::face_sample_points(body, face).ok()).collect();
        for i in 0..faces.len() {
            for j in (i + 1)..faces.len() {
                let (fa, fb) = (faces[i], faces[j]);
                let (Some(aabb_a), Some(aabb_b)) = (&aabbs[i], &aabbs[j]) else { continue };
                if !aabb_overlaps(aabb_a, aabb_b) || faces_share_edge(body, fa, fb) {
                    continue;
                }
                let (Some(pa), Some(pb)) = (&samples[i], &samples[j]) else { continue };
                let shared = shared_vertex_positions(body, fa, fb);
                let on_shared_vertex = |point: &Pnt3| shared.iter().any(|vertex| vertex.distance(*point) < PROBE_TOL);
                let mut best = f64::INFINITY;
                for p in pa.iter().filter(|p| !on_shared_vertex(p)) {
                    for q in pb.iter().filter(|q| !on_shared_vertex(q)) {
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

/// 🎯️ Whether one [`ValidationIssue`]'s entity label names anything inside `reach`.
///
/// A label is a run of `<store>-<raw index>` pairs — `"edge-3"`, `"solid-11-void-shell-12"`,
/// `"face-4-face-9"` — and the issue belongs to a shape when ANY pair it names is reachable from
/// it. A label none of whose pairs can be read is treated as belonging to every shape: a verdict
/// must refuse a shape it cannot vouch for, and an unreadable diagnostic is not a clean bill of
/// health (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn issue_reaches(reach: &ReachSet, label: &str) -> bool {
    let parts: Vec<&str> = label.split('-').collect();
    let mut readable = false;
    for pair in parts.windows(2) {
        let Ok(index) = pair[1].parse::<u32>() else { continue };
        let matched = match pair[0] {
            "vertex" => reach.vertices.iter().any(|id| id.raw_index() == index),
            "edge" => reach.edges.iter().any(|id| id.raw_index() == index),
            "coedge" => reach.coedges.iter().any(|id| id.raw_index() == index),
            "loop" => reach.loops.iter().any(|id| id.raw_index() == index),
            "face" => reach.faces.iter().any(|id| id.raw_index() == index),
            "shell" => reach.shells.iter().any(|id| id.raw_index() == index),
            "solid" => reach.solids.iter().any(|id| id.raw_index() == index),
            "curve" => reach.curves3.iter().any(|id| id.raw_index() == index),
            "surface" => reach.surfaces.iter().any(|id| id.raw_index() == index),
            _ => continue,
        };
        readable = true;
        if matched {
            return true;
        }
    }
    !readable
}

// #endregion 🔖️Report

// #region ⏱️ResumableValidation

/// ⏱️ What a [`BodyValidationJob`] is currently checking. Phases run in declaration order and
/// reproduce [`validate_body`]'s historical check order exactly; `Complete` is terminal.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BodyValidationPhase {
    /// 🎟️ One original arena slot or shell member per granted turn.
    #[default]
    CollectingTopology,
    /// 🔗 One original ring slot/member or neighboring coedge probe per unit.
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
            Self::CollectingTopology => "collectingTopology",
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

/// 📈 Original funded work completed; remaining work grows as cold topology is discovered.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BodyValidationProgress {
    pub units_done: usize,
    pub units_total: usize,
    pub phase: BodyValidationPhase,
}

/// ⚖️ One shell whose signed volume the orientation phase accumulates face by face.
struct ShellVolumeUnit {
    solid: crate::brep::representation::arena::SolidId,
    shell: crate::brep::representation::arena::ShellId,
    outer: bool,
    faces: Vec<FaceId>,
    /// ⚖️ `None` once any face refused to integrate — the same "skip this shell entirely" verdict
    /// the whole-shell `shell_signed_volume(..)?` produced before.
    total: Option<f64>,
}

#[derive(Default)]
struct FaceCoedgeCursor {
    face_slot: usize,
    face: Option<FaceId>,
    loop_slot: usize,
    start: Option<CoedgeId>,
    next: Option<CoedgeId>,
    visited: usize,
}
enum FaceCoedgeTurn { Pending, Coedge(FaceId, CoedgeId), Complete }
impl FaceCoedgeCursor {
    fn step(&mut self, body: &Body, live_coedges: usize) -> FaceCoedgeTurn {
        if let Some(coedge) = self.next {
            self.next = body.coedges.get(coedge).map(|value| value.next);
            self.visited += 1;
            if self.next == self.start || self.visited > live_coedges { self.next = None; }
            return FaceCoedgeTurn::Coedge(self.face.expect("original face ring admitted"), coedge);
        }
        if self.start.take().is_some() { return FaceCoedgeTurn::Pending; }
        if let Some(id) = self.face {
            let Some(face) = body.faces.get(id) else { self.face = None; return FaceCoedgeTurn::Pending; };
            if self.loop_slot > face.inners.len() { self.face = None; return FaceCoedgeTurn::Pending; }
            let loop_id = if self.loop_slot == 0 { face.outer } else { face.inners.get(self.loop_slot - 1).copied() };
            self.loop_slot += 1;
            self.start = loop_id.and_then(|id| body.loops.get(id)).map(|value| value.first);
            self.next = self.start;
            self.visited = 0;
            return FaceCoedgeTurn::Pending;
        }
        if self.face_slot >= body.faces.slot_count() { return FaceCoedgeTurn::Complete; }
        self.face = body.faces.slot_at(self.face_slot).map(|(id, _)| id);
        self.face_slot += 1;
        self.loop_slot = 0;
        FaceCoedgeTurn::Pending
    }
}

/// ⏱️ [`validate_body`] split into budgetable units so a host can run it inside an interactive
/// step ceiling across many turns, report progress, and never block a worker's event loop long
/// enough for a liveness watchdog to read the silence as death.
///
/// Cold planning, loop validation, edge valence, tolerance and pcurve presence borrow one original slot, member or coedge pair per unit.
/// Same-parameter validation evaluates at most one original geometry sample per unit; arbitrary
/// curve/surface evaluators still own their internal work and allocation costs.
/// Remaining geometric phases still invoke whole checks or whole-face quadrature; their units
/// describe scheduling progress and do not establish a strict interactive work ceiling.
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
    units_done: usize,
    units_total: usize,
    planning_stage: u8,
    planning_slot: usize,
    planning_solid: Option<crate::brep::representation::arena::SolidId>,
    planning_shell: usize,
    planning_face: Option<usize>,
    live_coedges: usize,
    loop_slot: usize,
    loop_id: Option<LoopId>,
    ring: Vec<CoedgeId>,
    ring_next: Option<CoedgeId>,
    ring_probe: usize,
    valence_edge_slot: usize,
    valence_edge: Option<EdgeId>,
    valence_coedge_slot: usize,
    valence_uses: usize,
    pcurve_slot: usize,
    tolerance_stage: u8,
    tolerance_edge_cursor: usize,
    tolerance_vertex: usize,
    tolerance_faces: FaceCoedgeCursor,
    same_faces: FaceCoedgeCursor,
    same_pair: Option<(FaceId, CoedgeId)>,
    same_stage: u8,
    same_cursor: usize,
    same_pass: usize,
    same_samples: Vec<(f64, f64)>,
    same_midpoints: Vec<f64>,
}

impl BodyValidationJob {
    /// 🧊 Admits an empty original frontier; topology is borrowed only inside funded steps.
    pub fn new(_body: &Body) -> Self {
        Self { phase: BodyValidationPhase::CollectingTopology, issues: Vec::new(), shells: Vec::new(), shell_cursor: 0, shell_face_cursor: 0, edges: Vec::new(), edge_cursor: 0, faces: Vec::new(), face_cursor: 0, units_done: 0, units_total: 1, planning_stage: 0, planning_slot: 0, planning_solid: None, planning_shell: 0, planning_face: None, live_coedges: 0, loop_slot: 0, loop_id: None, ring: Vec::new(), ring_next: None, ring_probe: 0, valence_edge_slot: 0, valence_edge: None, valence_coedge_slot: 0, valence_uses: 0, pcurve_slot: 0, tolerance_stage: 0, tolerance_edge_cursor: 0, tolerance_vertex: 0, tolerance_faces: FaceCoedgeCursor::default(), same_faces: FaceCoedgeCursor::default(), same_pair: None, same_stage: 0, same_cursor: 0, same_pass: 0, same_samples: Vec::new(), same_midpoints: Vec::new() }
    }

    /// 📈 Constant-time funded work progress, including cold planning turns.
    pub fn progress(&self) -> BodyValidationProgress {
        BodyValidationProgress { units_done: self.units_done, units_total: self.units_total, phase: self.phase }
    }

    fn probe_valence(&mut self, body: &Body) {
        let Some(edge) = self.valence_edge else {
            if self.valence_edge_slot >= body.edges.slot_count() { self.phase = BodyValidationPhase::ToleranceContainment; return; }
            self.valence_edge = body.edges.slot_at(self.valence_edge_slot).map(|(id, _)| id);
            self.valence_edge_slot += 1;
            self.valence_coedge_slot = 0;
            self.valence_uses = 0;
            return;
        };
        if self.valence_coedge_slot < body.coedges.slot_count() {
            if let Some((_, coedge)) = body.coedges.slot_at(self.valence_coedge_slot) {
                self.valence_uses += usize::from(crate::brep::representation::topology::coedge_uses_edge(coedge, edge));
            }
            self.valence_coedge_slot += 1;
        } else {
            let valence = self.valence_uses;
            if valence > 2 { self.issues.push(ValidationIssue { entity: format!("edge-{}", edge.raw_index()), code: "non-manifold-edge", message: format!("edge is used by {valence} coedges (2-manifold shapes use at most 2)") }); }
            self.valence_edge = None;
        }
    }

    fn probe_pcurve_presence(&mut self, body: &Body) {
        if self.pcurve_slot >= body.coedges.slot_count() { self.phase = BodyValidationPhase::SameParameter; return; }
        if let Some((id, coedge)) = body.coedges.slot_at(self.pcurve_slot) { check_missing_pcurve(id, coedge, &mut self.issues); }
        self.pcurve_slot += 1;
    }

    /// 🩺️ Original sixteen-interval sampler and three adaptive refinements, one probe per turn.
    fn probe_same_parameter(&mut self, body: &Body) {
        let Some((face_id, coedge_id)) = self.same_pair else {
            match self.same_faces.step(body, self.live_coedges) {
                FaceCoedgeTurn::Pending => {},
                FaceCoedgeTurn::Complete => self.phase = BodyValidationPhase::ShellClosure,
                FaceCoedgeTurn::Coedge(face, coedge) => {
                    self.same_pair = Some((face, coedge));
                    self.same_stage = 0;
                    self.same_cursor = 0;
                    self.same_pass = 0;
                    self.same_samples.clear();
                    self.same_midpoints.clear();
                }
            }
            return;
        };
        let refs = body.faces.get(face_id).and_then(|face| body.surfaces.get(face.surface)).zip(body.coedges.get(coedge_id)).and_then(|(surface, coedge)| {
            let pcurve = body.curves2.get(coedge.pcurve?)?;
            let edge = body.edges.get(coedge.edge)?;
            let curve = body.curves3.get(edge.curve)?;
            Some((surface, coedge, pcurve, edge, curve))
        });
        let Some((surface, coedge, pcurve, edge, curve)) = refs else { self.same_pair = None; return; };
        match self.same_stage {
            0 => {
                if self.same_cursor > 16 { self.same_stage = 1; self.same_cursor = 0; return; }
                let s = self.same_cursor as f64 / 16.0;
                self.same_samples.push((s, same_parameter_deviation_at(surface, pcurve, curve, coedge.prange, edge.range, s)));
                self.same_cursor += 1;
            }
            1 => {
                if self.same_cursor + 1 >= self.same_samples.len() {
                    self.same_stage = if self.same_midpoints.is_empty() { 3 } else { 2 };
                    self.same_cursor = 0;
                    return;
                }
                let (s0, d0) = self.same_samples[self.same_cursor];
                let (s1, d1) = self.same_samples[self.same_cursor + 1];
                if (d1 - d0).abs() > d0.max(d1).max(1e-12) * 0.5 { self.same_midpoints.push(0.5 * (s0 + s1)); }
                self.same_cursor += 1;
            }
            2 => {
                if let Some(&s) = self.same_midpoints.get(self.same_cursor) {
                    self.same_samples.push((s, same_parameter_deviation_at(surface, pcurve, curve, coedge.prange, edge.range, s)));
                    self.same_cursor += 1;
                } else {
                    self.same_samples.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
                    self.same_midpoints.clear();
                    self.same_pass += 1;
                    self.same_stage = if self.same_pass == 3 { 3 } else { 1 };
                    self.same_cursor = 0;
                }
            }
            _ => {
                if let Some(&(worst_s, worst_dev)) = self.same_samples.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)) {
                    if worst_dev > edge.tol.value() {
                        self.issues.push(ValidationIssue { entity: format!("coedge-{}", coedge_id.raw_index()), code: "same-parameter-violated", message: format!("pcurve and 3D curve disagree by {worst_dev} at s={worst_s} (tol {}; face-{} edge-{} prange {:?} range {:?})", edge.tol.value(), face_id.raw_index(), coedge.edge.raw_index(), coedge.prange, edge.range) });
                    }
                }
                self.same_pair = None;
            }
        }
    }

    fn probe_tolerance(&mut self, body: &Body) {
        if self.tolerance_stage == 0 {
            let Some(&edge_id) = self.edges.get(self.tolerance_edge_cursor) else { self.tolerance_stage = 1; return; };
            if self.tolerance_vertex >= 2 { self.tolerance_edge_cursor += 1; self.tolerance_vertex = 0; return; }
            let endpoint = self.tolerance_vertex;
            self.tolerance_vertex += 1;
            let Some(edge) = body.edges.get(edge_id) else { return; };
            let vertex_id = [edge.v0, edge.v1][endpoint];
            if let Some(vertex) = body.vertices.get(vertex_id) { check_tolerance_pair(format!("vertex-{}", vertex_id.raw_index()), vertex.tol, format!("edge-{}", edge_id.raw_index()), edge.tol, &mut self.issues); }
            return;
        }
        match self.tolerance_faces.step(body, self.live_coedges) {
            FaceCoedgeTurn::Pending => {},
            FaceCoedgeTurn::Complete => self.phase = BodyValidationPhase::MissingPcurves,
            FaceCoedgeTurn::Coedge(face_id, coedge_id) => {
                let Some(face) = body.faces.get(face_id) else { return; };
                let Some(coedge) = body.coedges.get(coedge_id) else { return; };
                let Some(edge) = body.edges.get(coedge.edge) else { return; };
                check_tolerance_pair(format!("edge-{}", coedge.edge.raw_index()), edge.tol, format!("face-{}", face_id.raw_index()), face.tol, &mut self.issues);
            }
        }
    }

    fn collect_topology(&mut self, body: &Body) {
        match self.planning_stage {
            0 => {
                if let Some(solid_id) = self.planning_solid {
                    let Some(solid) = body.solids.get(solid_id) else { self.planning_solid = None; self.planning_slot += 1; return; };
                    let shell_id = if self.planning_shell == 0 { Some(solid.outer) } else { solid.inners.get(self.planning_shell - 1).copied() };
                    let Some(shell_id) = shell_id else { self.planning_solid = None; self.planning_slot += 1; return; };
                    if let Some(index) = self.planning_face {
                        if let Some(face) = body.shells.get(shell_id).and_then(|shell| shell.faces.get(index)).copied() {
                            self.shells.last_mut().expect("original shell admitted").faces.push(face);
                            self.planning_face = Some(index + 1);
                        } else { self.planning_face = None; self.planning_shell += 1; }
                    } else {
                        self.shells.push(ShellVolumeUnit { solid: solid_id, shell: shell_id, outer: self.planning_shell == 0, faces: Vec::new(), total: Some(0.0) });
                        self.planning_face = Some(0);
                    }
                } else if self.planning_slot < body.solids.slot_count() {
                    self.planning_solid = body.solids.slot_at(self.planning_slot).map(|(id, _)| id);
                    self.planning_shell = 0;
                    if self.planning_solid.is_none() { self.planning_slot += 1; }
                } else { self.planning_stage = 1; self.planning_slot = 0; }
            }
            1 => {
                if self.planning_slot < body.edges.slot_count() {
                    if let Some((id, _)) = body.edges.slot_at(self.planning_slot) { self.edges.push(id); }
                    self.planning_slot += 1;
                } else { self.planning_stage = 2; self.planning_slot = 0; }
            }
            2 => {
                if self.planning_slot < body.faces.slot_count() {
                    if let Some((id, _)) = body.faces.slot_at(self.planning_slot) { self.faces.push(id); }
                    self.planning_slot += 1;
                } else { self.planning_stage = 3; self.planning_slot = 0; }
            }
            3 => {
                if self.planning_slot < body.coedges.slot_count() {
                    self.live_coedges += usize::from(body.coedges.slot_at(self.planning_slot).is_some());
                    self.planning_slot += 1;
                } else { self.phase = BodyValidationPhase::LoopRings; }
            }
            _ => unreachable!(),
        }
    }

    fn probe_loop(&mut self, body: &Body) {
        let Some(loop_id) = self.loop_id else {
            if self.loop_slot >= body.loops.slot_count() { self.phase = BodyValidationPhase::EdgeValence; return; }
            if let Some((id, lp)) = body.loops.slot_at(self.loop_slot) {
                self.loop_id = Some(id);
                self.ring_next = Some(lp.first);
                self.ring_probe = 0;
            }
            self.loop_slot += 1;
            return;
        };
        if let Some(current) = self.ring_next.take() {
            self.ring.push(current);
            if let Some(coedge) = body.coedges.get(current) {
                if self.ring.len() <= self.live_coedges && self.ring.first().copied() != Some(coedge.next) { self.ring_next = Some(coedge.next); }
            }
            return;
        }
        if self.ring_probe < self.ring.len() {
            check_loop_pair(body, loop_id, &self.ring, self.ring_probe, &mut self.issues);
            self.ring_probe += 1;
        } else { self.ring.clear(); self.loop_id = None; }
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
                BodyValidationPhase::CollectingTopology => self.collect_topology(body),
                BodyValidationPhase::LoopRings => self.probe_loop(body),
                BodyValidationPhase::EdgeValence => self.probe_valence(body),
                BodyValidationPhase::ToleranceContainment => self.probe_tolerance(body),
                BodyValidationPhase::MissingPcurves => self.probe_pcurve_presence(body),
                BodyValidationPhase::SameParameter => self.probe_same_parameter(body),
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
            self.units_done += 1;
            self.units_total = if self.is_complete() { self.units_done } else { self.units_total.max(self.units_done.saturating_add(1)) };
        }
        self.progress()
    }

    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    fn cheap(&mut self, body: &Body, check: fn(&Body, &mut Vec<ValidationIssue>), next: BodyValidationPhase) {
        check(body, &mut self.issues);
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


// #endregion ⏱️ResumableValidation

// #region 🔖️Tests

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

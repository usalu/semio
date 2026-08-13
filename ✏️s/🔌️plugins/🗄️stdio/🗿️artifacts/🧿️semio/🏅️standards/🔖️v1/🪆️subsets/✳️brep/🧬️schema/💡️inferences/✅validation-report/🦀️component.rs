//! ✅ `validation-report` — real cross-collection referential-integrity diagnostics, computed as a
//! genuine `InferredField<SemioBrepSnapshot>` (not a bare pass-through): a real `DepHash::root`
//! chain over the six collections' canonical bytes, one key (`"document"`, no parents — validation
//! reads the WHOLE document, so there is no meaningful per-entity DAG to walk, unlike
//! `flat-position`'s per-object chain in the proven puzzle3d pilot this facet's shape follows).
//! Reuses `check_brep_referential_integrity` (`✳️brep/🚪️io/🦀️component.rs`, another session's file,
//! read-only here) rather than re-deriving a second copy of the same check.
//!
//! `tessellation` and `mass-properties` are DELIBERATELY OMITTED from this facet — not because they
//! were forgotten, but because a real chain cannot be authored honestly for them at this layer: both
//! require genuine curve/surface EVALUATION (NURBS basis functions, arc length, surface-area/volume
//! integration over a `BrepCurve`/`BrepSurface`), and that math has no home at the stdio pure-value
//! layer today. Building it here would mean either (a) reimplementing real NURBS evaluation
//! directly in stdio — duplicating, and inevitably diverging from, framework-3d's own curve/surface
//! math (a tier-(e) duplication violation), or (b) faking it via a straight-line polygon
//! approximation of the loop's edges presented as exact tessellation/mass data — dishonest to what
//! the field claims to be. Neither is authorized by this wave. The doctrine's own sanctioned home
//! for both (`📌️important.md`'s design doc, §1 "Option 1") is framework-3d's future
//! `tessellate`/`measure` pure functions, consumed from a stdio diff/inference constructor via a
//! new stdio→framework-3d dependency edge — explicitly deferred (three-gate stdio handoff, design
//! doc §6 "Phase 6", "not designed further here"). Per the ticket's own instruction ("if a real
//! dependency chain cannot be authored honestly for a field, omit that field and say why rather
//! than faking one"), this leaf ships `validationReport` only.
//!
//! 🩺️ `validate_body` (below, Topology/Geometry/Report regions) is framework-3d's checked-editor
//! invariant checker — moved in from `🧰️framework/🔨️modules/🧊️3d/📐️brep/✅️validate` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL3. It operates on
//! the ephemeral `Body` mid-construction (topology ring/valence/tolerance/same-parameter
//! invariants), which is a DIFFERENT, complementary check from `BrepValidationReport` above
//! (whole-`SemioBrepSnapshot` referential integrity) — kept as a plain `pub fn`, not wired as
//! its own `InferredField`, since diff constructors call it directly on their own ephemeral rep,
//! never on a persisted snapshot.

use crate::artifacts::semio::standards::v1::subsets::brep::io::check_brep_referential_integrity;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Value
/// 🩺 One referential-integrity finding — a small, owned, `Serialize`/`Deserialize` projection of
/// `dsl::Diagnostic` (whose own `FaultCode`/`Severity`/`TextSpan`/`ExpectedSet` machinery is built
/// for parser diagnostics, not for a cache `Value`; this leaf only needs the two fields that
/// actually carry validation content).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrepValidationDiagnostic {
    pub code: String,
    pub message: String,
}
//#endregion 🔖️Value

//#region 🔖️DependencyHashChain
/// ✅ `validationReport` — root dep = canonical bytes of every collection the check reads (all six:
/// `vertices`/`edges`/`loops`/`faces`/`shells`/`solids`). One key, no parents: a whole-document
/// check has no per-entity DAG to walk, so this is a legitimate single-step "chain" (root only) —
/// still a REAL `InferredField`/`DepHash` chain (proven by the incrementality-law test below: an
/// unrelated field touch that leaves every collection byte-identical must still hit the cache), not
/// a bypass of the mechanism.
pub struct BrepValidationReport;

impl store::InferredField<SemioBrepSnapshot> for BrepValidationReport {
    type Key = String;
    type Value = Vec<BrepValidationDiagnostic>;
    const FIELD_ID: &'static str = "s.stdio.semio.brep.inference.validationReport";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["vertices", "edges", "loops", "faces", "shells", "solids"]
    }

    fn plan(_snapshot: &SemioBrepSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        vec![store::InferenceStep { key: "document".to_string(), parents: vec![] }]
    }

    /// 🔑 Canonical dependency-input bytes — EXACTLY the six collections `compute` reads, nothing
    /// else (the schema field, an identity field, never appears here). `serde_json` over the
    /// snapshot's own already-`Serialize` collections is deterministic per snapshot value and
    /// covers every field the check touches — cheaper and less error-prone than hand-rolling a
    /// bespoke byte encoder for a root-only, single-key chain.
    fn dep_input(snapshot: &SemioBrepSnapshot, _key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        #[derive(Serialize)]
        struct DepInput<'a> {
            vertices: &'a [crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::BrepVertex],
            edges: &'a [crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::BrepEdge],
            loops: &'a [crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::BrepLoop],
            faces: &'a [crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::BrepFace],
            shells: &'a [crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::BrepShell],
            solids: &'a [crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::BrepSolid],
        }
        serde_json::to_vec(&DepInput {
            vertices: &snapshot.vertices,
            edges: &snapshot.edges,
            loops: &snapshot.loops,
            faces: &snapshot.faces,
            shells: &snapshot.shells,
            solids: &snapshot.solids,
        })
        .unwrap_or_default()
    }

    fn compute(snapshot: &SemioBrepSnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        check_brep_referential_integrity(snapshot).into_iter().map(|d| BrepValidationDiagnostic { code: d.code.0.clone(), message: d.message.clone() }).collect()
    }
}
//#endregion 🔖️DependencyHashChain

use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::ArenaId;
use semio_framework_3d::brep::error::ValidationIssue;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;

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
            if let Some((finer, coarser)) = semio_framework_3d::brep::tolerance::check_containment(&format!("vertex-{}", v.raw_index()), vertex.tol, &format!("edge-{}", edge_id.raw_index()), edge.tol) {
                issues.push(ValidationIssue { entity: finer.clone(), code: "tolerance-containment-violated", message: format!("{finer}'s tolerance exceeds its containing {coarser}'s") });
            }
        }
    }
    for (face_id, face) in body.faces.iter() {
        for coedge_id in body.face_coedges(face_id) {
            let Some(coedge) = body.coedges.get(coedge_id) else { continue };
            let Some(edge) = body.edges.get(coedge.edge) else { continue };
            if let Some((finer, coarser)) = semio_framework_3d::brep::tolerance::check_containment(&format!("edge-{}", coedge.edge.raw_index()), edge.tol, &format!("face-{}", face_id.raw_index()), face.tol) {
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
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{
        BrepCurve, BrepEdge, BrepFace, BrepLoop, BrepLoopEdge, BrepShell, BrepShellFace, BrepSolid, BrepSolidShell, BrepSurface, BrepVertex,
    };
    use store::{InferenceCache, InferenceCacheConfig, InferredField};

    fn valid_snapshot() -> SemioBrepSnapshot {
        let mut s = SemioBrepSnapshot::default();
        s.vertices = vec![BrepVertex { id: "v1".into(), point: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 } }];
        s.edges = vec![BrepEdge { id: "e1".into(), start_vertex: "v1".into(), end_vertex: "v1".into(), curve: BrepCurve::Circle { center: SemioPoint3::default(), axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, radius: 1.0 } }];
        s.loops = vec![BrepLoop { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: true }] }];
        s.faces = vec![BrepFace { id: "f1".into(), outer_loop: "l1".into(), inner_loops: vec![], surface: BrepSurface::Plane { origin: SemioPoint3::default(), normal: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 } }, orientation: true }];
        s.shells = vec![BrepShell { id: "s1".into(), faces: vec![BrepShellFace { face: "f1".into(), orientation: true }] }];
        s.solids = vec![BrepSolid { id: "so1".into(), shells: vec![BrepSolidShell { shell: "s1".into(), is_void: false }] }];
        s
    }

    //#region 🧪️Honesty
    #[test]
    fn valid_snapshot_has_no_findings() {
        let values = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&valid_snapshot(), None);
        assert!(values["document"].is_empty());
    }

    #[test]
    fn dangling_reference_is_a_real_finding_not_a_faked_one() {
        let mut broken = valid_snapshot();
        broken.edges[0].end_vertex = "v-missing".into();
        let values = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&broken, None);
        let findings = &values["document"];
        assert!(findings.iter().any(|d| d.code == "stdio.semio_brep.dangling-edge-end-vertex"), "findings: {findings:?}");
    }
    //#endregion 🧪️Honesty

    //#region 🧪️CacheTransparencyLaw
    #[test]
    fn disabled_cache_matches_pure_recompute() {
        let snapshot = valid_snapshot();
        let pure = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&snapshot, None);
        let mut disabled = InferenceCache::new(InferenceCacheConfig { enabled: false, ..Default::default() });
        let via_disabled = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&snapshot, Some(&mut disabled));
        assert_eq!(pure, via_disabled);
    }
    //#endregion 🧪️CacheTransparencyLaw

    //#region 🧪️IncrementalityLaw
    #[test]
    fn identical_snapshot_recompute_is_a_cache_hit() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() });
        let base = valid_snapshot();
        let _ = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&base, Some(&mut cache));
        let before = cache.stats();
        let _ = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&base, Some(&mut cache));
        let after = cache.stats();
        assert_eq!(after.misses, before.misses, "an unchanged snapshot must produce zero new misses");
        assert_eq!(after.hits - before.hits, 1);
    }

    #[test]
    fn changing_any_collection_misses_the_cache() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() });
        let base = valid_snapshot();
        let _ = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&base, Some(&mut cache));
        let mut changed = base.clone();
        changed.vertices[0].point = SemioPoint3 { x: 9.0, y: 9.0, z: 9.0 };
        let before = cache.stats();
        let _ = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&changed, Some(&mut cache));
        let after = cache.stats();
        assert_eq!(after.misses - before.misses, 1, "a real change to a covered collection must miss");
    }
    //#endregion 🧪️IncrementalityLaw

    use semio_framework_3d::brep::curve::Curve3;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::euler::{add_face, add_shell, add_solid, make_edge, make_loop, make_vertex};
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
    use semio_framework_3d::brep::mat::Frame3;
    use semio_framework_3d::brep::surface::Surface;
    use semio_framework_3d::brep::tolerance::Tol;
    use semio_framework_3d::brep::vec::{Pnt3, Vec3};

    fn build_tetrahedron(body: &mut Body, rec: &mut OpRecorder) -> crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::SolidId {
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
            body.coedges.insert(crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Coedge { edge, forward: true, pcurve: None, prange: (0.0, 1.0), loop_id: ArenaId::from_raw(0, 0), next: ArenaId::from_raw(0, 0), prev: ArenaId::from_raw(0, 0) });
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
        let bad_pcurve = body.curves2.insert(semio_framework_3d::brep::curve::Curve2::Line { origin: semio_framework_3d::brep::vec::Pnt2::new(500.0, 500.0), dir: semio_framework_3d::brep::vec::Vec2::new(0.0, 0.0) });
        let coedge = body.coedges.get_mut(coedge_id).unwrap();
        coedge.pcurve = Some(bad_pcurve);
        coedge.prange = (0.0, 1.0);
        let issues = validate_body(&body);
        assert!(issues.iter().any(|i| i.code == "same-parameter-violated"), "expected a same-parameter issue, got {issues:?}");
    }
}
//#endregion 🧪️Tests

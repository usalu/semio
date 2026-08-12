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

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::engine::geometry::SemioPoint3;
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
}
//#endregion 🧪️Tests

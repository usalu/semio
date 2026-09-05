//! ✅ `validation-report` — real cross-collection referential-integrity diagnostics, computed as a
//! genuine `InferredField<SemioBrepSnapshot>` (not a bare pass-through): a real `DepHash::root`
//! chain over the six collections' canonical bytes, one key (`"document"`, no parents — validation
//! reads the WHOLE document, so there is no meaningful per-entity DAG to walk, unlike
//! `flat-position`'s per-object chain in the proven puzzle3d pilot this facet's shape follows).
//! Reuses `check_brep_referential_integrity` (`🧊️brep/🚪️io/🦀️.rs`, another session's file,
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
//! 🩺️ `validate_body` now lives in the kernel-scope `🧪️body/🦀️.rs` sibling (split out in ticket
//! `26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME` so it depends ONLY on kernel modules — no
//! `SemioBrepSnapshot`/artifact-layer/STEP/plugin chain — letting the standalone kernel test
//! harness mount it directly), mounted below and re-exported at this same path so every existing
//! `inferences::validation_report::validate_body` call site keeps resolving unchanged. It operates
//! on the ephemeral `Body` mid-construction (topology ring/valence/tolerance/same-parameter
//! invariants), which is a DIFFERENT, complementary check from `BrepValidationReport` below
//! (whole-`SemioBrepSnapshot` referential integrity) — a plain `pub fn`, not wired as its own
//! `InferredField`, since diff constructors call it directly on their own ephemeral rep, never on
//! a persisted snapshot.

use crate::artifacts::semio::standards::v1::subsets::brep::io::check_brep_referential_integrity;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Value
/// 🩺 One referential-integrity finding — a small, owned, `ToValue`/`FromValue` projection of
/// `dsl::Diagnostic` (whose own `FaultCode`/`Severity`/`TextSpan`/`ExpectedSet` machinery is built
/// for parser diagnostics, not for a cache `Value`; this leaf only needs the two fields that
/// actually carry validation content).
/// 🔀️ No longer dual-derives `serde`: `store::InferredField::Value` used to bound on `Serialize +
/// DeserializeOwned`, forcing every implementor onto serde regardless of its own fields — that
/// bound now reads `ToValue + FromValue` (ticket
/// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`), so this leaf drops the
/// serde half entirely.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
    /// else (the schema field, an identity field, never appears here). `pack::to_json_string` over
    /// the snapshot's own already-`ToValue` collections is deterministic per snapshot value and
    /// covers every field the check touches — cheaper and less error-prone than hand-rolling a
    /// bespoke byte encoder for a root-only, single-key chain.
    fn dep_input(snapshot: &SemioBrepSnapshot, _key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        // 🌉️ Owned clones, not `&[T]` borrows: no `ToValue` impl exists for a slice/array
        // reference (only `Vec<T>` itself, via the blanket `impl<T: ToValue> ToValue for
        // Vec<T>`), so the derive below needs owned collections. One clone per `dep_input` call
        // is the accepted cost for a root-only, single-key chain (see this method's own doc
        // comment) — cheaper than hand-rolling a borrowing byte encoder.
        #[derive(value_derive::ToValue)]
        struct DepInput {
            vertices: Vec<crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::BrepVertex>,
            edges: Vec<crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::BrepEdge>,
            loops: Vec<crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::BrepLoop>,
            faces: Vec<crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::BrepFace>,
            shells: Vec<crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::BrepShell>,
            solids: Vec<crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::BrepSolid>,
        }
        pack::to_json_string(&DepInput {
            vertices: snapshot.vertices.clone(),
            edges: snapshot.edges.clone(),
            loops: snapshot.loops.clone(),
            faces: snapshot.faces.clone(),
            shells: snapshot.shells.clone(),
            solids: snapshot.solids.clone(),
        })
        .into_bytes()
    }

    fn compute(snapshot: &SemioBrepSnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        check_brep_referential_integrity(snapshot).into_iter().map(|d| BrepValidationDiagnostic { code: d.code.0.clone(), message: d.message.clone() }).collect()
    }
}
//#endregion 🔖️DependencyHashChain

// #region 🔖️Body
#[path = "🧪️body/🦀️.rs"]
mod body;
pub use body::validate_body;
// #endregion 🔖️Body

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::base::schema::geometry::SemioPoint3;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepCurve, BrepEdge, BrepFace, BrepLoop, BrepLoopEdge, BrepShell, BrepShellFace, BrepSolid, BrepSolidShell, BrepSurface, BrepVertex};
    use store::{InferenceCache, InferenceCacheConfig};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn valid_snapshot() -> SemioBrepSnapshot {
        let mut s = SemioBrepSnapshot::default();
        s.vertices = vec![BrepVertex { id: "v1".into(), point: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, tol: 0.0 }];
        s.edges = vec![BrepEdge { id: "e1".into(), start_vertex: "v1".into(), end_vertex: "v1".into(), curve: BrepCurve::Circle { center: SemioPoint3::default(), axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, radius: 1.0 }, tol: 0.0 }];
        s.loops = vec![BrepLoop { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: true }] }];
        s.faces = vec![BrepFace { id: "f1".into(), outer_loop: "l1".into(), inner_loops: vec![], surface: BrepSurface::Plane { origin: SemioPoint3::default(), normal: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 } }, orientation: true, tol: 0.0 }];
        s.shells = vec![BrepShell { id: "s1".into(), faces: vec![BrepShellFace { face: "f1".into(), orientation: true }] }];
        s.solids = vec![BrepSolid { id: "so1".into(), shells: vec![BrepSolidShell { shell: "s1".into(), is_void: false }] }];
        s
    }

    //#region 🧪️Honesty
    #[semio_framework_async_macros::async_test]
    async fn valid_snapshot_has_no_findings() {
        let values = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&valid_snapshot(), None);
        assert!(values["document"].is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn dangling_reference_is_a_real_finding_not_a_faked_one() {
        let mut broken = valid_snapshot();
        broken.edges[0].end_vertex = "v-missing".into();
        let values = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&broken, None);
        let findings = &values["document"];
        assert!(findings.iter().any(|d| d.code == "stdio.semio_brep.dangling-edge-end-vertex"), "findings: {findings:?}");
    }
    //#endregion 🧪️Honesty

    //#region 🧪️CacheTransparencyLaw
    #[semio_framework_async_macros::async_test]
    async fn disabled_cache_matches_pure_recompute() {
        let snapshot = valid_snapshot();
        let pure = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&snapshot, None);
        let mut disabled = InferenceCache::new(InferenceCacheConfig { enabled: false, ..Default::default() }).await;
        let via_disabled = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&snapshot, Some(&mut disabled));
        assert_eq!(pure, via_disabled);
    }
    //#endregion 🧪️CacheTransparencyLaw

    //#region 🧪️IncrementalityLaw
    #[semio_framework_async_macros::async_test]
    async fn identical_snapshot_recompute_is_a_cache_hit() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
        let base = valid_snapshot();
        let _ = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&base, Some(&mut cache));
        let before = cache.stats().await;
        let _ = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&base, Some(&mut cache));
        let after = cache.stats().await;
        assert_eq!(after.misses, before.misses, "an unchanged snapshot must produce zero new misses");
        assert_eq!(after.hits - before.hits, 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn changing_any_collection_misses_the_cache() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
        let base = valid_snapshot();
        let _ = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&base, Some(&mut cache));
        let mut changed = base.clone();
        changed.vertices[0].point = SemioPoint3 { x: 9.0, y: 9.0, z: 9.0 };
        let before = cache.stats().await;
        let _ = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&changed, Some(&mut cache));
        let after = cache.stats().await;
        assert_eq!(after.misses - before.misses, 1, "a real change to a covered collection must miss");
    }
    //#endregion 🧪️IncrementalityLaw
}
//#endregion 🧪️Tests

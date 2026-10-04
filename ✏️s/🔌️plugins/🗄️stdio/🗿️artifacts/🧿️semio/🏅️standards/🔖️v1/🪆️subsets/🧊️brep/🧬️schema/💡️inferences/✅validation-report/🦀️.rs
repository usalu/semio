//! ✅️ Semio document referential-integrity inference over canonical collection dependencies.
//! Pure computational geometry validation belongs to `semio_framework_3d::brep::queries::validation`.

use crate::standards::v1::subsets::brep::io::check_brep_referential_integrity;
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Value
/// 🩺 One referential-integrity finding — a small, owned, `ToValue`/`FromValue` projection of
/// `semio_framework_diagnostic::Diagnostic` (whose own `FaultCode`/`Severity`/`TextSpan`/`ExpectedSet` machinery is built
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
            vertices: Vec<crate::standards::v1::subsets::brep::schema::snapshot::BrepVertex>,
            edges: Vec<crate::standards::v1::subsets::brep::schema::snapshot::BrepEdge>,
            loops: Vec<crate::standards::v1::subsets::brep::schema::snapshot::BrepLoop>,
            faces: Vec<crate::standards::v1::subsets::brep::schema::snapshot::BrepFace>,
            shells: Vec<crate::standards::v1::subsets::brep::schema::snapshot::BrepShell>,
            solids: Vec<crate::standards::v1::subsets::brep::schema::snapshot::BrepSolid>,
        }
        semio_framework_pack_json::to_json_string(&DepInput { vertices: snapshot.vertices.clone(), edges: snapshot.edges.clone(), loops: snapshot.loops.clone(), faces: snapshot.faces.clone(), shells: snapshot.shells.clone(), solids: snapshot.solids.clone() })
            .into_bytes()
    }

    fn compute(snapshot: &SemioBrepSnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        check_brep_referential_integrity(snapshot).into_iter().map(|d| BrepValidationDiagnostic { code: d.code.0.clone(), message: d.message }).collect()
    }
}
//#endregion 🔖️DependencyHashChain

// #region 🔖️Body

// #endregion 🔖️Body

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

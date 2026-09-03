//! 💡️ SemioBrepInference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING, authored here
//! by DKM per the standing exclusion: IIF's inference fan-out explicitly excludes `✳️brep`/
//! `✳️drawing`/`✳️mesh` and defers them). Directory shape mirrors `🧬️mutations/`: this file is the
//! family-root assembly (never mod's/includes the slug dirs directly — `🦀️.rs` is the sole
//! mounting mechanism, same as mutations); each named inference gets its own `<emoji><slug>/` child
//! (currently: `✅validation-report/`).
//!
//! 🔓 `tessellation`/`massProperties` landed here in ticket 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-
//! RUNTIME wave W3-A — the `✅validation-report/🦀️.rs` rationale for omitting them ("real curve/
//! surface evaluation math has no honest home at this layer yet") no longer holds: W1-A moved the
//! kernel's neutral contract types (`MeshTransfer`, `FaceGroup`, …) into this same subset's own
//! `⚙️engine/🔖️contract`, and `📸️snapshot/🔁️body/🦀️.rs` (this same wave) gives this layer a real,
//! lossless `SemioBrepSnapshot → Body` bridge — so `tessellate_solid`/`solid_mass_properties`
//! (already real, kernel-scope functions) are now reachable from here honestly, no
//! straight-line-approximation shim.

use crate::artifacts::semio::standards::v1::subsets::base::schema::geometry::SemioPoint3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::engine::contract::MeshTransfer;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::mass_properties::solid_mass_properties;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::tessellation::tessellate_solid;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::validation_report::{BrepValidationDiagnostic, BrepValidationReport};

//#region 🔖️Tessellation
/// 💡️ Deflection (chordal error tolerance, model units) `tessellation`/`massProperties` compute
/// with — a fixed default rather than a per-document config field, since neither inference has a
/// parameter input today; documented here as the "inference parameter with a default" this
/// wave's brief calls for.
pub const BREP_INFERENCE_DEFAULT_DEFLECTION: f64 = 0.1;

/// 🧩️ Tessellates every solid in `body` and merges them into ONE [`MeshTransfer`] (index/vertex
/// offsets adjusted per solid, `face_groups`/`edge_groups` keyed by each entity's own
/// [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::PersistentLabel`]
/// so a picked triangle/segment still resolves to the right face/edge regardless of which solid it
/// came from) — the inference field is a single `MeshTransfer` (not one per solid), so a
/// multi-solid document's whole tessellated scene is exactly one dependency-hash-chained value.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn tessellate_document(body: &Body, deflection: f64) -> MeshTransfer {
    let mut merged = MeshTransfer::default();
    for solid_id in body.solids.ids() {
        let Ok(mesh) = tessellate_solid(body, solid_id, deflection) else { continue };
        let vertex_offset = (merged.position.len() / 3) as u32;
        let index_offset = merged.index.len() as u32;
        let edge_segment_offset = (merged.edges.len() / 6) as u32;
        merged.position.extend(mesh.position);
        merged.normal.extend(mesh.normal);
        merged.index.extend(mesh.index.into_iter().map(|i| i + vertex_offset));
        merged.edges.extend(mesh.edges);
        merged.points.extend(mesh.points);
        merged.face_groups.extend(mesh.face_groups.into_iter().map(|g| crate::artifacts::semio::standards::v1::subsets::brep::schema::engine::contract::FaceGroup { start: g.start + index_offset, count: g.count, entity_id: g.entity_id }));
        merged.edge_groups.extend(mesh.edge_groups.into_iter().map(|g| crate::artifacts::semio::standards::v1::subsets::brep::schema::engine::contract::EdgeGroup { start: g.start + edge_segment_offset, count: g.count, entity_id: g.entity_id }));
        merged.face_infos.extend(mesh.face_infos);
        merged.edge_infos.extend(mesh.edge_infos);
    }
    merged
}

/// 📏️ `mass_properties`'s value: `MassProperties` minus the inertia tensor (not asked for by this
/// wave's brief) — summed across every solid in the document (a document's total volume/area,
/// centroid mass-weighted by each solid's own volume; `error_estimate` summed as an upper bound).
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BrepMassProperties {
    pub volume: f64,
    pub area: f64,
    pub centroid: SemioPoint3,
    pub error_estimate: f64,
}

/// 📏️ Mass properties over the whole document — see [`BrepMassProperties`]'s own doc comment for
/// the multi-solid aggregation rule.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn document_mass_properties(body: &Body, tol: f64) -> BrepMassProperties {
    let mut total_volume = 0.0;
    let mut total_area = 0.0;
    let (mut wx, mut wy, mut wz) = (0.0, 0.0, 0.0);
    let mut error_estimate = 0.0;
    for solid_id in body.solids.ids() {
        let Ok(props) = solid_mass_properties(body, solid_id, tol) else { continue };
        total_volume += props.volume;
        total_area += props.area;
        let weight = props.volume.abs();
        wx += props.centroid.x * weight;
        wy += props.centroid.y * weight;
        wz += props.centroid.z * weight;
        error_estimate += props.error_estimate;
    }
    let centroid = if total_volume.abs() > 1e-12 { SemioPoint3 { x: wx / total_volume.abs(), y: wy / total_volume.abs(), z: wz / total_volume.abs() } } else { SemioPoint3 { x: wx, y: wy, z: wz } };
    BrepMassProperties { volume: total_volume, area: total_area, centroid, error_estimate }
}
//#endregion 🔖️Tessellation

//#region 🔖️Inference
/// 💡️ Everything inferable from a brep snapshot. One field per named inference under
/// `💡️inferences/` (`validationReport` backed by `✅validation-report/`; `tessellation`/
/// `massProperties` computed in this file directly, see the module doc comment for why they now
/// have an honest home here).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.brep.inference")]
pub struct SemioBrepInference {
    #[derived]
    pub validation_report: Vec<BrepValidationDiagnostic>,
    #[derived]
    pub tessellation: MeshTransfer,
    #[derived]
    pub mass_properties: BrepMassProperties,
}

impl protocol::Inference<SemioBrepSnapshot> for SemioBrepInference {
    fn infer(snapshot: &SemioBrepSnapshot) -> Self {
        let validation_report = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(snapshot, None).remove("document").unwrap_or_default();
        let (tessellation, mass_properties) = match Body::from_snapshot(snapshot) {
            Ok(body) => (tessellate_document(&body, BREP_INFERENCE_DEFAULT_DEFLECTION), document_mass_properties(&body, BREP_INFERENCE_DEFAULT_DEFLECTION)),
            Err(_) => (MeshTransfer::default(), BrepMassProperties::default()),
        };
        Self { validation_report, tessellation, mass_properties }
    }
}

impl protocol::InferenceSpec<SemioBrepSnapshot> for SemioBrepInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.semio.brep.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[
            protocol::InferenceFieldSpec { id: "s.stdio.semio.brep.inference.validationReport", reads: &["vertices", "edges", "loops", "faces", "shells", "solids"] },
            protocol::InferenceFieldSpec { id: "s.stdio.semio.brep.inference.tessellation", reads: &["vertices", "edges", "loops", "faces", "shells", "solids", "coedges", "nextLabel"] },
            protocol::InferenceFieldSpec { id: "s.stdio.semio.brep.inference.massProperties", reads: &["vertices", "edges", "loops", "faces", "shells", "solids", "coedges", "nextLabel"] },
        ]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::semio::standards::v1::subsets::brep::schema::SemioBrepBuilder {
    type Snapshot = SemioBrepSnapshot;
    type Inference = SemioBrepInference;

    async fn infer_cached(snapshot: &Self::Snapshot, cache: &mut store::InferenceCache, session: &mut store::InferenceSession) -> Self::Inference {
        let _ = session;
        let _ = cache;
        // 🔓 `tessellation`/`massProperties` are computed straight (not yet threaded through the
        // `InferredField`/`DepHash` cache — that needs a real per-entity chain analogous to
        // `BrepValidationReport`'s, deferred: unlike a flat referential-integrity walk, tessellation
        // itself doesn't naturally decompose into a small per-key DAG without duplicating
        // `tessellate_solid`'s own face/edge iteration here). `protocol::Inference::infer` (the pure
        // path) computes the exact same values, so this is honest-but-uncached, never a fake.
        <SemioBrepInference as protocol::Inference<SemioBrepSnapshot>>::infer(snapshot)
    }
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.brep.inference`'s facet leaves into the OS-wide inference catalog.
/// The `register_artifact_inferences()` call site itself lives in the SHARED
/// `🏅️standards/🔖️v1/⚙️engine/🦀️.rs` (aggregates all 14 `s.stdio.semio.*` subsets'
/// `register()` calls) — out of this ticket's `✳️brep/`-only edit scope, same boundary
/// `✳️brep/🚪️io/🦀️.rs`'s own conformance-law doc comment already notes for the composer
/// registration. Flagged under `## sharedFileRequests` in the wave report, not wired here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_brep_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.brep.inference",
        inference: schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use protocol::Inference;

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = SemioBrepSnapshot::default();
        assert_eq!(SemioBrepInference::infer(&snapshot), SemioBrepInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(SemioBrepInference::infer(&SemioBrepSnapshot::default()), SemioBrepInference::default());
    }
}
//#endregion 🧪️Tests
